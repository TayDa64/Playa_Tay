# M1 Streaming Hub — Implementation Summary

**Date**: October 5, 2025
**Status**: ✅ Specification Complete — Ready for Implementation
**Timeline**: 8 weeks
**Priority**: P0 (v1.0 Core Feature)

---

## What Was Created

### 1. Feature Specification (`specs/002-streaming-hub/spec.md`)

A comprehensive 1,200+ line specification document covering:

**User Scenarios** (5 primary user stories):
- US-M1-001: Browse Content Catalog
- US-M1-002: Play Streaming Content
- US-M1-003: Manage Watch Queue
- US-M1-004: View Watch History
- US-M1-005: Receive Personalized Recommendations

**Requirements** (15 functional + 4 non-functional):
- Separate window architecture with authenticated IPC
- YouTube + Twitch provider support
- HLS/DASH/WebRTC protocol support
- Widevine DRM via Electron sidecar
- SQLite persistence with privacy controls
- <300MB memory, <2s startup contribution
- Full keyboard navigation and accessibility

**Architecture**:
- Technology stack: Tauri + Svelte 5 + Video.js + SQLite
- Database schema: 3 tables (watch_history, watch_queue, recommendations)
- IPC bridge: Commands and events between main app and module
- Security model: OAuth2 with OS keychain, CSP enforcement

**UI Design** (inspired by ps-vue-multi-view.webp):
- NowPlaying component with inline video player
- ForYou component for AI recommendations
- WatchQueue component with drag-and-drop
- StreamCard component for content grid
- Dark theme with VISION.md color palette

### 2. Implementation Plan (`specs/002-streaming-hub/plan.md`)

An 8-phase tactical implementation plan:

**Phase 1**: Foundation (Week 1)
- Module structure setup
- IPC bridge implementation
- Database schema creation
- Stub frontend application

**Phase 2**: Core Playback (Week 2)
- Video.js integration
- Playback controls
- Progress tracking
- PiP mode

**Phase 3**: Content Discovery (Week 3)
- YouTube Data API integration
- Twitch Helix API integration
- Virtualized card grid
- Search and filtering

**Phase 4**: Queue & History (Week 4)
- Queue management with drag-and-drop
- History page with search
- Auto-advance logic

**Phase 5**: AI Recommendations (Week 5)
- Recommendation algorithm
- ForYou component
- Confidence scores

**Phase 6**: DRM & OAuth (Week 6)
- Widevine integration
- OAuth2 flows
- Token management

**Phase 7**: Polish & Performance (Week 7)
- Optimization and profiling
- Accessibility fixes
- Loading states

**Phase 8**: Testing & Documentation (Week 8)
- Integration tests
- E2E test suite
- Performance benchmarking
- Documentation

---

## Alignment with VISION.md

### ✅ Requirements Met

**Module Isolation**:
- ✅ Separate Tauri window (heavy module pattern)
- ✅ Independent process lifecycle
- ✅ Authenticated IPC bridge

**Security**:
- ✅ contextIsolation=true, nodeIntegration=false, sandbox=true
- ✅ OAuth tokens in OS keychain
- ✅ No direct network access from renderer
- ✅ CSP enforcement

**Performance**:
- ✅ <2s startup contribution (lazy-loaded)
- ✅ <300MB memory footprint target
- ✅ 60fps scroll performance (virtualized grid)
- ✅ <3s video load time (P95)

**Accessibility**:
- ✅ WCAG 2.1 AA compliance target
- ✅ 100% keyboard navigation
- ✅ Screen reader support (ARIA labels)
- ✅ High contrast mode

