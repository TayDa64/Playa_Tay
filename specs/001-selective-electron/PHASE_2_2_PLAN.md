# Phase 2.2: Detection & Registry System

**Status:** Planning
**Duration:** 12 weeks (April - July 2026)
**Dependencies:** Phase 2.1 (Foundation & Packaging) must be complete
**Budget Estimate:** $185K - $220K

---

## 📋 Executive Summary

Phase 2.2 establishes the intelligent detection and registry infrastructure that enables Tauri applications to discover, verify, and manage optional Electron modules. This phase transforms Pattern B from a static bundling system into a dynamic, secure module ecosystem.

**Core Deliverables:**
1. Local module registry with SQLite persistence
2. Signature verification system for module authenticity
3. Module discovery API with caching
4. Version compatibility checking
5. Health monitoring and diagnostics
6. Comprehensive security audit

---

## 🎯 Objectives

### Primary Goals
- **Enable Dynamic Discovery**: Applications can detect installed modules at runtime
- **Ensure Security**: All modules are cryptographically verified before use
- **Maintain Performance**: Module lookups cached, <100ms for typical queries
- **Support Versioning**: Compatible module versions automatically selected
- **Provide Diagnostics**: Clear health status and troubleshooting tools

### Success Metrics
- Module discovery completes in <100ms (cached) / <500ms (fresh)
- Signature verification success rate >99.99%
- Zero false negatives in compatibility checking
- Registry corruption recovery in <1 second
- API response time <50ms for cached queries

---

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri Application                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Module Discovery API (Rust)                │  │
│  │  - list_modules()                                    │  │
│  │  - get_module(name, version)                         │  │
│  │  - verify_module(path)                               │  │
│  │  - check_compatibility(module, app_version)          │  │
│  └──────────────────┬───────────────────────────────────┘  │
│                     │                                        │
│  ┌──────────────────▼───────────────────────────────────┐  │
│  │         Module Registry (SQLite + Cache)             │  │
│  │  - Installed modules metadata                        │  │
│  │  - Version compatibility matrix                      │  │
│  │  - Signature verification cache                      │  │
│  │  - Health status tracking                            │  │
│  └──────────────────┬───────────────────────────────────┘  │
│                     │                                        │
│  ┌──────────────────▼───────────────────────────────────┐  │
│  │      Signature Verification System                   │  │
│  │  - Ed25519 signature validation                      │  │
│  │  - Certificate chain verification                    │  │
│  │  - Revocation checking (OCSP/CRL)                    │  │
│  │  - Tamper detection                                  │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
                              │
                              │
                              ▼
                    ┌──────────────────┐
                    │  File System     │
                    │  - Modules       │
                    │  - Signatures    │
                    │  - Metadata      │
                    └──────────────────┘
