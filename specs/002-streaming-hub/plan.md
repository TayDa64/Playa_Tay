# M1 Streaming Hub — Implementation Plan

**Feature**: M1 - Streaming Hub
**Status**: Ready for Implementation
**Timeline**: 8 weeks
**Owner**: TBD

---

## Quick Start

This document provides the tactical implementation plan for M1 (Streaming Hub). For full requirements and architecture, see [spec.md](./spec.md).

**Prerequisites**:
- ✅ Pattern A (Electron sidecar) implemented
- ✅ VISION.md architecture defined
- ✅ Design reference (ps-vue-multi-view.webp) available
- ⏳ YouTube/Twitch API keys obtained

---

## Directory Structure

```
modules/
  streaming-hub/
    src/
      components/
        NowPlaying.svelte       # Video player component
        StreamCard.svelte        # Reusable card component
        ForYou.svelte           # Recommendations carousel
        WatchQueue.svelte       # Queue management
        History.svelte          # Watch history page
      lib/
        api/
          youtube.ts            # YouTube Data API client
          twitch.ts             # Twitch Helix API client
          provider.ts           # Abstract provider interface
        player/
          videojs-setup.ts      # Video.js configuration
          drm-handler.ts        # Widevine integration
        store/
          streaming.ts          # Zustand state management
        ipc/
          commands.ts           # IPC bridge to main app
      views/
        Home.svelte             # Main streaming hub view
        Player.svelte           # Dedicated player view
        HistoryView.svelte      # History page view
      App.svelte                # Root component
      main.ts                   # Entry point
    public/
      assets/                   # Static assets
    package.json
    vite.config.ts
    tsconfig.json
    uno.config.ts               # UnoCSS configuration

examples/api/src-tauri/
  src/
    streaming/
      mod.rs                    # Module exports
      commands.rs               # Tauri commands
      database.rs               # SQLite operations
      oauth.rs                  # OAuth2 token management
      recommendations.rs        # AI recommendation engine
    lib.rs                      # Register streaming commands
  migrations/
    002_streaming_schema.sql    # Database schema
```

---

## Phase 1: Foundation (Week 1)

### Goals
- Set up module structure
- Implement IPC bridge
- Create database schema
- Stub frontend application

### Tasks

**Backend (Rust)**

1. **Create streaming module** (`examples/api/src-tauri/src/streaming/mod.rs`)
   ```rust
   pub mod commands;
   pub mod database;
   pub mod oauth;
   pub mod recommendations;
   ```

2. **Database schema** (`examples/api/src-tauri/migrations/002_streaming_schema.sql`)
   ```sql
   -- Watch history, queue, recommendations tables
   -- See spec.md for full schema
   ```

3. **Tauri commands** (`examples/api/src-tauri/src/streaming/commands.rs`)
   ```rust
   #[tauri::command]
   pub async fn get_watch_history(
       state: State<'_, AppState>,
       limit: Option<usize>,
   ) -> Result<Vec<WatchHistoryEntry>, String> {
       // Implementation
   }

   #[tauri::command]
   pub async fn add_to_queue(
       state: State<'_, AppState>,
       stream_id: String,
   ) -> Result<(), String> {
       // Implementation
   }

   #[tauri::command]
   pub async fn play_stream(
       state: State<'_, AppState>,
       stream_id: String,
   ) -> Result<StreamMetadata, String> {
       // Implementation
   }
   ```

4. **Register commands** (`examples/api/src-tauri/src/lib.rs`)
   ```rust
   use streaming::commands::*;

   tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![
           // ... existing commands
           get_watch_history,
           add_to_queue,
           play_stream,
           get_recommendations,
       ])
   ```

**Frontend (Svelte)**

5. **Create module directory**
   ```bash
   mkdir -p modules/streaming-hub/src/{components,lib,views}
   ```

6. **Package setup** (`modules/streaming-hub/package.json`)
   ```json
   {
     "name": "@playa/streaming-hub",
     "version": "0.1.0",
     "type": "module",
     "scripts": {
       "dev": "vite",
       "build": "vite build",
       "preview": "vite preview"
     },
     "dependencies": {
       "svelte": "^5.0.0",
       "@tauri-apps/api": "workspace:*",
       "zustand": "^4.5.0",
       "video.js": "^8.10.0"
     },
     "devDependencies": {
       "@sveltejs/vite-plugin-svelte": "^4.0.0",
       "vite": "^7.0.0",
       "@unocss/vite": "^0.62.0",
       "typescript": "^5.0.0"
     }
   }
   ```

7. **Vite config** (`modules/streaming-hub/vite.config.ts`)
   ```typescript
   import { defineConfig } from 'vite'
   import { svelte } from '@sveltejs/vite-plugin-svelte'
   import UnoCSS from '@unocss/vite'

   export default defineConfig({
     plugins: [svelte(), UnoCSS()],
     clearScreen: false,
     server: {
       port: 1421,
       strictPort: true,
     },
   })
   ```

