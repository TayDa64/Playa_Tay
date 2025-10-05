// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use tauri::{
  command,
  ipc::{Channel, CommandScope},
};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[derive(Debug, Deserialize)]
pub struct LogScope {
  event: String,
}

#[command]
pub fn log_operation(
  event: String,
  payload: Option<String>,
  command_scope: CommandScope<LogScope>,
) -> Result<(), &'static str> {
  if command_scope.denies().iter().any(|s| s.event == event) {
    Err("denied")
  } else if !command_scope.allows().iter().any(|s| s.event == event) {
    Err("not allowed")
  } else {
    log::info!("{event} {payload:?}");
    Ok(())
  }
}

#[derive(Serialize)]
pub struct ApiResponse {
  message: String,
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> ApiResponse {
  println!("{endpoint} {body:?}");
  ApiResponse {
    message: "message response".into(),
  }
}

#[command]
pub fn echo(request: tauri::ipc::Request<'_>) -> tauri::ipc::Response {
  tauri::ipc::Response::new(request.body().clone())
}

#[command]
pub fn spam(channel: Channel<i32>) -> tauri::Result<()> {
  for i in 1..=1_000 {
    channel.send(i)?;
  }
  Ok(())
}

/// Error codes for Electron feature operations.
/// These codes form a stable contract with the frontend.
#[derive(Debug, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum ElectronError {
  /// Electron runtime is not installed or not found in expected paths
  #[serde(rename = "not_installed")]
  NotInstalled(String),
  /// Failed to spawn the Electron process
  #[serde(rename = "spawn_error")]
  SpawnError(String),
}

impl std::fmt::Display for ElectronError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ElectronError::NotInstalled(msg) => write!(f, "not_installed: {}", msg),
      ElectronError::SpawnError(msg) => write!(f, "spawn_error: {}", msg),
    }
  }
}

impl From<ElectronError> for String {
  fn from(err: ElectronError) -> String {
    err.to_string()
  }
}

/// Launch the minimal Electron sidecar (Pattern A) to load a given URL.
///
/// # Security notes
/// - Uses a locally installed Electron binary under packages/electron-shell/node_modules/.bin
/// - Passes a per-session auth token via environment variable for optional coordination
/// - Renderer is sandboxed; no Node integration
///
/// # Error codes
/// - `not_installed`: Electron binary not found
/// - `spawn_error`: Failed to start Electron process
#[command]
#[allow(unused_variables)]
pub async fn launch_electron(url: String) -> Result<(), String> {
  use rand::{distributions::Alphanumeric, Rng};
  use std::process::Command;

  // Generate an ephemeral auth token (if the sidecar chooses to use it)
  let auth_token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .map(char::from)
    .collect();

  // In dev, the current dir is typically examples/api. The sidecar lives at ../../packages/electron-shell
  let (electron_pkg_dir, electron_bin) = detect_electron_binary();
  if electron_bin.is_none() {
    return Err(ElectronError::NotInstalled(
      "Electron binary not found. Run 'pnpm -F @playa/electron-shell install' in dev.".to_string()
    ).into());
  }
  let electron_pkg_dir = electron_pkg_dir.expect("pkg dir");
  let electron_bin = electron_bin.expect("bin path");

  let mut cmd = Command::new(&electron_bin);
  cmd.current_dir(&electron_pkg_dir)
    .env("ELECTRON_TARGET_URL", &url)
    .env("PLAYA_AUTH_TOKEN", &auth_token)
    .env("ELECTRON_ENABLE_SECURITY_WARNINGS", "true")
    .env("NODE_OPTIONS", "--no-experimental-fetch");

  // Spawn without waiting; Electron will run until closed by the user
  match cmd.spawn() {
    Ok(_child) => Ok(()),
    Err(e) => Err(ElectronError::SpawnError(format!("Failed to spawn electron: {e}")).into()),
  }
}

/// Stable contract for the UI: open a feature that may require Electron.
///
/// # Error codes (returned as String for Tauri IPC compatibility)
/// - `not_installed: <msg>`: Electron runtime not available
/// - `spawn_error: <msg>`: Failed to launch Electron process
///
/// # Pattern A vs B
/// - Pattern A (current): spawns sidecar from workspace package (dev) or bundled resource (prod)
/// - Pattern B (future): detects/installs optional Electron module with signed updates

