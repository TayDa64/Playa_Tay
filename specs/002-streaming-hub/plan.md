# Implementation Plan: M1 Streaming Hub

## Summary

Implement M1 (Streaming Hub) as a core module for Playa Tay, providing video streaming capabilities with YouTube integration, watch queue management, and secure module isolation. Phase 1 focuses on foundation: protocol handlers, basic UI, YouTube provider, and data persistence.

## Execution Flow

```
1. Review spec.md and VISION.md for requirements
2. Phase 1: Foundation Implementation
   - Set up backend infrastructure (protocol handlers, commands)
   - Implement data models and persistence
   - Create UI components following design reference
   - Integrate YouTube provider
   - Enforce security and performance budgets
3. Testing and validation
4. Documentation and handoff
```

## Project Structure

```
specs/002-streaming-hub/
├── spec.md                           # This spec
├── plan.md                           # This file
└── tasks.md                          # Generated task list (future)

examples/api/src-tauri/src/
├── streaming/                        # NEW: Streaming module backend
│   ├── mod.rs                       # Module exports
│   ├── protocol.rs                  # Custom protocol handler (extends existing)
│   ├── commands.rs                  # Tauri commands for streaming
│   ├── providers/                   # Provider integrations
│   │   ├── mod.rs
│   │   └── youtube.rs               # YouTube API client
│   ├── models.rs                    # Data structures
│   └── db.rs                        # SQLite operations for queue/history
├── lib.rs                           # MODIFY: Register streaming commands
└── cmd.rs                           # Existing commands

examples/api/src/
├── views/
│   └── Streaming.svelte             # NEW: Main streaming hub view
├── components/
│   └── streaming/                   # NEW: Streaming components
│       ├── VideoPlayer.svelte       # Video player component
│       ├── ContentCard.svelte       # Thumbnail card for grid
│       ├── WatchQueue.svelte        # Queue sidebar
│       └── SearchBar.svelte         # Search input
└── lib/
    └── streaming/                   # NEW: Frontend utilities
        ├── player.ts                # Video player logic
        ├── queue.ts                 # Queue management
        └── api.ts                   # Tauri command wrappers

Database: SQLite (reuse existing or add tables)
```

## Phase 0: Prerequisites

**Validate Environment:**
- [x] Cargo builds successfully
- [x] Frontend builds (Svelte + Vite)
- [x] Existing streaming example reviewed
- [x] VISION.md and spec.md understood

**Dependencies:**
- YouTube API client: `reqwest` (already in workspace)
- Database: `rusqlite` (check if available, add if needed)
- Video protocols: Leverage existing Tauri protocol registration

## Phase 1: Design & Contracts

### Data Models

**Rust Structures:**
```rust
// examples/api/src-tauri/src/streaming/models.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingContent {
    pub id: String,
    pub provider: StreamProvider,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>,
    pub creator: Option<String>,
    pub view_count: Option<u64>,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamProvider {
    YouTube,
    Twitch,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchQueueItem {
    pub id: String,
    pub content_id: String,
    pub content: StreamingContent,
    pub added_at: String,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistoryEntry {
    pub id: String,
    pub content_id: String,
    pub content: StreamingContent,
    pub watched_at: String,
    pub progress: f64,
    pub completed: bool,
    pub duration: f64,
}
```

### Database Schema

```sql
-- Watch queue
CREATE TABLE IF NOT EXISTS watch_queue (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_json TEXT NOT NULL,
    added_at TEXT NOT NULL,
    position INTEGER NOT NULL,
    UNIQUE(content_id)
);

-- Watch history (with consent)
CREATE TABLE IF NOT EXISTS watch_history (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL,
    content_json TEXT NOT NULL,
    watched_at TEXT NOT NULL,
    progress REAL NOT NULL,
    completed INTEGER NOT NULL,
    duration REAL NOT NULL
);

-- User consent for history tracking
CREATE TABLE IF NOT EXISTS user_consent (
    feature TEXT PRIMARY KEY,
    enabled INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
```

### API Contracts

**Tauri Commands** (registered in `lib.rs`):
```rust
// Module lifecycle
open_streaming_hub() -> Result<(), String>
stream_video(content_id: String) -> Result<(), String>

// Search
search_youtube(query: String, limit: Option<u32>) -> Result<Vec<StreamingContent>, String>

// Queue
add_to_queue(content_id: String) -> Result<(), String>
remove_from_queue(queue_id: String) -> Result<(), String>
get_watch_queue() -> Result<Vec<WatchQueueItem>, String>

// History
save_watch_progress(content_id: String, progress: f64) -> Result<(), String>
get_watch_history(limit: Option<u32>) -> Result<Vec<WatchHistoryEntry>, String>
clear_watch_history() -> Result<(), String>
```