8. **Stub App.svelte** (`modules/streaming-hub/src/App.svelte`)
   ```svelte
   <script lang="ts">
     import { onMount } from 'svelte'

     onMount(() => {
       console.log('Streaming Hub initialized')
     })
   </script>

   <div class="app">
     <h1>Streaming Hub</h1>
     <p>Module placeholder</p>
   </div>

   <style>
     .app {
       font-family: Inter, system-ui, sans-serif;
       background: #0A0A0A;
       color: #E5E5E5;
       min-height: 100vh;
       padding: 2rem;
     }
   </style>
   ```

9. **IPC bridge** (`modules/streaming-hub/src/lib/ipc/commands.ts`)
   ```typescript
   import { invoke } from '@tauri-apps/api/core'

   export interface WatchHistoryEntry {
     id: number
     stream_id: string
     title: string
     channel: string
     watched_at: Date
     progress_seconds: number
     completion_percent: number
   }

   export async function getWatchHistory(limit?: number): Promise<WatchHistoryEntry[]> {
     return await invoke('get_watch_history', { limit })
   }

   export async function addToQueue(streamId: string): Promise<void> {
     return await invoke('add_to_queue', { streamId })
   }

   export async function playStream(streamId: string): Promise<any> {
     return await invoke('play_stream', { streamId })
   }
   ```

**Testing**

10. **Integration test** (`examples/api/src-tauri/tests/streaming_integration.rs`)
    ```rust
    #[tokio::test]
    async fn test_get_watch_history() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_add_to_queue() {
        // Test implementation
    }
    ```

### Deliverables
- ✅ Module structure created
- ✅ Database schema applied
- ✅ IPC commands implemented and tested
- ✅ Frontend builds and runs (stub)

---

## Phase 2: Core Playback (Week 2)

### Goals
- Integrate Video.js player
- Implement playback controls
- Add progress tracking
- Enable PiP mode

### Tasks

**Frontend Components**

1. **Video.js setup** (`modules/streaming-hub/src/lib/player/videojs-setup.ts`)
   ```typescript
   import videojs from 'video.js'
   import 'video.js/dist/video-js.css'

   export function createPlayer(element: HTMLVideoElement, options: any) {
     const player = videojs(element, {
       controls: true,
       autoplay: false,
       preload: 'auto',
       fluid: true,
       ...options,
     })

     // Add HLS/DASH support
     // Configure quality selector
     // Add custom controls

     return player
   }
   ```

2. **NowPlaying component** (`modules/streaming-hub/src/components/NowPlaying.svelte`)
   ```svelte
   <script lang="ts">
     import { onMount, onDestroy } from 'svelte'
     import { createPlayer } from '$lib/player/videojs-setup'

     export let streamUrl: string
     export let metadata: any

     let videoElement: HTMLVideoElement
     let player: any

     onMount(() => {
       player = createPlayer(videoElement, {
         sources: [{ src: streamUrl, type: 'application/x-mpegURL' }]
       })

       player.on('timeupdate', handleProgress)
       player.on('ended', handleEnded)
     })

     onDestroy(() => {
       player?.dispose()
     })

     function handleProgress() {
       // Save progress to backend
     }

     function handleEnded() {
       // Advance queue
     }
   </script>

   <div class="now-playing">
     <video bind:this={videoElement} class="video-js"></video>
     <div class="metadata">
       <h2>{metadata.title}</h2>
       <p>{metadata.channel} • {metadata.views} views</p>
     </div>
   </div>
   ```

3. **Progress tracking** (Backend)
   ```rust
   #[tauri::command]
   pub async fn save_playback_progress(
       state: State<'_, AppState>,
       stream_id: String,
       progress_seconds: i64,
   ) -> Result<(), String> {
       // Update watch_history table
   }
   ```

4. **PiP mode** (`modules/streaming-hub/src/lib/player/pip.ts`)
   ```typescript
   export async function enterPiP(videoElement: HTMLVideoElement) {
     if (document.pictureInPictureEnabled) {
       await videoElement.requestPictureInPicture()
     }
   }

   export async function exitPiP() {
     if (document.pictureInPictureElement) {
       await document.exitPictureInPicture()
     }
   }
   ```

### Deliverables
- ✅ Video.js integrated and playing HLS streams
- ✅ Playback controls functional
- ✅ Progress saved to database every 5s
- ✅ PiP mode working

---

## Phase 3: Content Discovery (Week 3)

### Goals
- Integrate YouTube and Twitch APIs
- Create card grid with virtualization
- Implement search and filtering

### Tasks