#[command]
pub async fn open_electron_feature(url: String) -> Result<(), String> {
  // Pattern B detection: check if optional module is installed
  // TODO: Implement detect_electron_module() for Pattern B
  // See: packages/electron-drm-shell/README.md for implementation plan

  // if let Some(module_path) = detect_electron_module() {
  //   return launch_electron_module(url, module_path).await;
  // }

  // Fallback to Pattern A (bundled sidecar)
  launch_electron(url).await
}

// TODO: Pattern B helpers (post-v1 implementation)
// fn detect_electron_module() -> Option<PathBuf> {
//   // 1. Read app config: ~/.playa/modules.json
//   // 2. Check if "electron-drm-shell" entry exists
//   // 3. Verify binary path exists and signature valid
//   // 4. Check version compatibility
//   // 5. Return Some(path) or None
// }
//
// async fn launch_electron_module(url: String, module_path: PathBuf) -> Result<(), String> {
//   // Similar to launch_electron but uses module_path
//   // and validates signature before spawn
// }

fn detect_electron_binary() -> (Option<std::path::PathBuf>, Option<std::path::PathBuf>) {
  use std::path::PathBuf;
  // Dev path: workspace sidecar package
  let dev_pkg = PathBuf::from("../../packages/electron-shell");
  let dev_bin_rel = if cfg!(target_os = "windows") {
    "node_modules/.bin/electron.cmd"
  } else {
    "node_modules/.bin/electron"
  };
  let dev_bin = dev_pkg.join(dev_bin_rel);
  if dev_bin.exists() {
    return (Some(dev_pkg), Some(dev_bin));
  }

  // Bundled path: resources directory (platform-specific at runtime). Example placeholder:
  // Note: In a real app, you would use tauri APIs to read $RESOURCE paths or bundle a
  // platform-specific electron binary. Here we fall back to None.
  (None, None)
}

#[command]
pub async fn is_electron_available() -> Result<bool, String> {
  let (_pkg, bin) = detect_electron_binary();
  Ok(bin.is_some())
}

#[command]
pub async fn ensure_electron_sidecar() -> Result<(), String> {
  // Dev-only helper: instructs user to install sidecar dependencies.
  let (_pkg, bin) = detect_electron_binary();
  if bin.is_some() {
    return Ok(());
  }
  Err("not_installed".into())
}

// ===== M1: Streaming Hub Commands =====

/// Represents a streaming content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamItem {
  pub id: String,
  pub title: String,
  pub url: String,
  pub thumbnail: Option<String>,
  pub provider: String, // "youtube", "twitch", "hls", "dash"
  pub duration: Option<u32>, // seconds
  pub watched_progress: Option<u32>, // seconds
}

/// Play a stream using the appropriate protocol handler
/// For DRM content, uses Pattern A Electron integration
#[command]
pub async fn play_stream(item: StreamItem) -> Result<(), String> {
  log::info!("Playing stream: {} ({})", item.title, item.provider);
  
  // Check if DRM content requires Electron
  let requires_drm = item.provider == "widevine" || item.url.contains("drm");
  
  if requires_drm {
    // Use Electron sidecar for DRM content
    open_electron_feature(item.url).await
  } else {
    // For non-DRM content, can use native webview
    // This is handled on the frontend with video.js or similar
    Ok(())
  }
}

/// Add item to watch queue
#[command]
pub fn add_to_queue(item: StreamItem) -> Result<(), String> {
  log::info!("Adding to queue: {}", item.title);
  // TODO: Persist to SQLite in future implementation
  // For now, just log the action
  Ok(())
}

/// Get watch history
#[command]
pub fn get_watch_history(limit: Option<u32>) -> Result<Vec<StreamItem>, String> {
  log::info!("Fetching watch history (limit: {:?})", limit);
  // TODO: Retrieve from SQLite in future implementation
  // For now, return empty list
  Ok(vec![])
}

/// Save watch progress
#[command]
pub fn save_watch_progress(id: String, progress: u32) -> Result<(), String> {
  log::info!("Saving watch progress for {}: {} seconds", id, progress);
  // TODO: Persist to SQLite in future implementation
  Ok(())
}

/// Get recommendations based on watch history
#[command]
pub fn get_recommendations() -> Result<Vec<StreamItem>, String> {
  log::info!("Fetching recommendations");
  // TODO: Implement AI-based recommendations in future
  // For now, return empty list
  Ok(vec![])
}
