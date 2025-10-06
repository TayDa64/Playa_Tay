# Phase 2.4: Distribution & CDN Infrastructure

**Status:** Planning
**Duration:** 10 weeks (October - December 2026)
**Dependencies:** Phase 2.3 (Installation & Updates) must be complete
**Budget Estimate:** $215K - $260K

---

## 📋 Executive Summary

Phase 2.4 establishes the production-grade content delivery infrastructure for the optional module ecosystem. This phase focuses on global distribution, performance optimization, cost efficiency, and reliability. By leveraging CDN technology, geographic distribution, and intelligent caching, we ensure fast, reliable module delivery to users worldwide.

**Core Deliverables:**
1. Multi-CDN architecture with automatic failover
2. Update server with versioned module catalog
3. Delta patch generation and serving
4. Geographic load balancing and edge caching
5. Bandwidth optimization and cost monitoring
6. Analytics and telemetry infrastructure

---

## 🎯 Objectives

### Primary Goals
- **Global Performance**: <100ms latency to CDN edge in 95% of locations
- **High Availability**: 99.95% uptime with automatic failover
- **Cost Efficiency**: <$0.50 per GB transferred at scale
- **Fast Downloads**: Average download speed >10MB/s globally
- **Bandwidth Optimization**: Delta updates reduce traffic by 60%+

### Success Metrics
- Average CDN latency <100ms (95th percentile)
- Download success rate >99.9%
- Average download speed >10MB/s
- CDN failover time <5 seconds
- Bandwidth cost <$15K/month at 100K active users
- Delta patch generation time <5 minutes per release

---

## 🏗️ Architecture Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                         Global Architecture                       │
└──────────────────────────────────────────────────────────────────┘

                    ┌───────────────────────┐
                    │   Origin Servers      │
                    │   (Multi-Region)      │
                    │  - Module storage     │
                    │  - Delta patches      │
                    │  - Metadata catalog   │
                    └──────────┬────────────┘
                               │
                               │ Push/Sync
                               │
        ┏━━━━━━━━━━━━━━━━━━━━━━▼━━━━━━━━━━━━━━━━━━━━━━┓
        ┃         CDN Layer (Multi-Provider)           ┃
        ┃                                              ┃
        ┃  ┌─────────────┐  ┌─────────────┐          ┃
        ┃  │ CloudFront  │  │  Fastly     │          ┃
        ┃  │   (Primary) │  │  (Backup)   │          ┃
        ┃  └─────────────┘  └─────────────┘          ┃
        ┃                                              ┃
        ┃  Edge Caching │ Gzip/Brotli │ TLS 1.3      ┃
        ┗━━━━━━━━━━━━━━━━━━━━━┬━━━━━━━━━━━━━━━━━━━━━━┛
                               │
                               │ HTTPS
                               │
        ┌──────────────────────▼───────────────────────┐
        │          Update Server (API)                 │
        │  - Version catalog                           │
        │  - Module metadata                           │
        │  - Client authentication                     │
        │  - Analytics collection                      │
        └──────────────────────┬───────────────────────┘
                               │
                               │ Queries
                               │
        ┌──────────────────────▼───────────────────────┐
        │        Metadata Database                     │
        │  - Module versions                           │
        │  - Download URLs                             │
        │  - Checksums                                 │
        │  - Release metadata                          │
        └──────────────────────────────────────────────┘

                               │
                               │ Analytics
                               ▼
        ┌──────────────────────────────────────────────┐
        │      Analytics Pipeline                      │
        │  - Download stats                            │
        │  - Geographic distribution                   │
        │  - Error tracking                            │
        │  - Cost monitoring                           │
        └──────────────────────────────────────────────┘
```

---

## 📦 Core Components

### 1. Update Server (API)

**Purpose:** Central API for module discovery, version checking, and download URL generation

**Implementation:**
```rust
// crates/update-server/src/main.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    cdn_manager: Arc<CdnManager>,
    analytics: Arc<AnalyticsCollector>,
}

#[tokio::main]
async fn main() {
    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let cdn_manager = Arc::new(CdnManager::new().await);
    let analytics = Arc::new(AnalyticsCollector::new());

    let state = AppState {
        db,
        cdn_manager,
        analytics,
    };

    let app = Router::new()
        .route("/api/v1/catalog", get(get_catalog))
        .route("/api/v1/modules/:id", get(get_module_info))
        .route("/api/v1/modules/:id/versions", get(get_module_versions))
        .route("/api/v1/download/:id/:version", get(get_download_url))
        .route("/api/v1/check-updates", post(check_updates))
        .route("/api/v1/delta/:id/:from/:to", get(get_delta_patch))
        .route("/api/v1/analytics", post(record_analytics))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Update server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

// Get full module catalog
async fn get_catalog(
    State(state): State<AppState>,
) -> Result<Json<ModuleCatalog>, ApiError> {
    let modules = sqlx::query_as!(
        ModuleRecord,
        r#"
        SELECT id, name, description, latest_version, download_count, updated_at
        FROM modules
        WHERE published = true
        ORDER BY name
        "#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(ModuleCatalog {
        modules: modules.into_iter().map(Into::into).collect(),
        timestamp: chrono::Utc::now(),
    }))
}

