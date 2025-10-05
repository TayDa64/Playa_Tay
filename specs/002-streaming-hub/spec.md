# Feature Specification: M1 - Streaming Hub

**Feature Branch**: `[002-streaming-hub]`
**Created**: 2025-10-05
**Priority**: P0 (v1.0 Core)
**Status**: Planning

---

## Executive Summary

The Streaming Hub (M1) is the flagship feature of Playa Tay v1.0, providing a unified interface for consuming content from multiple streaming providers with intelligent recommendations, watch history, and DRM support. It serves as the primary entry point demonstrating the app's core value proposition: a personalized, privacy-first streaming command center.

**Reference Design**: `ps-vue-multi-view.webp` — Multi-view card grid layout with inline player

---

## Vision Alignment

Per [VISION.md](../../VISION.md):
- **Primary Outcome**: Personalized streaming hub with integrated research and automation
- **Module Type**: Separate window (heavy, isolated)
- **Providers**: YouTube, Twitch, custom HLS/DASH
- **Protocols**: HLS, DASH, WebRTC (for live)
- **DRM**: Widevine via Electron sidecar (Pattern A)
- **UI Pattern**: Card grid, inline player, PiP mode
- **Performance Target**: <2s startup contribution, <300MB memory footprint
- **Security**: Full sandboxing, contextIsolation=true, authenticated IPC

---

## User Scenarios & Testing

### Primary User Stories

**US-M1-001: Browse Content Catalog**
> As a user, I want to browse available streams in a visually rich card grid so that I can discover content to watch.

**Acceptance Criteria**:
- Card grid displays with thumbnail, title, channel, view count, duration
- Cards load progressively (virtualized scrolling for performance)
- Hover states show additional metadata (description preview)
- Keyboard navigation (arrow keys, Enter to play)
- Filter by provider (YouTube, Twitch, All)

**US-M1-002: Play Streaming Content**
> As a user, I want to play video content inline or fullscreen with standard playback controls so that I can watch my selected streams.

**Acceptance Criteria**:
- Click card opens inline player in dedicated area
- Player shows: play/pause, volume, seek bar, quality selector, fullscreen
- Supports HLS/DASH adaptive streaming
- Handles live streams (WebRTC for Twitch)
- Picture-in-Picture mode available
- Playback state persists (resume from last position)

**US-M1-003: Manage Watch Queue**
> As a user, I want to add streams to a watch queue so that I can organize my viewing schedule.

**Acceptance Criteria**:
- "Add to Queue" button on cards and in player
- Queue panel shows ordered list with drag-to-reorder
- Auto-advance to next in queue when video ends
- Clear queue action with confirmation
- Queue persists across app restarts

**US-M1-004: View Watch History**
> As a user, I want to see my watch history so that I can revisit content and track my viewing patterns.

**Acceptance Criteria**:
- History page shows chronological list with thumbnails
- Each entry shows: thumbnail, title, channel, watch date, progress %
- Search/filter history by title or channel
- Clear history action with confirmation
- History respects privacy settings (can be disabled)

**US-M1-005: Receive Personalized Recommendations**
> As a user, I want intelligent recommendations based on my watch history so that I discover relevant content without manual searching.

**Acceptance Criteria**:
- "For You" section on home screen with recommended cards
- Recommendations update based on watch history (AI-powered)
- "More like this" action on cards to tune recommendations
- "Not interested" action to remove from recommendations
- Confidence score displayed on each recommendation

### Edge Cases

**EC-M1-001: DRM-Protected Content**
- When user plays DRM content (e.g., premium YouTube), system spawns Electron sidecar with Widevine
- If Widevine unavailable, show modal with instructions and fallback options
- Graceful degradation to non-DRM player for free content

**EC-M1-002: Network Interruption**
- Player shows buffering indicator during network issues
- Auto-retry with exponential backoff (3 attempts)
- Cache last 30s of buffer for seamless recovery
- Error message if unable to recover after retries

**EC-M1-003: High Memory Usage**
- Monitor memory usage per module (target <300MB)
- Evict thumbnail cache when approaching limit
- Unload background videos in queue
- Warning notification if exceeding budget

