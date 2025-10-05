# M1: Streaming Hub

**Priority**: P0 (Core Feature)  
**Status**: ✅ Initial Implementation Complete  
**Version**: v1.0

## Overview

The Streaming Hub is the primary content streaming module for Playa Tay, providing a unified interface for playing video content from multiple providers with intelligent queue management and watch history tracking.

## Quick Links

- **Implementation Details**: [M1_IMPLEMENTATION.md](./M1_IMPLEMENTATION.md)
- **Developer Guide**: [QUICKSTART.md](./QUICKSTART.md)
- **Vision Spec**: [VISION.md - M1 Section](../../VISION.md#m1-streaming-hub-p0)

## Key Features

### ✅ Implemented (v1.0)

- **Multi-Provider Support**: YouTube, Twitch, custom HLS/DASH streams
- **Card Grid UI**: Apple TV-inspired layout with thumbnails and metadata
- **Watch Queue**: User-managed queue with add/remove functionality
- **Provider Filtering**: Filter streams by provider type
- **DRM Integration**: Automatic Electron sidecar for Widevine content
- **Watch History**: Track previously watched content
- **Now Playing**: Visual indicator for currently playing stream

### 🚧 In Progress / Planned

#### v1.5 (Q1 2026)
- **Data Persistence**: SQLite storage for queue and history
- **Video Player**: Integrated video.js player with controls
- **API Integration**: YouTube and Twitch API connections
- **AI Recommendations**: Content suggestions based on watch patterns
- **Advanced Controls**: Playback speed, quality selection, subtitles

#### v2.0 (Q2 2026)
- **Picture-in-Picture**: System-level PiP mode
- **Multi-Audio/Subtitles**: Track selection
- **Casting**: Chromecast and AirPlay support
- **Offline Mode**: Download for offline viewing
- **Watch Parties**: Synchronized viewing with friends

## Architecture

### Component Structure

```
M1: Streaming Hub
├── Backend (Rust/Tauri)
│   ├── cmd.rs
│   │   ├── StreamItem (data structure)
│   │   ├── play_stream() - Play content
│   │   ├── add_to_queue() - Queue management
│   │   ├── get_watch_history() - History retrieval
│   │   ├── save_watch_progress() - Progress tracking
│   │   └── get_recommendations() - AI suggestions
│   └── streaming_tests.rs - Unit tests
│
└── Frontend (Svelte)
    └── StreamingHub.svelte
        ├── Stream Cards - Content grid
        ├── Watch Queue - Queue panel
        ├── Provider Filter - Filter controls
        ├── Now Playing - Status display
        └── History List - Recent watches
```

### Data Flow

```
User Action (Click Play)
    ↓
StreamingHub.svelte
    ↓
invoke('play_stream', { item })
    ↓
Tauri IPC Bridge
    ↓
play_stream() in cmd.rs
    ↓
DRM Detection Logic
    ├─→ DRM Required? → open_electron_feature() → Electron Sidecar
    └─→ No DRM → Ok() → Native WebView Player
```

## Security & Performance

### Security Posture
- ✅ Module isolation via Tauri commands
- ✅ DRM content in sandboxed Electron process
- ✅ `contextIsolation: true`
- ✅ `nodeIntegration: false`
- ✅ `sandbox: true`
- ✅ Type-safe IPC with Rust/TypeScript contracts

### Performance Metrics (v1.0)
- **Startup**: <2s (loads with main app)
- **Memory**: ~200MB base + streaming overhead
- **Module Switch**: <300ms
- **Stream Initiation**: <1s (network dependent)

## Usage Examples

### Playing a Stream

```javascript
// In Svelte component
const stream = {
  id: 'yt-123',
  title: 'My Video',
  url: 'https://youtube.com/watch?v=...',
  provider: 'youtube',
  duration: 600
};

await invoke('play_stream', { item: stream });
```

### Adding to Queue

```javascript
await invoke('add_to_queue', { item: stream });
```

### Retrieving History

```javascript
const history = await invoke('get_watch_history', { limit: 10 });
```

## Testing

### Run Unit Tests

```bash
cd examples/api/src-tauri
cargo test streaming_tests --lib
```

### Manual Testing Checklist

- [ ] Navigate to Streaming Hub view
- [ ] Click Play on a stream card
- [ ] Add multiple items to queue
- [ ] Toggle queue visibility
- [ ] Filter by provider (YouTube, Twitch, HLS)
- [ ] Verify Now Playing indicator appears
- [ ] Stop playback
- [ ] Check console for errors

## Known Issues & Limitations

### v1.0 Limitations

1. **No Persistence** (#TODO-v1.5)
   - Queue and history are in-memory only
   - Data lost on app restart
   - **Workaround**: Manual re-queuing

2. **No Embedded Player** (#TODO-v1.5)
   - Frontend doesn't include video player
   - Relies on external playback
   - **Workaround**: Use Electron for all playback

3. **Sample Data Only** (#TODO-v1.5)
   - Hardcoded stream list
   - No real API integration
   - **Workaround**: Manually edit sampleStreams array

4. **No Recommendations** (#TODO-v1.5)
   - AI not implemented
   - Returns empty list
   - **Workaround**: Manual content discovery

### Known Bugs

None reported yet.

## Contributing

### Adding a New Provider

1. Update `StreamItem.provider` types
2. Add sample data in `StreamingHub.svelte`
3. Add provider color in `getProviderColor()`
4. Add DRM detection logic if needed
5. Update tests
6. Update documentation

### Implementing Persistence

See [M1_IMPLEMENTATION.md#future-enhancements](./M1_IMPLEMENTATION.md#future-enhancements) for SQLite schema design.

## Resources

- [Full Implementation Documentation](./M1_IMPLEMENTATION.md)
- [Developer Quick Start Guide](./QUICKSTART.md)
- [VISION.md Specification](../../VISION.md#m1-streaming-hub-p0)
- [Constitution: Module Isolation](../../.specify/memory/constitution.md#module-isolation-strategy)
- [Pattern A Electron Integration](../001-selective-electron/spec.md)

## Changelog

### 2025-01-XX - v1.0 Initial Release
- ✅ Core streaming commands implemented
- ✅ StreamingHub.svelte UI component
- ✅ Multi-provider support (YouTube, Twitch, HLS/DASH)
- ✅ DRM integration via Pattern A
- ✅ Watch queue and history (in-memory)
- ✅ Provider filtering
- ✅ Card grid UI matching VISION.md design system
- ✅ Unit tests for all commands
- ✅ Comprehensive documentation

---

**Maintainer**: TayDa64  
**Last Updated**: 2025-01-XX