### Protocol Handler

Extend existing `stream://` protocol or use Tauri's asset protocol:
- Path format: `stream://youtube/VIDEO_ID`
- Byte-range support for seeking
- Security: Validate source before serving

### UI Components

**Main View Structure:**
```svelte
<!-- Streaming.svelte -->
<div class="streaming-hub">
  <SearchBar on:search={handleSearch} />
  <div class="content-area">
    <div class="main-grid">
      {#each results as content}
        <ContentCard {content} on:play={playVideo} on:queue={addToQueue} />
      {/each}
    </div>
    <WatchQueue items={queueItems} on:play={playFromQueue} />
  </div>
  {#if currentVideo}
    <VideoPlayer video={currentVideo} on:progress={saveProgress} />
  {/if}
</div>
```

### Security Configuration

**Window Creation** (in `open_streaming_hub` command):
```rust
use tauri::Manager;

WebviewWindowBuilder::new(
    app,
    "streaming-hub",
    WebviewUrl::App("/streaming".into())
)
.title("Streaming Hub")
.inner_size(1280.0, 720.0)
.min_inner_size(800.0, 600.0)
.build()?;
```

Tauri enforces security by default:
- `contextIsolation: true`
- `nodeIntegration: false`
- `sandbox: true` (platform-dependent)

## Phase 2: Task Planning Approach

Tasks will be derived from contracts and data model, following test-first approach:

1. **Backend Foundation**
   - Set up database schema and migrations
   - Implement data models
   - Create YouTube API client
   - Implement protocol handler

2. **Tauri Commands**
   - Module lifecycle commands
   - Search commands with tests
   - Queue commands with tests
   - History commands with tests

3. **Frontend Components**
   - Search bar with debouncing
   - Content card grid (responsive)
   - Video player with controls
   - Watch queue sidebar

4. **Integration**
   - Wire up commands to UI
   - Implement state management
   - Add error handling
   - Performance optimization

5. **Testing & Validation**
   - Unit tests for commands
   - Integration tests for database
   - E2E tests for user flows
   - Security validation
   - Performance benchmarks

## Phase 3: Implementation Details

### Step 1: Backend Infrastructure

**1.1 Database Setup**
- Create `examples/api/src-tauri/src/streaming/db.rs`
- Initialize SQLite connection
- Run migrations for schema
- Implement CRUD operations for queue and history

**1.2 Data Models**
- Create `examples/api/src-tauri/src/streaming/models.rs`
- Define all structs with Serialize/Deserialize
- Add validation methods

**1.3 YouTube Provider**
- Create `examples/api/src-tauri/src/streaming/providers/youtube.rs`
- Implement search using YouTube Data API v3
- Handle API errors and rate limits
- Mock for testing

### Step 2: Tauri Commands

**2.1 Module Lifecycle**
- Implement `open_streaming_hub` command
- Create window with security flags
- Register window lifecycle events

**2.2 Search Commands**
- Implement `search_youtube` command
- Add caching layer (in-memory)
- Return structured StreamingContent array

**2.3 Queue Commands**
- Implement `add_to_queue` with deduplication
- Implement `remove_from_queue`
- Implement `get_watch_queue` with ordering

**2.4 History Commands**
- Check user consent before saving
- Implement `save_watch_progress` with throttling (5s interval)
- Implement `get_watch_history` with limit
- Implement `clear_watch_history`

### Step 3: Frontend Components

**3.1 Search Bar**
- Input with debouncing (300ms)
- Loading indicator
- Error handling

**3.2 Content Card**
- Thumbnail with lazy loading
- Metadata display (title, creator, duration)
- Action buttons (play, add to queue)
- Hover effects

**3.3 Video Player**
- HTML5 video element
- Custom controls (play, pause, seek, volume, fullscreen)
- Progress tracking
- Keyboard shortcuts

**3.4 Watch Queue**
- Sidebar component
- Drag-to-reorder (future)
- Remove button per item
- Empty state

**3.5 Main View**
- Responsive grid layout (CSS Grid)
- Search integration
- Player modal/inline
- Queue sidebar toggle

