# Feature Specification: M1 Streaming Hub

## Overview

M1 (Streaming Hub) is the primary content consumption module for Playa Tay. It provides a unified interface for streaming video content from multiple providers (YouTube, Twitch, custom HLS/DASH) with intelligent recommendations, watch queue management, and DRM support via Electron sidecar.

**Status**: Phase 1 (Foundation) - In Development
**Priority**: P0 (Core Module)
**Owner**: TayDa64

## Vision Alignment

Per [VISION.md](../../VISION.md):
- **Primary Outcome**: Personalized streaming hub as the first deliverable in v1
- **Module Type**: Separate Window (heavy, isolated) for streaming players
- **Security**: Enforces module isolation with contextIsolation, sandbox, nodeIntegration=false
- **Performance**: Targets <2s startup, lazy loading, <800MB active memory

## Goals

### Phase 1 (Foundation) - Current Focus

1. **Core Streaming Infrastructure**
   - Register custom protocol handlers for streaming (`stream://`)
   - Support for HLS, DASH, and standard MP4/WebM formats
   - Byte-range request handling for seeking/buffering

2. **Basic UI Components**
   - Video player with standard controls (play, pause, seek, volume)
   - Card grid layout for content browsing (following ps-vue-multi-view.webp design)
   - Basic metadata display (title, description, duration)

3. **YouTube Provider Integration**
   - Search YouTube content via API
   - Stream YouTube videos (non-DRM content initially)
   - Display thumbnails, titles, view counts

4. **Watch Queue & History**
   - Persistent watch queue (SQLite)
   - Watch history tracking with consent
   - Resume playback from last position

5. **Security Implementation**
   - Module runs in isolated window
   - Security flags enforced: `contextIsolation=true`, `sandbox=true`, `nodeIntegration=false`
   - Network policy: allowlist for YouTube, streaming CDNs
   - No direct filesystem access from renderer

### Phase 2 (Enhancement) - Future

- Twitch integration
- DRM/Widevine support via Electron Pattern A
- Picture-in-Picture (PiP) mode
- Advanced recommendations (AI-powered)
- Custom HLS/DASH sources
- Subtitles and multi-audio tracks

## Non-Goals (Out of Scope)

- Music streaming (different module: M7)
- Live TV/DVR functionality
- Video editing or transcoding
- P2P streaming or torrents
- Mobile app streaming (v1.5+)

## Architecture

### Module Structure

```
specs/002-streaming-hub/
├── spec.md                    # This file
├── plan.md                    # Implementation plan
└── tasks.md                   # Task breakdown (generated)

examples/api/src-tauri/src/
├── streaming/                 # Streaming module backend
│   ├── mod.rs                # Module exports
│   ├── protocol.rs           # Custom protocol handler
│   ├── providers/            # Provider integrations
│   │   ├── mod.rs
│   │   └── youtube.rs        # YouTube API client
│   └── models.rs             # Data models

examples/api/src/
├── views/
│   └── Streaming.svelte      # Main streaming view
└── lib/
    └── streaming/            # Streaming utilities
        ├── player.ts         # Video player logic
        └── queue.ts          # Queue management
```

### Data Model

#### StreamingContent
```typescript
interface StreamingContent {
  id: string              // Unique identifier
  provider: 'youtube' | 'twitch' | 'custom'
  url: string            // Stream URL
  title: string
  description?: string
  thumbnail?: string
  duration?: number      // seconds
  creator?: string
  viewCount?: number
  publishedAt?: Date
}
```

#### WatchQueueItem
```typescript
interface WatchQueueItem {
  id: string
  contentId: string
  content: StreamingContent
  addedAt: Date
  position: number       // queue order
}
```

#### WatchHistoryEntry
```typescript
interface WatchHistoryEntry {
  id: string
  contentId: string
  content: StreamingContent
  watchedAt: Date
  progress: number       // seconds watched
  completed: boolean     // watched >90%
  duration: number       // total watch time
}
```

### API Contracts

#### Tauri Commands

```rust
// Open streaming hub module
#[tauri::command]
async fn open_streaming_hub() -> Result<(), String>

// Stream a specific video
#[tauri::command]
async fn stream_video(content_id: String) -> Result<(), String>

// Search YouTube
#[tauri::command]
async fn search_youtube(query: String, limit: Option<u32>) -> Result<Vec<StreamingContent>, String>

// Queue management
#[tauri::command]
async fn add_to_queue(content_id: String) -> Result<(), String>

#[tauri::command]
async fn remove_from_queue(queue_id: String) -> Result<(), String>

#[tauri::command]
async fn get_watch_queue() -> Result<Vec<WatchQueueItem>, String>

// History management
#[tauri::command]
async fn save_watch_progress(content_id: String, progress: f64) -> Result<(), String>

#[tauri::command]
async fn get_watch_history(limit: Option<u32>) -> Result<Vec<WatchHistoryEntry>, String>

#[tauri::command]
async fn clear_watch_history() -> Result<(), String>
```

#### Protocol Handler

- **Protocol**: `stream://`
- **Purpose**: Serve video content with byte-range support
- **Security**: Only serves content from approved providers
- **Example**: `stream://youtube/VIDEO_ID`

### Security Requirements

1. **Module Isolation**
   - Streaming module runs in separate Tauri window
   - No shared JavaScript context with main app
   - IPC communication only via Tauri commands