```

---

## 📦 Core Components

### 1. Module Registry System

**Purpose:** Central database tracking all installed modules and their metadata

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/registry/mod.rs

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use lru::LruCache;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEntry {
    pub id: String,                    // "electron-shell"
    pub version: semver::Version,      // "1.0.0"
    pub install_path: PathBuf,         // Path to module directory
    pub signature_path: PathBuf,       // Path to .sig file
    pub manifest_hash: String,         // SHA256 of manifest
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub last_verified: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ModuleStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleStatus {
    Healthy,
    VerificationPending,
    SignatureInvalid,
    Corrupted,
    Deprecated,
}

pub struct ModuleRegistry {
    db: Arc<RwLock<Connection>>,
    cache: Arc<RwLock<LruCache<String, ModuleEntry>>>,
}

impl ModuleRegistry {
    pub fn new(db_path: &Path) -> Result<Self, RegistryError> {
        let conn = Connection::open(db_path)?;

        // Initialize schema
        conn.execute_batch(include_str!("schema.sql"))?;

        Ok(Self {
            db: Arc::new(RwLock::new(conn)),
            cache: Arc::new(RwLock::new(LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap()
            ))),
        })
    }

    pub fn register_module(&self, entry: ModuleEntry) -> Result<(), RegistryError> {
        let db = self.db.write().unwrap();

        db.execute(
            "INSERT INTO modules (id, version, install_path, signature_path,
                                  manifest_hash, installed_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id, version)
             DO UPDATE SET install_path=excluded.install_path,
                          last_verified=NULL",
            params![
                entry.id,
                entry.version.to_string(),
                entry.install_path.to_str(),
                entry.signature_path.to_str(),
                entry.manifest_hash,
                entry.installed_at.to_rfc3339(),
                serde_json::to_string(&entry.status)?,
            ],
        )?;

        // Invalidate cache
        let cache_key = format!("{}@{}", entry.id, entry.version);
        self.cache.write().unwrap().pop(&cache_key);

        Ok(())
    }

    pub fn get_module(&self, id: &str, version: &semver::Version)
        -> Result<Option<ModuleEntry>, RegistryError> {
        let cache_key = format!("{}@{}", id, version);

        // Check cache first
        if let Some(entry) = self.cache.write().unwrap().get(&cache_key) {
            return Ok(Some(entry.clone()));
        }

        // Query database
        let db = self.db.read().unwrap();
        let mut stmt = db.prepare(
            "SELECT id, version, install_path, signature_path, manifest_hash,
                    installed_at, last_verified, status
             FROM modules
             WHERE id = ?1 AND version = ?2"
        )?;

        let result = stmt.query_row(params![id, version.to_string()], |row| {
            Ok(ModuleEntry {
                id: row.get(0)?,
                version: semver::Version::parse(&row.get::<_, String>(1)?).unwrap(),
                install_path: PathBuf::from(row.get::<_, String>(2)?),
                signature_path: PathBuf::from(row.get::<_, String>(3)?),
                manifest_hash: row.get(4)?,
                installed_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap().into(),
                last_verified: row.get::<_, Option<String>>(6)?
                    .map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().into()),
                status: serde_json::from_str(&row.get::<_, String>(7)?).unwrap(),
            })
        });

        match result {
            Ok(entry) => {
                // Cache the result
                self.cache.write().unwrap().put(cache_key, entry.clone());
                Ok(Some(entry))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list_modules(&self, id: Option<&str>)
        -> Result<Vec<ModuleEntry>, RegistryError> {
        let db = self.db.read().unwrap();

        let query = if let Some(id) = id {
            format!("SELECT id, version, install_path, signature_path, manifest_hash,
                            installed_at, last_verified, status
                     FROM modules
                     WHERE id = '{}'
                     ORDER BY version DESC", id)
        } else {
            "SELECT id, version, install_path, signature_path, manifest_hash,
                    installed_at, last_verified, status
             FROM modules
             ORDER BY id, version DESC".to_string()
        };

        let mut stmt = db.prepare(&query)?;
        let entries = stmt.query_map([], |row| {
            Ok(ModuleEntry {
                id: row.get(0)?,
                version: semver::Version::parse(&row.get::<_, String>(1)?).unwrap(),
                install_path: PathBuf::from(row.get::<_, String>(2)?),
                signature_path: PathBuf::from(row.get::<_, String>(3)?),
                manifest_hash: row.get(4)?,
                installed_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap().into(),
                last_verified: row.get::<_, Option<String>>(6)?
                    .map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().into()),
                status: serde_json::from_str(&row.get::<_, String>(7)?).unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn update_status(&self, id: &str, version: &semver::Version,
                        status: ModuleStatus) -> Result<(), RegistryError> {
        let db = self.db.write().unwrap();

        db.execute(
            "UPDATE modules SET status = ?1, last_verified = ?2
             WHERE id = ?3 AND version = ?4",
            params![
                serde_json::to_string(&status)?,
                chrono::Utc::now().to_rfc3339(),
                id,
                version.to_string(),
            ],
        )?;

        // Invalidate cache
        let cache_key = format!("{}@{}", id, version);
        self.cache.write().unwrap().pop(&cache_key);

        Ok(())
    }

    pub fn remove_module(&self, id: &str, version: &semver::Version)
        -> Result<(), RegistryError> {
        let db = self.db.write().unwrap();

        db.execute(
            "DELETE FROM modules WHERE id = ?1 AND version = ?2",
            params![id, version.to_string()],
        )?;

        // Invalidate cache
        let cache_key = format!("{}@{}", id, version);
        self.cache.write().unwrap().pop(&cache_key);

        Ok(())
    }

    pub fn health_check(&self) -> Result<RegistryHealth, RegistryError> {
        let db = self.db.read().unwrap();

        let mut stmt = db.prepare(
            "SELECT status, COUNT(*) FROM modules GROUP BY status"
        )?;

        let status_counts = stmt.query_map([], |row| {
            Ok((
                serde_json::from_str::<ModuleStatus>(&row.get::<_, String>(0)?).unwrap(),
                row.get::<_, i64>(1)?
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(RegistryHealth {
            total_modules: status_counts.iter().map(|(_, count)| count).sum(),
            status_breakdown: status_counts.into_iter().collect(),
            last_check: chrono::Utc::now(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RegistryHealth {
    pub total_modules: i64,
    pub status_breakdown: std::collections::HashMap<ModuleStatus, i64>,
    pub last_check: chrono::DateTime<chrono::Utc>,
}
```

