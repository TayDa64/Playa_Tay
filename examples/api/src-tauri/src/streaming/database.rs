// M1 Streaming Hub - Database Operations
// Phase 1 (Foundation): SQLite operations with sqlx
// Performance target: <100ms per query

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::error::Error;

// ============================================================================
// Data Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WatchHistoryEntry {
    pub id: i64,
    pub user_id: String,
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
    pub watched_at: String, // ISO 8601 datetime
    pub progress_seconds: i64,
    pub completion_percent: i64,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WatchQueueEntry {
    pub id: i64,
    pub user_id: String,
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
    pub position: i64,
    pub added_at: String, // ISO 8601 datetime
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Recommendation {
    pub id: i64,
    pub user_id: String,
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
    pub confidence_score: f64,
    pub reasoning: Option<String>,
    pub generated_at: String, // ISO 8601 datetime
    pub dismissed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub stream_id: String,
    pub provider: String,
    pub title: String,
    pub channel: String,
    pub thumbnail_url: Option<String>,
    pub stream_url: String,
    pub live: bool,
}

// ============================================================================
// Database Operations
// ============================================================================

/// Get watch history for a user, ordered by most recent
/// Performance: <100ms (indexed query)
pub async fn get_watch_history(
    pool: &SqlitePool,
    user_id: &str,
    limit: Option<usize>,
) -> Result<Vec<WatchHistoryEntry>, Box<dyn Error>> {
    let limit = limit.unwrap_or(50).min(500); // Cap at 500 for performance

    let entries = sqlx::query_as::<_, WatchHistoryEntry>(
        r#"
        SELECT id, user_id, stream_id, provider, title, channel,
               thumbnail_url, watched_at, progress_seconds,
               completion_percent, source
        FROM watch_history
        WHERE user_id = ?
        ORDER BY watched_at DESC
        LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

/// Get watch queue for a user, ordered by position
/// Performance: <100ms (indexed query)
pub async fn get_watch_queue(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<WatchQueueEntry>, Box<dyn Error>> {
    let entries = sqlx::query_as::<_, WatchQueueEntry>(
        r#"
        SELECT id, user_id, stream_id, provider, title, channel,
               thumbnail_url, position, added_at
        FROM watch_queue
        WHERE user_id = ?
        ORDER BY position ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(entries)
}

/// Get recommendations for a user, ordered by confidence score
/// Only returns non-dismissed recommendations
/// Performance: <100ms (indexed query)
pub async fn get_recommendations(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<Recommendation>, Box<dyn Error>> {
    let recommendations = sqlx::query_as::<_, Recommendation>(
        r#"
        SELECT id, user_id, stream_id, provider, title, channel,
               thumbnail_url, confidence_score, reasoning,
               generated_at, dismissed
        FROM recommendations
        WHERE user_id = ? AND dismissed = 0
        ORDER BY confidence_score DESC
        LIMIT 20
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(recommendations)
}

/// Add a stream to the watch queue
/// Position is auto-calculated as max(position) + 1
pub async fn add_to_queue(
    pool: &SqlitePool,
    user_id: &str,
    stream_id: &str,
    provider: &str,
    title: &str,
    channel: &str,
    thumbnail_url: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // Get the next position
    let max_position: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT MAX(position)
        FROM watch_queue
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let next_position = max_position.unwrap_or(-1) + 1;

    // Insert the new queue entry
    sqlx::query(
        r#"
        INSERT INTO watch_queue
          (user_id, stream_id, provider, title, channel, thumbnail_url, position)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(user_id, stream_id) DO UPDATE SET
          position = excluded.position
        "#,
    )
    .bind(user_id)
    .bind(stream_id)
    .bind(provider)
    .bind(title)
    .bind(channel)
    .bind(thumbnail_url)
    .bind(next_position)
    .execute(pool)
    .await?;

    Ok(())
}

/// Save playback progress for a stream
/// Creates or updates watch history entry
pub async fn save_playback_progress(
    pool: &SqlitePool,
    user_id: &str,
    stream_id: &str,
    provider: &str,
    title: &str,
    channel: &str,
    thumbnail_url: Option<&str>,
    progress_seconds: i64,
    completion_percent: i64,
    source: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        INSERT INTO watch_history
          (user_id, stream_id, provider, title, channel, thumbnail_url,
           progress_seconds, completion_percent, source)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(user_id, stream_id, watched_at) DO UPDATE SET
          progress_seconds = excluded.progress_seconds,
          completion_percent = excluded.completion_percent
        "#,
    )
    .bind(user_id)
    .bind(stream_id)
    .bind(provider)
    .bind(title)
    .bind(channel)
    .bind(thumbnail_url)
    .bind(progress_seconds)
    .bind(completion_percent)
    .bind(source)
    .execute(pool)
    .await?;

    Ok(())
}

/// Remove a stream from the watch queue
/// Triggers automatic position renumbering
pub async fn remove_from_queue(
    pool: &SqlitePool,
    user_id: &str,
    stream_id: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        DELETE FROM watch_queue
        WHERE user_id = ? AND stream_id = ?
        "#,
    )
    .bind(user_id)
    .bind(stream_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Dismiss a recommendation
pub async fn dismiss_recommendation(
    pool: &SqlitePool,
    user_id: &str,
    stream_id: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        UPDATE recommendations
        SET dismissed = 1
        WHERE user_id = ? AND stream_id = ?
        "#,
    )
    .bind(user_id)
    .bind(stream_id)
    .execute(pool)
    .await?;

    Ok(())
}
