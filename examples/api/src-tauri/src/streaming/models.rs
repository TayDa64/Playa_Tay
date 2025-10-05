// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Supported streaming providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamProvider {
    YouTube,
    Twitch,
    Custom,
}

/// Streaming content metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingContent {
    pub id: String,
    pub provider: StreamProvider,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>, // seconds
    pub creator: Option<String>,
    pub view_count: Option<u64>,
    pub published_at: Option<String>,
}

/// Watch queue item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchQueueItem {
    pub id: String,
    pub content_id: String,
    pub content: StreamingContent,
    pub added_at: String, // ISO 8601 timestamp
    pub position: i32,
}

/// Watch history entry (requires user consent)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistoryEntry {
    pub id: String,
    pub content_id: String,
    pub content: StreamingContent,
    pub watched_at: String, // ISO 8601 timestamp
    pub progress: f64,      // seconds
    pub completed: bool,    // true if watched >90%
    pub duration: f64,      // total watch time in seconds
}

/// User consent settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConsent {
    pub feature: String,
    pub enabled: bool,
    pub updated_at: String, // ISO 8601 timestamp
}

impl StreamingContent {
    /// Create a new YouTube content item
    pub fn new_youtube(
        id: String,
        url: String,
        title: String,
        description: Option<String>,
        thumbnail: Option<String>,
        duration: Option<u64>,
        creator: Option<String>,
        view_count: Option<u64>,
        published_at: Option<String>,
    ) -> Self {
        Self {
            id,
            provider: StreamProvider::YouTube,
            url,
            title,
            description,
            thumbnail,
            duration,
            creator,
            view_count,
            published_at,
        }
    }
}