2. **Security Flags** (Enforced)
   ```rust
   WebviewWindowBuilder::new()
     .context_menu(false)           // Disable context menu in production
     .devtools(cfg!(debug_assertions)) // DevTools in dev only
     // These are enforced by Tauri by default:
     // - contextIsolation: true
     // - nodeIntegration: false
     // - sandbox: true (on supported platforms)
   ```

3. **Network Policies**
   - Allowlist domains:
     - `*.youtube.com`
     - `*.googlevideo.com`
     - `*.ytimg.com` (thumbnails)
   - No arbitrary URL loading
   - CSP headers enforced

4. **Data Protection**
   - Watch history: User consent required
   - API keys: Stored in OS keychain
   - No PII in logs

### Performance Budgets

Per [VISION.md](../../VISION.md):

1. **Startup Time**
   - Module load: <1.5s from activation
   - First video frame: <3s after URL provided

2. **Memory Footprint**
   - Idle (hub open, no playback): <100MB
   - Active playback: <300MB
   - Peak (with thumbnails cache): <500MB

3. **Network Efficiency**
   - Thumbnail prefetch: Top 20 results
   - Adaptive bitrate streaming
   - Cache thumbnails for 7 days

4. **UI Responsiveness**
   - Seek latency: <200ms
   - Grid scroll: 60fps minimum
   - Search results: <500ms

### Caching Strategy

1. **Thumbnail Cache**
   - LRU eviction, max 100MB
   - TTL: 7 days
   - Prefetch on search results

2. **Metadata Cache**
   - In-memory cache for recent searches
   - TTL: 1 hour
   - Max 1000 entries

3. **Playback Position**
   - Persisted to SQLite every 5 seconds during playback
   - Restored on resume

## User Experience

### UI Design

Following `ps-vue-multi-view.webp` design reference:

1. **Layout**
   - Top: Search bar + filters
   - Main: Card grid (3-4 columns responsive)
   - Sidebar: Watch queue, history
   - Player: Full-screen capable, PiP-ready

2. **Card Design**
   - Thumbnail with play overlay
   - Title (2 lines max, ellipsis)
   - Creator + view count
   - Duration badge
   - Queue/bookmark actions

3. **Player Controls**
   - Standard: Play/pause, volume, fullscreen
   - Seek bar with preview thumbnails (future)
   - Settings: Quality, speed, captions (future)

### User Flows

1. **Search & Play**
   ```
   User → Opens Streaming Hub → Searches "keyword" 
   → Results load (<500ms) → Clicks thumbnail 
   → Player loads (<3s) → Video plays
   ```

2. **Queue Management**
   ```
   User → Browses results → Clicks "Add to Queue" 
   → Item added to sidebar → Opens queue 
   → Clicks item → Plays immediately
   ```

3. **Resume Watching**
   ```
   User → Opens Streaming Hub → Sees "Continue Watching" section
   → Clicks partially watched video → Resumes from last position
   ```

## Integration Points

### With Core Modules

1. **M6 (Settings)**
   - Streaming quality preferences
   - Privacy settings for watch history
   - API key management for YouTube

2. **M5 (Unified Search)**
   - Streaming results appear in global search
   - Deep links to streaming content

3. **AI Agents (Future)**
   - Recommender agent suggests content
   - Summarizer generates TL;DR for long videos

### External APIs

1. **YouTube Data API v3**
   - Search: `youtube.search.list`
   - Video details: `youtube.videos.list`
   - Rate limit: 10,000 units/day (free tier)

2. **Streaming Protocols**
   - HLS (HTTP Live Streaming)
   - DASH (Dynamic Adaptive Streaming over HTTP)
   - Progressive download (MP4/WebM)

## Testing Strategy

### Unit Tests
- Protocol handler byte-range logic
- Queue operations (add, remove, reorder)
- History tracking with consent checks

### Integration Tests
- Tauri command invocations
- SQLite persistence
- YouTube API mocking

### E2E Tests
- Search → play flow
- Queue management
- Resume playback
- Security flag verification

## Acceptance Criteria

### Must Have (Phase 1)
- [ ] Custom `stream://` protocol registered and working
- [ ] YouTube search returns relevant results (<500ms)
- [ ] Video playback works for at least 3 YouTube videos
- [ ] Watch queue persists across app restarts
- [ ] Watch history tracks progress (with user consent)
- [ ] Security flags enforced (manual verification)
- [ ] Startup time <2s from module activation
- [ ] Memory usage <300MB during playback

### Nice to Have (Phase 1)
- [ ] Thumbnail prefetching for smooth scrolling
- [ ] Keyboard shortcuts (space=play/pause, arrows=seek)
- [ ] Responsive grid layout (3-5 columns)

### Future (Phase 2+)
- [ ] Twitch integration
- [ ] DRM/Widevine support
- [ ] PiP mode
- [ ] AI recommendations
- [ ] Custom HLS/DASH sources

## Open Questions

1. **YouTube API Key Management**
   - Where to store API key? (User provides, or bundled dev key?)
   - Quota handling when limit reached?

2. **Offline Mode**
   - Support downloaded videos? (out of scope for v1)

3. **Content Moderation**
   - Age-restricted content handling?
   - Parental controls?

## References

- [VISION.md](../../VISION.md) - Overall architecture and module strategy
- [spec.md](../../spec.md) - Top-level specification
- [examples/streaming/](../../examples/streaming/) - Existing streaming example
- [ps-vue-multi-view.webp](../../ps-vue-multi-view.webp) - UI design reference

## Changelog

### 2025-10-05
- Initial specification created
- Defined Phase 1 scope (Foundation)
- Documented data models and API contracts
- Security and performance requirements established
