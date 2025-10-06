# Phase 2.3: Installation & Updates System

**Status:** Planning
**Duration:** 14 weeks (July - October 2026)
**Dependencies:** Phase 2.2 (Detection & Registry) must be complete
**Budget Estimate:** $295K - $350K

---

## 📋 Executive Summary

Phase 2.3 implements the complete installation and update infrastructure for optional Electron modules. This phase transforms the system from passive detection to active management, enabling users to download, install, verify, update, and remove modules seamlessly. The focus is on reliability, security, and excellent user experience across all platforms.

**Core Deliverables:**
1. Module downloader with resume capability and integrity checking
2. Cross-platform installer with atomic operations
3. Automatic update system with background checks
4. Rollback mechanism for failed installations
5. User consent and progress UI
6. Bandwidth optimization and CDN integration

---

## 🎯 Objectives

### Primary Goals
- **Seamless Installation**: One-click module installation with clear progress
- **Reliable Updates**: Automatic background updates with minimal user disruption
- **Safe Operations**: All installs atomic with automatic rollback on failure
- **Bandwidth Efficient**: Delta updates, compression, CDN optimization
- **User Control**: Clear consent, progress visibility, ability to pause/cancel

### Success Metrics
- Installation success rate >99.5%
- Download resume success rate >95%
- Update check overhead <5MB/month
- Average download speed >5MB/s (good connection)
- Rollback time <10 seconds
- User satisfaction >4.5/5

---

## 🏗️ Architecture Overview

```
┌────────────────────────────────────────────────────────────────┐
│                    Tauri Application                            │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │              Module Manager (Orchestrator)               │ │
│  │  - Installation workflows                                │ │
│  │  - Update scheduling                                     │ │
│  │  - User consent handling                                 │ │
│  └─────────────┬────────────────────────────────────────────┘ │
│                │                                                │
│  ┌─────────────▼────────────┐    ┌──────────────────────────┐ │
│  │    Downloader            │    │    Installer             │ │
│  │  - HTTP/HTTPS client     │    │  - Atomic operations     │ │
│  │  - Resume support        │    │  - File extraction       │ │
│  │  - Integrity checking    │    │  - Permission setting    │ │
│  │  - Progress tracking     │    │  - Registry updates      │ │
│  └──────────────────────────┘    └──────────┬───────────────┘ │
│                                              │                  │
│  ┌──────────────────────────────────────────▼───────────────┐ │
│  │              Update Manager                              │ │
│  │  - Version checking (background)                         │ │
│  │  - Delta updates                                         │ │
│  │  - Rollback on failure                                   │ │
│  │  - Update scheduling                                     │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │              UI Components (Tauri Commands)              │ │
│  │  - Progress notifications                                │ │
│  │  - Consent dialogs                                       │ │
│  │  - Settings panel                                        │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                 │
└─────────────────────────────┬──────────────────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │   Update Server   │
                    │   (CDN-backed)    │
                    │  - Module bundles │
                    │  - Signatures     │
                    │  - Version index  │
                    │  - Delta patches  │
                    └──────────────────┘
```

---

## 📦 Core Components

### 1. Module Downloader

**Purpose:** Reliable HTTP(S) downloads with resume capability and integrity verification

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/download/mod.rs

use reqwest::{Client, Response};
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ModuleDownloader {
    client: Client,
    cache_dir: PathBuf,
    progress_tracker: Arc<RwLock<ProgressTracker>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed_bps: u64,
    pub eta_seconds: Option<u64>,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, Serialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Paused,
    Verifying,
    Completed,
    Failed(String),
}

