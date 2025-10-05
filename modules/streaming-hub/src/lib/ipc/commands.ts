// M1 Streaming Hub - IPC Command Client
// Phase 1 (Foundation): Type-safe Tauri command wrappers
// Security: All network requests proxied through main process

import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types (matching Rust backend)
// ============================================================================

export interface WatchHistoryEntry {
  id: number;
  user_id: string;
  stream_id: string;
  provider: string;
  title: string;
  channel: string;
  thumbnail_url?: string;
  watched_at: string;
  progress_seconds: number;
  completion_percent: number;
  source?: string;
}

export interface WatchQueueEntry {
  id: number;
  user_id: string;
  stream_id: string;
  provider: string;
  title: string;
  channel: string;
  thumbnail_url?: string;
  position: number;
  added_at: string;
}

export interface Recommendation {
  id: number;
  user_id: string;
  stream_id: string;
  provider: string;
  title: string;
  channel: string;
  thumbnail_url?: string;
  confidence_score: number;
  reasoning?: string;
  generated_at: string;
  dismissed: boolean;
}

export interface StreamMetadata {
  stream_id: string;
  provider: string;
  title: string;
  channel: string;
  thumbnail_url?: string;
  stream_url: string;
  live: boolean;
}

export type Provider = 'youtube' | 'twitch' | 'other';
export type PlaybackSource = 'manual' | 'queue' | 'recommendation';

// ============================================================================
// IPC Commands
// ============================================================================

/**
 * Get watch history for the current user
 * @param limit Maximum number of entries to return (default: 50, max: 500)
 * @returns Array of watch history entries, ordered by most recent
 */
export async function getWatchHistory(
  limit?: number
): Promise<WatchHistoryEntry[]> {
  return invoke<WatchHistoryEntry[]>('get_watch_history', {
    request: { limit },
  });
}

/**
 * Get watch queue for the current user
 * @returns Array of queue entries, ordered by position
 */
export async function getWatchQueue(): Promise<WatchQueueEntry[]> {
  return invoke<WatchQueueEntry[]>('get_watch_queue');
}

/**
 * Get AI-generated recommendations for the current user
 * @returns Array of recommendations, ordered by confidence score
 */
export async function getRecommendations(): Promise<Recommendation[]> {
  return invoke<Recommendation[]>('get_recommendations');
}

/**
 * Play a stream
 * Phase 1: Returns stub metadata
 * @param stream_id Unique stream identifier
 * @param provider Stream provider ('youtube', 'twitch', 'other')
 * @returns Stream metadata including playback URL
 */
export async function playStream(
  stream_id: string,
  provider: Provider
): Promise<StreamMetadata> {
  return invoke<StreamMetadata>('play_stream', {
    request: { stream_id, provider },
  });
}

/**
 * Add a stream to the watch queue
 * Position is auto-calculated as next available
 * @param stream Stream details to add
 */
export async function addToQueue(stream: {
  stream_id: string;
  provider: Provider;
  title: string;
  channel: string;
  thumbnail_url?: string;
}): Promise<void> {
  return invoke<void>('add_to_queue', {
    request: stream,
  });
}

/**
 * Save playback progress for a stream
 * Creates or updates watch history entry
 * @param progress Playback progress details
 */
export async function savePlaybackProgress(progress: {
  stream_id: string;
  provider: Provider;
  title: string;
  channel: string;
  thumbnail_url?: string;
  progress_seconds: number;
  completion_percent: number;
  source?: PlaybackSource;
}): Promise<void> {
  return invoke<void>('save_playback_progress', {
    request: progress,
  });
}

/**
 * Remove a stream from the watch queue
 * @param stream_id Stream identifier to remove
 */
export async function removeFromQueue(stream_id: string): Promise<void> {
  return invoke<void>('remove_from_queue', {
    request: { stream_id },
  });
}

/**
 * Dismiss a recommendation
 * @param stream_id Stream identifier to dismiss
 */
export async function dismissRecommendation(stream_id: string): Promise<void> {
  return invoke<void>('dismiss_recommendation', {
    request: { stream_id },
  });
}

// ============================================================================
// Error Handling Utilities
// ============================================================================

/**
 * Type guard for IPC errors
 */
export function isIPCError(error: unknown): error is Error {
  return error instanceof Error;
}

/**
 * Format IPC error for display
 */
export function formatIPCError(error: unknown): string {
  if (isIPCError(error)) {
    return error.message;
  }
  return String(error);
}
