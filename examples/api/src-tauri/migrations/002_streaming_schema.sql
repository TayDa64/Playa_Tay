-- M1 Streaming Hub Database Schema
-- Migration 002: Create streaming-related tables
-- Created: Phase 1 (Foundation)

-- Watch History: Track user's streaming viewing history
-- Retention: 365 days (per VISION.md)
CREATE TABLE IF NOT EXISTS watch_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK(provider IN ('youtube', 'twitch', 'other')),
  title TEXT NOT NULL,
  channel TEXT NOT NULL,
  thumbnail_url TEXT,
  watched_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  progress_seconds INTEGER DEFAULT 0 CHECK(progress_seconds >= 0),
  completion_percent INTEGER DEFAULT 0 CHECK(completion_percent BETWEEN 0 AND 100),
  source TEXT CHECK(source IN ('manual', 'queue', 'recommendation')),
  UNIQUE(user_id, stream_id, watched_at)
);

-- Watch Queue: User's playlist of streams to watch next
-- FIFO ordering with manual reordering support
CREATE TABLE IF NOT EXISTS watch_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK(provider IN ('youtube', 'twitch', 'other')),
  title TEXT NOT NULL,
  channel TEXT NOT NULL,
  thumbnail_url TEXT,
  position INTEGER NOT NULL CHECK(position >= 0),
  added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(user_id, stream_id)
);

-- Recommendations: AI-generated stream suggestions
-- Hybrid local + cloud recommendations (per VISION.md)
CREATE TABLE IF NOT EXISTS recommendations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK(provider IN ('youtube', 'twitch', 'other')),
  title TEXT NOT NULL,
  channel TEXT NOT NULL,
  thumbnail_url TEXT,
  confidence_score REAL NOT NULL CHECK(confidence_score BETWEEN 0.0 AND 1.0),
  reasoning TEXT,
  generated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  dismissed BOOLEAN DEFAULT 0,
  UNIQUE(user_id, stream_id)
);

-- Performance Indexes (target: <100ms query time)
CREATE INDEX IF NOT EXISTS idx_history_user_watched
  ON watch_history(user_id, watched_at DESC);

CREATE INDEX IF NOT EXISTS idx_queue_user_position
  ON watch_queue(user_id, position);

CREATE INDEX IF NOT EXISTS idx_recommendations_user_score
  ON recommendations(user_id, confidence_score DESC, dismissed);

-- Cleanup trigger for 365-day retention policy
-- Runs after each insert to maintain performance
CREATE TRIGGER IF NOT EXISTS cleanup_old_history
AFTER INSERT ON watch_history
BEGIN
  DELETE FROM watch_history
  WHERE watched_at < datetime('now', '-365 days');
END;

-- Data integrity: Ensure queue positions are sequential
-- This trigger renumbers positions after deletions
CREATE TRIGGER IF NOT EXISTS renumber_queue_positions
AFTER DELETE ON watch_queue
BEGIN
  UPDATE watch_queue
  SET position = (
    SELECT COUNT(*)
    FROM watch_queue AS wq2
    WHERE wq2.user_id = watch_queue.user_id
      AND wq2.position < watch_queue.position
  )
  WHERE user_id = OLD.user_id;
END;