// Get specific module information
async fn get_module_info(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ModuleInfo>, ApiError> {
    let module = sqlx::query_as!(
        ModuleRecord,
        r#"
        SELECT id, name, description, latest_version, download_count, updated_at
        FROM modules
        WHERE id = $1 AND published = true
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(module.into()))
}

// Get all versions of a module
async fn get_module_versions(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ModuleVersion>>, ApiError> {
    let versions = sqlx::query_as!(
        VersionRecord,
        r#"
        SELECT version, released_at, size_bytes, checksum, release_notes, yanked
        FROM module_versions
        WHERE module_id = $1
        ORDER BY released_at DESC
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(versions.into_iter().map(Into::into).collect()))
}

// Get CDN download URL for a specific version
async fn get_download_url(
    Path((id, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<DownloadInfo>, ApiError> {
    // Verify module exists
    let version_info = sqlx::query!(
        r#"
        SELECT size_bytes, checksum, signature_checksum
        FROM module_versions
        WHERE module_id = $1 AND version = $2 AND yanked = false
        "#,
        id,
        version
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    // Generate CDN URL with signature
    let (primary_url, fallback_urls) = state.cdn_manager
        .generate_signed_urls(&id, &version)
        .await?;

    // Record request
    state.analytics.record_download_request(&id, &version).await;

    Ok(Json(DownloadInfo {
        module_id: id,
        version,
        primary_url,
        fallback_urls,
        size_bytes: version_info.size_bytes as u64,
        checksum: version_info.checksum,
        signature_url: format!("{}.sig", primary_url),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
    }))
}

// Check for available updates
#[derive(Deserialize)]
struct UpdateCheckRequest {
    modules: Vec<InstalledModule>,
    client_version: String,
}

#[derive(Deserialize)]
struct InstalledModule {
    id: String,
    version: String,
}

async fn check_updates(
    State(state): State<AppState>,
    Json(payload): Json<UpdateCheckRequest>,
) -> Result<Json<UpdateManifest>, ApiError> {
    let mut available_updates = Vec::new();

    for installed in payload.modules {
        // Query latest version
        let latest = sqlx::query!(
            r#"
            SELECT version, size_bytes, checksum, release_notes, critical
            FROM module_versions
            WHERE module_id = $1 AND yanked = false
            ORDER BY released_at DESC
            LIMIT 1
            "#,
            installed.id
        )
        .fetch_optional(&state.db)
        .await?;

        if let Some(latest) = latest {
            let current = semver::Version::parse(&installed.version)?;
            let new = semver::Version::parse(&latest.version)?;

            if new > current {
                // Check if delta patch available
                let delta = sqlx::query!(
                    r#"
                    SELECT size_bytes, checksum
                    FROM delta_patches
                    WHERE module_id = $1 AND from_version = $2 AND to_version = $3
                    "#,
                    installed.id,
                    installed.version,
                    latest.version
                )
                .fetch_optional(&state.db)
                .await?;

                let (download_url, _) = state.cdn_manager
                    .generate_signed_urls(&installed.id, &latest.version)
                    .await?;

                let delta_info = if let Some(delta) = delta {
                    let (delta_url, _) = state.cdn_manager
                        .generate_delta_url(&installed.id, &installed.version, &latest.version)
                        .await?;

                    Some(DeltaInfo {
                        url: delta_url,
                        size_bytes: delta.size_bytes as u64,
                        checksum: delta.checksum,
                    })
                } else {
                    None
                };

                available_updates.push(AvailableUpdate {
                    module_id: installed.id.clone(),
                    current_version: installed.version,
                    new_version: latest.version,
                    download_url,
                    size_bytes: latest.size_bytes as u64,
                    checksum: latest.checksum,
                    release_notes: latest.release_notes,
                    critical: latest.critical,
                    delta: delta_info,
                });
            }
        }
    }

    Ok(Json(UpdateManifest {
        available_updates,
        timestamp: chrono::Utc::now(),
    }))
}

// Get delta patch information
async fn get_delta_patch(
    Path((id, from, to)): Path<(String, String, String)>,
    State(state): State<AppState>,
) -> Result<Json<DeltaPatchInfo>, ApiError> {
    let patch = sqlx::query!(
        r#"
        SELECT size_bytes, checksum, compression, created_at
        FROM delta_patches
        WHERE module_id = $1 AND from_version = $2 AND to_version = $3
        "#,
        id,
        from,
        to
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    let (url, _) = state.cdn_manager
        .generate_delta_url(&id, &from, &to)
        .await?;

    Ok(Json(DeltaPatchInfo {
        module_id: id,
        from_version: from,
        to_version: to,
        download_url: url,
        size_bytes: patch.size_bytes as u64,
        checksum: patch.checksum,
        compression: patch.compression,
    }))
}

// Record analytics event
#[derive(Deserialize)]
struct AnalyticsEvent {
    event_type: String,
    module_id: Option<String>,
    version: Option<String>,
    success: bool,
    error_code: Option<String>,
    client_info: ClientInfo,
}

#[derive(Deserialize)]
struct ClientInfo {
    platform: String,
    arch: String,
    client_version: String,
    country: Option<String>,
}

async fn record_analytics(
    State(state): State<AppState>,
    Json(event): Json<AnalyticsEvent>,
) -> Result<StatusCode, ApiError> {
    state.analytics.record_event(event).await?;
    Ok(StatusCode::ACCEPTED)
}

// Health check endpoint
async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let db_healthy = state.db.acquire().await.is_ok();
    let cdn_healthy = state.cdn_manager.health_check().await;

    if db_healthy && cdn_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

// DTOs
#[derive(Serialize)]
struct ModuleCatalog {
    modules: Vec<ModuleInfo>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct ModuleInfo {
    id: String,
    name: String,
    description: String,
    latest_version: String,
    download_count: i64,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct ModuleVersion {
    version: String,
    released_at: chrono::DateTime<chrono::Utc>,
    size_bytes: u64,
    checksum: String,
    release_notes: String,
    yanked: bool,
}

#[derive(Serialize)]
struct DownloadInfo {
    module_id: String,
    version: String,
    primary_url: String,
    fallback_urls: Vec<String>,
    size_bytes: u64,
    checksum: String,
    signature_url: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct UpdateManifest {
    available_updates: Vec<AvailableUpdate>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct AvailableUpdate {
    module_id: String,
    current_version: String,
    new_version: String,
    download_url: String,
    size_bytes: u64,
    checksum: String,
    release_notes: String,
    critical: bool,
    delta: Option<DeltaInfo>,
}

#[derive(Serialize)]
struct DeltaInfo {
    url: String,
    size_bytes: u64,
    checksum: String,
}

#[derive(Serialize)]
struct DeltaPatchInfo {
    module_id: String,
    from_version: String,
    to_version: String,
    download_url: String,
    size_bytes: u64,
    checksum: String,
    compression: String,
}

// Database records
struct ModuleRecord {
    id: String,
    name: String,
    description: String,
    latest_version: String,
    download_count: i64,
    updated_at: chrono::DateTime<chrono::Utc>,
}

struct VersionRecord {
    version: String,
    released_at: chrono::DateTime<chrono::Utc>,
    size_bytes: i64,
    checksum: String,
    release_notes: String,
    yanked: bool,
}

// Error handling
enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
    CdnError(String),
    InvalidVersion(semver::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::CdnError(_) => (StatusCode::SERVICE_UNAVAILABLE, "CDN error"),
            ApiError::InvalidVersion(_) => (StatusCode::BAD_REQUEST, "Invalid version"),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::DatabaseError(err)
    }
}

impl From<semver::Error> for ApiError {
    fn from(err: semver::Error) -> Self {
        ApiError::InvalidVersion(err)
    }
}
```

---

### 2. CDN Manager (Multi-Provider)

**Purpose:** Intelligent routing across multiple CDN providers with automatic failover

**Implementation:**
```rust
// crates/update-server/src/cdn_manager.rs

use aws_sdk_cloudfront::Client as CloudFrontClient;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CdnManager {
    cloudfront: CloudFrontClient,
    fastly_client: HttpClient,
    config: CdnConfig,
    health_status: Arc<RwLock<CdnHealthStatus>>,
}

#[derive(Clone)]
struct CdnConfig {
    cloudfront_distribution_id: String,
    cloudfront_domain: String,
    fastly_service_id: String,
    fastly_domain: String,
    signing_key: String,
}

#[derive(Default)]
struct CdnHealthStatus {
    cloudfront_healthy: bool,
    fastly_healthy: bool,
    last_check: Option<chrono::DateTime<chrono::Utc>>,
}

impl CdnManager {
    pub async fn new() -> Self {
        let aws_config = aws_config::load_from_env().await;
        let cloudfront = CloudFrontClient::new(&aws_config);

        let fastly_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        let config = CdnConfig {
            cloudfront_distribution_id: std::env::var("CLOUDFRONT_DISTRIBUTION_ID").unwrap(),
            cloudfront_domain: std::env::var("CLOUDFRONT_DOMAIN").unwrap(),
            fastly_service_id: std::env::var("FASTLY_SERVICE_ID").unwrap(),
            fastly_domain: std::env::var("FASTLY_DOMAIN").unwrap(),
            signing_key: std::env::var("CDN_SIGNING_KEY").unwrap(),
        };

        let manager = Self {
            cloudfront,
            fastly_client,
            config,
            health_status: Arc::new(RwLock::new(CdnHealthStatus::default())),
        };

        // Start background health checks
        manager.start_health_checks();

        manager
    }

    /// Generate signed URLs for primary and fallback CDNs
    pub async fn generate_signed_urls(
        &self,
        module_id: &str,
        version: &str,
    ) -> Result<(String, Vec<String>), CdnError> {
        let filename = format!("{}-{}.tar.gz", module_id, version);
        let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

        let health = self.health_status.read().await;

        let primary_url = if health.cloudfront_healthy {
            self.generate_cloudfront_url(&filename, expiry)?
        } else if health.fastly_healthy {
            self.generate_fastly_url(&filename, expiry)?
        } else {
            return Err(CdnError::AllProvidersDown);
        };

        let mut fallback_urls = Vec::new();

        // Add Fastly as fallback if CloudFront is primary
        if health.cloudfront_healthy && health.fastly_healthy {
            fallback_urls.push(self.generate_fastly_url(&filename, expiry)?);
        }

        Ok((primary_url, fallback_urls))
    }

    /// Generate signed URL for delta patch
    pub async fn generate_delta_url(
        &self,
        module_id: &str,
        from_version: &str,
        to_version: &str,
    ) -> Result<(String, Vec<String>), CdnError> {
        let filename = format!("deltas/{}-{}-to-{}.delta", module_id, from_version, to_version);
        let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

        let primary_url = self.generate_cloudfront_url(&filename, expiry)?;
        let fallback_url = self.generate_fastly_url(&filename, expiry)?;

        Ok((primary_url, vec![fallback_url]))
    }

    fn generate_cloudfront_url(
        &self,
        filename: &str,
        expiry: chrono::DateTime<chrono::Utc>,
    ) -> Result<String, CdnError> {
        let url = format!("https://{}/modules/{}", self.config.cloudfront_domain, filename);

        // Generate CloudFront signed URL
        let policy = format!(
            r#"{{"Statement":[{{"Resource":"{}","Condition":{{"DateLessThan":{{"AWS:EpochTime":{}}}}}}}]}}"#,
            url,
            expiry.timestamp()
        );

        // Sign with private key (simplified - use aws-sdk in production)
        let signature = self.sign_policy(&policy)?;

        Ok(format!(
            "{}?Expires={}&Signature={}&Key-Pair-Id={}",
            url,
            expiry.timestamp(),
            signature,
            "CLOUDFRONT_KEY_PAIR_ID"
        ))
    }

    fn generate_fastly_url(
        &self,
        filename: &str,
        expiry: chrono::DateTime<chrono::Utc>,
    ) -> Result<String, CdnError> {
        let url = format!("https://{}/modules/{}", self.config.fastly_domain, filename);

        // Generate Fastly signed URL
        let token = self.generate_fastly_token(&url, expiry)?;

        Ok(format!("{}?token={}", url, token))
    }

    fn sign_policy(&self, policy: &str) -> Result<String, CdnError> {
        use base64::{engine::general_purpose, Engine as _};
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(policy.as_bytes());
        hasher.update(self.config.signing_key.as_bytes());

        Ok(general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize()))
    }

    fn generate_fastly_token(
        &self,
        url: &str,
        expiry: chrono::DateTime<chrono::Utc>,
    ) -> Result<String, CdnError> {
        // Simplified Fastly token generation
        let payload = format!("{}:{}", url, expiry.timestamp());
        self.sign_policy(&payload)
    }

    /// Check health of all CDN providers
    pub async fn health_check(&self) -> bool {
        let mut status = self.health_status.write().await;

        // Check CloudFront
        status.cloudfront_healthy = self.check_cloudfront_health().await;

        // Check Fastly
        status.fastly_healthy = self.check_fastly_health().await;

        status.last_check = Some(chrono::Utc::now());

        status.cloudfront_healthy || status.fastly_healthy
    }

    async fn check_cloudfront_health(&self) -> bool {
        let test_url = format!("https://{}/health", self.config.cloudfront_domain);

        reqwest::get(&test_url)
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    async fn check_fastly_health(&self) -> bool {
        let test_url = format!("https://{}/health", self.config.fastly_domain);

        self.fastly_client
            .get(&test_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    fn start_health_checks(&self) {
        let manager = self.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

            loop {
                interval.tick().await;
                manager.health_check().await;
            }
        });
    }
}

impl Clone for CdnManager {
    fn clone(&self) -> Self {
        Self {
            cloudfront: self.cloudfront.clone(),
            fastly_client: self.fastly_client.clone(),
            config: self.config.clone(),
            health_status: Arc::clone(&self.health_status),
        }
    }
}

#[derive(Debug)]
pub enum CdnError {
    SigningError(String),
    AllProvidersDown,
}
```

---

### 3. Delta Patch Generator

**Purpose:** Generate efficient binary diffs between module versions

**Implementation:**
```rust
// crates/delta-generator/src/main.rs

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Read, Write};
use flate2::write::GzEncoder;
use flate2::Compression;
use bsdiff::diff;

pub struct DeltaGenerator {
    workspace: PathBuf,
}

impl DeltaGenerator {
    pub fn new(workspace: PathBuf) -> Self {
        fs::create_dir_all(&workspace).unwrap();
        Self { workspace }
    }

    /// Generate delta patch between two module versions
    pub async fn generate_delta(
        &self,
        module_id: &str,
        from_version: &str,
        to_version: &str,
        from_archive: &Path,
        to_archive: &Path,
    ) -> Result<DeltaResult, DeltaError> {
        let start = std::time::Instant::now();

        // Extract both archives
        let from_dir = self.extract_archive(from_archive, "from").await?;
        let to_dir = self.extract_archive(to_archive, "to").await?;

        // Generate file-level diffs
        let patches = self.generate_file_patches(&from_dir, &to_dir).await?;

        // Create delta package
        let delta_path = self.workspace.join(format!(
            "{}-{}-to-{}.delta",
            module_id, from_version, to_version
        ));

        let compressed_size = self.create_delta_package(&patches, &delta_path).await?;

        // Calculate checksums
        let checksum = self.calculate_checksum(&delta_path).await?;

        // Cleanup
        fs::remove_dir_all(&from_dir)?;
        fs::remove_dir_all(&to_dir)?;

        let original_size = fs::metadata(to_archive)?.len();
        let savings_percent = (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0;

        Ok(DeltaResult {
            delta_path,
            size_bytes: compressed_size,
            checksum,
            original_size,
            savings_percent,
            generation_time: start.elapsed(),
        })
    }

    async fn extract_archive(&self, archive: &Path, prefix: &str) -> Result<PathBuf, DeltaError> {
        let extract_dir = self.workspace.join(format!("extract-{}", prefix));
        fs::create_dir_all(&extract_dir)?;

        let file = fs::File::open(archive)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);

        archive.unpack(&extract_dir)?;

        Ok(extract_dir)
    }

    async fn generate_file_patches(
        &self,
        from_dir: &Path,
        to_dir: &Path,
    ) -> Result<Vec<FilePatch>, DeltaError> {
        let mut patches = Vec::new();

        // Walk destination directory
        for entry in walkdir::WalkDir::new(to_dir) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            let rel_path = entry.path().strip_prefix(to_dir).unwrap();
            let from_path = from_dir.join(rel_path);
            let to_path = entry.path();

            let patch = if from_path.exists() {
                // File exists in both versions - create diff
                self.create_binary_diff(&from_path, to_path, rel_path).await?
            } else {
                // New file - include full content
                FilePatch::NewFile {
                    path: rel_path.to_path_buf(),
                    content: fs::read(to_path)?,
                }
            };

            patches.push(patch);
        }

        // Check for deleted files
        for entry in walkdir::WalkDir::new(from_dir) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            let rel_path = entry.path().strip_prefix(from_dir).unwrap();
            let to_path = to_dir.join(rel_path);

            if !to_path.exists() {
                patches.push(FilePatch::DeletedFile {
                    path: rel_path.to_path_buf(),
                });
            }
        }

        Ok(patches)
    }

    async fn create_binary_diff(
        &self,
        from: &Path,
        to: &Path,
        rel_path: &Path,
    ) -> Result<FilePatch, DeltaError> {
        let from_data = fs::read(from)?;
        let to_data = fs::read(to)?;

        // Use bsdiff for binary diffing
        let mut patch_data = Vec::new();
        diff(&from_data, &to_data, &mut patch_data)?;

        // Only use diff if it's smaller than full file
        if patch_data.len() < to_data.len() {
            Ok(FilePatch::BinaryDiff {
                path: rel_path.to_path_buf(),
                diff: patch_data,
            })
        } else {
            Ok(FilePatch::ReplacedFile {
                path: rel_path.to_path_buf(),
                content: to_data,
            })
        }
    }

    async fn create_delta_package(
        &self,
        patches: &[FilePatch],
        output: &Path,
    ) -> Result<u64, DeltaError> {
        // Serialize patches
        let serialized = bincode::serialize(patches)?;

        // Compress with gzip
        let file = fs::File::create(output)?;
        let mut encoder = GzEncoder::new(file, Compression::best());
        encoder.write_all(&serialized)?;
        encoder.finish()?;

        Ok(fs::metadata(output)?.len())
    }

    async fn calculate_checksum(&self, path: &Path) -> Result<String, DeltaError> {
        use sha2::{Sha256, Digest};

        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 8192];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
enum FilePatch {
    NewFile {
        path: PathBuf,
        content: Vec<u8>,
    },
    BinaryDiff {
        path: PathBuf,
        diff: Vec<u8>,
    },
    ReplacedFile {
        path: PathBuf,
        content: Vec<u8>,
    },
    DeletedFile {
        path: PathBuf,
    },
}

pub struct DeltaResult {
    pub delta_path: PathBuf,
    pub size_bytes: u64,
    pub checksum: String,
    pub original_size: u64,
    pub savings_percent: f64,
    pub generation_time: std::time::Duration,
}

#[derive(Debug)]
pub enum DeltaError {
    IoError(std::io::Error),
    WalkdirError(walkdir::Error),
    BsdiffError(bsdiff::Error),
    SerializationError(bincode::Error),
}

impl From<std::io::Error> for DeltaError {
    fn from(err: std::io::Error) -> Self {
        DeltaError::IoError(err)
    }
}

impl From<walkdir::Error> for DeltaError {
    fn from(err: walkdir::Error) -> Self {
        DeltaError::WalkdirError(err)
    }
}

impl From<bsdiff::Error> for DeltaError {
    fn from(err: bsdiff::Error) -> Self {
        DeltaError::BsdiffError(err)
    }
}

impl From<bincode::Error> for DeltaError {
    fn from(err: bincode::Error) -> Self {
        DeltaError::SerializationError(err)
    }
}
```

---

### 4. Analytics & Telemetry

**Purpose:** Track usage, performance, and costs

**Implementation:**
```rust
// crates/update-server/src/analytics.rs

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct AnalyticsCollector {
    event_tx: mpsc::UnboundedSender<AnalyticsEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_type: String,
    pub module_id: Option<String>,
    pub version: Option<String>,
    pub success: bool,
    pub error_code: Option<String>,
    pub client_info: ClientInfo,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub platform: String,
    pub arch: String,
    pub client_version: String,
    pub country: Option<String>,
}

impl AnalyticsCollector {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Start background processor
        tokio::spawn(Self::process_events(rx));

        Self { event_tx: tx }
    }

    pub async fn record_event(&self, mut event: AnalyticsEvent) -> Result<(), String> {
        event.timestamp = chrono::Utc::now();

        self.event_tx
            .send(event)
            .map_err(|e| e.to_string())
    }

    pub async fn record_download_request(&self, module_id: &str, version: &str) {
        let event = AnalyticsEvent {
            event_type: "download_request".to_string(),
            module_id: Some(module_id.to_string()),
            version: Some(version.to_string()),
            success: true,
            error_code: None,
            client_info: ClientInfo::default(),
            timestamp: chrono::Utc::now(),
        };

        let _ = self.event_tx.send(event);
    }

    async fn process_events(mut rx: mpsc::UnboundedReceiver<AnalyticsEvent>) {
        let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let mut batch = Vec::new();
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    batch.push(event);

                    if batch.len() >= 100 {
                        Self::flush_batch(&db, &mut batch).await;
                    }
                }
                _ = interval.tick() => {
                    if !batch.is_empty() {
                        Self::flush_batch(&db, &mut batch).await;
                    }
                }
            }
        }
    }

    async fn flush_batch(db: &PgPool, batch: &mut Vec<AnalyticsEvent>) {
        for event in batch.drain(..) {
            let _ = sqlx::query!(
                r#"
                INSERT INTO analytics_events
                (event_type, module_id, version, success, error_code,
                 platform, arch, client_version, country, timestamp)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                "#,
                event.event_type,
                event.module_id,
                event.version,
                event.success,
                event.error_code,
                event.client_info.platform,
                event.client_info.arch,
                event.client_info.client_version,
                event.client_info.country,
                event.timestamp
            )
            .execute(db)
            .await;
        }
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            platform: "unknown".to_string(),
            arch: "unknown".to_string(),
            client_version: "0.0.0".to_string(),
            country: None,
        }
    }
}
```

---

## 📝 Task Breakdown

### Task 1: Update Server API (Weeks 1-3)

**Owner:** Lead Developer + Backend Engineer (new role)

**Deliverables:**
- [ ] REST API with all endpoints
- [ ] PostgreSQL database schema
- [ ] Authentication and rate limiting
- [ ] API documentation (OpenAPI)
- [ ] Load testing results
- [ ] Monitoring integration

**Acceptance Criteria:**
- API handles 1000 req/sec
- Response time <50ms (99th percentile)
- Database queries optimized (<10ms)
- Rate limiting prevents abuse
- API docs complete and accurate

**Dependencies:** None (can start immediately after Phase 2.3)

---

### Task 2: CDN Integration (Weeks 2-4)

**Owner:** DevOps Engineer + Lead Developer

**Deliverables:**
- [ ] CloudFront configuration
- [ ] Fastly backup configuration
- [ ] Automatic failover logic
- [ ] Signed URL generation
- [ ] Edge caching rules
- [ ] Geographic routing

**Acceptance Criteria:**
- Latency <100ms to edge (95th percentile)
- Failover time <5 seconds
- Cache hit rate >80%
- Signed URLs expire correctly
- Works globally in all regions

**Dependencies:** Task 1 (API for URL generation)

---

### Task 3: Delta Patch System (Weeks 3-5)

**Owner:** Lead Developer + DevOps Engineer

**Deliverables:**
- [ ] Delta generation tool
- [ ] Binary diff algorithm integration
- [ ] Automated generation pipeline
- [ ] Delta application client code
- [ ] Performance benchmarks
- [ ] CI/CD integration

**Acceptance Criteria:**
- Delta patches 60%+ smaller than full downloads
- Generation time <5 minutes per release
- Application success rate >99%
- Automated in release pipeline
- Fallback to full download works

**Dependencies:** Task 1 (API for serving deltas)

---

### Task 4: Analytics Infrastructure (Weeks 4-6)

**Owner:** Backend Engineer + Data Engineer (new role)

**Deliverables:**
- [ ] Event collection pipeline
- [ ] Time-series database setup
- [ ] Grafana dashboards
- [ ] Cost monitoring alerts
- [ ] Usage reports
- [ ] Data retention policies

**Acceptance Criteria:**
- Events processed in real-time (<10s delay)
- Dashboards show key metrics
- Cost alerts functional
- Reports generated automatically
- Data complies with privacy policies

**Dependencies:** Task 1 (API for event ingestion)

---

### Task 5: Performance Optimization (Weeks 5-7)

**Owner:** DevOps Engineer + Lead Developer

**Deliverables:**
- [ ] Compression optimization
- [ ] Cache tuning
- [ ] Database indexing
- [ ] Load balancing configuration
- [ ] Performance benchmarks
- [ ] Capacity planning analysis

**Acceptance Criteria:**
- Download speeds >10MB/s globally
- API latency <50ms (99th percentile)
- Database queries <10ms
- Cache hit rate >85%
- System scales to 100K concurrent users

**Dependencies:** Tasks 1-3 (all core features)

---

### Task 6: Testing & Documentation (Weeks 8-10)

**Owner:** QA Engineer + Technical Writer + DevOps Engineer

**Deliverables:**
- [ ] Load testing (100K concurrent users)
- [ ] Geographic testing (all continents)
- [ ] Failure scenario testing
- [ ] API documentation
- [ ] Operations runbook
- [ ] Disaster recovery procedures

**Acceptance Criteria:**
- System stable under peak load
- Works in all target regions
- All failure modes handled
- Documentation complete
- DR procedures tested
- Monitoring coverage >95%

**Dependencies:** Tasks 1-5 (complete system)

---

## 🏃 Sprint Structure

### Sprint 1-2 (Weeks 1-4): Core Infrastructure
- **Goal:** API and CDN operational
- **Demos:** Module downloads via CDN
- **Risks:** CDN configuration issues, API performance

### Sprint 3-4 (Weeks 5-7): Optimization
- **Goal:** Delta patches, analytics, performance tuning
- **Demos:** Delta updates, analytics dashboards
- **Risks:** Delta generation complexity, cache tuning

### Sprint 5 (Weeks 8-10): Testing & Launch
- **Goal:** Production-ready system
- **Demos:** Full system under load, monitoring
- **Risks:** Load testing issues, geographic problems

---

## 👥 Team & Resources

### Core Team (Full-time)
- **Lead Developer** (10 weeks)
  - Architecture, CDN integration, delta system
  - Rate: $150/hr × 40hr/wk × 10wks = $60,000

- **DevOps Engineer** (10 weeks)
  - CDN setup, infrastructure, performance
  - Rate: $140/hr × 40hr/wk × 10wks = $56,000

### New Roles (Full-time)
- **Backend Engineer** (10 weeks)
  - Update server API, database, authentication
  - Rate: $145/hr × 40hr/wk × 10wks = $58,000

- **Data Engineer** (6 weeks)
  - Analytics pipeline, dashboards, reports
  - Rate: $150/hr × 40hr/wk × 6wks = $36,000

### Supporting Team (Part-time)
- **QA Engineer** (50% × 8 weeks)
  - Testing, load testing, validation
  - Rate: $110/hr × 20hr/wk × 8wks = $17,600

- **Technical Writer** (50% × 6 weeks)
  - API docs, runbooks, procedures
  - Rate: $100/hr × 20hr/wk × 6wks = $12,000

**Total Personnel:** $239,600

---

## 💰 Budget Breakdown

| Category | Item | Cost |
|----------|------|------|
| **Personnel** | Core team (2 FT) | $116,000 |
| | New roles (2 FT, varying weeks) | $94,000 |
| | Supporting team (2 PT) | $29,600 |
| **Infrastructure** | CDN bandwidth (3 months, 50TB) | $25,000 |
| | CloudFront configuration & usage | $8,000 |
| | Fastly backup configuration | $6,000 |
| | Origin servers (3 regions) | $4,500 |
| | PostgreSQL database (managed) | $3,000 |
| | Redis caching layer | $1,800 |
| | Load balancers | $2,400 |
| **Tools & Services** | Monitoring & alerting (3 months) | $2,100 |
| | Analytics platform | $1,500 |
| | Load testing tools | $2,000 |
| | SSL certificates | $500 |
| **Testing** | Geographic testing infrastructure | $3,000 |
| | Penetration testing | $5,000 |
| **Contingency** | 10% buffer | $30,440 |
| **TOTAL** | | **$334,840** |

**Budget Range:** $300K - $370K

---

## 🎯 Success Criteria

### Performance Requirements
- ✅ CDN latency <100ms (95th percentile globally)
- ✅ Download speeds >10MB/s on good connections
- ✅ API response time <50ms (99th percentile)
- ✅ Delta patches reduce bandwidth by 60%+
- ✅ Cache hit rate >85%

### Reliability Requirements
- ✅ System uptime 99.95%
- ✅ CDN failover time <5 seconds
- ✅ Download success rate >99.9%
- ✅ Zero data corruption incidents
- ✅ Graceful degradation under load

### Cost Efficiency Requirements
- ✅ Bandwidth cost <$0.50/GB at scale
- ✅ Total CDN costs <$15K/month (100K users)
- ✅ Infrastructure costs <$20K/month
- ✅ Delta updates save 60%+ bandwidth
- ✅ Cost monitoring and alerts functional

### Operational Requirements
- ✅ Monitoring coverage >95%
- ✅ Alert response time <5 minutes
- ✅ Deployment automation complete
- ✅ DR procedures tested and documented
- ✅ On-call rotation established

---

## 🚨 Risk Management

### High-Impact Risks

#### 1. CDN Costs Exceed Budget
- **Probability:** High
- **Impact:** High
- **Mitigation:**
  - Aggressive caching strategies
  - Delta updates to reduce traffic
  - Cost monitoring and alerts
  - Multi-CDN pricing negotiation
- **Contingency:** Reduce redundancy, optimize cache, raise pricing

#### 2. Geographic Performance Issues
- **Probability:** Medium
- **Impact:** High
- **Mitigation:**
  - Early testing in all regions
  - Multiple CDN providers
  - Regional origin servers
  - Performance monitoring per region
- **Contingency:** Add regional edge locations, optimize routing

#### 3. CDN Provider Outage
- **Probability:** Low
- **Impact:** Critical
- **Mitigation:**
  - Multi-CDN architecture
  - Automatic failover (tested)
  - Health monitoring
  - Clear escalation procedures
- **Contingency:** Manual failover, status page updates

### Medium-Impact Risks

#### 4. Database Performance Bottleneck
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Proper indexing from start
  - Redis caching layer
  - Read replicas for scaling
  - Query optimization
- **Contingency:** Add read replicas, optimize queries

#### 5. Delta Generation Failures
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Robust error handling
  - Fallback to full downloads
  - Automated testing
  - Manual verification process
- **Contingency:** Disable delta updates, investigate, fix

---

## 📊 Milestones & Timeline

```
Week 1-3: Update Server API
├─ Week 1: Database schema, basic API
├─ Week 2: All endpoints, authentication
└─ Week 3: Rate limiting, optimization
    ✓ Milestone: API operational

Week 2-4: CDN Integration
├─ Week 2-3: CloudFront + Fastly setup
└─ Week 4: Failover, signed URLs, testing
    ✓ Milestone: CDN serving content

Week 3-5: Delta Patch System
├─ Week 3-4: Generation tool, algorithm
└─ Week 5: CI/CD integration, testing
    ✓ Milestone: Delta updates working

Week 4-6: Analytics Infrastructure
├─ Week 4-5: Pipeline, database, collection
└─ Week 6: Dashboards, reports, alerts
    ✓ Milestone: Full visibility

Week 5-7: Performance Optimization
├─ Week 5-6: Caching, compression, tuning
└─ Week 7: Benchmarking, validation
    ✓ Milestone: Performance targets met

Week 8-10: Testing & Launch
├─ Week 8-9: Load testing, geo testing
└─ Week 10: Documentation, launch prep
    ✓ Milestone: Phase 2.4 complete
```

### Key Dates
- **Week 1:** Kickoff, CDN vendor contracts
- **Week 4:** CDN operational demo
- **Week 7:** Performance optimization complete
- **Week 9:** Load testing passes
- **Week 10:** Phase 2.4 launch, handoff to Phase 2.5

---

## 🔄 Integration Points

### With Phase 2.3 (Installation & Updates)
- Provides download URLs for installer
- Serves delta patches for updates
- Tracks download analytics
- Monitors bandwidth usage

### With Phase 2.5 (Marketplace & Discovery)
- Powers module discovery catalog
- Provides download statistics
- Enables featured modules
- Supports search and filtering

### With Phase 2.6 (Enterprise Features)
- Private CDN endpoints for enterprise
- Custom analytics dashboards
- SLA monitoring
- Priority support integration

---

## 📚 Documentation Deliverables

1. **API Documentation**
   - OpenAPI specification
   - Authentication guide
   - Rate limiting details
   - Example requests/responses

2. **Operations Runbook**
   - Deployment procedures
   - Monitoring setup
   - Alert response playbook
   - Common troubleshooting

3. **CDN Configuration Guide**
   - CloudFront setup
   - Fastly configuration
   - Failover procedures
   - Cache invalidation

4. **Analytics Guide**
   - Dashboard usage
   - Report interpretation
   - Cost monitoring
   - Data export procedures

5. **Disaster Recovery Plan**
   - Backup procedures
   - Failover steps
   - Recovery time objectives
   - Communication protocols

---

## ✅ Phase 2.4 Complete When

- [ ] Update server API operational (1000 req/sec)
- [ ] Multi-CDN architecture deployed
- [ ] Automatic failover tested and working
- [ ] Delta patch system generating patches
- [ ] Analytics dashboards showing data
- [ ] Performance targets met globally
- [ ] Load testing passed (100K concurrent)
- [ ] Geographic testing completed
- [ ] Monitoring and alerts configured
- [ ] Documentation complete
- [ ] DR procedures tested
- [ ] Cost monitoring operational
- [ ] Team trained on operations
- [ ] Handoff to Phase 2.5 complete

---

## 📞 Stakeholder Communication

### Weekly Updates
- Performance metrics (latency, throughput)
- Bandwidth usage and costs
- Error rates and incidents
- Progress against timeline

### Demos
- Week 4: CDN integration demo
- Week 7: Delta updates and analytics
- Week 9: Load testing results
- Week 10: Final system walkthrough

### Decision Points
- Week 2: CDN provider contracts approval
- Week 5: Analytics requirements review
- Week 8: Load testing go/no-go
- Week 10: Production launch approval

---

## 🔗 Related Documents

- **Phase 2 Overview:** [../../PHASE_2_PLANNING.md](../../PHASE_2_PLANNING.md)
- **Phase 2.1 Plan:** [PHASE_2_1_PLAN.md](./PHASE_2_1_PLAN.md)
- **Phase 2.2 Plan:** [PHASE_2_2_PLAN.md](./PHASE_2_2_PLAN.md)
- **Phase 2.3 Plan:** [PHASE_2_3_PLAN.md](./PHASE_2_3_PLAN.md)
- **Verification Report:** [../../VERIFICATION_REPORT.md](../../VERIFICATION_REPORT.md)

---

**Status:** Ready for Review
**Next Action:** CDN vendor selection, stakeholder approval, team allocation
**Target Start Date:** October 2026
**Target Completion:** December 2026
**Budget:** $335K (includes significant CDN infrastructure costs)
