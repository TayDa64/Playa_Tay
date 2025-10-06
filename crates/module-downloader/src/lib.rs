//! HTTP downloader with resume capability and integrity verification.
//!
//! This crate provides a robust HTTP downloader that supports:
//! - Resumable downloads using HTTP range requests
//! - SHA-256 checksum verification
//! - Rate limiting
//! - Retry with exponential backoff
//! - Progress callbacks
//! - Automatic cleanup of temporary files

use futures::StreamExt;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tracing::{debug, info, warn};

mod error;
mod progress;
mod rate_limiter;

pub use error::{DownloadError, Result};
pub use progress::{ProgressCallback, ProgressInfo};
pub use rate_limiter::RateLimiter;

/// Configuration for a download operation.
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Initial backoff duration in milliseconds
    pub initial_backoff_ms: u64,
    /// Maximum backoff duration in milliseconds
    pub max_backoff_ms: u64,
    /// Rate limit in bytes per second (None = unlimited)
    pub rate_limit_bps: Option<u64>,
    /// Timeout for the entire download operation
    pub timeout: Option<Duration>,
    /// Expected SHA-256 checksum (hex string)
    pub expected_checksum: Option<String>,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 32000,
            rate_limit_bps: None,
            timeout: Some(Duration::from_secs(3600)), // 1 hour default
            expected_checksum: None,
        }
    }
}

/// Result of a successful download operation.
#[derive(Debug, Clone)]
pub struct DownloadResult {
    /// Final path where the file was saved
    pub path: PathBuf,
    /// Total bytes downloaded
    pub bytes_downloaded: u64,
    /// SHA-256 checksum of the downloaded file
    pub checksum: String,
    /// Number of retry attempts used
    pub retries_used: usize,
    /// Whether the download was resumed
    pub was_resumed: bool,
}

/// HTTP downloader with resume and integrity verification.
pub struct Downloader {
    client: reqwest::Client,
    config: DownloadConfig,
}

impl Downloader {
    /// Create a new downloader with the given configuration.
    pub fn new(config: DownloadConfig) -> Result<Self> {
        let mut client_builder = reqwest::Client::builder();
        
        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(timeout);
        }
        
        let client = client_builder.build()?;
        