1. **YouTube API client** (`modules/streaming-hub/src/lib/api/youtube.ts`)
   ```typescript
   export class YouTubeClient {
     private apiKey: string

     async search(query: string, maxResults = 20) {
       // Call YouTube Data API v3
     }

     async getVideoDetails(videoId: string) {
       // Fetch metadata
     }

     async getChannelInfo(channelId: string) {
       // Fetch channel details
     }
   }
   ```

2. **Twitch API client** (`modules/streaming-hub/src/lib/api/twitch.ts`)
   ```typescript
   export class TwitchClient {
     private clientId: string
     private accessToken: string

     async searchStreams(query: string) {
       // Call Twitch Helix API
     }

     async getStreamInfo(streamId: string) {
       // Fetch stream metadata
     }
   }
   ```

3. **StreamCard component** (`modules/streaming-hub/src/components/StreamCard.svelte`)
   ```svelte
   <script lang="ts">
     export let stream: any
     export let onClick: () => void
     export let onAddToQueue: () => void
   </script>

   <div class="stream-card" onclick={onClick}>
     <img src={stream.thumbnail_url} alt={stream.title} />
     <div class="info">
       <h3>{stream.title}</h3>
       <p>{stream.channel}</p>
       <p>{stream.view_count} views • {stream.duration}</p>
     </div>
     <button onclick|stopPropagation={onAddToQueue}>+</button>
   </div>

   <style>
     .stream-card {
       background: #1A1A1A;
       border-radius: 8px;
       overflow: hidden;
       cursor: pointer;
       transition: transform 0.2s;
     }

     .stream-card:hover {
       transform: scale(1.05);
     }
   </style>
   ```

4. **Virtualized grid** (`modules/streaming-hub/src/views/Home.svelte`)
   ```svelte
   <script lang="ts">
     import { VirtualList } from 'svelte-virtual'
     import StreamCard from '$lib/components/StreamCard.svelte'

     let streams = $state<any[]>([])

     async function loadStreams() {
       // Fetch from API
     }
   </script>

   <VirtualList items={streams} let:item>
     <StreamCard stream={item} />
   </VirtualList>
   ```

### Deliverables
- ✅ YouTube and Twitch search working
- ✅ Card grid rendering with virtualization
- ✅ Search and filter UI
- ✅ Cards clickable and add to queue

---

## Phase 4-8: Remaining Phases

*(Following similar pattern for Queue & History, AI Recommendations, DRM & OAuth, Polish, Testing)*

**Phase 4**: Queue and history management
**Phase 5**: AI-powered recommendations
**Phase 6**: DRM support and OAuth integration
**Phase 7**: Performance optimization and accessibility
**Phase 8**: Testing and documentation

---

## Testing Checklist

### Unit Tests (Backend)
- [ ] `get_watch_history` with various filters
- [ ] `add_to_queue` with duplicates
- [ ] `save_playback_progress` persistence
- [ ] `get_recommendations` scoring algorithm
- [ ] OAuth token refresh logic

### Unit Tests (Frontend)
- [ ] StreamCard renders with mock data
- [ ] NowPlaying handles player events
- [ ] WatchQueue drag-and-drop logic
- [ ] IPC error handling

### Integration Tests
- [ ] Launch streaming module from main app
- [ ] Play video end-to-end
- [ ] DRM fallback when Widevine unavailable
- [ ] Network interruption recovery

### E2E Tests (Playwright)
- [ ] Browse and play workflow
- [ ] Queue management
- [ ] History and recommendations
- [ ] OAuth login flow

### Performance Tests
- [ ] Startup time <2s
- [ ] Memory usage <300MB peak
- [ ] 60fps scroll performance
- [ ] Video load time <3s (P95)

---

## Deployment Checklist

- [ ] All P0 requirements implemented
- [ ] All tests passing (95%+ coverage)
- [ ] Performance targets met
- [ ] Security audit passed
- [ ] Accessibility audit passed (WCAG 2.1 AA)
- [ ] Documentation complete
- [ ] API keys configured (YouTube, Twitch)
- [ ] OAuth redirect URIs registered
- [ ] Build pipeline working (CI/CD)

---

## Success Criteria

**v1.0 Launch**:
- [ ] Users can browse YouTube and Twitch content
- [ ] Users can play videos with HLS/DASH support
- [ ] Users can manage watch queue and history
- [ ] Users receive AI-powered recommendations
- [ ] DRM content plays via Electron sidecar
- [ ] Performance and security targets met

---

## Next Steps

1. **Review this plan** with team
2. **Obtain API keys** (YouTube Data API, Twitch Helix)
3. **Set up OAuth apps** (Google, Twitch)
4. **Assign ownership** for each phase
5. **Begin Phase 1** (Foundation)

---

**Ready to start implementation!** 🚀
