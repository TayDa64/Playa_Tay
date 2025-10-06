//! Error types for the downloader.

use thiserror::Error;

/// Result type for downloader operations.
pub type Result<T> = std::result::Result<T, DownloadError>;

/// Errors that can occur during download operations.
#[derive(Debug, Error)]
pub enum DownloadError {
    /// HTTP request failed with the given status code
    #[error("HTTP request failed with status {0}")]
    HttpError(u16),

    /// Network error occurred
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// File I/O error occurred
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Checksum verification failed
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    /// Download timed out
    #[error("Download timed out")]
    Timeout,

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
