// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::streaming::{
    db::StreamingDb,
    models::{StreamingContent, UserConsent, WatchHistoryEntry, WatchQueueItem},
    providers::youtube::YouTubeClient,
};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};
use uuid::Uuid;

/// Shared state for streaming module
pub struct StreamingState {
    pub db: Arc<StreamingDb>,
    pub youtube_client: Arc<YouTubeClient>,
}

/// Open the streaming hub in a new window
#[tauri::command]
pub async fn open_streaming_hub(app: AppHandle) -> Result<(), String> {
    // Check if window already exists
    if app.get_webview_window("streaming-hub").is_some() {
        return Err("Streaming hub is already open".to_string());
    }

    // Create new window for streaming module
    WebviewWindowBuilder::new(&app, "streaming-hub", WebviewUrl::App("/streaming".into()))
        .title("Streaming Hub")
        .inner_size(1280.0, 720.0)
        .min_inner_size(800.0, 600.0)
        .build()
        .map_err(|e| format!("Failed to create streaming window: {}", e))?;

    Ok(())
}

/// Search YouTube for videos
#[tauri::command]
pub async fn search_youtube(
    query: String,
    limit: Option<u32>,
    state: State<'_, StreamingState>,
) -> Result<Vec<StreamingContent>, String> {
    let max_results = limit.unwrap_or(10).min(50); // Cap at 50
    state
        .youtube_client
        .search(&query, max_results)
        .await
}

/// Add a video to the watch queue
#[tauri::command]
pub async fn add_to_queue(
    content: StreamingContent,
    state: State<'_, StreamingState>,
) -> Result<(), String> {
    // Get current queue to determine position
    let queue = state
        .db
        .get_watch_queue()
        .map_err(|e| format!("Failed to get queue: {}", e))?;

    let position = queue.len() as i32 + 1;
    let added_at = chrono::Utc::now().to_rfc3339();

    let item = WatchQueueItem {
        id: Uuid::new_v4().to_string(),
        content_id: content.id.clone(),
        content,
        added_at,
        position,
    };

    state
        .db
        .add_to_queue(&item)
        .map_err(|e| format!("Failed to add to queue: {}", e))?;

    Ok(())
}

/// Remove an item from the watch queue
#[tauri::command]
pub async fn remove_from_queue(
    queue_id: String,
    state: State<'_, StreamingState>,
) -> Result<(), String> {
    state
        .db
        .remove_from_queue(&queue_id)
        .map_err(|e| format!("Failed to remove from queue: {}", e))?;
    Ok(())
}

/// Get all items in the watch queue
#[tauri::command]
pub async fn get_watch_queue(
    state: State<'_, StreamingState>,
) -> Result<Vec<WatchQueueItem>, String> {
    state
        .db
        .get_watch_queue()
        .map_err(|e| format!("Failed to get queue: {}", e))
}

/// Save watch progress for a video
#[tauri::command]
pub async fn save_watch_progress(
    content: StreamingContent,
    progress: f64,
    duration: f64,
    state: State<'_, StreamingState>,
) -> Result<(), String> {
    let watched_at = chrono::Utc::now().to_rfc3339();
    let completed = progress / duration > 0.9; // Consider completed if >90% watched

    let entry = WatchHistoryEntry {
        id: Uuid::new_v4().to_string(),
        content_id: content.id.clone(),
        content,
        watched_at,
        progress,
        completed,
        duration,
    };

    state
        .db
        .save_watch_progress(&entry)
        .map_err(|e| format!("Failed to save progress: {}", e))?;

    Ok(())
}

/// Get watch history
#[tauri::command]
pub async fn get_watch_history(
    limit: Option<u32>,
    state: State<'_, StreamingState>,
) -> Result<Vec<WatchHistoryEntry>, String> {
    state
        .db
        .get_watch_history(limit)
        .map_err(|e| format!("Failed to get history: {}", e))
}

/// Clear all watch history
#[tauri::command]
pub async fn clear_watch_history(state: State<'_, StreamingState>) -> Result<(), String> {
    state
        .db
        .clear_watch_history()
        .map_err(|e| format!("Failed to clear history: {}", e))?;
    Ok(())
}

/// Set user consent for a feature
#[tauri::command]
pub async fn set_streaming_consent(
    feature: String,
    enabled: bool,
    state: State<'_, StreamingState>,
) -> Result<(), String> {
    let consent = UserConsent {
        feature,
        enabled,
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    state
        .db
        .set_consent(&consent)
        .map_err(|e| format!("Failed to set consent: {}", e))?;
    Ok(())
}

/// Check user consent for a feature
#[tauri::command]
pub async fn check_streaming_consent(
    feature: String,
    state: State<'_, StreamingState>,
) -> Result<bool, String> {
    state
        .db
        .check_consent(&feature)
        .map_err(|e| format!("Failed to check consent: {}", e))
}