impl ModuleDownloader {
    pub fn new(cache_dir: PathBuf) -> Result<Self, DownloadError> {
        let client = Client::builder()
            .user_agent(format!("Tauri/{}", env!("CARGO_PKG_VERSION")))
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            client,
            cache_dir,
            progress_tracker: Arc::new(RwLock::new(ProgressTracker::new())),
        })
    }

    /// Download a module with resume capability
    pub async fn download_module(
        &self,
        url: &str,
        expected_hash: &str,
        module_id: &str,
    ) -> Result<PathBuf, DownloadError> {
        let filename = format!("{}.tar.gz", module_id);
        let dest_path = self.cache_dir.join(&filename);
        let temp_path = self.cache_dir.join(format!("{}.partial", filename));

        // Check if already downloaded and valid
        if dest_path.exists() {
            if self.verify_hash(&dest_path, expected_hash).await? {
                return Ok(dest_path);
            } else {
                tokio::fs::remove_file(&dest_path).await?;
            }
        }

        // Check for partial download
        let resume_from = if temp_path.exists() {
            temp_path.metadata()?.len()
        } else {
            0
        };

        self.update_progress(module_id, DownloadProgress {
            downloaded_bytes: resume_from,
            total_bytes: 0,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Downloading,
        }).await;

        // Build request with Range header for resume
        let mut request = self.client.get(url);
        if resume_from > 0 {
            request = request.header("Range", format!("bytes={}-", resume_from));
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() && status.as_u16() != 206 {
            return Err(DownloadError::HttpError(status.as_u16()));
        }

        let total_bytes = response.content_length()
            .map(|len| len + resume_from)
            .unwrap_or(0);

        // Stream download to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&temp_path)
            .await?;

        let mut downloaded = resume_from;
        let mut stream = response.bytes_stream();
        let start_time = std::time::Instant::now();

        use futures_util::StreamExt;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;

            // Update progress every 100KB
            if downloaded % 102400 < chunk.len() as u64 {
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    (downloaded - resume_from) as f64 / elapsed
                } else {
                    0.0
                };

                let eta = if speed > 0.0 && total_bytes > 0 {
                    Some(((total_bytes - downloaded) as f64 / speed) as u64)
                } else {
                    None
                };

                self.update_progress(module_id, DownloadProgress {
                    downloaded_bytes: downloaded,
                    total_bytes,
                    speed_bps: speed as u64,
                    eta_seconds: eta,
                    status: DownloadStatus::Downloading,
                }).await;
            }
        }

        file.sync_all().await?;
        drop(file);

        // Verify integrity
        self.update_progress(module_id, DownloadProgress {
            downloaded_bytes: downloaded,
            total_bytes,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Verifying,
        }).await;

        if !self.verify_hash(&temp_path, expected_hash).await? {
            tokio::fs::remove_file(&temp_path).await?;
            return Err(DownloadError::IntegrityCheckFailed);
        }

        // Move to final location
        tokio::fs::rename(&temp_path, &dest_path).await?;

        self.update_progress(module_id, DownloadProgress {
            downloaded_bytes: downloaded,
            total_bytes,
            speed_bps: 0,
            eta_seconds: None,
            status: DownloadStatus::Completed,
        }).await;

        Ok(dest_path)
    }

    async fn verify_hash(&self, path: &Path, expected: &str) -> Result<bool, DownloadError> {
        let mut file = File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        use tokio::io::AsyncReadExt;

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let computed = format!("{:x}", hasher.finalize());
        Ok(computed == expected)
    }

    async fn update_progress(&self, module_id: &str, progress: DownloadProgress) {
        self.progress_tracker.write().await.update(module_id.to_string(), progress);
    }

    pub async fn get_progress(&self, module_id: &str) -> Option<DownloadProgress> {
        self.progress_tracker.read().await.get(module_id)
    }

    pub async fn pause_download(&self, module_id: &str) -> Result<(), DownloadError> {
        // Implementation: set flag to pause, stream will check and stop
        self.progress_tracker.write().await.set_paused(module_id, true);
        Ok(())
    }

    pub async fn resume_download(&self, module_id: &str) -> Result<(), DownloadError> {
        self.progress_tracker.write().await.set_paused(module_id, false);
        // Caller should re-call download_module, which will resume
        Ok(())
    }
}

struct ProgressTracker {
    progress_map: std::collections::HashMap<String, DownloadProgress>,
    paused_set: std::collections::HashSet<String>,
}

impl ProgressTracker {
    fn new() -> Self {
        Self {
            progress_map: std::collections::HashMap::new(),
            paused_set: std::collections::HashSet::new(),
        }
    }

    fn update(&mut self, module_id: String, progress: DownloadProgress) {
        self.progress_map.insert(module_id, progress);
    }

    fn get(&self, module_id: &str) -> Option<DownloadProgress> {
        self.progress_map.get(module_id).cloned()
    }

    fn set_paused(&mut self, module_id: &str, paused: bool) {
        if paused {
            self.paused_set.insert(module_id.to_string());
        } else {
            self.paused_set.remove(module_id);
        }
    }