**EC-M1-004: Multi-Provider Auth**
- Support OAuth2 login for YouTube (Google), Twitch
- Persist tokens in OS keychain (secure storage)
- Automatic token refresh with silent re-auth
- Graceful handling of expired/revoked tokens

---

## Requirements

### Functional Requirements

**FR-M1-001**: The module MUST run in a separate Tauri window with its own process
**FR-M1-002**: The module MUST communicate with main app via authenticated IPC (ephemeral tokens)
**FR-M1-003**: The module MUST support YouTube and Twitch as launch providers
**FR-M1-004**: The module MUST support HLS, DASH, and WebRTC streaming protocols
**FR-M1-005**: The module MUST spawn Electron sidecar for DRM content (Widevine)
**FR-M1-006**: The module MUST enforce security flags: contextIsolation=true, nodeIntegration=false, sandbox=true
**FR-M1-007**: The module MUST persist watch history to SQLite with configurable retention (default 365 days)
**FR-M1-008**: The module MUST implement virtualized scrolling for card grid (performance optimization)
**FR-M1-009**: The module MUST provide PiP (Picture-in-Picture) mode for continuing playback while using other modules
**FR-M1-010**: The module MUST respect privacy settings (opt-out of history, recommendations)
**FR-M1-011**: The module MUST handle network interruptions gracefully with retry logic
**FR-M1-012**: The module MUST stay within <300MB memory footprint (measured at peak usage)
**FR-M1-013**: The module MUST contribute <2s to overall app startup time (lazy-loaded)
**FR-M1-014**: The module MUST support keyboard navigation (100% coverage per accessibility target)
**FR-M1-015**: The module MUST display confidence scores on AI recommendations

### Non-Functional Requirements

**NFR-M1-001**: **Performance**
- Initial render: <500ms
- Card grid virtualization: support 10,000+ items without lag
- Video load time: <3s (P95)
- Memory footprint: <300MB (target), <500MB (hard limit)
- Thumbnail loading: progressive, lazy (IntersectionObserver)

**NFR-M1-002**: **Security**
- OAuth tokens stored in OS keychain
- All API calls proxied through main app (no direct network from renderer)
- CSP enforced: no inline scripts, trusted domains only
- No user-generated content execution (XSS protection)

**NFR-M1-003**: **Accessibility**
- WCAG 2.1 AA compliance
- Full keyboard navigation with visible focus states
- Screen reader support (ARIA labels on all interactive elements)
- High contrast mode support
- Configurable font sizes

**NFR-M1-004**: **Reliability**
- Crash recovery: module crashes don't affect main app
- State persistence: queue and playback position saved every 5s
- Offline graceful degradation: show cached thumbnails, disable player
- Error logging: structured errors sent to main app for telemetry

### Key Entities

**Stream**
```typescript
{
  id: string              // Unique identifier (provider:video_id)
  provider: 'youtube' | 'twitch' | 'hls' | 'dash'
  title: string
  channel: string
  thumbnail_url: string
  duration: number        // seconds, null for live
  view_count: number
  published_at: Date
  url: string
  is_live: boolean
  requires_drm: boolean
  metadata: Record<string, any>
}
```

**WatchHistory**
```typescript
{
  id: number              // Auto-increment primary key
  user_id: string
  stream_id: string
  provider: string
  watched_at: Date
  progress_seconds: number
  completion_percent: number
  source: 'manual' | 'queue' | 'recommendation'
}
```

**WatchQueue**
```typescript
{
  id: number
  user_id: string
  stream_id: string
  position: number        // Order in queue
  added_at: Date
}
```

**Recommendation**
```typescript
{
  id: number
  user_id: string
  stream_id: string
  confidence_score: number  // 0-100
  reasoning: string         // Explainable AI
  generated_at: Date
  dismissed: boolean
}
```

---

## Architecture

### Module Isolation

**Window Type**: Separate Tauri Window (heavy module)
- Dedicated process for streaming UI
- Independent lifecycle (can crash without affecting main app)
- Communicates with main app via IPC bridge