### Step 4: Integration

**4.1 State Management**
- Use Svelte stores for global state
- Queue state
- Current video state
- Search results state

**4.2 Command Wrappers**
- Create `lib/streaming/api.ts`
- Wrap Tauri commands with TypeScript types
- Add error handling

**4.3 Routing**
- Add `/streaming` route to App.svelte
- Deep linking support (open specific video)

### Step 5: Performance Optimization

**5.1 Lazy Loading**
- Load streaming module on demand
- Code splitting (Vite dynamic imports)
- Defer non-critical scripts

**5.2 Caching**
- Thumbnail cache with LRU eviction
- Metadata cache (1 hour TTL)
- Prefetch next page of results

**5.3 Monitoring**
- Add performance marks for startup time
- Memory profiling
- Network request monitoring

### Step 6: Testing

**6.1 Unit Tests (Rust)**
```rust
// examples/api/src-tauri/src/streaming/db.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_queue() {
        // Test queue add operation
    }

    #[test]
    fn test_save_progress_requires_consent() {
        // Test consent check
    }
}
```

**6.2 Integration Tests**
- Mock YouTube API responses
- Test command invocations
- Validate database persistence

**6.3 E2E Tests (Playwright)**
- Search flow
- Play video flow
- Queue management

**6.4 Security Validation**
- Manual inspection of window creation
- Verify security flags in DevTools (dev mode)
- Check CSP headers

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|--------------------------------------|
| New module directory | Clean separation of concerns | Mixing with existing code would violate VISION.md module isolation |
| SQLite for persistence | Local-first data requirement | In-memory state would lose queue/history on restart |
| YouTube API integration | Core feature requirement | No simpler way to search YouTube |
| Custom protocol handler | Security and performance | Direct file URLs would bypass security boundaries |

## Non-Regression Validation

**Must Not Break:**
- Existing Electron sidecar functionality
- Main app startup time
- Other view navigation
- CI pipeline

**Testing:**
- Run existing tests: `cargo test`
- Build example app: `pnpm run build:api`
- Manual smoke test of Welcome.svelte button

## Performance Budgets

**Measurements:**
```rust
// Add performance marks
use std::time::Instant;

let start = Instant::now();
// ... operation ...
println!("Operation took: {:?}", start.elapsed());
```

**Targets:**
- Module window open: <1.5s
- Search results: <500ms
- Video first frame: <3s
- Memory (idle): <100MB
- Memory (active): <300MB

## Success Criteria

### Definition of Done (Phase 1)

- [ ] All Tauri commands implemented and tested
- [ ] YouTube search returns results
- [ ] Video playback works (at least 3 test cases)
- [ ] Watch queue persists and displays correctly
- [ ] Watch history saves progress (with consent)
- [ ] UI follows design reference (ps-vue-multi-view.webp)
- [ ] Security flags verified
- [ ] Performance budgets met
- [ ] No regressions in existing functionality
- [ ] Documentation updated

### Acceptance Tests

1. **Search Test**: Search "programming tutorials", verify results load <500ms
2. **Playback Test**: Play video, verify video loads and plays within 3s
3. **Queue Test**: Add 3 videos to queue, restart app, verify queue persists
4. **History Test**: Play video for 30s, close, reopen, verify progress saved
5. **Security Test**: Open DevTools, verify contextIsolation is true
6. **Performance Test**: Measure module open time, verify <1.5s

## Open Questions

1. **YouTube API Key**
   - Decision needed: User-provided or bundled dev key?
   - Resolution: Start with dev key (environment variable), add user input in settings (Phase 2)

2. **Video Source**
   - Do we use youtube-dl/yt-dlp or direct iframe embed?
   - Resolution: Phase 1 uses iframe embed (simpler), Phase 2 can add direct streaming

3. **Consent UI**
   - Where to show consent dialog for watch history?
   - Resolution: First-run dialog when opening streaming hub, with "Don't ask again" option

## Next Steps

1. ✅ Create spec.md and plan.md
2. 🔄 Set up backend infrastructure (database, models)
3. ⏸️ Implement Tauri commands
4. ⏸️ Create UI components
5. ⏸️ Integration and testing
6. ⏸️ Documentation and handoff

## Changelog

### 2025-10-05
- Initial implementation plan created
- Defined Phase 1 scope and tasks
- Documented architecture and contracts
- Established success criteria
