# M1: Streaming Hub - Implementation Guide

## Overview

M1 (Streaming Hub) is the primary content consumption module for Playa Tay. It provides a unified interface for streaming video content from YouTube with intelligent search, watch queue management, and watch history tracking.

## Features (Phase 1)

### Implemented ✅

- **YouTube Search**: Search for videos with debounced input (300ms)
- **Video Grid**: Responsive card layout with thumbnails, titles, creators, and view counts
- **Video Player**: YouTube iframe embed with automatic playback
- **Watch Queue**: Persistent queue that survives app restarts
- **Watch History**: Optional progress tracking with user consent
- **Consent Management**: Privacy-first approach with explicit user permission
- **Separate Window**: Streaming hub opens in isolated window for better performance and security

### Planned 🔮

- Twitch integration
- Custom HLS/DASH sources
- DRM/Widevine support via Electron Pattern A
- Picture-in-Picture (PiP) mode
- AI-powered recommendations
- Download for offline viewing

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Main App Window                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Welcome View                                         │  │
│  │  ┌──────────────────────────────────┐                │  │
│  │  │  [Open Streaming Hub (M1)]  ◄────┼─────┐          │  │
│  │  └──────────────────────────────────┘                │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                                │
                                │ Opens separate window
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                   Streaming Hub Window                       │
│  ┌────────────────────────────────────────────┬───────────┐ │
│  │  [Search: "programming tutorials"]         │  Queue    │ │
│  ├────────────────────────────────────────────┤  ┌──────┐ │ │
│  │  ┌────────┐  ┌────────┐  ┌────────┐       │  │Video1│ │ │
│  │  │Video 1 │  │Video 2 │  │Video 3 │       │  │Video2│ │ │
│  │  │[Play]  │  │[Play]  │  │[Play]  │       │  │Video3│ │ │
│  │  └────────┘  └────────┘  └────────┘       │  └──────┘ │ │
│  │  ┌────────┐  ┌────────┐  ┌────────┐       │           │ │
│  │  │Video 4 │  │Video 5 │  │Video 6 │       │           │ │
│  │  └────────┘  └────────┘  └────────┘       │           │ │
│  └────────────────────────────────────────────┴───────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Setup

### Prerequisites