**IPC Bridge**:
```typescript
// Main App → Streaming Module
interface StreamingCommands {
  open_stream(stream_id: string): Promise<void>
  get_history(): Promise<WatchHistory[]>
  get_queue(): Promise<WatchQueue[]>
  get_recommendations(): Promise<Recommendation[]>
}

// Streaming Module → Main App
interface StreamingEvents {
  playback_started: { stream_id: string, timestamp: number }
  playback_progress: { stream_id: string, progress: number }
  playback_ended: { stream_id: string, completion: number }
  queue_updated: { queue_length: number }
}
```

### Technology Stack

**Frontend** (Streaming Module Window):
- **Framework**: Svelte 5 (runes for reactive state)
- **Styles**: UnoCSS (Tailwind-compatible, per VISION design system)
- **Video Player**: Video.js with HLS/DASH plugins
- **WebRTC**: Simple-peer for Twitch live streams
- **State**: Zustand (shared state management)
- **Virtualization**: svelte-virtual (for card grid)

**Backend** (Tauri Commands):
- **Language**: Rust
- **Database**: SQLite with sqlx (async queries)
- **HTTP Client**: reqwest (for provider APIs)
- **OAuth**: oauth2 crate for token management

**Third-Party Services**:
- **YouTube Data API v3**: Search, video details, channel info
- **Twitch Helix API**: Streams, videos, channels
- **Widevine CDM**: Via Electron sidecar (Pattern A integration)

### Database Schema

```sql
-- Watch history
CREATE TABLE watch_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  title TEXT NOT NULL,
  channel TEXT NOT NULL,
  thumbnail_url TEXT,
  watched_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  progress_seconds INTEGER DEFAULT 0,
  completion_percent INTEGER DEFAULT 0,
  source TEXT CHECK(source IN ('manual', 'queue', 'recommendation')),
  UNIQUE(user_id, stream_id, watched_at)
);

-- Watch queue
CREATE TABLE watch_queue (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  position INTEGER NOT NULL,
  added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(user_id, stream_id)
);

-- Recommendations cache
CREATE TABLE recommendations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  stream_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  confidence_score REAL NOT NULL,
  reasoning TEXT,
  generated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  dismissed BOOLEAN DEFAULT 0,
  UNIQUE(user_id, stream_id)
);

-- Indexes for performance
CREATE INDEX idx_history_user_watched ON watch_history(user_id, watched_at DESC);
CREATE INDEX idx_queue_user_position ON watch_queue(user_id, position);
CREATE INDEX idx_recommendations_user_score ON recommendations(user_id, confidence_score DESC, dismissed);
```

### Security Model

**OAuth2 Flow** (per VISION.md):
1. User clicks "Connect YouTube/Twitch"
2. Main app opens system browser with OAuth URL
3. Callback redirected to `tauri://oauth-callback`
4. Main app exchanges code for tokens
5. Tokens stored in OS keychain (Keychain/DPAPI/Secret Service)
6. Streaming module requests tokens via IPC (never stored in renderer)

**Network Isolation**:
- Streaming module has no direct internet access
- All API calls proxied through main app commands
- CSP enforces trusted domains only
- No user-generated content execution

**DRM Isolation** (Electron Sidecar):
- Widevine runs in separate Electron process
- IPC with ephemeral tokens (same as Pattern A)
- Sidecar killed after playback ends
- No persistent state in sidecar

---

## UI Design (Per Reference Image)

**Layout Structure** (ps-vue-multi-view.webp inspired):

