// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(test)]
mod integration_tests {
    use crate::streaming::{
        db::StreamingDb,
        models::{StreamProvider, StreamingContent},
    };
    use std::path::PathBuf;

    #[test]
    fn test_streaming_workflow() {
        // Create in-memory database
        let db = StreamingDb::new(PathBuf::from(":memory:")).expect("Failed to create DB");

        // Test: Add video to queue
        let content = StreamingContent {
            id: "dQw4w9WgXcQ".to_string(),
            provider: StreamProvider::YouTube,
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
            title: "Test Video".to_string(),
            description: Some("A test video".to_string()),
            thumbnail: Some("https://example.com/thumb.jpg".to_string()),
            duration: Some(212),
            creator: Some("Test Channel".to_string()),
            view_count: Some(1000000),
            published_at: Some("2025-01-01T00:00:00Z".to_string()),
        };

        let queue_item = crate::streaming::models::WatchQueueItem {
            id: "queue-1".to_string(),
            content_id: content.id.clone(),
            content: content.clone(),
            added_at: "2025-10-05T12:00:00Z".to_string(),
            position: 1,
        };

        db.add_to_queue(&queue_item).expect("Failed to add to queue");

        // Test: Retrieve queue
        let queue = db.get_watch_queue().expect("Failed to get queue");
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].content.id, "dQw4w9WgXcQ");
        assert_eq!(queue[0].content.title, "Test Video");

        // Test: Watch history without consent (should not save)
        let history_entry = crate::streaming::models::WatchHistoryEntry {
            id: "hist-1".to_string(),
            content_id: content.id.clone(),
            content: content.clone(),
            watched_at: "2025-10-05T12:30:00Z".to_string(),
            progress: 106.0,
            completed: false,
            duration: 106.0,
        };

        db.save_watch_progress(&history_entry)
            .expect("Failed to save progress");
        let history = db.get_watch_history(None).expect("Failed to get history");
        assert_eq!(history.len(), 0, "History should be empty without consent");

        // Test: Enable consent
        let consent = crate::streaming::models::UserConsent {
            feature: "watch_history".to_string(),
            enabled: true,
            updated_at: "2025-10-05T12:00:00Z".to_string(),
        };
        db.set_consent(&consent).expect("Failed to set consent");

        // Test: Watch history with consent (should save)
        db.save_watch_progress(&history_entry)
            .expect("Failed to save progress");
        let history = db.get_watch_history(None).expect("Failed to get history");
        assert_eq!(history.len(), 1, "History should have one entry");
        assert_eq!(history[0].content.id, "dQw4w9WgXcQ");
        assert_eq!(history[0].progress, 106.0);

        // Test: Remove from queue
        db.remove_from_queue("queue-1")
            .expect("Failed to remove from queue");
        let queue = db.get_watch_queue().expect("Failed to get queue");
        assert_eq!(queue.len(), 0, "Queue should be empty");

        // Test: Clear history
        db.clear_watch_history().expect("Failed to clear history");
        let history = db.get_watch_history(None).expect("Failed to get history");
        assert_eq!(history.len(), 0, "History should be empty");
    }

    #[test]
    fn test_youtube_duration_parsing() {
        use crate::streaming::providers::youtube::*;

        // Test various duration formats
        assert_eq!(parse_iso8601_duration("PT1H2M10S"), Some(3730));
        assert_eq!(parse_iso8601_duration("PT15M30S"), Some(930));
        assert_eq!(parse_iso8601_duration("PT45S"), Some(45));
        assert_eq!(parse_iso8601_duration("PT2H"), Some(7200));
        assert_eq!(parse_iso8601_duration("PT30M"), Some(1800));
    }

    #[test]
    fn test_consent_management() {
        let db = StreamingDb::new(PathBuf::from(":memory:")).expect("Failed to create DB");

        // Default: no consent
        assert_eq!(
            db.check_consent("watch_history").unwrap(),
            false,
            "Default should be no consent"
        );

        // Enable consent
        let consent = crate::streaming::models::UserConsent {
            feature: "watch_history".to_string(),
            enabled: true,
            updated_at: "2025-10-05T12:00:00Z".to_string(),
        };
        db.set_consent(&consent).expect("Failed to set consent");
        assert_eq!(
            db.check_consent("watch_history").unwrap(),
            true,
            "Consent should be enabled"
        );

        // Disable consent
        let consent = crate::streaming::models::UserConsent {
            feature: "watch_history".to_string(),
            enabled: false,
            updated_at: "2025-10-05T12:05:00Z".to_string(),
        };
        db.set_consent(&consent).expect("Failed to set consent");
        assert_eq!(
            db.check_consent("watch_history").unwrap(),
            false,
            "Consent should be disabled"
        );
    }
}