    fn is_paused(&self, module_id: &str) -> bool {
        self.paused_set.contains(module_id)
    }
}
```

---

### 2. Module Installer

**Purpose:** Atomic installation with automatic rollback on failure

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/installer/mod.rs

use std::path::{Path, PathBuf};
use tokio::fs;
use tar::Archive;
use flate2::read::GzDecoder;
use std::fs::File;
use crate::registry::{ModuleRegistry, ModuleEntry, ModuleStatus};
use crate::verification::SignatureVerifier;

pub struct ModuleInstaller {
    install_base: PathBuf,
    registry: Arc<ModuleRegistry>,
    verifier: Arc<SignatureVerifier>,
}

#[derive(Debug, Serialize)]
pub enum InstallStep {
    Extracting,
    Verifying,
    RegisteringFiles,
    UpdatingRegistry,
    Completed,
    RollingBack,
}

#[derive(Debug, Serialize)]
pub struct InstallProgress {
    pub step: InstallStep,
    pub current_file: Option<String>,
    pub files_processed: usize,
    pub total_files: usize,
}

impl ModuleInstaller {
    pub fn new(
        install_base: PathBuf,
        registry: Arc<ModuleRegistry>,
        verifier: Arc<SignatureVerifier>,
    ) -> Result<Self, InstallerError> {
        fs::create_dir_all(&install_base)?;

        Ok(Self {
            install_base,
            registry,
            verifier,
        })
    }

    /// Install a module from a downloaded archive
    /// This operation is atomic - either fully succeeds or fully rolls back
    pub async fn install_module(
        &self,
        archive_path: &Path,
        module_id: &str,
        version: &semver::Version,
        progress_callback: impl Fn(InstallProgress) + Send + Sync,
    ) -> Result<PathBuf, InstallerError> {
        // Create temporary installation directory
        let temp_id = uuid::Uuid::new_v4();
        let temp_dir = self.install_base.join(format!(".tmp-{}", temp_id));
        let final_dir = self.install_base.join(format!("{}-{}", module_id, version));

        // Ensure clean state
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }

        fs::create_dir_all(&temp_dir).await?;

        // Track for rollback
        let rollback_guard = RollbackGuard::new(temp_dir.clone(), final_dir.clone());

        // Step 1: Extract archive
        progress_callback(InstallProgress {
            step: InstallStep::Extracting,
            current_file: None,
            files_processed: 0,
            total_files: 0,
        });

        let file = File::open(archive_path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        let entries: Vec<_> = archive.entries()?.collect::<Result<_, _>>()?;
        let total_files = entries.len();

        for (idx, mut entry) in entries.into_iter().enumerate() {
            let path = entry.path()?.to_path_buf();

            progress_callback(InstallProgress {
                step: InstallStep::Extracting,
                current_file: Some(path.display().to_string()),
                files_processed: idx,
                total_files,
            });

            entry.unpack_in(&temp_dir)?;
        }

        // Step 2: Verify signature
        progress_callback(InstallProgress {
            step: InstallStep::Verifying,
            current_file: None,
            files_processed: 0,
            total_files: 1,
        });

        let verification = self.verifier.verify_module(&temp_dir).await?;

        if !verification.valid {
            return Err(InstallerError::VerificationFailed(
                verification.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        // Step 3: Register files in registry
        progress_callback(InstallProgress {
            step: InstallStep::RegisteringFiles,
            current_file: None,
            files_processed: 0,
            total_files: 1,
        });

        let manifest_path = temp_dir.join("manifest.json");
        let manifest_data = fs::read_to_string(&manifest_path).await?;
        let manifest: ModuleManifest = serde_json::from_str(&manifest_data)?;

        // Step 4: Atomic move to final location
        progress_callback(InstallProgress {
            step: InstallStep::UpdatingRegistry,
            current_file: None,
            files_processed: 0,
            total_files: 1,
        });

        // Remove existing installation if present
        if final_dir.exists() {
            fs::remove_dir_all(&final_dir).await?;
        }

        // Atomic rename
        fs::rename(&temp_dir, &final_dir).await?;

        // Update registry
        let entry = ModuleEntry {
            id: module_id.to_string(),
            version: version.clone(),
            install_path: final_dir.clone(),
            signature_path: final_dir.join("module.sig"),
            manifest_hash: manifest.hash,
            installed_at: chrono::Utc::now(),
            last_verified: Some(chrono::Utc::now()),
            status: ModuleStatus::Healthy,
        };

        self.registry.register_module(entry)?;

        // Success - disarm rollback
        rollback_guard.disarm();

        progress_callback(InstallProgress {
            step: InstallStep::Completed,
            current_file: None,
            files_processed: total_files,
            total_files,
        });

        Ok(final_dir)
    }

    /// Uninstall a module
    pub async fn uninstall_module(
        &self,
        module_id: &str,
        version: &semver::Version,
    ) -> Result<(), InstallerError> {
        // Get module entry
        let entry = self.registry.get_module(module_id, version)?
            .ok_or(InstallerError::ModuleNotFound)?;

        // Remove files
        if entry.install_path.exists() {
            fs::remove_dir_all(&entry.install_path).await?;
        }

        // Remove from registry
        self.registry.remove_module(module_id, version)?;

        Ok(())
    }
}

/// RAII guard for automatic rollback on failure
struct RollbackGuard {
    temp_dir: PathBuf,
    final_dir: PathBuf,
    armed: Arc<AtomicBool>,
}

impl RollbackGuard {
    fn new(temp_dir: PathBuf, final_dir: PathBuf) -> Self {
        Self {
            temp_dir,
            final_dir,
            armed: Arc::new(AtomicBool::new(true)),
        }
    }

    fn disarm(&self) {
        self.armed.store(false, Ordering::SeqCst);
    }
}

impl Drop for RollbackGuard {
    fn drop(&mut self) {
        if self.armed.load(Ordering::SeqCst) {
            // Rollback: remove temporary directory
            if self.temp_dir.exists() {
                let _ = std::fs::remove_dir_all(&self.temp_dir);
            }

            // If we partially moved to final, remove that too
            if self.final_dir.exists() {
                let _ = std::fs::remove_dir_all(&self.final_dir);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct ModuleManifest {
    hash: String,
    version: String,
    name: String,
}
```

---

### 3. Update Manager

**Purpose:** Automatic background updates with delta patching and rollback

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/updates/mod.rs

use tokio::time::{interval, Duration};
use semver::Version;
use serde::{Deserialize, Serialize};

