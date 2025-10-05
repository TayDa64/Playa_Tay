// M1 Streaming Hub - Tauri Command Handlers
// Phase 1 (Foundation): IPC commands for streaming operations
// Security: All commands validated, no direct network from renderer

use crate::streaming::database::{
    self, Recommendation, StreamMetadata, WatchHistoryEntry, WatchQueueEntry,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWatchHistoryRequest {
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayStreamRequest {
    pub stream_id: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToQueueRequest {
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePlaybackProgressRequest {
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
    pub progress_seconds: i64,
    pub completion_percent: i64,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveFromQueueRequest {
    pub stream_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DismissRecommendationRequest {
    pub stream_id: String,
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Get watch history for the current user
/// Returns up to `limit` entries (default: 50, max: 500)
#[tauri::command]
pub async fn get_watch_history(
    request: GetWatchHistoryRequest,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<WatchHistoryEntry>, String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    database::get_watch_history(&pool, user_id, request.limit)
        .await
        .map_err(|e| format!("Failed to get watch history: {}", e))
}

/// Get watch queue for the current user
/// Returns all queue entries ordered by position
#[tauri::command]
pub async fn get_watch_queue(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<WatchQueueEntry>, String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    database::get_watch_queue(&pool, user_id)
        .await
        .map_err(|e| format!("Failed to get watch queue: {}", e))
}

/// Get AI-generated recommendations for the current user
/// Returns up to 20 non-dismissed recommendations
#[tauri::command]
pub async fn get_recommendations(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Recommendation>, String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    database::get_recommendations(&pool, user_id)
        .await
        .map_err(|e| format!("Failed to get recommendations: {}", e))
}

/// Play a stream
/// Phase 1: Returns stub metadata
/// TODO: Integrate with YouTube/Twitch APIs (Phase 3)
#[tauri::command]
pub async fn play_stream(
    request: PlayStreamRequest,
) -> Result<StreamMetadata, String> {
    log::info!("Playing stream: {} ({})", request.stream_id, request.provider);

    // Input validation
    if request.stream_id.is_empty() {
        return Err("stream_id cannot be empty".to_string());
    }
    if !["youtube", "twitch", "other"].contains(&request.provider.as_str()) {
        return Err(format!("Invalid provider: {}", request.provider));
    }

    // Stub: Return mock metadata
    // TODO Phase 3:
    // 1. Validate OAuth tokens for provider
    // 2. Call YouTube/Twitch API to get stream details
    // 3. Get actual stream URL (HLS/DASH manifest)
    // 4. Check for DRM requirements
    // 5. Return complete metadata

    Ok(StreamMetadata {
        stream_id: request.stream_id.clone(),
        provider: request.provider,
        title: format!("Stream {}", request.stream_id),
        channel: "Example Channel".to_string(),
        thumbnail_url: None,
        stream_url: format!(
            "https://example.com/stream/{}.m3u8",
            request.stream_id
        ),
        live: false,
    })
}

/// Add a stream to the watch queue
#[tauri::command]
pub async fn add_to_queue(
    request: AddToQueueRequest,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    // Input validation
    if request.stream_id.is_empty() {
        return Err("stream_id cannot be empty".to_string());
    }
    if request.title.is_empty() {
        return Err("title cannot be empty".to_string());
    }
    if request.channel.is_empty() {
        return Err("channel cannot be empty".to_string());
    }

    database::add_to_queue(
        &pool,
        user_id,
        &request.stream_id,
        &request.provider,
        &request.title,
        &request.channel,
        request.thumbnail_url.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to add to queue: {}", e))
}

/// Save playback progress for a stream
/// Creates or updates watch history entry
#[tauri::command]
pub async fn save_playback_progress(
    request: SavePlaybackProgressRequest,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    // Input validation
    if request.stream_id.is_empty() {
        return Err("stream_id cannot be empty".to_string());
    }
    if request.progress_seconds < 0 {
        return Err("progress_seconds cannot be negative".to_string());
    }
    if !(0..=100).contains(&request.completion_percent) {
        return Err("completion_percent must be between 0 and 100".to_string());
    }

    database::save_playback_progress(
        &pool,
        user_id,
        &request.stream_id,
        &request.provider,
        &request.title,
        &request.channel,
        request.thumbnail_url.as_deref(),
        request.progress_seconds,
        request.completion_percent,
        request.source.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to save playback progress: {}", e))
}

/// Remove a stream from the watch queue
#[tauri::command]
pub async fn remove_from_queue(
    request: RemoveFromQueueRequest,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    if request.stream_id.is_empty() {
        return Err("stream_id cannot be empty".to_string());
    }

    database::remove_from_queue(&pool, user_id, &request.stream_id)
        .await
        .map_err(|e| format!("Failed to remove from queue: {}", e))
}

/// Dismiss a recommendation
#[tauri::command]
pub async fn dismiss_recommendation(
    request: DismissRecommendationRequest,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    // TODO: Get actual user_id from auth context (Phase 6)
    let user_id = "default_user";

    if request.stream_id.is_empty() {
        return Err("stream_id cannot be empty".to_string());
    }

    database::dismiss_recommendation(&pool, user_id, &request.stream_id)
        .await
        .map_err(|e| format!("Failed to dismiss recommendation: {}", e))
}
