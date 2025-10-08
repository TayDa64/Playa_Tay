// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Integration tests for selective Electron integration (Pattern A)

#[cfg(test)]
mod tests {
  use api_lib::{ensure_electron_sidecar, is_electron_available, open_electron_feature};

  /// Test that open_electron_feature returns appropriate error when Electron is not installed
  #[tokio::test]
  async fn test_open_electron_feature_not_installed() {
    // Check availability first
    let available = is_electron_available().await.unwrap_or(false);

    if !available {
      // If not available, open_electron_feature should return not_installed error
      let result = open_electron_feature("https://example.com".to_string()).await;
      assert!(
        result.is_err(),
        "Should return error when Electron not installed"
      );

      let error = result.unwrap_err();
      assert!(
        error.contains("not_installed"),
        "Error should contain 'not_installed' code, got: {}",
        error
      );
    } else {
      println!("Skipping not_installed test: Electron is available");
    }
  }

  /// Test that is_electron_available returns boolean without error
  #[tokio::test]
  async fn test_is_electron_available() {
    let result = is_electron_available().await;
    assert!(result.is_ok(), "is_electron_available should not error");

    let available = result.unwrap();
    println!("Electron available: {}", available);
  }

  /// Test that ensure_electron_sidecar returns appropriate result
  #[tokio::test]
  async fn test_ensure_electron_sidecar() {
    let result = ensure_electron_sidecar().await;

    match result {
      Ok(_) => println!("Electron sidecar is available"),
      Err(e) => {
        assert!(
          e.contains("not_installed"),
          "Error should contain 'not_installed', got: {}",
          e
        );
        println!("Electron sidecar not available (expected in CI)");
      }
    }
  }

  /// Test that open_electron_feature succeeds when Electron is available
  /// Note: This test will be skipped in headless CI environments
  #[tokio::test]
  async fn test_open_electron_feature_success() {
    let available = is_electron_available().await.unwrap_or(false);

    if available {
      // Only test spawning if DISPLAY is set (not headless)
      if std::env::var("DISPLAY").is_ok()
        || cfg!(target_os = "windows")
        || cfg!(target_os = "macos")
      {
        let result = open_electron_feature("https://example.com".to_string()).await;

        // Should either succeed or fail with spawn_error (but not not_installed)
        if let Err(e) = result {
          assert!(
            !e.contains("not_installed"),
            "Should not return not_installed when Electron is available"
          );
          println!("Spawn result: {}", e);
        }
      } else {
        println!("Skipping spawn test: headless environment (no DISPLAY)");
      }
    } else {
      println!("Skipping spawn test: Electron not available");
    }
  }
}