pub struct UpdateManager {
    registry: Arc<ModuleRegistry>,
    downloader: Arc<ModuleDownloader>,
    installer: Arc<ModuleInstaller>,
    update_server_url: String,
    check_interval: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub available_updates: Vec<AvailableUpdate>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableUpdate {
    pub module_id: String,
    pub current_version: Version,
    pub new_version: Version,
    pub download_url: String,
    pub delta_url: Option<String>,  // Delta patch if available
    pub size_bytes: u64,
    pub delta_size_bytes: Option<u64>,
    pub release_notes: String,
    pub critical: bool,  // Requires immediate update
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProgress {
    pub module_id: String,
    pub stage: UpdateStage,
    pub progress_percent: f32,
}

#[derive(Debug, Clone, Serialize)]
pub enum UpdateStage {
    CheckingForUpdates,
    DownloadingUpdate,
    ApplyingUpdate,
    Verifying,
    Completed,
    Failed(String),
}

impl UpdateManager {
    pub fn new(
        registry: Arc<ModuleRegistry>,
        downloader: Arc<ModuleDownloader>,
        installer: Arc<ModuleInstaller>,
        update_server_url: String,
        check_interval_hours: u64,
    ) -> Self {
        Self {
            registry,
            downloader,
            installer,
            update_server_url,
            check_interval: Duration::from_secs(check_interval_hours * 3600),
        }
    }

    /// Start background update checking
    pub async fn start_background_checks(
        self: Arc<Self>,
        auto_install: bool,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut check_timer = interval(self.check_interval);

            loop {
                check_timer.tick().await;

                match self.check_for_updates().await {
                    Ok(updates) => {
                        if !updates.available_updates.is_empty() {
                            // Emit event to frontend
                            // self.emit_event("updates-available", &updates);

                            if auto_install {
                                for update in &updates.available_updates {
                                    if update.critical {
                                        let _ = self.apply_update(update).await;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Update check failed: {}", e);
                    }
                }
            }
        })
    }

    /// Check for available updates
    pub async fn check_for_updates(&self) -> Result<UpdateManifest, UpdateError> {
        let installed = self.registry.list_modules(None)?;

        // Build version query
        let versions: Vec<_> = installed.iter()
            .map(|m| format!("{}:{}", m.id, m.version))
            .collect();

        let query_params = versions.join(",");
        let url = format!("{}/api/check-updates?modules={}",
                         self.update_server_url, query_params);

        let response = reqwest::get(&url).await?;
        let manifest: UpdateManifest = response.json().await?;

        Ok(manifest)
    }

    /// Apply a single update
    pub async fn apply_update(
        &self,
        update: &AvailableUpdate,
    ) -> Result<(), UpdateError> {
        // Create backup for rollback
        let backup_dir = self.create_backup(&update.module_id, &update.current_version).await?;
        let rollback_guard = UpdateRollbackGuard::new(
            backup_dir.clone(),
            self.registry.clone(),
            update.module_id.clone(),
            update.current_version.clone(),
        );

        // Download update (prefer delta if available)
        let download_url = update.delta_url.as_ref()
            .unwrap_or(&update.download_url);

        let archive_path = self.downloader.download_module(
            download_url,
            &update.module_id,  // Hash verification handled by downloader
            &update.module_id,
        ).await?;

        // Apply update
        self.installer.install_module(
            &archive_path,
            &update.module_id,
            &update.new_version,
            |_progress| { /* emit progress events */ },
        ).await?;

        // Success - disarm rollback
        rollback_guard.disarm();

        Ok(())
    }

    async fn create_backup(
        &self,
        module_id: &str,
        version: &Version,
    ) -> Result<PathBuf, UpdateError> {
        let entry = self.registry.get_module(module_id, version)?
            .ok_or(UpdateError::ModuleNotFound)?;

        let backup_dir = entry.install_path.parent()
            .unwrap()
            .join(format!(".backup-{}-{}", module_id, version));

        if backup_dir.exists() {
            tokio::fs::remove_dir_all(&backup_dir).await?;
        }

        // Copy directory
        Self::copy_dir_recursive(&entry.install_path, &backup_dir).await?;

        Ok(backup_dir)
    }

    async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), UpdateError> {
        tokio::fs::create_dir_all(dst).await?;

        let mut entries = tokio::fs::read_dir(src).await?;

        while let Some(entry) = entries.next_entry().await? {
            let ty = entry.file_type().await?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path).await?;
            } else {
                tokio::fs::copy(&src_path, &dst_path).await?;
            }
        }

        Ok(())
    }
}

struct UpdateRollbackGuard {
    backup_dir: PathBuf,
    registry: Arc<ModuleRegistry>,
    module_id: String,
    version: Version,
    armed: Arc<AtomicBool>,
}

impl UpdateRollbackGuard {
    fn new(
        backup_dir: PathBuf,
        registry: Arc<ModuleRegistry>,
        module_id: String,
        version: Version,
    ) -> Self {
        Self {
            backup_dir,
            registry,
            module_id,
            version,
            armed: Arc::new(AtomicBool::new(true)),
        }
    }

