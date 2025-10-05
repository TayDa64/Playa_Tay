# M1: Streaming Hub Implementation

**Status**: Implemented (v1.0)
**Created**: 2025-10-05
**Reference**: VISION.md - M1: Streaming Hub (P0)

## Overview

This document describes the implementation of M1: Streaming Hub, following the specifications outlined in VISION.md. The implementation provides a unified interface for streaming content from multiple providers with module isolation, security enforcement, and performance optimization.

## Vision Alignment

Per VISION.md section "Integrations and Feature Modules":

> **M1: Streaming Hub** (P0)
> - **Providers**: YouTube, Twitch, custom HLS/DASH
> - **Protocols**: HLS, DASH, WebRTC (for live)
> - **DRM**: Widevine via Electron sidecar (Pattern A)
> - **Features**: Watch queue, history, recommendations
> - **UI**: Card grid, inline player, PiP mode

## Implementation Details

### Frontend Component

**File**: `examples/api/src/views/StreamingHub.svelte`

**Features Implemented**:
- ✅ Provider tabs (YouTube, Twitch, Custom HLS/DASH)
- ✅ Search functionality per provider
- ✅ Watch queue management with add/remove
- ✅ Watch history tracking with timestamps
- ✅ Recommendations grid with card-based UI
- ✅ DRM status indicator (checks Electron availability)
- ✅ Modal error handling for playback failures
- ✅ Responsive card grid layout
- ⏳ Inline player (future)
- ⏳ PiP mode (future)

**UI Design**:
- Card grid using `grid-template-columns: repeat(auto-fill, minmax(280px, 1fr))`
- Hover effects with `transform: translateY(-4px)` for card elevation
- Duration badges on thumbnails
- Play overlay button appearing on hover
- Compact list view for queue and history
- Color-coded status indicators (DRM ready/unavailable)

### Backend Commands

**File**: `examples/api/src-tauri/src/cmd.rs`

**Commands Implemented**:

1. **`open_video(url: String)`**
   - Opens video URL in system default player
   - Platform-specific: uses `start` (Windows), `open` (macOS), `xdg-open` (Linux)
   - Future: Native player integration

2. **`get_watch_history()`**
   - Returns watch history with timestamps
   - Currently returns placeholder data
   - Production: Would read from SQLite database

3. **`get_watch_queue()`**
   - Returns user's watch queue
   - Currently returns placeholder data
   - Production: Would read from SQLite database

4. **`add_to_watch_queue(video: Video)`**
   - Adds video to queue
   - Logs action for now
   - Production: Would persist to database

5. **`remove_from_watch_queue(video_id: String)`**
   - Removes video from queue
   - Production: Would delete from database

6. **`add_to_watch_history(video: Video)`**
   - Tracks watched videos with timestamps
   - Production: Would persist to database

7. **`clear_watch_history()`**
   - Clears all watch history
   - Production: Would delete from database

8. **`search_videos(query: String, provider: String)`**
   - Searches provider APIs
   - Production: Would integrate with YouTube API, Twitch API, etc.

**Data Model**:
```rust
pub struct Video {
  pub id: String,
  pub title: String,
  pub provider: String,
  pub url: String,
  pub thumbnail: String,
  pub duration: String,
  pub watched_at: Option<String>,
  pub requires_drm: bool,
}
```

### Module Isolation Strategy

Per VISION.md "Module Isolation Strategy":

> **Module Types**:
> - **BrowserView Modules** (light, embedded): Settings, search, AI chat
> - **Separate Windows** (heavy, isolated): Streaming players, terminal, financial charts

**Implementation**:
- ✅ Streaming Hub is a separate Svelte component (`StreamingHub.svelte`)
- ✅ Backend commands follow Tauri's IPC isolation pattern
- ✅ DRM content uses Electron sidecar (Pattern A) as separate process
- ✅ Each video playback can spawn isolated Electron window
- ✅ No shared state between modules except through Tauri IPC

**Security Boundaries** (from VISION.md):
- ✅ Each module has its own V8 context with contextIsolation=true
- ✅ IPC via authenticated channels (Tauri commands)
- ⏳ Network policies per module (future: allowlist domains in manifest)

### Security Flags

Per VISION.md "Sandboxing" section and `specs/001-selective-electron/spec.md`:

**Electron Sidecar Security** (already enforced in `packages/electron-shell/src/main.ts`):
```typescript
webPreferences: {
  contextIsolation: true,      // ✅ Enforced
  nodeIntegration: false,      // ✅ Enforced
  sandbox: true,               // ✅ Enforced
  devTools: !isProduction,     // ✅ Disabled in production
}
```

**Content Security Policy**:
```typescript
'Content-Security-Policy': [
  "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https:;"
]
```

