// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::streaming::models::StreamingContent;
use serde::{Deserialize, Serialize};

/// YouTube API client
pub struct YouTubeClient {
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YouTubeSearchResponse {
    items: Vec<YouTubeSearchItem>,
}

#[derive(Debug, Deserialize)]
struct YouTubeSearchItem {
    id: YouTubeId,
    snippet: YouTubeSnippet,
}

#[derive(Debug, Deserialize)]
struct YouTubeId {
    #[serde(rename = "videoId")]
    video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YouTubeSnippet {
    title: String,
    description: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    thumbnails: YouTubeThumbnails,
    #[serde(rename = "publishedAt")]
    published_at: String,
}

#[derive(Debug, Deserialize)]
struct YouTubeThumbnails {
    #[serde(default)]
    high: Option<YouTubeThumbnail>,
    #[serde(default)]
    medium: Option<YouTubeThumbnail>,
    #[serde(default)]
    default: Option<YouTubeThumbnail>,
}

#[derive(Debug, Deserialize)]
struct YouTubeThumbnail {
    url: String,
}

#[derive(Debug, Deserialize)]
struct YouTubeVideoResponse {
    items: Vec<YouTubeVideoItem>,
}

#[derive(Debug, Deserialize)]
struct YouTubeVideoItem {
    id: String,
    snippet: YouTubeSnippet,
    #[serde(rename = "contentDetails")]
    content_details: YouTubeContentDetails,
    statistics: YouTubeStatistics,
}

#[derive(Debug, Deserialize)]
struct YouTubeContentDetails {
    duration: String, // ISO 8601 format (e.g., "PT1H2M10S")
}

#[derive(Debug, Deserialize)]
struct YouTubeStatistics {
    #[serde(rename = "viewCount")]
    view_count: Option<String>,
}

impl YouTubeClient {
    /// Create a new YouTube client
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    /// Search for videos on YouTube
    pub async fn search(
        &self,
        query: &str,
        max_results: u32,
    ) -> Result<Vec<StreamingContent>, String> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| "YouTube API key not configured".to_string())?;

        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&type=video&q={}&maxResults={}&key={}",
            urlencoding::encode(query),
            max_results,
            api_key
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to search YouTube: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("YouTube API error ({}): {}", status, text));
        }

        let search_response: YouTubeSearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse YouTube response: {}", e))?;

        // Extract video IDs for fetching detailed info
        let video_ids: Vec<String> = search_response
            .items
            .iter()
            .filter_map(|item| item.id.video_id.clone())
            .collect();

        if video_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Fetch video details (duration, view count)
        self.get_video_details(&video_ids).await
    }

    /// Get detailed information for multiple videos
    async fn get_video_details(&self, video_ids: &[String]) -> Result<Vec<StreamingContent>, String> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| "YouTube API key not configured".to_string())?;

        let ids = video_ids.join(",");
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet,contentDetails,statistics&id={}&key={}",
            ids, api_key
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch video details: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("YouTube API error: {}", response.status()));
        }

        let video_response: YouTubeVideoResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse video details: {}", e))?;

        let contents: Vec<StreamingContent> = video_response
            .items
            .into_iter()
            .map(|item| {
                let thumbnail = item
                    .snippet
                    .thumbnails
                    .high
                    .or(item.snippet.thumbnails.medium)
                    .or(item.snippet.thumbnails.default)
                    .map(|t| t.url);

                let duration = parse_iso8601_duration(&item.content_details.duration);
                let view_count = item
                    .statistics
                    .view_count
                    .and_then(|s| s.parse::<u64>().ok());

                StreamingContent::new_youtube(
                    item.id.clone(),
                    format!("https://www.youtube.com/watch?v={}", item.id),
                    item.snippet.title,
                    Some(item.snippet.description),
                    thumbnail,
                    duration,
                    Some(item.snippet.channel_title),
                    view_count,
                    Some(item.snippet.published_at),
                )
            })
            .collect();

        Ok(contents)
    }
}

/// Parse ISO 8601 duration string (e.g., "PT1H2M10S") to seconds
fn parse_iso8601_duration(duration: &str) -> Option<u64> {
    // Simple parser for YouTube's ISO 8601 duration format
    // Format: PT[hours]H[minutes]M[seconds]S
    if !duration.starts_with("PT") {
        return None;
    }

    let duration = &duration[2..]; // Skip "PT"
    let mut total_seconds = 0u64;

    // Parse hours
    if let Some(h_pos) = duration.find('H') {
        if let Ok(hours) = duration[..h_pos].parse::<u64>() {
            total_seconds += hours * 3600;
        }
    }

    // Parse minutes
    if let Some(m_start) = duration.find('H').map(|p| p + 1) {
        if let Some(m_pos) = duration[m_start..].find('M') {
            let m_pos = m_start + m_pos;
            if let Ok(minutes) = duration[m_start..m_pos].parse::<u64>() {
                total_seconds += minutes * 60;
            }
        }
    } else if let Some(m_pos) = duration.find('M') {
        if let Ok(minutes) = duration[..m_pos].parse::<u64>() {
            total_seconds += minutes * 60;
        }
    }

    // Parse seconds
    if let Some(s_start) = duration.rfind(|c| c == 'M' || c == 'H').map(|p| p + 1) {
        if let Some(s_pos) = duration[s_start..].find('S') {
            let s_pos = s_start + s_pos;
            if let Ok(seconds) = duration[s_start..s_pos].parse::<u64>() {
                total_seconds += seconds;
            }
        }
    } else if let Some(s_pos) = duration.find('S') {
        if let Ok(seconds) = duration[..s_pos].parse::<u64>() {
            total_seconds += seconds;
        }
    }

    Some(total_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso8601_duration() {
        assert_eq!(parse_iso8601_duration("PT1H2M10S"), Some(3730)); // 1h 2m 10s
        assert_eq!(parse_iso8601_duration("PT15M30S"), Some(930)); // 15m 30s
        assert_eq!(parse_iso8601_duration("PT45S"), Some(45)); // 45s
        assert_eq!(parse_iso8601_duration("PT2H"), Some(7200)); // 2h
        assert_eq!(parse_iso8601_duration("PT30M"), Some(1800)); // 30m
        assert_eq!(parse_iso8601_duration("PT1H30S"), Some(3630)); // 1h 30s
    }
}