**Database Schema:**
```sql
-- crates/tauri-plugin-electron/src/registry/schema.sql

CREATE TABLE IF NOT EXISTS modules (
    id TEXT NOT NULL,
    version TEXT NOT NULL,
    install_path TEXT NOT NULL,
    signature_path TEXT NOT NULL,
    manifest_hash TEXT NOT NULL,
    installed_at TEXT NOT NULL,
    last_verified TEXT,
    status TEXT NOT NULL,
    metadata TEXT,  -- JSON for extensibility
    PRIMARY KEY (id, version)
);

CREATE INDEX IF NOT EXISTS idx_modules_id ON modules(id);
CREATE INDEX IF NOT EXISTS idx_modules_status ON modules(status);
CREATE INDEX IF NOT EXISTS idx_modules_installed_at ON modules(installed_at);

-- Version compatibility tracking
CREATE TABLE IF NOT EXISTS compatibility (
    module_id TEXT NOT NULL,
    module_version TEXT NOT NULL,
    tauri_version_req TEXT NOT NULL,  -- semver requirement ">=2.0.0,<3.0.0"
    electron_version_req TEXT,
    notes TEXT,
    FOREIGN KEY (module_id, module_version) REFERENCES modules(id, version)
);

CREATE INDEX IF NOT EXISTS idx_compat_module ON compatibility(module_id, module_version);

-- Verification audit log
CREATE TABLE IF NOT EXISTS verification_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_id TEXT NOT NULL,
    module_version TEXT NOT NULL,
    verified_at TEXT NOT NULL,
    result TEXT NOT NULL,  -- 'success', 'signature_invalid', 'corrupted'
    error_details TEXT
);

CREATE INDEX IF NOT EXISTS idx_verif_log_module ON verification_log(module_id, module_version);
CREATE INDEX IF NOT EXISTS idx_verif_log_time ON verification_log(verified_at);
```

---

### 2. Signature Verification System

**Purpose:** Cryptographically verify module authenticity and integrity

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/verification/mod.rs

use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::fs;