1. **YouTube Data API v3 Key**
   - Visit [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project or select existing
   - Enable "YouTube Data API v3"
   - Create credentials (API Key)
   - Set the environment variable:
     ```bash
     export YOUTUBE_API_KEY="your-api-key-here"
     ```

2. **System Dependencies**
   - Tauri build requirements (Rust, Node.js, system libraries)
   - See main project README for details

### Development

```bash
# From project root
cd examples/api

# Install dependencies
pnpm install

# Set YouTube API key
export YOUTUBE_API_KEY="your-api-key-here"

# Run in dev mode
pnpm tauri dev
```

### Building

```bash
# Build for production
pnpm tauri build
```

## Usage

### Opening Streaming Hub

1. Launch the main app
2. Click "Open Streaming Hub (M1)" button on Welcome screen
3. Streaming hub opens in a new window

### Searching Videos

1. Type search query in the search bar
2. Results appear after 300ms debounce
3. Click on video card to play

### Managing Queue

1. Click "Queue" button on any video card
2. Video is added to sidebar queue
3. Queue persists across app restarts
4. Click "X" to remove from queue

### Watch History

On first use, you'll see a consent dialog:

- **Accept**: Your watch progress is saved (can resume videos later)
- **Decline**: No history tracking (videos start from beginning each time)

You can change this setting later in the Settings module (future).

## API Reference

### Tauri Commands

#### `open_streaming_hub()`
Opens the streaming hub in a new window.

```typescript
import { openStreamingHub } from '../lib/streaming/api'
await openStreamingHub()
```

#### `search_youtube(query: string, limit?: number)`
Search YouTube for videos.

```typescript
import { searchYouTube } from '../lib/streaming/api'
const results = await searchYouTube('rust programming', 10)
```

#### `add_to_queue(content: StreamingContent)`
Add a video to watch queue.

```typescript
import { addToQueue } from '../lib/streaming/api'
await addToQueue(videoContent)
```

#### `get_watch_queue()`
Get all items in watch queue.

```typescript
import { getWatchQueue } from '../lib/streaming/api'
const queue = await getWatchQueue()
```

#### `save_watch_progress(content, progress, duration)`
Save watch progress (requires consent).

```typescript
import { saveWatchProgress } from '../lib/streaming/api'
await saveWatchProgress(videoContent, 120.5, 300.0)
```

#### `get_watch_history(limit?: number)`
Get watch history.

```typescript
import { getWatchHistory } from '../lib/streaming/api'
const history = await getWatchHistory(20)
```

### Data Models

#### StreamingContent
```typescript
interface StreamingContent {
  id: string                                    // Video ID
  provider: 'youtube' | 'twitch' | 'custom'    // Provider
  url: string                                   // Full URL
  title: string                                 // Video title
  description?: string                          // Description
  thumbnail?: string                            // Thumbnail URL
  duration?: number                             // Duration in seconds
  creator?: string                              // Channel/creator name
  view_count?: number                           // View count
  published_at?: string                         // Publication date
}
```

## Database Schema

```sql
-- Watch queue
CREATE TABLE watch_queue (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_json TEXT NOT NULL,
    added_at TEXT NOT NULL,
    position INTEGER NOT NULL,
    UNIQUE(content_id)
);

-- Watch history (with consent)
CREATE TABLE watch_history (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_json TEXT NOT NULL,
    watched_at TEXT NOT NULL,
    progress REAL NOT NULL,
    completed INTEGER NOT NULL,
    duration REAL NOT NULL
);

-- User consent
CREATE TABLE user_consent (
    feature TEXT PRIMARY KEY,
    enabled INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
```

Database location: `{app_data_dir}/streaming.db`

## Security

### Module Isolation

- Streaming hub runs in **separate Tauri window**
- No shared JavaScript context with main app
- Communication only via Tauri IPC commands
- Each window has its own V8 isolate

### Security Flags (Enforced by Tauri)

```rust
// Tauri enforces by default:
contextIsolation: true      // Separate contexts
nodeIntegration: false      // No Node.js access from renderer
sandbox: true              // OS-level sandboxing (platform-dependent)
webSecurity: true          // CORS, CSP enforcement
```

### Privacy

- **Local-First**: All data stored on device in SQLite
- **Consent-Based**: Watch history requires explicit permission
- **No Tracking**: No telemetry or analytics sent to servers
- **User Control**: Clear history anytime

## Performance

### Targets (from VISION.md)

- **Module Load**: <1.5s from activation
- **Search Results**: <500ms
- **Video First Frame**: <3s after URL provided
- **Memory (Idle)**: <100MB
- **Memory (Active)**: <300MB

### Optimizations

1. **Debounced Search**: 300ms delay reduces API calls
2. **Lazy Loading**: Module only loads when opened
3. **Separate Window**: Isolates performance impact
4. **Thumbnail Caching**: Planned for Phase 2
5. **YouTube Iframe**: Leverages YouTube's CDN and caching

## Troubleshooting

### "YouTube API key not configured"

Set the environment variable before starting:
```bash
export YOUTUBE_API_KEY="your-key"
pnpm tauri dev
```

### No search results

- Check your API key is valid
- Check you have quota remaining (10,000 units/day free tier)
- Check your network connection
- Check YouTube API is enabled in Google Cloud Console

### Window won't open

- Check console for errors
- Ensure Tauri commands are registered in `lib.rs`
- Try closing and reopening the main app

### Database errors

- Database is created automatically in app data directory
- Check file permissions
- Delete `streaming.db` to reset (loses queue and history)

## Testing

### Unit Tests (Rust)

```bash
cd examples/api/src-tauri
cargo test streaming::
```

### Manual Testing Checklist

- [ ] Open streaming hub window
- [ ] Search for "programming tutorials"
- [ ] Results load within 500ms
- [ ] Click video card to play
- [ ] Video plays in YouTube player
- [ ] Add video to queue
- [ ] Queue persists after restart
- [ ] Consent dialog appears on first use
- [ ] Accept consent and verify history saves
- [ ] Decline consent and verify no history
- [ ] Remove video from queue

## Future Enhancements (Phase 2+)

- [ ] Twitch live streaming support
- [ ] Custom HLS/DASH URL input
- [ ] DRM/Widevine for premium content
- [ ] Picture-in-Picture mode
- [ ] Keyboard shortcuts (Space = play/pause, etc.)
- [ ] Video quality selection
- [ ] Playback speed control
- [ ] Subtitle support
- [ ] Multi-audio track selection
- [ ] AI-powered recommendations
- [ ] Download for offline viewing
- [ ] Watch together (multi-user sync)

## Contributing

See main project CONTRIBUTING.md for guidelines.

## License

Apache-2.0 OR MIT
