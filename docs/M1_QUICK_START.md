# M1 Streaming Hub - Quick Start Guide

## Overview

This guide helps you quickly integrate and test the M1 Streaming Hub module.

## Prerequisites

```bash
# Install pnpm if not already installed
npm install -g pnpm@10.16.0

# Install Electron for DRM support (optional but recommended)
cd /path/to/Playa_Tay
pnpm -F @playa/electron-shell install
```

## Running the Application

```bash
# From repository root
cd examples/api

# Development mode
pnpm run tauri dev
```

## Accessing Streaming Hub

1. Launch the application
2. Click "Streaming Hub" in the left sidebar
3. Browse the video library
4. Test features:
   - Click "Play" on any video
   - Click "+ Queue" to add videos to watch queue
   - Try DRM-protected content (requires Electron)

## Testing DRM Support

The Streaming Hub automatically detects if Electron (with Widevine) is available:

- **Green ✓**: DRM support available
- **Red ✗**: DRM not available (install Electron)

To test DRM:
1. Click on "DRM Protected Content" video
2. If Electron is available, it will launch in a separate window
3. If not, you'll see installation instructions

## Architecture Components

### Frontend
- **File**: `examples/api/src/views/StreamingHub.svelte`
- **Features**: Video library, watch queue, player overlay

### Backend
- **Files**: 
  - `examples/api/src-tauri/src/cmd.rs` (commands)
  - `examples/api/src-tauri/src/lib.rs` (protocol registration)
- **Commands**:
  - `get_streaming_capabilities()` - Check available features
  - `open_electron_feature(url)` - Launch DRM player

## Key Features

### 1. Video Library
- Card grid layout
- Thumbnail, title, duration display
- DRM badge for protected content
- Play and queue actions

### 2. Watch Queue
- Persistent across sessions (localStorage)
- Add/remove videos
- Quick play from queue

### 3. Player Support
- **In-app player**: Non-DRM content
- **Electron player**: DRM-protected content (Widevine)

### 4. Security
- Module isolation enforced
- CSP headers on all streaming content
- Electron sidecar sandboxed (contextIsolation, no nodeIntegration)

## Customization

### Adding New Video Providers

Edit `StreamingHub.svelte`:

```javascript
const videoLibrary = [
  {
    id: 4,
    title: 'Your Video',
    thumbnail: 'https://example.com/thumb.jpg',
    duration: '12:34',
    provider: 'custom',
    requiresDRM: false
  }
]
```

### Adding Custom Protocols

Edit `examples/api/src-tauri/src/lib.rs`:

```rust
.register_asynchronous_uri_scheme_protocol("myprotocol", |_ctx, request, responder| {
    // Your protocol handler logic
})
```

## Troubleshooting

### Issue: DRM content won't play

**Solution**: Install Electron dependencies
```bash
pnpm -F @playa/electron-shell install
```

### Issue: Videos not loading

**Check**:
1. Browser console for errors
2. Protocol registration in backend
3. Video URL format

### Issue: Queue not persisting

**Solution**: Check browser localStorage permissions

## Performance Tips

1. **Limit queue size**: Keep under 50 items for best performance
2. **Clear cache periodically**: 
   ```javascript
   localStorage.removeItem('watchQueue')
   ```
3. **Use appropriate video formats**: MP4 for best compatibility

## Next Steps

- [ ] Integrate with real video providers (YouTube, Twitch)
- [ ] Implement HLS/DASH adaptive streaming
- [ ] Add history tracking with user consent
- [ ] Build recommendation engine
- [ ] Performance profiling and optimization

## Resources

- [Full Documentation](./M1_STREAMING_HUB.md)
- [VISION.md](../VISION.md)
- [Tauri Documentation](https://tauri.app/)
- [Electron Security](https://www.electronjs.org/docs/latest/tutorial/security)

---

**Need Help?** Open an issue on GitHub or check existing documentation.