pub struct SignatureVerifier {
    trusted_keys: Vec<PublicKey>,
    revocation_list: Arc<RwLock<HashSet<String>>>,  // Revoked key fingerprints
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSignature {
    pub version: u8,                    // Signature format version
    pub algorithm: String,              // "Ed25519"
    pub key_id: String,                 // Public key fingerprint
    pub signature: Vec<u8>,             // Binary signature
    pub signed_at: chrono::DateTime<chrono::Utc>,
    pub manifest_hash: String,          // SHA256 of manifest.json
    pub content_hashes: Vec<FileHash>, // SHA256 of each file
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHash {
    pub path: String,
    pub hash: String,
}

impl SignatureVerifier {
    pub fn new() -> Result<Self, VerificationError> {
        // Load trusted public keys from embedded resources
        let trusted_keys = Self::load_trusted_keys()?;

        Ok(Self {
            trusted_keys,
            revocation_list: Arc::new(RwLock::new(HashSet::new())),
        })
    }

    fn load_trusted_keys() -> Result<Vec<PublicKey>, VerificationError> {
        // In production, embed keys at compile time
        let keys_pem = include_str!("../../../keys/trusted_public_keys.pem");

        keys_pem.lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<String>()
            .as_bytes()
            .chunks(32)
            .map(|chunk| PublicKey::from_bytes(chunk).map_err(Into::into))
            .collect()
    }

    pub async fn verify_module(&self, module_path: &Path)
        -> Result<VerificationResult, VerificationError> {
        let start = std::time::Instant::now();

        // 1. Load signature file
        let sig_path = module_path.join("module.sig");
        let sig_data = fs::read_to_string(&sig_path)?;
        let signature: ModuleSignature = serde_json::from_str(&sig_data)?;

        // 2. Check if key is revoked
        if self.revocation_list.read().unwrap().contains(&signature.key_id) {
            return Ok(VerificationResult {
                valid: false,
                error: Some("Signing key has been revoked".to_string()),
                duration: start.elapsed(),
            });
        }

        // 3. Find matching trusted key
        let public_key = self.trusted_keys.iter()
            .find(|key| Self::key_fingerprint(key) == signature.key_id)
            .ok_or(VerificationError::UnknownKey)?;

        // 4. Verify manifest signature
        let manifest_path = module_path.join("manifest.json");
        let manifest_data = fs::read(&manifest_path)?;
        let manifest_hash = Self::hash_data(&manifest_data);

        if manifest_hash != signature.manifest_hash {
            return Ok(VerificationResult {
                valid: false,
                error: Some("Manifest hash mismatch".to_string()),
                duration: start.elapsed(),
            });
        }

        // 5. Construct signed message (manifest hash + content hashes)
        let mut message = Vec::new();
        message.extend_from_slice(signature.manifest_hash.as_bytes());
        for file_hash in &signature.content_hashes {
            message.extend_from_slice(file_hash.path.as_bytes());
            message.extend_from_slice(file_hash.hash.as_bytes());
        }

        // 6. Verify Ed25519 signature
        let sig = Signature::from_bytes(&signature.signature)?;
        let message_hash = Self::hash_data(&message);

        match public_key.verify(&message_hash, &sig) {
            Ok(_) => {
                // 7. Verify content hashes
                for file_hash in &signature.content_hashes {
                    let file_path = module_path.join(&file_hash.path);
                    let content = fs::read(&file_path)?;
                    let computed_hash = Self::hash_data(&content);

                    if computed_hash != file_hash.hash {
                        return Ok(VerificationResult {
                            valid: false,
                            error: Some(format!("File corrupted: {}", file_hash.path)),
                            duration: start.elapsed(),
                        });
                    }
                }

                Ok(VerificationResult {
                    valid: true,
                    error: None,
                    duration: start.elapsed(),
                })
            }
            Err(_) => Ok(VerificationResult {
                valid: false,
                error: Some("Invalid signature".to_string()),
                duration: start.elapsed(),
            }),
        }
    }

    fn hash_data(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn key_fingerprint(key: &PublicKey) -> String {
        Self::hash_data(&key.to_bytes())
    }

    pub async fn check_revocation(&self, key_id: &str)
        -> Result<bool, VerificationError> {
        // In production, query OCSP/CRL server
        // For now, check local cache
        Ok(self.revocation_list.read().unwrap().contains(key_id))
    }

    pub fn add_revoked_key(&self, key_id: String) {
        self.revocation_list.write().unwrap().insert(key_id);
    }
}

#[derive(Debug, Serialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub error: Option<String>,
    pub duration: std::time::Duration,
}
```

---

### 3. Module Discovery API

**Purpose:** High-level API for applications to discover and interact with modules

**Implementation:**
```rust
// crates/tauri-plugin-electron/src/api/mod.rs

use tauri::{command, Runtime, State};
use crate::registry::{ModuleRegistry, ModuleEntry, ModuleStatus};
use crate::verification::SignatureVerifier;
use semver::{Version, VersionReq};

pub struct ModuleManager {
    registry: ModuleRegistry,
    verifier: SignatureVerifier,
}

#[command]
pub async fn list_electron_modules<R: Runtime>(
    manager: State<'_, ModuleManager>,
) -> Result<Vec<ModuleInfo>, String> {
    let entries = manager.registry
        .list_modules(Some("electron-shell"))
        .map_err(|e| e.to_string())?;

    Ok(entries.into_iter().map(|e| e.into()).collect())
}

#[command]
pub async fn get_best_electron_module<R: Runtime>(
    version_req: Option<String>,
    manager: State<'_, ModuleManager>,
) -> Result<Option<ModuleInfo>, String> {
    let entries = manager.registry
        .list_modules(Some("electron-shell"))
        .map_err(|e| e.to_string())?;

    // Filter healthy modules
    let healthy: Vec<_> = entries.into_iter()
        .filter(|e| matches!(e.status, ModuleStatus::Healthy))
        .collect();

    if healthy.is_empty() {
        return Ok(None);
    }

    // Match version requirement
    let best = if let Some(req_str) = version_req {
        let req = VersionReq::parse(&req_str).map_err(|e| e.to_string())?;
        healthy.into_iter()
            .filter(|e| req.matches(&e.version))
            .max_by(|a, b| a.version.cmp(&b.version))
    } else {
        // Return latest version
        healthy.into_iter()
            .max_by(|a, b| a.version.cmp(&b.version))
    };

    Ok(best.map(|e| e.into()))
}

#[command]
pub async fn verify_electron_module<R: Runtime>(
    id: String,
    version: String,
    manager: State<'_, ModuleManager>,
) -> Result<VerificationStatus, String> {
    let version = Version::parse(&version).map_err(|e| e.to_string())?;

    // Get module entry
    let entry = manager.registry
        .get_module(&id, &version)
        .map_err(|e| e.to_string())?
        .ok_or("Module not found")?;

    // Verify signature
    let result = manager.verifier
        .verify_module(&entry.install_path)
        .await
        .map_err(|e| e.to_string())?;

    // Update registry status
    let new_status = if result.valid {
        ModuleStatus::Healthy
    } else {
        ModuleStatus::SignatureInvalid
    };

    manager.registry
        .update_status(&id, &version, new_status)
        .map_err(|e| e.to_string())?;

    Ok(VerificationStatus {
        valid: result.valid,
        error: result.error,
        verified_at: chrono::Utc::now(),
    })
}

#[command]
pub async fn check_electron_compatibility<R: Runtime>(
    module_version: String,
    tauri_version: String,
    manager: State<'_, ModuleManager>,
) -> Result<CompatibilityResult, String> {
    let module_ver = Version::parse(&module_version).map_err(|e| e.to_string())?;
    let tauri_ver = Version::parse(&tauri_version).map_err(|e| e.to_string())?;

    // Query compatibility table
    // For Phase 2.2, use simple version matching
    // Phase 2.3+ will use full compatibility matrix

    let compatible = module_ver.major == tauri_ver.major
                     && module_ver.minor <= tauri_ver.minor;

    Ok(CompatibilityResult {
        compatible,
        reason: if compatible {
            None
        } else {
            Some(format!("Module requires Tauri {}.x, found {}",
                        module_ver.major, tauri_ver))
        },
    })
}

#[command]
pub async fn get_registry_health<R: Runtime>(
    manager: State<'_, ModuleManager>,
) -> Result<RegistryHealthStatus, String> {
    let health = manager.registry
        .health_check()
        .map_err(|e| e.to_string())?;

    Ok(health.into())
}

// DTOs for frontend
#[derive(Debug, Serialize)]
pub struct ModuleInfo {
    pub id: String,
    pub version: String,
    pub install_path: String,
    pub status: String,
    pub installed_at: String,
    pub last_verified: Option<String>,
}

impl From<ModuleEntry> for ModuleInfo {
    fn from(entry: ModuleEntry) -> Self {
        Self {
            id: entry.id,
            version: entry.version.to_string(),
            install_path: entry.install_path.display().to_string(),
            status: format!("{:?}", entry.status),
            installed_at: entry.installed_at.to_rfc3339(),
            last_verified: entry.last_verified.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct VerificationStatus {
    pub valid: bool,
    pub error: Option<String>,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CompatibilityResult {
    pub compatible: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegistryHealthStatus {
    pub total_modules: i64,
    pub healthy: i64,
    pub unhealthy: i64,
    pub last_check: String,
}
```

---

### 4. Frontend Integration (TypeScript)

**Purpose:** JavaScript API for frontend applications

**Implementation:**
```typescript
// packages/api/src/electron-modules.ts

import { invoke } from '@tauri-apps/api/core';

export interface ModuleInfo {
  id: string;
  version: string;
  installPath: string;
  status: 'Healthy' | 'VerificationPending' | 'SignatureInvalid' | 'Corrupted';
  installedAt: string;
  lastVerified?: string;
}

export interface VerificationStatus {
  valid: boolean;
  error?: string;
  verifiedAt: string;
}

export interface CompatibilityResult {
  compatible: boolean;
  reason?: string;
}

export interface RegistryHealth {
  totalModules: number;
  healthy: number;
  unhealthy: number;
  lastCheck: string;
}

/**
 * List all installed Electron modules
 */
export async function listElectronModules(): Promise<ModuleInfo[]> {
  return await invoke('plugin:electron|list_electron_modules');
}

/**
 * Get the best matching Electron module for the given version requirement
 * @param versionReq - Semver requirement (e.g., ">=1.0.0, <2.0.0")
 */
export async function getBestElectronModule(
  versionReq?: string
): Promise<ModuleInfo | null> {
  return await invoke('plugin:electron|get_best_electron_module', {
    versionReq
  });
}

/**
 * Verify the signature of an installed Electron module
 */
export async function verifyElectronModule(
  id: string,
  version: string
): Promise<VerificationStatus> {
  return await invoke('plugin:electron|verify_electron_module', {
    id,
    version
  });
}

/**
 * Check if a module version is compatible with the current Tauri version
 */
export async function checkElectronCompatibility(
  moduleVersion: string,
  tauriVersion: string
): Promise<CompatibilityResult> {
  return await invoke('plugin:electron|check_electron_compatibility', {
    moduleVersion,
    tauriVersion,
  });
}

/**
 * Get health status of the module registry
 */
export async function getRegistryHealth(): Promise<RegistryHealth> {
  return await invoke('plugin:electron|get_registry_health');
}

/**
 * High-level helper: Get a verified, compatible Electron module
 */
export async function getVerifiedElectronModule(
  versionReq?: string
): Promise<ModuleInfo | null> {
  const module = await getBestElectronModule(versionReq);

  if (!module) {
    return null;
  }

  // Verify if not recently verified
  if (module.status !== 'Healthy' || !module.lastVerified) {
    const verification = await verifyElectronModule(module.id, module.version);

    if (!verification.valid) {
      console.error('Module verification failed:', verification.error);
      return null;
    }
  }

  return module;
}
```

---

## 📝 Task Breakdown

### Task 1: Registry Infrastructure (Weeks 1-3)

**Owner:** Lead Developer + DevOps Engineer

**Deliverables:**
- [ ] SQLite schema designed and tested
- [ ] Module registry Rust implementation
- [ ] LRU cache for performance
- [ ] Database migrations system
- [ ] Registry health checks
- [ ] Unit tests (>90% coverage)

**Acceptance Criteria:**
- Registry handles 1000+ modules without performance degradation
- Cache hit rate >80% in typical usage
- Database operations <10ms (99th percentile)
- Automatic corruption recovery
- Thread-safe concurrent access

**Dependencies:** None (can start immediately after Phase 2.1)

---

### Task 2: Signature Verification (Weeks 2-5)

**Owner:** Security Engineer + Lead Developer

**Deliverables:**
- [ ] Ed25519 signature verification implementation
- [ ] Module signing script for CI/CD
- [ ] Trusted key management system
- [ ] Revocation checking (OCSP/CRL)
- [ ] Verification audit logging
- [ ] Performance benchmarks

**Acceptance Criteria:**
- Signature verification <50ms per module
- Zero false positives/negatives in testing
- Handles revoked keys correctly
- Tamper detection for all module files
- Comprehensive security audit passed

**Dependencies:** Task 1 (registry for storing verification results)

---

### Task 3: Discovery API (Weeks 4-6)

**Owner:** Lead Developer + Frontend Developer

**Deliverables:**
- [ ] Rust command implementations
- [ ] TypeScript API bindings
- [ ] Version compatibility checking
- [ ] Module selection algorithm
- [ ] API documentation
- [ ] Integration tests

**Acceptance Criteria:**
- API response time <50ms (cached)
- Correct version resolution in all test cases
- Type-safe TypeScript bindings
- Comprehensive error handling
- Example applications work correctly

**Dependencies:** Task 1 (registry), Task 2 (verification)

---

### Task 4: Performance & Caching (Weeks 5-7)

**Owner:** Lead Developer + DevOps Engineer

**Deliverables:**
- [ ] Multi-level caching strategy
- [ ] Performance benchmarks
- [ ] Memory usage optimization
- [ ] Background verification tasks
- [ ] Monitoring instrumentation

**Acceptance Criteria:**
- Module lookup <100ms (cold cache)
- Module lookup <10ms (warm cache)
- Memory usage <10MB for typical registry
- Background verification doesn't block UI
- Prometheus metrics exported

**Dependencies:** Task 3 (API complete for testing)

---

### Task 5: Diagnostics & Health (Weeks 7-9)

**Owner:** QA Engineer + Lead Developer

**Deliverables:**
- [ ] Health check endpoints
- [ ] Diagnostic CLI commands
- [ ] Registry repair tools
- [ ] Troubleshooting documentation
- [ ] Admin UI (optional)

**Acceptance Criteria:**
- Health checks detect all known failure modes
- Repair tools fix common corruptions
- Clear error messages for users
- Diagnostic data helps support team
- CLI tools easy to use

**Dependencies:** Task 3 (API for diagnostics)

---

### Task 6: Security Audit & Documentation (Weeks 10-12)

**Owner:** Security Engineer + Technical Writer

**Deliverables:**
- [ ] External security audit completed
- [ ] All audit findings remediated
- [ ] Security documentation
- [ ] API reference documentation
- [ ] Integration guide
- [ ] Migration guide (from Phase 2.1)

**Acceptance Criteria:**
- Zero critical/high security findings
- Documentation covers all APIs
- Integration examples for common scenarios
- Migration path clearly documented
- Developer feedback incorporated

**Dependencies:** Tasks 1-5 (all features complete)

---

## 🏃 Sprint Structure

### Sprint 1-2 (Weeks 1-4): Foundation
- **Goal:** Registry and verification systems operational
- **Demos:** Registry CRUD, signature verification working
- **Risks:** Database design changes, crypto library integration

### Sprint 3-4 (Weeks 5-8): Integration
- **Goal:** Full API surface complete and tested
- **Demos:** Discovery API, caching, frontend integration
- **Risks:** Performance issues, API design feedback

### Sprint 5-6 (Weeks 9-12): Polish & Security
- **Goal:** Production-ready with security audit passed
- **Demos:** Health monitoring, diagnostics, documentation
- **Risks:** Audit findings, documentation scope creep

---

## 👥 Team & Resources

### Core Team (Full-time)
- **Lead Developer** (12 weeks)
  - Architecture, registry, discovery API
  - Rate: $150/hr × 40hr/wk × 12wks = $72,000

- **Security Engineer** (12 weeks)
  - Signature verification, security audit coordination
  - Rate: $160/hr × 40hr/wk × 12wks = $76,800

- **DevOps Engineer** (8 weeks)
  - Performance, caching, monitoring infrastructure
  - Rate: $140/hr × 40hr/wk × 8wks = $44,800

### Supporting Team (Part-time)
- **Frontend Developer** (50% × 8 weeks)
  - TypeScript API, integration examples
  - Rate: $130/hr × 20hr/wk × 8wks = $20,800

- **QA Engineer** (50% × 10 weeks)
  - Testing, diagnostics, health checks
  - Rate: $110/hr × 20hr/wk × 10wks = $22,000

- **Technical Writer** (50% × 6 weeks)
  - Documentation, migration guides
  - Rate: $100/hr × 20hr/wk × 6wks = $12,000

**Total Personnel:** $248,400

---

## 💰 Budget Breakdown

| Category | Item | Cost |
|----------|------|------|
| **Personnel** | Core team (3 FT) | $193,600 |
| | Supporting team (3 PT) | $54,800 |
| **Infrastructure** | Cloud testing environments | $2,500 |
| | Database & storage services | $1,200 |
| | Monitoring & logging (3 months) | $900 |
| **Security** | External security audit | $15,000 |
| | Code signing certificates (renewal) | $800 |
| | Penetration testing | $8,000 |
| **Tools** | Development licenses | $1,200 |
| | Testing tools & services | $800 |
| **Contingency** | 10% buffer | $27,880 |
| **TOTAL** | | **$306,680** |

**Revised Budget Range:** $280K - $330K (higher due to security focus)

---

## 🎯 Success Criteria

### Functional Requirements
- ✅ Applications can discover installed modules at runtime
- ✅ All modules verified before use
- ✅ Version compatibility correctly determined
- ✅ Registry handles 1000+ modules efficiently
- ✅ Health status accurate and actionable

### Performance Requirements
- ✅ Module discovery <100ms (cold) / <10ms (warm)
- ✅ Signature verification <50ms per module
- ✅ API response time <50ms (99th percentile)
- ✅ Memory usage <10MB for typical registry
- ✅ Zero blocking operations on UI thread

### Security Requirements
- ✅ External security audit passed (zero critical findings)
- ✅ All modules cryptographically verified
- ✅ Revocation checking operational
- ✅ Tamper detection for all files
- ✅ Audit logging for all verification operations

### Quality Requirements
- ✅ Test coverage >85% for all components
- ✅ Zero known bugs at release
- ✅ Documentation complete and accurate
- ✅ Developer feedback positive (>4/5 rating)
- ✅ Migration from Phase 2.1 smooth (<1 hour)

---

## 🚨 Risk Management

### High-Impact Risks

#### 1. Performance Degradation
- **Probability:** Medium
- **Impact:** High
- **Mitigation:**
  - Benchmark early and often
  - Multi-level caching strategy
  - Async operations for all I/O
  - Load testing with 10,000+ modules
- **Contingency:** Simplify features, optimize critical paths

#### 2. Security Vulnerabilities
- **Probability:** Medium
- **Impact:** Critical
- **Mitigation:**
  - Security-first design (threat modeling)
  - External audit at week 10
  - Penetration testing
  - Regular dependency updates
- **Contingency:** Delay release until all findings resolved

#### 3. Database Corruption
- **Probability:** Low
- **Impact:** High
- **Mitigation:**
  - Write-ahead logging (WAL mode)
  - Automatic backups
  - Corruption detection on startup
  - Self-healing repair tools
- **Contingency:** Registry rebuild from installed modules

### Medium-Impact Risks

#### 4. API Design Issues
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Early prototype testing
  - Developer feedback sessions
  - Iterative design refinement
- **Contingency:** Breaking changes acceptable in Phase 2.x

#### 5. Cross-Platform Issues
- **Probability:** High
- **Impact:** Medium
- **Mitigation:**
  - Test on all platforms weekly
  - Platform-specific code isolated
  - CI/CD testing matrix
- **Contingency:** Platform-specific workarounds

---

## 📊 Milestones & Timeline

```
Week 1-3: Registry Infrastructure
├─ Week 1: Schema design, database setup
├─ Week 2: Registry implementation, basic CRUD
└─ Week 3: Caching, performance testing
    ✓ Milestone: Registry operational

Week 2-5: Signature Verification
├─ Week 2-3: Verification implementation
├─ Week 4: Revocation checking, audit logging
└─ Week 5: Performance optimization, testing
    ✓ Milestone: Verification system complete

Week 4-6: Discovery API
├─ Week 4-5: Rust commands, TypeScript bindings
└─ Week 6: Integration testing, examples
    ✓ Milestone: API surface complete

Week 5-7: Performance & Caching
├─ Week 5-6: Caching strategy, benchmarking
└─ Week 7: Optimization, monitoring
    ✓ Milestone: Performance targets met

Week 7-9: Diagnostics & Health
├─ Week 7-8: Health checks, diagnostics
└─ Week 9: CLI tools, troubleshooting
    ✓ Milestone: Production-ready tooling

Week 10-12: Security & Documentation
├─ Week 10: External security audit
├─ Week 11: Remediation, documentation
└─ Week 12: Final testing, release prep
    ✓ Milestone: Phase 2.2 complete
```

### Key Dates
- **Week 1:** Kickoff, schema design review
- **Week 3:** Registry demo to stakeholders
- **Week 6:** API freeze, integration testing starts
- **Week 9:** Feature complete, security audit starts
- **Week 10:** Security audit results
- **Week 12:** Phase 2.2 release, handoff to Phase 2.3

---

## 🔄 Integration Points

### With Phase 2.1 (Foundation & Packaging)
- Uses module packaging format from Phase 2.1
- Builds on code signing infrastructure
- Extends update server for registry queries
- Shares CI/CD pipeline

### With Phase 2.3 (Installation & Updates)
- Registry provides module discovery
- Verification gates all installations
- Health checks guide update decisions
- Diagnostics help troubleshoot install failures

### With Phase 2.4+ (Future Phases)
- Registry tracks module usage metrics
- Verification logs feed analytics
- Health data guides deprecation decisions

---

## 📚 Documentation Deliverables

1. **API Reference**
   - All Rust commands documented
   - TypeScript bindings with examples
   - Error codes and handling

2. **Integration Guide**
   - Step-by-step setup
   - Common patterns and recipes
   - Migration from Phase 2.1

3. **Security Documentation**
   - Threat model
   - Verification flow
   - Key management procedures
   - Audit results summary

4. **Operations Guide**
   - Health monitoring
   - Troubleshooting common issues
   - Registry maintenance
   - Performance tuning

5. **Developer Guide**
   - Architecture overview
   - Extension points
   - Testing strategies
   - Contributing guidelines

---

## ✅ Phase 2.2 Complete When

- [ ] Registry handles 1000+ modules with <100ms queries
- [ ] All modules verified before use
- [ ] Zero critical/high security findings
- [ ] Test coverage >85%
- [ ] Performance benchmarks met
- [ ] Documentation complete
- [ ] External security audit passed
- [ ] Developer preview feedback positive
- [ ] Migration from Phase 2.1 tested
- [ ] Handoff to Phase 2.3 team complete

---

## 📞 Stakeholder Communication

### Weekly Updates
- Progress against milestones
- Blockers and risks
- Upcoming decisions

### Demos
- Week 3: Registry CRUD operations
- Week 6: Discovery API integration
- Week 9: Complete system walkthrough
- Week 12: Final release demo

### Decision Points
- Week 2: Database schema approval
- Week 5: API design freeze
- Week 10: Security audit go/no-go
- Week 12: Release approval

---

## 🔗 Related Documents

- **Phase 2 Overview:** [../../PHASE_2_PLANNING.md](../../PHASE_2_PLANNING.md)
- **Phase 2.1 Plan:** [PHASE_2_1_PLAN.md](./PHASE_2_1_PLAN.md)
- **Phase 2.1 Resources:** [PHASE_2_1_RESOURCE_PLAN.md](./PHASE_2_1_RESOURCE_PLAN.md)
- **Verification Report:** [../../VERIFICATION_REPORT.md](../../VERIFICATION_REPORT.md)

---

**Status:** Ready for Review
**Next Action:** Stakeholder approval, team allocation
**Target Start Date:** April 2026
**Target Completion:** July 2026