```
┌─────────────────────────────────────────────────────────────┐
│ [App Sidebar]  │  Streaming Hub                             │
├────────────────┼────────────────────────────────────────────┤
│ [M1]           │  ┌──────────────────────────────────────┐  │
│ [M2]           │  │  Now Playing                         │  │
│ [M5]           │  │  ┌──────────────────────────────┐    │  │
│ [M6]           │  │  │     Video Player             │    │  │
│                │  │  │  [Play/Pause] [Volume] [••]  │    │  │
│                │  │  └──────────────────────────────┘    │  │
│                │  │  Title • Channel • 1.2M views        │  │
│                │  └──────────────────────────────────────┘  │
│                │                                            │
│                │  ┌──────────────────────────────────────┐  │
│                │  │  For You                        [⚙]   │  │
│                │  ├──────────────────────────────────────┤  │
│                │  │  ┌────┐ ┌────┐ ┌────┐ ┌────┐         │  │
│                │  │  │ 🖼 │ │ 🖼 │ │ 🖼 │ │ 🖼 │  [→]    │  │
│                │  │  │ ···│ │ ···│ │ ···│ │ ···│         │  │
│                │  │  └────┘ └────┘ └────┘ └────┘         │  │
│                │  └──────────────────────────────────────┘  │
│                │                                            │
│                │  ┌──────────────────────────────────────┐  │
│                │  │  Watch Queue               [Clear]    │  │
│                │  ├──────────────────────────────────────┤  │
│                │  │  1. [🖼] Video Title       [×]        │  │
│                │  │  2. [🖼] Another Video     [×]        │  │
│                │  │  3. [🖼] Third Video       [×]        │  │
│                │  └──────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

**Component Breakdown**:

1. **NowPlaying Component**
   - Inline video player (Video.js)
   - Playback controls overlay
   - Metadata display (title, channel, stats)
   - PiP toggle button

2. **ForYou Component** (Recommendations)
   - Horizontal scrolling card carousel
   - Confidence score badges
   - "More like this" / "Not interested" actions
   - Lazy-loaded thumbnails

3. **WatchQueue Component**
   - Vertical list with drag handles
   - Remove buttons per item
   - Auto-scroll to current playing
   - Empty state with CTA

4. **StreamCard Component** (Reusable)
   - Thumbnail with duration badge
   - Title (truncated with tooltip)
   - Channel name, view count
   - Hover state: preview GIF (optional), actions

**Color Palette** (per VISION.md):
- Background: `#0A0A0A`
- Surface: `#1A1A1A`
- Primary: `#3B82F6`
- Text Primary: `#E5E5E5`
- Text Secondary: `#A3A3A3`

**Typography**:
- Font: Inter (system-ui fallback)
- Heading: 24px/700
- Body: 14px/400
- Caption: 12px/400

---

## Performance Targets

**Startup**:
- Module window creation: <500ms
- Initial data fetch: <1s
- First meaningful paint: <1.5s
- Total contribution to app startup: <2s (lazy-loaded from main)

**Runtime**:
- Card grid scroll: 60fps minimum
- Video load time: <3s (P95)
- Thumbnail load: <500ms per batch (20 cards)
- History query: <100ms
- Queue update: <50ms

**Memory**:
- Idle (no video playing): <150MB
- Active (video playing): <300MB
- Peak (multiple thumbnails cached): <500MB (hard limit)

**Optimization Strategies**:
- Virtualized scrolling (only render visible cards)
- Progressive image loading (blur-up technique)
- Video preloading (next in queue pre-buffered)
- Thumbnail cache with LRU eviction (max 100MB)
- Lazy module loading (not instantiated until first access)

---

## Testing Strategy

### Unit Tests (95% coverage target)

**Backend (Rust)**:
- `get_watch_history()` — Query filtering, pagination
- `add_to_queue()` — Position calculation, duplicates
- `get_recommendations()` — Scoring algorithm, filtering
- `save_progress()` — Persistence, edge cases
- OAuth token refresh logic

**Frontend (TypeScript)**:
- StreamCard rendering with various data states
- WatchQueue drag-and-drop logic
- Video player event handling
- IPC command mocking and error cases

### Integration Tests

**IT-M1-001**: Launch streaming module from main app
- Verify separate window opens
- Check IPC authentication
- Confirm security flags enforced

**IT-M1-002**: Play video end-to-end
- Select card → player loads → playback starts
- Progress saved to history
- Queue advances to next

**IT-M1-003**: DRM content fallback
- Attempt DRM content without Widevine
- Verify modal shown with instructions
- Confirm graceful degradation

**IT-M1-004**: Network interruption recovery
- Simulate network disconnect during playback
- Verify retry logic triggers
- Confirm seamless reconnection

### E2E Tests (Playwright)

**E2E-M1-001**: Browse and play workflow
- User journey: Open app → Navigate to streaming → Browse cards → Play video
- Verify all UI elements render
- Check video plays without errors

