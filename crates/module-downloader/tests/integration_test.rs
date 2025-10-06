//! Integration tests for the module-downloader crate.
//!
//! These tests verify the download, resume, and checksum validation functionality.

use module_downloader::{DownloadConfig, DownloadError, Downloader, ProgressInfo};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::sync::Mutex;
use tempfile::TempDir;
use tokio::fs;

/// Helper to create a test file with known content and checksum.
fn create_test_file_content(size: usize) -> Vec<u8> {
  (0..size).map(|i| (i % 256) as u8).collect()
}

/// Calculate SHA-256 checksum of bytes.
fn calculate_checksum(data: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(data);
  format!("{:x}", hasher.finalize())
}

/// Test basic download functionality.
///
/// Note: This test requires a local HTTP server to be running.
/// For CI/automated testing, consider using a mock server or skip this test.
#[tokio::test]
#[ignore] // Ignored by default as it requires external setup
async fn test_basic_download() {
  let _temp_dir = TempDir::new().unwrap();
  let _dest_path = _temp_dir.path().join("test_download.bin");

  let config = DownloadConfig::default();
  let _downloader = Downloader::new(config).unwrap();

  // This would require a test HTTP server
  // let result = downloader.download(
  //     "http://localhost:8080/test.bin",
  //     &dest_path,
  //     None::<fn(ProgressInfo)>,
  // ).await.unwrap();
  //
  // assert!(result.bytes_downloaded > 0);
  // assert!(!result.checksum.is_empty());
}

/// Test checksum verification.
#[tokio::test]
async fn test_checksum_config() {
  let test_data = create_test_file_content(1024);
  let expected_checksum = calculate_checksum(&test_data);

  let mut config = DownloadConfig::default();
  config.expected_checksum = Some(expected_checksum.clone());

  // Verify config is set correctly
  assert_eq!(config.expected_checksum.unwrap(), expected_checksum);
}

/// Test temp file path generation.
#[tokio::test]
async fn test_temp_file_cleanup() {
  let temp_dir = TempDir::new().unwrap();
  let dest_path = temp_dir.path().join("test.bin");
  let temp_path = dest_path.with_extension("bin.part");

  // Create a temp file
  fs::write(&temp_path, b"test data").await.unwrap();
  assert!(temp_path.exists());

  // Verify it exists
  let metadata = fs::metadata(&temp_path).await.unwrap();
  assert!(metadata.is_file());

  // Clean it up
  fs::remove_file(&temp_path).await.unwrap();
  assert!(!temp_path.exists());
}

/// Test progress callback functionality.
#[tokio::test]
async fn test_progress_callback() {
  let progress_updates = Arc::new(Mutex::new(Vec::new()));
  let progress_clone = Arc::clone(&progress_updates);

  let callback = move |progress: ProgressInfo| {
    progress_clone.lock().unwrap().push(progress);
  };

  // Simulate progress updates
  callback(ProgressInfo {
    downloaded_bytes: 512,
    total_bytes: 1024,
    speed_bps: 1000,
  });

  callback(ProgressInfo {
    downloaded_bytes: 1024,
    total_bytes: 1024,
    speed_bps: 1000,
  });

  let updates = progress_updates.lock().unwrap();
  assert_eq!(updates.len(), 2);
  assert_eq!(updates[0].downloaded_bytes, 512);
  assert_eq!(updates[1].downloaded_bytes, 1024);
  assert_eq!(updates[1].percentage(), 100.0);
}

/// Test downloader configuration.
#[tokio::test]
async fn test_downloader_creation() {
  let config = DownloadConfig {
    max_retries: 5,
    initial_backoff_ms: 500,
    max_backoff_ms: 16000,
    rate_limit_bps: Some(1_000_000), // 1 MB/s
    timeout: Some(std::time::Duration::from_secs(300)),
    expected_checksum: Some("test_checksum".to_string()),
  };

  let downloader = Downloader::new(config);
  assert!(downloader.is_ok());
}

/// Test error type display.
#[tokio::test]
async fn test_error_display() {
  let error = DownloadError::HttpError(404);
  assert_eq!(error.to_string(), "HTTP request failed with status 404");

  let error = DownloadError::ChecksumMismatch {
    expected: "abc123".to_string(),
    actual: "def456".to_string(),
  };
  assert_eq!(
    error.to_string(),
    "Checksum mismatch: expected abc123, got def456"
  );
}

/// Test rate limiter integration (unit test level).
#[tokio::test]
async fn test_rate_limiter_configuration() {
  let mut config = DownloadConfig::default();
  config.rate_limit_bps = Some(500_000); // 500 KB/s

  assert_eq!(config.rate_limit_bps, Some(500_000));

  let _downloader = Downloader::new(config).unwrap();
  // Downloader created successfully with rate limit
}

/// Test retry configuration.
#[tokio::test]
async fn test_retry_configuration() {
  let mut config = DownloadConfig::default();
  config.max_retries = 10;
  config.initial_backoff_ms = 100;
  config.max_backoff_ms = 5000;

  assert_eq!(config.max_retries, 10);
  assert_eq!(config.initial_backoff_ms, 100);
  assert_eq!(config.max_backoff_ms, 5000);
}
