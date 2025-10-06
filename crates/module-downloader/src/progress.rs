//! Progress tracking for downloads.

/// Progress information for a download operation.
#[derive(Debug, Clone, Copy)]
pub struct ProgressInfo {
  /// Number of bytes downloaded so far
  pub downloaded_bytes: u64,
  /// Total number of bytes to download
  pub total_bytes: u64,
  /// Current download speed in bytes per second
  pub speed_bps: u64,
}

impl ProgressInfo {
  /// Calculate the progress percentage (0-100).
  pub fn percentage(&self) -> f64 {
    if self.total_bytes == 0 {
      0.0
    } else {
      (self.downloaded_bytes as f64 / self.total_bytes as f64) * 100.0
    }
  }
}

/// Type alias for progress callbacks.
pub type ProgressCallback = Box<dyn Fn(ProgressInfo) + Send + Sync>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_progress_percentage() {
    let progress = ProgressInfo {
      downloaded_bytes: 50,
      total_bytes: 100,
      speed_bps: 1024,
    };
    assert_eq!(progress.percentage(), 50.0);
  }

  #[test]
  fn test_progress_percentage_zero_total() {
    let progress = ProgressInfo {
      downloaded_bytes: 0,
      total_bytes: 0,
      speed_bps: 0,
    };
    assert_eq!(progress.percentage(), 0.0);
  }
}
