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

/// Launch the minimal Electron sidecar (Pattern A) to load a given URL.
/// Security notes:
/// - Uses a locally installed Electron binary under packages/electron-shell/node_modules/.bin
/// - Passes a per-session auth token via environment variable for optional coordination
/// - Renderer is sandboxed; no Node integration
#[command]
#[allow(unused_variables)]
pub async fn launch_electron(url: String) -> Result<(), String> {
  use rand::{distributions::Alphanumeric, Rng};
  use std::{path::PathBuf, process::Command};

  // Generate an ephemeral auth token (if the sidecar chooses to use it)
  let auth_token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .map(char::from)
    .collect();

  // In dev, the current dir is typically examples/api. The sidecar lives at ../../packages/electron-shell
  let (electron_pkg_dir, electron_bin) = detect_electron_binary();
  if electron_bin.is_none() {
    return Err("not_installed".to_string());
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
    Err(e) => Err(format!("Failed to spawn electron: {e}")),
  }
}

/// Stable contract for the UI: open a feature that may require Electron.
/// Today this delegates to launch_electron (Pattern A). In Pattern B, this
/// will detect/install/connect the optional module transparently.
#[command]
pub async fn open_electron_feature(url: String) -> Result<(), String> {
  // In the future, detect module availability here and branch to Pattern B.
  launch_electron(url).await
}

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