**IPC Security**:
- Ephemeral auth tokens generated per Electron spawn
- `PLAYA_AUTH_TOKEN` env var passed to sidecar
- No direct Node.js access from renderer process

### Performance Budgets

Per VISION.md "Performance, Reliability, and Operations":

**Target Metrics**:
- ⏳ Cold Start: <2 seconds (overall app)
- ⏳ Module load time: <500ms (lazy-loaded)
- ⏳ Memory footprint: <200MB idle, <800MB with 3 modules

**Current Status**:
- Module is lazy-loaded (only imports when tab is selected)
- Placeholder data prevents database overhead during development
- Card grid uses CSS Grid for GPU-accelerated layout
- Hover animations use `transform` for 60fps performance

**Future Optimizations**:
- Pre-fetch thumbnails based on usage patterns
- Virtual scrolling for large video lists
- Image lazy-loading for thumbnails
- Cache API responses (1h TTL per VISION.md)

## Integration with Existing Features

### Electron Pattern A Integration

The Streaming Hub leverages the existing Electron sidecar (Pattern A) for DRM content:

1. **Availability Check**: 
   - `isElectronAvailable()` checks if Electron sidecar is installed
   - Shows DRM status indicator in UI

2. **DRM Playback**:
   - Videos with `requiresDRM: true` use `openElectronFeature(url)`
   - Spawns Electron window with security flags enforced
   - Falls back to browser for non-DRM content

3. **Error Handling**:
   - `not_installed` error shows modal with installation instructions
   - User can retry after installing dependencies

### Navigation Integration

**File**: `examples/api/src/App.svelte`

Added to views array:
```svelte
{
  label: 'M1: Streaming Hub',
  component: StreamingHub,
  icon: 'i-ph-play-circle'
}
```

- Appears after "Welcome" in sidebar navigation
- Icon: `i-ph-play-circle` (phosphor icon set)
- Accessible via keyboard navigation

## Testing

### Manual Testing

To test the Streaming Hub:

1. **Start Development Server**:
   ```bash
   pnpm run example:api:dev
   ```

2. **Navigate to M1: Streaming Hub**:
   - Click "M1: Streaming Hub" in sidebar
   - Or use keyboard navigation (arrow keys)

3. **Test Provider Switching**:
   - Click YouTube/Twitch/Custom tabs
   - Verify active tab styling

4. **Test Search**:
   - Enter query in search bar
   - Press Enter or click Search button
   - Check console for search action

5. **Test Video Actions**:
   - Click "Add to Queue" on recommendation cards
   - Click "Play" on queue items
   - Verify queue updates

6. **Test DRM Indicator**:
   - Check for "DRM ready" or "DRM content unavailable"
   - Depends on Electron sidecar installation status

### Integration Testing

Currently, no automated tests for Streaming Hub. Recommended tests:

- [ ] Unit test for Video data model serialization
- [ ] Integration test for Tauri commands (mock database)
- [ ] E2E test for search flow (Playwright)
- [ ] E2E test for queue management
- [ ] Visual regression test for card grid layout

## Future Enhancements

Per VISION.md roadmap:

### v1.0 Completion
- [ ] Provider API integration (YouTube Data API v3, Twitch Helix API)
- [ ] SQLite database for persistence (via Prisma)
- [ ] Inline player with controls
- [ ] Picture-in-Picture mode support

### v1.5
- [ ] Multi-account support (switch between profiles)
- [ ] Advanced filtering (by genre, duration, upload date)
- [ ] Keyboard shortcuts (Space = play/pause, Left/Right = seek)

### v2.0
- [ ] AI-powered recommendations (local embeddings + cloud LLM)
- [ ] Cross-device sync (watch queue, history)
- [ ] Custom dashboards (pinned streams, favorite channels)
- [ ] TV app integration (cast to Apple TV, Android TV)

## Open Questions

1. **DRM Licensing** (blocker for Widevine):
   - How to obtain Widevine CDM for indie app?
   - Distribution method for CDM
   - Cost model (per-install, flat fee, revenue share?)

2. **Provider API Keys**:
   - YouTube Data API quota management
   - Twitch Client ID registration
   - Rate limiting strategy

3. **Data Retention**:
   - Per VISION.md: Stream history = 365 days
   - Should users be able to configure TTL?
   - Export format for history data?

## References

- [VISION.md](../../VISION.md) - Overall architecture and feature specs
- [spec.md](../../spec.md) - Non-regression invariants
- [specs/001-selective-electron/](../001-selective-electron/) - Electron Pattern A/B
- [Tauri IPC Documentation](https://tauri.app/v2/reference/javascript/api/namespacecore/)
- [Electron Security Best Practices](https://www.electronjs.org/docs/latest/tutorial/security)

---

**Last Updated**: 2025-10-05
**Maintainer**: TayDa64