        Ok(Self { client, config })
    }

    /// Create a new downloader with default configuration.
    pub fn new_default() -> Result<Self> {
        Self::new(DownloadConfig::default())
    }

    /// Download a file from the given URL to the specified destination.
    ///
    /// If the destination file already exists partially, the download will
    /// attempt to resume from where it left off.
    pub async fn download<F>(
        &self,
        url: &str,
        dest: &Path,
        progress_callback: Option<F>,
    ) -> Result<DownloadResult>
    where
        F: Fn(ProgressInfo) + Send + Sync,
    {
        let temp_path = self.get_temp_path(dest);
        let mut retries = 0;
        let mut backoff_ms = self.config.initial_backoff_ms;

        loop {
            match self
                .try_download(url, dest, &temp_path, progress_callback.as_ref())
                .await
            {
                Ok(result) => return Ok(result),
                Err(e) if retries < self.config.max_retries => {
                    retries += 1;
                    warn!(
                        "Download attempt {} failed: {}. Retrying in {}ms...",
                        retries, e, backoff_ms
                    );
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    backoff_ms = (backoff_ms * 2).min(self.config.max_backoff_ms);
                }
                Err(e) => {
                    self.cleanup_temp_file(&temp_path).await;
                    return Err(e);
                }
            }
        }
    }

    async fn try_download<F>(
        &self,
        url: &str,
        dest: &Path,
        temp_path: &Path,
        progress_callback: Option<&F>,
    ) -> Result<DownloadResult>
    where
        F: Fn(ProgressInfo) + Send + Sync,
    {
        // Check if we can resume
        let existing_size = self.get_existing_file_size(temp_path).await?;
        let was_resumed = existing_size > 0;

        if was_resumed {
            info!("Resuming download from byte {}", existing_size);
        }

        // Build request with range header if resuming
        let mut request = self.client.get(url);
        if existing_size > 0 {
            request = request.header("Range", format!("bytes={}-", existing_size));
        }

        // Execute request
        let response = request.send().await?;
        
        // Check if server supports resume
        if was_resumed && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            warn!("Server does not support resume, starting from beginning");
            // Delete the partial file and start over
            let _ = tokio::fs::remove_file(temp_path).await;
            // Retry by calling the outer download method which will handle the recursion properly
            // For now, just fail and let the retry logic in download() handle it
            return Err(DownloadError::InvalidConfig(
                "Server does not support resume".to_string(),
            ));
        }

        if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(DownloadError::HttpError(response.status().as_u16()));
        }

        // Get content length
        let content_length = response.content_length().unwrap_or(0);
        let total_size = if was_resumed {
            existing_size + content_length
        } else {
            content_length
        };

        debug!(
            "Starting download: {} bytes (resumed: {})",
            total_size, was_resumed
        );

        // Open file for writing (append if resuming)
        let file = OpenOptions::new()
            .create(true)
            .append(was_resumed)
            .write(!was_resumed)
            .open(temp_path)
            .await?;

        let mut writer = BufWriter::new(file);
        let mut stream = response.bytes_stream();
        let mut hasher = Sha256::new();
        let mut downloaded = existing_size;
        
        // If resuming, we need to hash the existing content
        if was_resumed {
            let existing_content = tokio::fs::read(temp_path).await?;
            hasher.update(&existing_content);
        }

        let rate_limiter = self.config.rate_limit_bps.map(RateLimiter::new);

        // Download and stream to file
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            
            // Apply rate limiting if configured
            if let Some(ref limiter) = rate_limiter {
                limiter.wait_for_capacity(chunk.len()).await;
            }

            hasher.update(&chunk);
            writer.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            // Call progress callback
            if let Some(callback) = progress_callback {
                callback(ProgressInfo {
                    downloaded_bytes: downloaded,
                    total_bytes: total_size,
                    speed_bps: 0, // TODO: Calculate actual speed
                });
            }
        }

        writer.flush().await?;
        drop(writer);

        // Verify checksum if provided
        let checksum = format!("{:x}", hasher.finalize());
        if let Some(expected) = &self.config.expected_checksum {
            if checksum != *expected {
                return Err(DownloadError::ChecksumMismatch {
                    expected: expected.clone(),
                    actual: checksum,
                });
            }
        }

        // Move from temp to final destination
        tokio::fs::rename(temp_path, dest).await?;

        info!("Download completed successfully: {:?}", dest);

        Ok(DownloadResult {
            path: dest.to_path_buf(),
            bytes_downloaded: downloaded,
            checksum,
            retries_used: 0, // This will be tracked by the caller
            was_resumed,
        })
    }

    async fn get_existing_file_size(&self, path: &Path) -> Result<u64> {
        match tokio::fs::metadata(path).await {
            Ok(metadata) => Ok(metadata.len()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(e) => Err(e.into()),
        }
    }

    fn get_temp_path(&self, dest: &Path) -> PathBuf {
        let mut temp = dest.as_os_str().to_owned();
        temp.push(".part");
        PathBuf::from(temp)
    }

    async fn cleanup_temp_file(&self, temp_path: &Path) {
        if let Err(e) = tokio::fs::remove_file(temp_path).await {
            if e.kind() != std::io::ErrorKind::NotFound {
                warn!("Failed to cleanup temp file {:?}: {}", temp_path, e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_config_default() {
        let config = DownloadConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff_ms, 1000);
        assert!(config.timeout.is_some());
    }

    #[test]
    fn test_temp_path_generation() {
        let downloader = Downloader::new_default().unwrap();
        let dest = PathBuf::from("/tmp/test.zip");
        let temp = downloader.get_temp_path(&dest);
        assert_eq!(temp, PathBuf::from("/tmp/test.zip.part"));
    }
}