**E2E-M1-002**: Queue management
- Add multiple videos to queue
- Reorder via drag-and-drop
- Verify auto-advance works

**E2E-M1-003**: History and recommendations
- Play 5+ videos
- Check history page shows correct entries
- Verify recommendations appear with confidence scores

---

## Implementation Plan

### Phase 1: Foundation (Week 1)
- [ ] Create module directory structure (`modules/streaming-hub/`)
- [ ] Set up Tauri window configuration for separate process
- [ ] Implement IPC bridge (commands + events)
- [ ] Create SQLite schema and migrations
- [ ] Stub frontend (empty Svelte app with routing)

### Phase 2: Core Playback (Week 2)
- [ ] Implement Video.js integration with HLS/DASH support
- [ ] Create NowPlaying component with playback controls
- [ ] Add progress tracking and persistence
- [ ] Implement PiP mode
- [ ] Unit tests for playback logic

### Phase 3: Content Discovery (Week 3)
- [ ] Integrate YouTube Data API
- [ ] Integrate Twitch Helix API
- [ ] Create StreamCard component
- [ ] Implement virtualized card grid
- [ ] Add search and filtering

### Phase 4: Queue & History (Week 4)
- [ ] Build WatchQueue component with drag-and-drop
- [ ] Create History page with search
- [ ] Implement queue auto-advance logic
- [ ] Add persistence for queue state

### Phase 5: AI Recommendations (Week 5)
- [ ] Implement recommendation algorithm (collaborative filtering)
- [ ] Create ForYou component
- [ ] Add "More like this" / "Not interested" actions
- [ ] Display confidence scores

### Phase 6: DRM & OAuth (Week 6)
- [ ] Integrate Electron sidecar for Widevine
- [ ] Implement OAuth2 flow for YouTube/Twitch
- [ ] Add token storage in OS keychain
- [ ] Test DRM playback end-to-end

### Phase 7: Polish & Performance (Week 7)
- [ ] Implement thumbnail cache with LRU eviction
- [ ] Add loading skeletons and empty states
- [ ] Optimize memory usage (profiling)
- [ ] Accessibility audit and fixes

### Phase 8: Testing & Documentation (Week 8)
- [ ] Write integration tests (all scenarios)
- [ ] E2E test suite (Playwright)
- [ ] Performance benchmarking
- [ ] User documentation and tooltips

---

## Open Questions

1. **YouTube API Quota**: Free tier is 10,000 units/day. Do we need to apply for higher quota or implement caching strategy?
2. **Twitch Live Streams**: WebRTC via simple-peer or fallback to HLS? Latency vs compatibility trade-off.
3. **Recommendation Algorithm**: Start with simple collaborative filtering or integrate cloud LLM (OpenAI) from day 1?
4. **Thumbnail Storage**: Local cache or CDN? If local, max size and eviction policy?
5. **Offline Mode**: Should we support downloaded videos for offline viewing (v1 or v2)?

---

## Success Metrics

**Launch Criteria (v1.0)**:
- [ ] All P0 requirements implemented and tested
- [ ] Performance targets met (<2s startup, <300MB memory)
- [ ] Security audit passed (no critical vulnerabilities)
- [ ] Accessibility audit passed (WCAG 2.1 AA)
- [ ] 95%+ unit test coverage
- [ ] 100% E2E test coverage for critical paths
- [ ] Documentation complete (user guide + API docs)

**KPIs (Post-Launch)**:
- Daily active users (DAU)
- Average session duration
- Videos played per session
- Recommendation click-through rate
- Queue utilization rate
- Crash-free rate (target: >99.5%)

---

## References

- [VISION.md](../../VISION.md) — Application architecture blueprint
- [Pattern A Spec](../001-selective-electron/spec.md) — Electron sidecar integration
- [ps-vue-multi-view.webp](../../ps-vue-multi-view.webp) — Reference UI design
- [YouTube Data API v3](https://developers.google.com/youtube/v3)
- [Twitch Helix API](https://dev.twitch.tv/docs/api/)
- [Video.js Documentation](https://videojs.com/)

---

## Changelog

**2025-10-05**: Initial specification created based on VISION.md requirements
