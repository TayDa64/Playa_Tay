// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { invoke } from '@tauri-apps/api/core'

export interface StreamingContent {
  id: string
  provider: 'youtube' | 'twitch' | 'custom'
  url: string
  title: string
  description?: string
  thumbnail?: string
  duration?: number
  creator?: string
  view_count?: number
  published_at?: string
}

export interface WatchQueueItem {
  id: string
  content_id: string
  content: StreamingContent
  added_at: string
  position: number
}

export interface WatchHistoryEntry {
  id: string
  content_id: string
  content: StreamingContent
  watched_at: string
  progress: number
  completed: boolean
  duration: number
}

/**
 * Open the streaming hub in a new window
 */
export async function openStreamingHub(): Promise<void> {
  await invoke('open_streaming_hub')
}

/**
 * Search YouTube for videos
 */
export async function searchYouTube(
  query: string,
  limit?: number
): Promise<StreamingContent[]> {
  return await invoke('search_youtube', { query, limit })
}

/**
 * Add a video to the watch queue
 */
export async function addToQueue(content: StreamingContent): Promise<void> {
  await invoke('add_to_queue', { content })
}

/**
 * Remove an item from the watch queue
 */
export async function removeFromQueue(queueId: string): Promise<void> {
  await invoke('remove_from_queue', { queueId })
}

/**
 * Get all items in the watch queue
 */
export async function getWatchQueue(): Promise<WatchQueueItem[]> {
  return await invoke('get_watch_queue')
}

/**
 * Save watch progress for a video
 */
export async function saveWatchProgress(
  content: StreamingContent,
  progress: number,
  duration: number
): Promise<void> {
  await invoke('save_watch_progress', { content, progress, duration })
}

/**
 * Get watch history
 */
export async function getWatchHistory(
  limit?: number
): Promise<WatchHistoryEntry[]> {
  return await invoke('get_watch_history', { limit })
}

/**
 * Clear all watch history
 */
export async function clearWatchHistory(): Promise<void> {
  await invoke('clear_watch_history')
}

/**
 * Set user consent for a feature
 */
export async function setStreamingConsent(
  feature: string,
  enabled: boolean
): Promise<void> {
  await invoke('set_streaming_consent', { feature, enabled })
}

/**
 * Check user consent for a feature
 */
export async function checkStreamingConsent(feature: string): Promise<boolean> {
  return await invoke('check_streaming_consent', { feature })
}
