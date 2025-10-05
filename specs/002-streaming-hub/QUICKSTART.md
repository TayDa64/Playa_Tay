# M1 Streaming Hub - Quick Start Guide

## For Developers

### Running the Streaming Hub

1. **Start the Tauri dev server**:
   ```bash
   cd examples/api
   pnpm tauri dev
   ```

2. **Navigate to Streaming Hub**:
   - Click "Streaming Hub" in the left sidebar
   - Or press `Cmd/Ctrl+1` (first module hotkey)

3. **Test Streaming Features**:
   - Click "Play" on any stream card to test playback
   - Click "+ Queue" to add streams to watch queue
   - Use provider filter to filter by YouTube, Twitch, or HLS
   - Click "Queue (N)" button to toggle queue visibility

### Adding New Stream Providers

1. **Update StreamItem provider types** in `cmd.rs`:
   ```rust
   pub provider: String, // Add new provider name here
   ```

2. **Add sample data** in `StreamingHub.svelte`:
   ```javascript
   const sampleStreams = [
     {
       id: 'newprovider-1',
       title: 'New Provider Stream',
       url: 'https://...',
       provider: 'newprovider',
       // ...
     }
   ]
   ```

3. **Add provider color** in `getProviderColor()`:
   ```javascript
   function getProviderColor(provider) {
     const colors = {
       newprovider: '#FF5733',
       // ...
     }
   }
   ```

### Testing DRM Integration

To test DRM content handling:

1. Add a stream with `provider: 'widevine'` or URL containing 'drm'
2. Click "Play" - should trigger Electron sidecar
3. Check console for: `"Playing stream: ... (widevine)"`

### API Reference

#### Backend Commands

```rust
// Play a stream
invoke('play_stream', { item: StreamItem })

// Add to queue
invoke('add_to_queue', { item: StreamItem })

// Get watch history (limit optional)
invoke('get_watch_history', { limit: 10 })

// Save progress
invoke('save_watch_progress', { id: 'stream-id', progress: 120 })

// Get recommendations
invoke('get_recommendations')
```

#### StreamItem Schema

```typescript
interface StreamItem {
  id: string;              // Unique identifier
  title: string;           // Display title
  url: string;             // Playback URL
  thumbnail?: string;      // Thumbnail URL (optional)
  provider: string;        // 'youtube' | 'twitch' | 'hls' | 'dash' | 'widevine'
  duration?: number;       // Duration in seconds (optional, null for live)
  watched_progress?: number; // Progress in seconds (optional)
}
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Host Process                    │
│  ┌───────────────────────────────────────────────────┐  │
│  │            StreamingHub.svelte                    │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐       │  │
│  │  │Stream Card│  │  Queue   │  │ History  │       │  │
│  │  └──────────┘  └──────────┘  └──────────┘       │  │
│  └───────────────────────────────────────────────────┘  │
│                         │                                │
│                         │ invoke()                       │
│                         ▼                                │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Tauri Commands (cmd.rs)             │  │
│  │  • play_stream()      • get_watch_history()      │  │
│  │  • add_to_queue()     • save_watch_progress()    │  │
│  │  • get_recommendations()                         │  │
│  └───────────────────────────────────────────────────┘  │
│                         │                                │
│            ┌────────────┴────────────┐                   │
│            ▼                         ▼                   │
│    ┌──────────────┐         ┌──────────────┐            │
│    │ Native Player│         │ DRM Content? │            │
│    │  (WebView)   │         └──────────────┘            │
│    └──────────────┘                 │                   │
│                                     │ Yes                │
└─────────────────────────────────────┼────────────────────┘
                                      ▼
                            ┌──────────────────┐
                            │ Electron Sidecar │
                            │   (Pattern A)    │
                            │   Widevine DRM   │
                            └──────────────────┘
```

## Security & Performance

### Security Flags (Enforced)
- ✅ `contextIsolation: true` - Separate JS contexts
- ✅ `nodeIntegration: false` - No Node.js in renderer
- ✅ `sandbox: true` - OS-level sandboxing
- ✅ `webSecurity: true` - CORS, CSP enforcement

### Performance Targets (v1.0)
- Startup: <2s (component loads with main app)
- Memory: <200MB base + streaming overhead
- Module Switch: <300ms
- Stream Play: <1s (network dependent)

## Troubleshooting

### Common Issues

1. **"Electron runtime not available"**
   - Run: `pnpm -F @playa/electron-shell install`
   - Verify: `examples/api/src-tauri/src/cmd.rs` detects binary

2. **Streams not playing**
   - Check console for errors
   - Verify URL is valid
   - For DRM: ensure Electron sidecar is running

3. **Queue not persisting**
   - Expected in v1.0 (in-memory only)
   - Will be fixed in v1.5 with SQLite

4. **Thumbnails not loading**
   - Check CORS policy
   - Verify thumbnail URLs are HTTPS
   - Check network tab in DevTools

## Next Steps

### v1.5 Enhancements
- [ ] SQLite persistence for queue/history
- [ ] Real video player integration (video.js)
- [ ] API integrations (YouTube, Twitch APIs)
- [ ] AI-based recommendations
- [ ] Advanced playback controls

### v2.0 Features
- [ ] Picture-in-Picture mode
- [ ] Multi-audio/subtitle tracks
- [ ] Chromecast/AirPlay support
- [ ] Offline downloads
- [ ] Watch party sync viewing

## Resources

- [Full Implementation Docs](./M1_IMPLEMENTATION.md)
- [VISION.md Specification](../../VISION.md#m1-streaming-hub-p0)
- [Pattern A Integration](../001-selective-electron/spec.md)
- [Constitution: Module Isolation](../../.specify/memory/constitution.md)