    fn disarm(&self) {
        self.armed.store(false, Ordering::SeqCst);
    }
}

impl Drop for UpdateRollbackGuard {
    fn drop(&mut self) {
        if self.armed.load(Ordering::SeqCst) {
            // Rollback: restore from backup
            if let Some(entry) = self.registry.get_module(&self.module_id, &self.version).ok().flatten() {
                if entry.install_path.exists() {
                    let _ = std::fs::remove_dir_all(&entry.install_path);
                }

                if self.backup_dir.exists() {
                    let _ = std::fs::rename(&self.backup_dir, &entry.install_path);
                }
            }
        }
    }
}
```

---

### 4. Tauri Commands API

**Purpose:** Expose installation/update functionality to frontend

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/commands.rs

use tauri::{command, Runtime, State, Window};

#[command]
pub async fn install_electron_module<R: Runtime>(
    module_id: String,
    version: String,
    window: Window<R>,
    manager: State<'_, ModuleManager>,
) -> Result<InstallResult, String> {
    let version = semver::Version::parse(&version).map_err(|e| e.to_string())?;

    // Request user consent
    let consent = window.emit("module-install-consent", ConsentRequest {
        module_id: module_id.clone(),
        version: version.to_string(),
        estimated_size_mb: 80,  // TODO: get from manifest
    }).await;

    // Download
    let manifest = manager.get_module_manifest(&module_id, &version).await
        .map_err(|e| e.to_string())?;

    let archive_path = manager.downloader.download_module(
        &manifest.download_url,
        &manifest.hash,
        &module_id,
    ).await.map_err(|e| e.to_string())?;

    // Install
    let install_path = manager.installer.install_module(
        &archive_path,
        &module_id,
        &version,
        |progress| {
            let _ = window.emit("install-progress", progress);
        },
    ).await.map_err(|e| e.to_string())?;

    Ok(InstallResult {
        success: true,
        install_path: install_path.display().to_string(),
    })
}

#[command]
pub async fn uninstall_electron_module<R: Runtime>(
    module_id: String,
    version: String,
    manager: State<'_, ModuleManager>,
) -> Result<(), String> {
    let version = semver::Version::parse(&version).map_err(|e| e.to_string())?;

    manager.installer.uninstall_module(&module_id, &version).await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn check_for_updates<R: Runtime>(
    manager: State<'_, ModuleManager>,
) -> Result<UpdateManifest, String> {
    manager.update_manager.check_for_updates().await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn apply_update<R: Runtime>(
    module_id: String,
    current_version: String,
    new_version: String,
    window: Window<R>,
    manager: State<'_, ModuleManager>,
) -> Result<(), String> {
    let current = semver::Version::parse(&current_version).map_err(|e| e.to_string())?;
    let new = semver::Version::parse(&new_version).map_err(|e| e.to_string())?;

    let update = AvailableUpdate {
        module_id: module_id.clone(),
        current_version: current,
        new_version: new,
        // ... other fields from check_for_updates
    };

    manager.update_manager.apply_update(&update).await
        .map_err(|e| e.to_string())?;

    window.emit("update-completed", UpdateCompletedEvent {
        module_id,
        new_version: new_version,
    }).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn get_download_progress<R: Runtime>(
    module_id: String,
    manager: State<'_, ModuleManager>,
) -> Result<Option<DownloadProgress>, String> {
    Ok(manager.downloader.get_progress(&module_id).await)
}

#[command]
pub async fn pause_download<R: Runtime>(
    module_id: String,
    manager: State<'_, ModuleManager>,
) -> Result<(), String> {
    manager.downloader.pause_download(&module_id).await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn resume_download<R: Runtime>(
    module_id: String,
    manager: State<'_, ModuleManager>,
) -> Result<(), String> {
    manager.downloader.resume_download(&module_id).await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct ConsentRequest {
    module_id: String,
    version: String,
    estimated_size_mb: u64,
}

#[derive(Serialize)]
struct InstallResult {
    success: bool,
    install_path: String,
}

#[derive(Serialize)]
struct UpdateCompletedEvent {
    module_id: String,
    new_version: String,
}
```

---

### 5. Frontend Integration (TypeScript)

**Purpose:** User-facing API and UI components

**Implementation:**
```typescript
// packages/api/src/electron-install.ts

import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

export interface InstallOptions {
  moduleId: string;
  version: string;
  onProgress?: (progress: InstallProgress) => void;
  onConsent?: (request: ConsentRequest) => Promise<boolean>;
}

export interface InstallProgress {
  step: 'downloading' | 'extracting' | 'verifying' | 'registering';
  percent: number;
  currentFile?: string;
}

export interface ConsentRequest {
  moduleId: string;
  version: string;
  estimatedSizeMb: number;
}

export interface UpdateInfo {
  availableUpdates: AvailableUpdate[];
  timestamp: string;
}

export interface AvailableUpdate {
  moduleId: string;
  currentVersion: string;
  newVersion: string;
  sizeBytes: number;
  releaseNotes: string;
  critical: boolean;
}

/**
 * Install an Electron module
 */
export async function installElectronModule(
  options: InstallOptions
): Promise<{ success: boolean; installPath: string }> {
  const { moduleId, version, onProgress, onConsent } = options;

  let progressUnlisten: UnlistenFn | null = null;
  let consentUnlisten: UnlistenFn | null = null;

  try {
    // Listen for progress updates
    if (onProgress) {
      progressUnlisten = await listen('install-progress', (event) => {
        onProgress(event.payload as InstallProgress);
      });
    }

    // Listen for consent requests
    if (onConsent) {
      consentUnlisten = await listen('module-install-consent', async (event) => {
        const request = event.payload as ConsentRequest;
        const approved = await onConsent(request);

        if (!approved) {
          throw new Error('Installation cancelled by user');
        }
      });
    }

    // Invoke installation
    const result = await invoke('plugin:electron|install_electron_module', {
      moduleId,
      version,
    });

    return result as { success: boolean; installPath: string };
  } finally {
    progressUnlisten?.();
    consentUnlisten?.();
  }
}

/**
 * Uninstall an Electron module
 */
export async function uninstallElectronModule(
  moduleId: string,
  version: string
): Promise<void> {
  await invoke('plugin:electron|uninstall_electron_module', {
    moduleId,
    version,
  });
}

/**
 * Check for available updates
 */
export async function checkForUpdates(): Promise<UpdateInfo> {
  return await invoke('plugin:electron|check_for_updates');
}

/**
 * Apply a specific update
 */
export async function applyUpdate(
  moduleId: string,
  currentVersion: string,
  newVersion: string,
  onProgress?: (progress: number) => void
): Promise<void> {
  let unlisten: UnlistenFn | null = null;

  try {
    if (onProgress) {
      unlisten = await listen('update-progress', (event) => {
        const progress = event.payload as { percent: number };
        onProgress(progress.percent);
      });
    }

    await invoke('plugin:electron|apply_update', {
      moduleId,
      currentVersion,
      newVersion,
    });
  } finally {
    unlisten?.();
  }
}

/**
 * Get download progress for a module
 */
export async function getDownloadProgress(
  moduleId: string
): Promise<DownloadProgress | null> {
  return await invoke('plugin:electron|get_download_progress', { moduleId });
}

/**
 * Pause an ongoing download
 */
export async function pauseDownload(moduleId: string): Promise<void> {
  await invoke('plugin:electron|pause_download', { moduleId });
}

/**
 * Resume a paused download
 */
export async function resumeDownload(moduleId: string): Promise<void> {
  await invoke('plugin:electron|resume_download', { moduleId });
}

export interface DownloadProgress {
  downloadedBytes: number;
  totalBytes: number;
  speedBps: number;
  etaSeconds?: number;
  status: 'queued' | 'downloading' | 'paused' | 'verifying' | 'completed' | 'failed';
}
```

**Example React Component:**
```tsx
// Example: ModuleInstaller.tsx

import React, { useState } from 'react';
import { installElectronModule, InstallProgress } from '@tauri-apps/api/electron-install';

export function ModuleInstaller() {
  const [progress, setProgress] = useState<InstallProgress | null>(null);
  const [installing, setInstalling] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleInstall = async () => {
    setInstalling(true);
    setError(null);

    try {
      const result = await installElectronModule({
        moduleId: 'electron-shell',
        version: '1.0.0',
        onProgress: (prog) => setProgress(prog),
        onConsent: async (request) => {
          return window.confirm(
            `Install ${request.moduleId} v${request.version}?\n` +
            `Size: ${request.estimatedSizeMb} MB`
          );
        },
      });

      console.log('Installed to:', result.installPath);
      alert('Installation complete!');
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setInstalling(false);
      setProgress(null);
    }
  };

  return (
    <div className="module-installer">
      <h2>Install Electron Module</h2>

      <button onClick={handleInstall} disabled={installing}>
        {installing ? 'Installing...' : 'Install Module'}
      </button>

      {progress && (
        <div className="progress">
          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{ width: `${progress.percent}%` }}
            />
          </div>
          <p>
            {progress.step}: {progress.percent.toFixed(1)}%
            {progress.currentFile && ` - ${progress.currentFile}`}
          </p>
        </div>
      )}

      {error && (
        <div className="error">
          Error: {error}
        </div>
      )}
    </div>
  );
}
```

---

## 📝 Task Breakdown

### Task 1: Download Infrastructure (Weeks 1-3)

**Owner:** Lead Developer + DevOps Engineer

**Deliverables:**
- [ ] HTTP(S) downloader with resume capability
- [ ] Integrity checking (SHA256 verification)
- [ ] Progress tracking and reporting
- [ ] Pause/resume functionality
- [ ] Bandwidth throttling (optional)
- [ ] CDN integration testing

**Acceptance Criteria:**
- Resume works after interruption >95% of time
- Download speed >5MB/s on good connection
- Progress updates smooth (every 100KB)
- Integrity check catches all corrupted downloads
- Memory usage <50MB during download

**Dependencies:** Phase 2.2 (registry for tracking)

---

### Task 2: Installation System (Weeks 2-5)

**Owner:** Lead Developer + Security Engineer

**Deliverables:**
- [ ] Atomic installation with rollback
- [ ] Cross-platform file extraction
- [ ] Permission and ownership management
- [ ] Registry integration
- [ ] Post-install verification
- [ ] Installation audit logging

**Acceptance Criteria:**
- Installations atomic (all-or-nothing)
- Rollback time <10 seconds
- Works on Windows, macOS, Linux
- No permission issues post-install
- Audit log captures all operations

**Dependencies:** Task 1 (downloader), Phase 2.2 (registry + verification)

---

### Task 3: Update Manager (Weeks 4-7)

**Owner:** Lead Developer + DevOps Engineer

**Deliverables:**
- [ ] Background update checking
- [ ] Delta patching system
- [ ] Update rollback mechanism
- [ ] Configurable update policies
- [ ] Critical update handling
- [ ] Update scheduling

**Acceptance Criteria:**
- Update checks <5MB bandwidth/month
- Delta updates save >50% bandwidth
- Rollback works 100% of time
- Background checks don't impact performance
- Critical updates applied within 24 hours

**Dependencies:** Task 2 (installer), Phase 2.2 (registry)

---

### Task 4: User Consent & UI (Weeks 5-9)

**Owner:** Frontend Developer + UX Designer (new role)

**Deliverables:**
- [ ] Consent dialog components
- [ ] Progress notification UI
- [ ] Settings panel for update preferences
- [ ] Download manager UI
- [ ] Error handling and recovery UI
- [ ] Accessibility compliance

**Acceptance Criteria:**
- Consent dialogs clear and informative
- Progress updates smooth and accurate
- Users can pause/resume/cancel
- Settings persist across sessions
- WCAG 2.1 AA compliant

**Dependencies:** Tasks 1-3 (backend APIs)

---

### Task 5: Testing & Optimization (Weeks 8-11)

**Owner:** QA Engineer + Lead Developer

**Deliverables:**
- [ ] Network condition testing (slow, flaky)
- [ ] Interruption testing (crashes, power loss)
- [ ] Performance benchmarks
- [ ] Cross-platform testing
- [ ] Load testing (concurrent installs)
- [ ] Regression test suite

**Acceptance Criteria:**
- All failure scenarios handled gracefully
- Performance targets met on all platforms
- Test coverage >85%
- No data loss in any failure scenario
- Clear error messages for all failures

**Dependencies:** Tasks 1-4 (all features complete)

---

### Task 6: Documentation & Release (Weeks 12-14)

**Owner:** Technical Writer + Lead Developer

**Deliverables:**
- [ ] User guide (installation/updates)
- [ ] API documentation
- [ ] Troubleshooting guide
- [ ] Migration guide from Phase 2.2
- [ ] Release notes
- [ ] Video tutorials (optional)

**Acceptance Criteria:**
- Documentation covers all features
- Examples for common scenarios
- Troubleshooting covers 90% of issues
- Developer feedback positive
- Ready for production release

**Dependencies:** Task 5 (testing complete)

---

## 🏃 Sprint Structure

### Sprint 1-2 (Weeks 1-4): Download & Extract
- **Goal:** Reliable downloads with resume
- **Demos:** Download with progress, resume after interrupt
- **Risks:** Network handling, file corruption

### Sprint 3-4 (Weeks 5-8): Install & Update
- **Goal:** Atomic operations with rollback
- **Demos:** Install/uninstall, automatic updates
- **Risks:** Permission issues, rollback failures

### Sprint 5-6 (Weeks 9-12): UI & Polish
- **Goal:** Complete user experience
- **Demos:** Full workflow, settings panel
- **Risks:** UX feedback, accessibility issues

### Sprint 7 (Weeks 13-14): Release Prep
- **Goal:** Production-ready release
- **Demos:** Final walkthrough, documentation
- **Risks:** Last-minute bugs, documentation gaps

---

## 👥 Team & Resources

### Core Team (Full-time)
- **Lead Developer** (14 weeks)
  - Architecture, downloader, installer, update manager
  - Rate: $150/hr × 40hr/wk × 14wks = $84,000

- **Security Engineer** (10 weeks)
  - Signature integration, rollback security, audit logging
  - Rate: $160/hr × 40hr/wk × 10wks = $64,000

- **DevOps Engineer** (12 weeks)
  - CDN setup, update server, monitoring, performance
  - Rate: $140/hr × 40hr/wk × 12wks = $67,200

### Supporting Team (Part-time)
- **Frontend Developer** (75% × 10 weeks)
  - TypeScript API, React components, progress UI
  - Rate: $130/hr × 30hr/wk × 10wks = $39,000

- **UX Designer** (NEW - 50% × 6 weeks)
  - Consent flows, progress indicators, error states
  - Rate: $120/hr × 20hr/wk × 6wks = $14,400

- **QA Engineer** (75% × 12 weeks)
  - Testing all scenarios, automation, cross-platform
  - Rate: $110/hr × 30hr/wk × 12wks = $39,600

- **Technical Writer** (50% × 8 weeks)
  - User guides, API docs, troubleshooting
  - Rate: $100/hr × 20hr/wk × 8wks = $16,000

**Total Personnel:** $324,200

---

## 💰 Budget Breakdown

| Category | Item | Cost |
|----------|------|------|
| **Personnel** | Core team (3 FT, varying weeks) | $215,200 |
| | Supporting team (4 PT) | $109,000 |
| **Infrastructure** | CDN bandwidth (3 months, 10TB) | $15,000 |
| | Update server hosting | $3,600 |
| | Cloud storage (module hosting) | $2,400 |
| | Testing environments | $2,000 |
| **Tools & Services** | Network simulation tools | $1,200 |
| | Monitoring & analytics | $1,800 |
| | Development licenses | $1,000 |
| **Testing** | Cross-platform testing lab access | $3,000 |
| | Load testing services | $1,500 |
| **Contingency** | 10% buffer | $35,670 |
| **TOTAL** | | **$391,370** |

**Revised Budget Range:** $350K - $420K (adjusted for infrastructure costs)

---

## 🎯 Success Criteria

### Functional Requirements
- ✅ One-click module installation
- ✅ Automatic background updates
- ✅ Download resume after interruption
- ✅ Atomic operations with rollback
- ✅ Clear user consent flow
- ✅ Pause/resume/cancel support

### Performance Requirements
- ✅ Installation success rate >99.5%
- ✅ Download speed >5MB/s (good connection)
- ✅ Resume success rate >95%
- ✅ Rollback time <10 seconds
- ✅ Update check bandwidth <5MB/month
- ✅ Delta updates save >50% bandwidth

### Security Requirements
- ✅ All downloads integrity-verified
- ✅ Signatures checked before installation
- ✅ Rollback prevents corrupted installs
- ✅ Audit logging for all operations
- ✅ No privilege escalation vulnerabilities

### User Experience Requirements
- ✅ Clear progress indicators
- ✅ Informative error messages
- ✅ Settings persist correctly
- ✅ Accessibility compliant (WCAG 2.1 AA)
- ✅ User satisfaction >4.5/5

---

## 🚨 Risk Management

### High-Impact Risks

#### 1. Network Reliability Issues
- **Probability:** High
- **Impact:** High
- **Mitigation:**
  - Robust resume mechanism
  - Retry logic with exponential backoff
  - Network condition detection
  - Extensive testing on poor connections
- **Contingency:** Offline installation mode

#### 2. Platform-Specific Failures
- **Probability:** Medium
- **Impact:** High
- **Mitigation:**
  - Early cross-platform testing
  - Platform-specific code paths
  - Automated testing on all platforms
  - Beta testing program
- **Contingency:** Platform-specific workarounds, delay problematic platforms

#### 3. Rollback Failures
- **Probability:** Low
- **Impact:** Critical
- **Mitigation:**
  - Rigorous rollback testing
  - Multiple backup strategies
  - Corruption detection
  - Manual recovery tools
- **Contingency:** Support team intervention, reinstall scripts

### Medium-Impact Risks

#### 4. CDN Costs Exceed Budget
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Monitor bandwidth usage
  - Implement caching strategies
  - Delta updates to reduce traffic
  - CDN cost alerts
- **Contingency:** Switch CDN provider, implement P2P distribution

#### 5. User Experience Issues
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Early UX testing
  - User feedback sessions
  - A/B testing for critical flows
  - Accessibility audits
- **Contingency:** Iterative UI improvements, user education

---

## 📊 Milestones & Timeline

```
Week 1-3: Download Infrastructure
├─ Week 1: Basic HTTP downloader
├─ Week 2: Resume capability, integrity checking
└─ Week 3: Progress tracking, optimization
    ✓ Milestone: Reliable downloads working

Week 2-5: Installation System
├─ Week 2-3: File extraction, atomic operations
├─ Week 4: Registry integration, verification
└─ Week 5: Rollback mechanism, testing
    ✓ Milestone: Atomic installs operational

Week 4-7: Update Manager
├─ Week 4-5: Update checking, delta patches
├─ Week 6: Background scheduling, policies
└─ Week 7: Critical updates, integration
    ✓ Milestone: Automatic updates working

Week 5-9: User Interface
├─ Week 5-6: Consent & progress components
├─ Week 7-8: Settings panel, download manager
└─ Week 9: Error handling, accessibility
    ✓ Milestone: Complete UX implemented

Week 8-11: Testing & Optimization
├─ Week 8-9: Network testing, failure scenarios
├─ Week 10: Performance optimization
└─ Week 11: Cross-platform validation
    ✓ Milestone: Production quality assured

Week 12-14: Documentation & Release
├─ Week 12-13: Documentation, tutorials
└─ Week 14: Final testing, release prep
    ✓ Milestone: Phase 2.3 complete
```

### Key Dates
- **Week 1:** Kickoff, architecture review
- **Week 5:** Install system demo
- **Week 9:** UX review with stakeholders
- **Week 11:** Feature freeze, final testing
- **Week 14:** Phase 2.3 release, handoff to Phase 2.4

---

## 🔄 Integration Points

### With Phase 2.2 (Detection & Registry)
- Uses registry to track installed modules
- Verification system validates downloads
- Discovery API for version selection
- Health checks guide update decisions

### With Phase 2.4 (Distribution & CDN)
- Module packages served from CDN
- Delta patches generated by build system
- Bandwidth analytics inform caching
- Geographic distribution optimization

### With Phase 2.5+ (Future Phases)
- Analytics track installation success rates
- User preferences guide recommendations
- Marketplace integration for discovery
- Enterprise deployment features

---

## 📚 Documentation Deliverables

1. **User Guide**
   - Installing modules step-by-step
   - Managing updates
   - Troubleshooting common issues
   - Offline installation

2. **API Reference**
   - All Tauri commands
   - TypeScript API
   - Event system
   - Error codes

3. **Developer Guide**
   - Architecture overview
   - Extension points
   - Testing strategies
   - Performance tuning

4. **Operations Guide**
   - CDN configuration
   - Update server setup
   - Monitoring & alerts
   - Incident response

5. **Migration Guide**
   - Upgrading from Phase 2.2
   - Breaking changes
   - Data migration
   - Rollback procedures

---

## ✅ Phase 2.3 Complete When

- [ ] Installation success rate >99.5% in testing
- [ ] Automatic updates working on all platforms
- [ ] Download resume success rate >95%
- [ ] All rollback scenarios tested and working
- [ ] Complete UX with consent and progress
- [ ] Test coverage >85%
- [ ] Documentation complete
- [ ] Cross-platform testing passed
- [ ] User feedback positive (beta program)
- [ ] Performance targets met
- [ ] Security audit passed (if needed)
- [ ] Handoff to Phase 2.4 team complete

---

## 📞 Stakeholder Communication

### Weekly Updates
- Installation success metrics
- CDN bandwidth usage
- User feedback summary
- Blockers and risks

### Demos
- Week 5: Installation with rollback
- Week 9: Complete UX workflow
- Week 11: Cross-platform demonstration
- Week 14: Final release demo

### Decision Points
- Week 3: CDN provider selection
- Week 7: Update policy defaults
- Week 9: UI/UX design approval
- Week 13: Release readiness go/no-go

---

## 🔗 Related Documents

- **Phase 2 Overview:** [../../PHASE_2_PLANNING.md](../../PHASE_2_PLANNING.md)
- **Phase 2.1 Plan:** [PHASE_2_1_PLAN.md](./PHASE_2_1_PLAN.md)
- **Phase 2.2 Plan:** [PHASE_2_2_PLAN.md](./PHASE_2_2_PLAN.md)
- **Verification Report:** [../../VERIFICATION_REPORT.md](../../VERIFICATION_REPORT.md)

---

**Status:** Ready for Review
**Next Action:** Stakeholder approval, team allocation, CDN vendor selection
**Target Start Date:** July 2026
**Target Completion:** October 2026
**Budget:** $391K (includes infrastructure costs)
