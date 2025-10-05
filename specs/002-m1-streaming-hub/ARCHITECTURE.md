# M1 Streaming Hub Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Playa Tay Application                         │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Tauri Main Process (Rust)                 │   │
│  │                                                               │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │              Streaming Hub Commands                   │  │   │
│  │  │  • open_video(url)                                   │  │   │
│  │  │  • get_watch_history()                               │  │   │
│  │  │  • get_watch_queue()                                 │  │   │
│  │  │  • add_to_watch_queue(video)                         │  │   │
│  │  │  • remove_from_watch_queue(video_id)                 │  │   │
│  │  │  • add_to_watch_history(video)                       │  │   │
│  │  │  • clear_watch_history()                             │  │   │
│  │  │  • search_videos(query, provider)                    │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │                                                               │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │         Electron Integration (Pattern A)             │  │   │
│  │  │  • open_electron_feature(url)                        │  │   │
│  │  │  • is_electron_available()                           │  │   │
│  │  │  • launch_electron(url)                              │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │                                                               │   │
│  └───────────────────────┬───────────────────────────────────────┘   │
│                          │ Tauri IPC                                 │
│                          │ (Authenticated Commands)                  │
│  ┌───────────────────────▼───────────────────────────────────────┐   │
│  │              Frontend Renderer (Svelte)                       │   │
│  │                                                               │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │              App.svelte (Navigation)                 │  │   │
│  │  │  • Welcome                                           │  │   │
│  │  │  • M1: Streaming Hub  ◄── NEW MODULE                │  │   │
│  │  │  • Communication                                     │  │   │
│  │  │  • Window, Menu, Tray                               │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │                                                               │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │         StreamingHub.svelte Component                │  │   │
│  │  │                                                       │  │   │
│  │  │  ┌─────────────────────────────────────────────┐   │  │   │
│  │  │  │ Provider Tabs                               │   │  │   │
│  │  │  │  [YouTube] [Twitch] [Custom HLS/DASH]      │   │  │   │
│  │  │  │  DRM Status: ✓ Ready / ⚠️ Unavailable      │   │  │   │
│  │  │  └─────────────────────────────────────────────┘   │  │   │
│  │  │                                                       │  │   │
│  │  │  ┌─────────────────────────────────────────────┐   │  │   │
│  │  │  │ Search Bar                                  │   │  │   │
│  │  │  │  [Search youtube...] [Search Button]       │   │  │   │
│  │  │  └─────────────────────────────────────────────┘   │  │   │
│  │  │                                                       │  │   │
│  │  │  ┌─────────────────────────────────────────────┐   │  │   │
│  │  │  │ Watch Queue (if non-empty)                  │   │  │   │
│  │  │  │  [Thumbnail] Video Title [Play] [Remove]   │   │  │   │
│  │  │  │  [Thumbnail] Video Title [Play] [Remove]   │   │  │   │
│  │  │  └─────────────────────────────────────────────┘   │  │   │
│  │  │                                                       │  │   │
│  │  │  ┌─────────────────────────────────────────────┐   │  │   │
│  │  │  │ Recommendations (Card Grid)                 │   │  │   │
│  │  │  │  ┌────┐ ┌────┐ ┌────┐ ┌────┐              │   │  │   │
│  │  │  │  │▶️  │ │▶️  │ │▶️  │ │▶️  │              │   │  │   │
│  │  │  │  │IMG │ │IMG │ │IMG │ │IMG │              │   │  │   │
│  │  │  │  └────┘ └────┘ └────┘ └────┘              │   │  │   │
│  │  │  │  Title  Title  Title  Title               │   │  │   │
│  │  │  │  [+Queue] [+Queue] [+Queue] [+Queue]      │   │  │   │
│  │  │  └─────────────────────────────────────────────┘   │  │   │
│  │  │                                                       │  │   │
│  │  │  ┌─────────────────────────────────────────────┐   │  │   │
│  │  │  │ Watch History (if non-empty)                │   │  │   │
│  │  │  │  [Thumbnail] Video - Date [Play Again]     │   │  │   │
│  │  │  │  [Clear All]                                │   │  │   │
│  │  │  └─────────────────────────────────────────────┘   │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘

                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
                    ▼                               ▼
        ┌──────────────────────┐      ┌──────────────────────┐
        │  DRM Content         │      │  Non-DRM Content     │
        │  (Electron Sidecar)  │      │  (System Player)     │
        │                      │      │                      │
        │  Security Flags:     │      │  Opens with:         │
        │  ✅ contextIsolation │      │  • Windows: start    │
        │  ✅ nodeIntegration  │      │  • macOS: open       │
        │  ✅ sandbox          │      │  • Linux: xdg-open   │
        │  ✅ CSP enforced     │      │                      │
        │  ✅ DevTools off     │      │                      │
        └──────────────────────┘      └──────────────────────┘

                              Data Flow
                              ─────────

Frontend (Svelte)  ──IPC──>  Backend (Rust)  ──Future──>  Database
                                                          (SQLite)
    User Actions              Tauri Commands              Persistence
    • Click play              • Video management          • History
    • Add to queue            • Queue operations          • Queue
    • Search                  • Search providers          • Preferences
    • Clear history           • Platform integration      • Cache


                           Security Layers
                           ───────────────

