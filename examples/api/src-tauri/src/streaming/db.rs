// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::streaming::models::{
    StreamingContent, UserConsent, WatchHistoryEntry, WatchQueueItem,
};
use rusqlite::{params, Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;

/// Database manager for streaming module
pub struct StreamingDb {
    conn: Mutex<Connection>,
}

impl StreamingDb {
    /// Create a new database connection
    pub fn new(db_path: PathBuf) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();

        // Watch queue table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS watch_queue (
                id TEXT PRIMARY KEY,
                content_id TEXT NOT NULL,
                content_json TEXT NOT NULL,
                added_at TEXT NOT NULL,
                position INTEGER NOT NULL,
                UNIQUE(content_id)
            )",
            [],
        )?;

        // Watch history table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS watch_history (
                id TEXT PRIMARY KEY,
                content_id TEXT NOT NULL,
                content_json TEXT NOT NULL,
                watched_at TEXT NOT NULL,
                progress REAL NOT NULL,
                completed INTEGER NOT NULL,
                duration REAL NOT NULL
            )",
            [],
        )?;

        // User consent table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_consent (
                feature TEXT PRIMARY KEY,
                enabled INTEGER NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Add item to watch queue
    pub fn add_to_queue(&self, item: &WatchQueueItem) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        let content_json = serde_json::to_string(&item.content).unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO watch_queue (id, content_id, content_json, added_at, position)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &item.id,
                &item.content_id,
                &content_json,
                &item.added_at,
                &item.position
            ],
        )?;
        Ok(())
    }

    /// Remove item from watch queue
    pub fn remove_from_queue(&self, queue_id: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM watch_queue WHERE id = ?1", params![queue_id])?;
        Ok(())
    }

    /// Get all watch queue items ordered by position
    pub fn get_watch_queue(&self) -> SqlResult<Vec<WatchQueueItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content_id, content_json, added_at, position 
             FROM watch_queue 
             ORDER BY position ASC",
        )?;

        let items = stmt.query_map([], |row| {
            let content_json: String = row.get(2)?;
            let content: StreamingContent = serde_json::from_str(&content_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(WatchQueueItem {
                id: row.get(0)?,
                content_id: row.get(1)?,
                content,
                added_at: row.get(3)?,
                position: row.get(4)?,
            })
        })?;

        let mut result = Vec::new();
        for item in items {
            result.push(item?);
        }
        Ok(result)
    }

    /// Save watch progress (requires consent)
    pub fn save_watch_progress(&self, entry: &WatchHistoryEntry) -> SqlResult<()> {
        // Check consent first
        if !self.check_consent("watch_history")? {
            return Ok(()); // Silently skip if no consent
        }

        let conn = self.conn.lock().unwrap();
        let content_json = serde_json::to_string(&entry.content).unwrap();

        conn.execute(
            "INSERT OR REPLACE INTO watch_history 
             (id, content_id, content_json, watched_at, progress, completed, duration)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &entry.id,
                &entry.content_id,
                &content_json,
                &entry.watched_at,
                &entry.progress,
                if entry.completed { 1 } else { 0 },
                &entry.duration
            ],
        )?;
        Ok(())
    }

    /// Get watch history with optional limit
    pub fn get_watch_history(&self, limit: Option<u32>) -> SqlResult<Vec<WatchHistoryEntry>> {
        let conn = self.conn.lock().unwrap();
        let query = if let Some(lim) = limit {
            format!(
                "SELECT id, content_id, content_json, watched_at, progress, completed, duration 
                 FROM watch_history 
                 ORDER BY watched_at DESC 
                 LIMIT {}",
                lim
            )
        } else {
            "SELECT id, content_id, content_json, watched_at, progress, completed, duration 
             FROM watch_history 
             ORDER BY watched_at DESC"
                .to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let entries = stmt.query_map([], |row| {
            let content_json: String = row.get(2)?;
            let content: StreamingContent = serde_json::from_str(&content_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(WatchHistoryEntry {
                id: row.get(0)?,
                content_id: row.get(1)?,
                content,
                watched_at: row.get(3)?,
                progress: row.get(4)?,
                completed: row.get::<_, i32>(5)? == 1,
                duration: row.get(6)?,
            })
        })?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }
        Ok(result)
    }

    /// Clear all watch history
    pub fn clear_watch_history(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM watch_history", [])?;
        Ok(())
    }

    /// Check user consent for a feature
    pub fn check_consent(&self, feature: &str) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT enabled FROM user_consent WHERE feature = ?1")?;

        let result = stmt.query_row(params![feature], |row| {
            let enabled: i32 = row.get(0)?;
            Ok(enabled == 1)
        });

        match result {
            Ok(enabled) => Ok(enabled),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false), // Default to no consent
            Err(e) => Err(e),
        }
    }

    /// Set user consent for a feature
    pub fn set_consent(&self, consent: &UserConsent) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO user_consent (feature, enabled, updated_at)
             VALUES (?1, ?2, ?3)",
            params![
                &consent.feature,
                if consent.enabled { 1 } else { 0 },
                &consent.updated_at
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streaming::models::StreamProvider;

    #[test]
    fn test_database_init() {
        let db = StreamingDb::new(PathBuf::from(":memory:")).unwrap();
        assert!(db.get_watch_queue().is_ok());
    }

    #[test]
    fn test_queue_operations() {
        let db = StreamingDb::new(PathBuf::from(":memory:")).unwrap();

        let content = StreamingContent {
            id: "test123".to_string(),
            provider: StreamProvider::YouTube,
            url: "https://youtube.com/watch?v=test123".to_string(),
            title: "Test Video".to_string(),
            description: None,
            thumbnail: None,
            duration: Some(300),
            creator: Some("Test Creator".to_string()),
            view_count: Some(1000),
            published_at: None,
        };

        let item = WatchQueueItem {
            id: "queue1".to_string(),
            content_id: "test123".to_string(),
            content: content.clone(),
            added_at: "2025-10-05T12:00:00Z".to_string(),
            position: 1,
        };

        db.add_to_queue(&item).unwrap();
        let queue = db.get_watch_queue().unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].content_id, "test123");

        db.remove_from_queue("queue1").unwrap();
        let queue = db.get_watch_queue().unwrap();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_history_requires_consent() {
        let db = StreamingDb::new(PathBuf::from(":memory:")).unwrap();

        let content = StreamingContent {
            id: "test123".to_string(),
            provider: StreamProvider::YouTube,
            url: "https://youtube.com/watch?v=test123".to_string(),
            title: "Test Video".to_string(),
            description: None,
            thumbnail: None,
            duration: Some(300),
            creator: None,
            view_count: None,
            published_at: None,
        };

        let entry = WatchHistoryEntry {
            id: "hist1".to_string(),
            content_id: "test123".to_string(),
            content,
            watched_at: "2025-10-05T12:00:00Z".to_string(),
            progress: 150.0,
            completed: false,
            duration: 150.0,
        };

        // Without consent, save should succeed but not persist
        db.save_watch_progress(&entry).unwrap();
        let history = db.get_watch_history(None).unwrap();
        assert_eq!(history.len(), 0);

        // Enable consent
        let consent = UserConsent {
            feature: "watch_history".to_string(),
            enabled: true,
            updated_at: "2025-10-05T12:00:00Z".to_string(),
        };
        db.set_consent(&consent).unwrap();

        // Now save should persist
        db.save_watch_progress(&entry).unwrap();
        let history = db.get_watch_history(None).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content_id, "test123");
    }
}
