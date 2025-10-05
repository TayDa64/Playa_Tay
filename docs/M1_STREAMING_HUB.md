# M1: Streaming Hub - Implementation Documentation

**Status**: Core Implementation Complete  
**Version**: 1.0.0  
**Priority**: P0 (Critical Path)  
**Last Updated**: 2025-10-05

---

## Overview

The Streaming Hub (M1) is a modular video streaming component that provides multi-provider video playback with DRM support, following the architecture specifications defined in [VISION.md](../VISION.md).

### Key Features

- ✅ **Multi-provider Support**: YouTube, Twitch, custom HLS/DASH (extensible)
- ✅ **Protocol Support**: HTTP, HTTPS, HLS, DASH streaming protocols
- ✅ **DRM Support**: Widevine via Electron sidecar (Pattern A)
- ✅ **Watch Queue**: Persistent queue with localStorage
- ✅ **Security**: Full isolation with CSP enforcement and sandboxing
- ✅ **Module Isolation**: Separate context with secure IPC

---

## Architecture

### Frontend Component

**Location**: `examples/api/src/views/StreamingHub.svelte`

#### State Management

- `electronAvailable`: Widevine/DRM capability detection
- `watchQueue`: Persistent video queue (localStorage)
- `currentVideo`: Currently playing video
- `isPlaying`: Player state

### Backend Implementation

**Location**: `examples/api/src-tauri/src/cmd.rs`, `examples/api/src-tauri/src/lib.rs`

#### Commands

1. **`get_streaming_capabilities()`** - Returns streaming features and DRM availability
2. **`register_streaming_protocol()`** - Protocol registration placeholder
3. **`open_electron_feature(url)`** - Launches Electron with DRM support

#### Custom Protocol Handler

Registered `stream://` URI scheme for video streaming with proper HTTP headers and range request support.

---

## Security Implementation

### Module Isolation (VISION.md Compliance)

1. **Separate V8 Context**: Streaming instances isolated
2. **Secure IPC**: Tauri's authenticated channels only
3. **Network Policies**: Allowlisted domains per module

### Security Flags (Electron Sidecar)

```javascript
{
  contextIsolation: true,
  nodeIntegration: false,
  sandbox: true,
  webSecurity: true,
  allowRunningInsecureContent: false
}
```

### Content Security Policy

```
default-src 'self' customprotocol: asset:;
connect-src ipc: http://ipc.localhost;
media-src 'self' stream: http: https:;
```

---

## Performance Targets (VISION.md)

| Metric | Target | Status |
|--------|--------|--------|
| Cold Start | <2s | ⏳ To measure |
| Module Load | <1.5s | ⏳ To measure |
| Memory (Idle) | <200MB | ⏳ To measure |
| Memory (Active) | <800MB | ⏳ To measure |

---

## Usage

### User Workflow

1. Browse video library in card grid
2. Add videos to watch queue
3. Play content (in-app or via DRM-enabled Electron)
4. Manage queue (remove/reorder)

### Developer Integration

```svelte
<script>
  import StreamingHub from './views/StreamingHub.svelte'
  let onMessage = (event) => console.log('Stream event:', event)
</script>

<StreamingHub {onMessage} />
```

---

## DRM Integration

### Widevine Support (Pattern A)

1. User clicks DRM-protected video
2. Frontend checks `electronAvailable`
3. Calls `open_electron_feature(drmUrl)` if available
4. Backend spawns Electron with Widevine
5. Secure token via environment variable

### Prerequisites

```bash
pnpm -F @playa/electron-shell install
```

---

## Future Enhancements

### v1.5 (Q1 2026)
- Pattern B: Separate DRM module distribution
- YouTube/Twitch API integration
- History tracking with consent
- AI-powered recommendations
- PiP mode

### v2.0 (Q2 2026)
- Multi-device sync
- Playlist management
- WebRTC live streaming
- Chromecast support
- Offline downloads

---

## References

- [VISION.md](../VISION.md) - Architecture blueprint
- [spec.md](../spec.md) - Non-regression invariants
- [Tauri Custom Protocols](https://tauri.app/v2/guides/custom-protocols/)
- [Electron Security](https://www.electronjs.org/docs/latest/tutorial/security)

---

**Maintainer**: TayDa64  
**License**: Apache-2.0 OR MIT