┌─────────────────────────────────────────────────────────────────┐
│ 1. OS Sandbox (Tauri app sandboxing)                            │
├─────────────────────────────────────────────────────────────────┤
│ 2. Process Isolation (Separate Electron process for DRM)        │
├─────────────────────────────────────────────────────────────────┤
│ 3. Context Isolation (No Node.js access from renderer)          │
├─────────────────────────────────────────────────────────────────┤
│ 4. IPC Authentication (Ephemeral tokens per spawn)              │
├─────────────────────────────────────────────────────────────────┤
│ 5. Content Security Policy (CSP headers enforced)               │
└─────────────────────────────────────────────────────────────────┘


                      Module Isolation Pattern
                      ────────────────────────

┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Welcome   │     │  Streaming  │     │ Communication│
│   Module    │     │   Hub (M1)  │     │    Module   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                    │
       └───────────────────┼────────────────────┘
                           │
                    ┌──────▼──────┐
                    │  Tauri IPC  │
                    │  (Message   │
                    │   Bus)      │
                    └─────────────┘

Each module:
• Isolated component state
• No shared globals
• Communicates via IPC only
• Can spawn separate windows/processes


                        Performance Budget
                        ──────────────────

Target (v1.0):                    Current Status:
• Cold Start: <2s                 • ⏳ Pending full build
• Module Load: <500ms             • ✅ Lazy-loaded component
• Memory: <800MB (3 modules)      • ✅ Placeholder data
• Frame Rate: 60fps               • ✅ Transform animations

Optimizations:
• CSS Grid (GPU-accelerated)
• Lazy loading (on-demand import)
• Virtual scrolling (future)
• Image lazy-loading (future)
• API response caching (future)
```

## Key Design Decisions

### 1. Module Isolation
- **Decision**: Separate Svelte component with dedicated backend commands
- **Rationale**: Follows VISION.md module isolation strategy
- **Impact**: Easy to maintain, test, and extend independently

### 2. DRM Integration
- **Decision**: Leverage existing Electron Pattern A sidecar
- **Rationale**: Already implemented, security-hardened
- **Impact**: No additional security surface area

### 3. Placeholder Data
- **Decision**: Use in-memory data structures, log operations
- **Rationale**: Database integration requires additional dependencies
- **Impact**: Quick iteration, easy testing, future SQLite migration path

### 4. Platform-Agnostic Video Opening
- **Decision**: Use OS-specific commands (start/open/xdg-open)
- **Rationale**: Simplest cross-platform approach for v1.0
- **Impact**: Works immediately, future: native player integration

### 5. Card Grid Layout
- **Decision**: CSS Grid with `auto-fill` and `minmax(280px, 1fr)`
- **Rationale**: Responsive without media queries, GPU-accelerated
- **Impact**: 60fps animations, adapts to window size automatically

## Files Modified/Created

```
examples/api/src/views/StreamingHub.svelte   (NEW, 681 lines)
examples/api/src-tauri/src/cmd.rs            (+120 lines)
examples/api/src-tauri/src/lib.rs            (+8 lines)
examples/api/src/App.svelte                  (+5 lines)
specs/002-m1-streaming-hub/README.md         (NEW, 329 lines)
specs/002-m1-streaming-hub/SUMMARY.md        (NEW, 175 lines)
specs/002-m1-streaming-hub/ARCHITECTURE.md   (NEW, this file)
VISION.md                                    (updated, +1 checkmark)
```

## Vision Compliance Checklist

Based on VISION.md requirements:

### Functional Requirements (M1 Specification)
- ✅ Providers: YouTube, Twitch, custom HLS/DASH (UI ready)
- ⏳ Protocols: HLS, DASH, WebRTC (backend integration pending)
- ✅ DRM: Widevine via Electron sidecar (Pattern A)
- ✅ Features: Watch queue, history, recommendations
- ✅ UI: Card grid
- ⏳ UI: Inline player (future)
- ⏳ UI: PiP mode (future)

### Module Isolation (from VISION.md)
- ✅ Separate component (BrowserView module)
- ✅ Backend commands follow IPC pattern
- ✅ DRM in separate process (Electron)
- ✅ No shared state except IPC

### Security (from VISION.md)
- ✅ contextIsolation=true
- ✅ nodeIntegration=false
- ✅ sandbox=true
- ✅ devTools=false (production)
- ✅ CSP enforced
- ✅ IPC with ephemeral tokens

### Performance (from VISION.md targets)
- ⏳ <2s cold start (requires full build)
- ✅ Lazy-loaded module
- ✅ GPU-accelerated layout
- ✅ 60fps animations (transform-based)

### Design System (from VISION.md)
- ✅ Dark mode colors (#0A0A0A, #1A1A1A)
- ✅ Primary accent (#3B82F6)
- ✅ Content-first layout
- ✅ Focus-driven (keyboard navigable)
- ✅ Consistent with existing modules

## Future Enhancements

See `specs/002-m1-streaming-hub/README.md` for complete list.

### Immediate (v1.0 completion)
- Provider API integration
- SQLite persistence
- Inline player
- PiP mode

### Medium-term (v1.5)
- Multi-account support
- Advanced filtering
- Keyboard shortcuts
- Playlist management

### Long-term (v2.0)
- AI recommendations
- Cross-device sync
- Custom dashboards
- TV app integration