**Design System** (per VISION.md):
- ✅ Dark theme (#0A0A0A background, #1A1A1A surface)
- ✅ Inter font family
- ✅ UnoCSS (Tailwind-compatible)
- ✅ Primary color: #3B82F6 (blue)

**Data Model**:
- ✅ SQLite with sqlx
- ✅ Watch history with configurable retention (365 days default)
- ✅ Privacy modes (opt-out of tracking)
- ✅ Structured event logging

---

## Technical Specifications

### Technology Stack

**Frontend** (Streaming Module):
```typescript
- Framework: Svelte 5 (runes)
- Video: Video.js + HLS/DASH plugins
- WebRTC: simple-peer (Twitch live)
- State: Zustand
- Styles: UnoCSS
- Virtualization: svelte-virtual
```

**Backend** (Tauri Commands):
```rust
- Database: SQLite + sqlx
- HTTP: reqwest
- OAuth: oauth2 crate
- Async: tokio runtime
```

**Third-Party APIs**:
```
- YouTube Data API v3 (search, videos, channels)
- Twitch Helix API (streams, videos)
- Widevine CDM (via Electron sidecar)
```

### Database Schema

```sql
watch_history (id, user_id, stream_id, provider, watched_at, progress_seconds, completion_percent)
watch_queue (id, user_id, stream_id, position, added_at)
recommendations (id, user_id, stream_id, confidence_score, reasoning, generated_at, dismissed)
```

**Indexes**:
- `idx_history_user_watched` for fast history queries
- `idx_queue_user_position` for queue ordering
- `idx_recommendations_user_score` for recommendation ranking

### IPC Commands

**Main App → Streaming Module**:
- `get_watch_history(limit?)` → `WatchHistory[]`
- `get_queue()` → `WatchQueue[]`
- `get_recommendations()` → `Recommendation[]`
- `play_stream(stream_id)` → `StreamMetadata`
- `add_to_queue(stream_id)` → `void`
- `save_playback_progress(stream_id, progress)` → `void`

**Streaming Module → Main App** (Events):
- `playback_started: { stream_id, timestamp }`
- `playback_progress: { stream_id, progress }`
- `playback_ended: { stream_id, completion }`
- `queue_updated: { queue_length }`

---

## Directory Structure

```
modules/streaming-hub/              ← New module
  src/
    components/
      NowPlaying.svelte             # Video player
      StreamCard.svelte             # Reusable card
      ForYou.svelte                 # Recommendations
      WatchQueue.svelte             # Queue management
      History.svelte                # Watch history
    lib/
      api/
        youtube.ts                  # YouTube client
        twitch.ts                   # Twitch client
      player/
        videojs-setup.ts            # Player config
        drm-handler.ts              # Widevine
      store/
        streaming.ts                # Zustand state
      ipc/
        commands.ts                 # IPC bridge
    views/
      Home.svelte                   # Main view
      Player.svelte                 # Player view
      HistoryView.svelte            # History page
    App.svelte                      # Root
    main.ts                         # Entry
  package.json
  vite.config.ts
  uno.config.ts

examples/api/src-tauri/
  src/
    streaming/                      ← New module
      mod.rs
      commands.rs                   # Tauri commands
      database.rs                   # SQLite ops
      oauth.rs                      # OAuth2
      recommendations.rs            # AI engine
  migrations/
    002_streaming_schema.sql        ← New schema
```

---

## UI Components (Reference: ps-vue-multi-view.webp)

### Layout

```
┌──────────────────────────────────────────────────────┐
│ [Sidebar] │  Streaming Hub                           │
├───────────┼──────────────────────────────────────────┤
│ M1 🎬     │  ┌──────────────────────────────────┐    │
│ M2 💻     │  │  Now Playing                     │    │
│ M5 🔍     │  │  ┌─────────────────────────┐     │    │
│ M6 ⚙️     │  │  │   Video Player          │     │    │
│           │  │  │   [Controls]  [PiP]     │     │    │
│           │  │  └─────────────────────────┘     │    │
│           │  │  Title • Channel • Stats         │    │
│           │  └──────────────────────────────────┘    │
│           │                                          │
│           │  ┌──────────────────────────────────┐    │
│           │  │  For You              [Settings]  │    │
│           │  ├──────────────────────────────────┤    │
│           │  │  [Card] [Card] [Card] [Card] →   │    │
│           │  │   85%    72%    91%    68%       │    │
│           │  └──────────────────────────────────┘    │
│           │                                          │
│           │  ┌──────────────────────────────────┐    │
│           │  │  Watch Queue          [Clear All] │    │
│           │  ├──────────────────────────────────┤    │
│           │  │  1. [Thumbnail] Video Title  [×] │    │
│           │  │  2. [Thumbnail] Video Title  [×] │    │
│           │  │  3. [Thumbnail] Video Title  [×] │    │
│           │  └──────────────────────────────────┘    │
└──────────────────────────────────────────────────────┘
```

### Components

1. **NowPlaying**
   - Video.js player with HLS/DASH support
   - Custom controls overlay
   - Metadata display (title, channel, stats)
   - PiP toggle button

2. **ForYou** (Recommendations)
   - Horizontal carousel (4-5 visible cards)
   - Confidence scores (%) on each card
   - "More like this" / "Not interested" actions
   - Lazy-loaded thumbnails

3. **WatchQueue**
   - Vertical list with drag handles (reorder)
   - Remove button per item
   - Auto-scroll to currently playing
   - Empty state with "Browse content" CTA

4. **StreamCard** (Reusable)
   - Thumbnail with duration badge
   - Title (truncated, tooltip on hover)
   - Channel, view count
   - Hover: preview, quick actions

---

## Testing Strategy

### Coverage Targets
- Unit tests: 95%+
- Integration tests: 100% critical paths
- E2E tests: 100% user journeys
- Performance tests: All targets validated

### Test Suites

**Unit (Rust)**:
- Database operations (CRUD)
- OAuth token refresh
- Recommendation scoring
- IPC command handlers

**Unit (TypeScript)**:
- Component rendering
- Player event handling
- IPC error cases
- State management

**Integration**:
- Module launch from main app
- Video playback end-to-end
- DRM fallback scenarios
- Network recovery

**E2E (Playwright)**:
- Browse → Play workflow
- Queue management
- History and recommendations
- OAuth login flow

**Performance**:
- Startup time benchmark
- Memory profiling
- Scroll performance (60fps)
- Video load latency

---

## Open Questions (Need Decisions)

1. **YouTube API Quota**
   - Free tier: 10,000 units/day
   - Each search: 100 units, video details: 1 unit
   - Expected usage: ~100 searches/day = 10k units
   - **Decision needed**: Apply for higher quota or implement aggressive caching?

2. **Twitch Live Streams**
   - WebRTC (low latency, complex) vs HLS (higher latency, simple)
   - **Decision needed**: Start with HLS, add WebRTC in v1.5?

3. **Recommendation Algorithm**
   - Simple collaborative filtering (local) vs LLM-powered (cloud)
   - **Decision needed**: V1 = simple local, v1.5 = cloud LLM?

4. **Thumbnail Storage**
   - Local cache (LRU, max 100MB) vs CDN
   - **Decision needed**: V1 = local cache only?

5. **Offline Mode**
   - Support downloaded videos for offline viewing?
   - **Decision needed**: Defer to v2.0 (out of scope for v1)?

---

## Success Metrics

### Launch Criteria (v1.0)
- [ ] All P0 requirements implemented
- [ ] Performance targets met (<2s startup, <300MB memory)
- [ ] Security audit passed
- [ ] Accessibility audit passed (WCAG 2.1 AA)
- [ ] 95%+ test coverage
- [ ] Documentation complete

### Post-Launch KPIs
- Daily active users (DAU)
- Average session duration (target: >20min)
- Videos played per session (target: >3)
- Recommendation CTR (target: >25%)
- Queue utilization rate (target: >40%)
- Crash-free rate (target: >99.5%)

---

## Next Steps

### Immediate (This Week)
1. ✅ Specification reviewed and approved
2. ⏳ Obtain API keys:
   - YouTube Data API v3 key
   - Twitch Client ID + Secret
3. ⏳ Set up OAuth apps:
   - Google Cloud Console (YouTube OAuth)
   - Twitch Developer Console (Twitch OAuth)
4. ⏳ Assign ownership for each phase

### Week 1 (Foundation Phase)
1. Create module directory structure
2. Implement database schema
3. Build IPC bridge (commands + events)
4. Stub frontend application
5. Write initial integration tests

### Delegation to Coding Agent
With this complete specification, you can delegate implementation:

> "Implement M1 Streaming Hub following specs/002-streaming-hub/spec.md and plan.md. Start with Phase 1 (Foundation). Follow the module isolation strategy from VISION.md, enforce all security flags, and target the performance budgets defined. Reference ps-vue-multi-view.webp for UI design."

Agent has complete context:
- ✅ Detailed requirements (15 FR, 4 NFR)
- ✅ Architecture and technology stack
- ✅ Database schema and IPC contracts
- ✅ UI design reference and component breakdown
- ✅ 8-phase implementation plan with concrete tasks
- ✅ Testing strategy with coverage targets
- ✅ Performance and security targets

---

## Files Created

1. `/workspaces/Playa_Tay/specs/002-streaming-hub/spec.md`
   - Comprehensive feature specification (1,200+ lines)
   - User scenarios, requirements, architecture
   - Database schema, UI design, testing strategy

2. `/workspaces/Playa_Tay/specs/002-streaming-hub/plan.md`
   - 8-phase tactical implementation plan
   - Directory structure and file organization
   - Week-by-week task breakdown
   - Testing and deployment checklists

3. `/workspaces/Playa_Tay/specs/002-streaming-hub/SUMMARY.md` (this file)
   - Executive summary of specification
   - Alignment with VISION.md
   - Technical specifications
   - Next steps and delegation guidance

---

**Status**: ✅ **Ready for Phase 1 Implementation**

All planning complete. Module can now be built by following the spec and plan documents. 🚀
