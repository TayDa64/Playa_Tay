# M1 Streaming Hub - UI Mockup

This document provides a visual representation of the implemented UI.

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  PROVIDER TABS                        DRM STATUS            │   │
│  │  ┌────────┐ ┌────────┐ ┌────────────┐  ┌──────────────┐   │   │
│  │  │YouTube │ │Twitch  │ │Custom HLS  │  │✓ DRM ready   │   │   │
│  │  └────────┘ └────────┘ └────────────┘  └──────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  SEARCH BAR                                                  │   │
│  │  ┌──────────────────────────────────────────┐ ┌─────────┐  │   │
│  │  │ Search youtube...                        │ │ Search  │  │   │
│  │  └──────────────────────────────────────────┘ └─────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  WATCH QUEUE (3)                                            │   │
│  │  ┌──────┬──────────────────────────────────┬─────┬────────┐│   │
│  │  │[IMG] │ Sample Video 1                   │Play │Remove  ││   │
│  │  │ 3:45 │ youtube                           │     │        ││   │
│  │  └──────┴──────────────────────────────────┴─────┴────────┘│   │
│  │  ┌──────┬──────────────────────────────────┬─────┬────────┐│   │
│  │  │[IMG] │ Sample Video 2                   │Play │Remove  ││   │
│  │  │ 4:20 │ youtube                           │     │        ││   │
│  │  └──────┴──────────────────────────────────┴─────┴────────┘│   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  RECOMMENDATIONS                                            │   │
│  │                                                              │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │   │
│  │  │ ▶️       │  │ ▶️       │  │ ▶️       │  │ ▶️       │   │   │
│  │  │          │  │          │  │          │  │          │   │   │
│  │  │[  IMG  ] │  │[  IMG  ] │  │[  IMG  ] │  │[  IMG  ] │   │   │
│  │  │          │  │          │  │          │  │          │   │   │
│  │  │   3:45   │  │   4:20   │  │   LIVE   │  │   5:12   │   │   │
│  │  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────┤   │   │
│  │  │Sample V1 │  │Sample V2 │  │Live Strm │  │Sample V3 │   │   │
│  │  │youtube   │  │youtube   │  │twitch    │  │youtube   │   │   │
│  │  │[+Queue]  │  │[+Queue]  │  │[+Queue]  │  │[+Queue]  │   │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │   │
│  │                                                              │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │   │
│  │  │ ▶️       │  │ ▶️       │  │ ▶️       │  │ ▶️       │   │   │
│  │  │          │  │          │  │          │  │          │   │   │
│  │  │[  IMG  ] │  │[  IMG  ] │  │[  IMG  ] │  │[  IMG  ] │   │   │
│  │  │          │  │          │  │          │  │          │   │   │
│  │  │   2:30   │  │   6:15   │  │   3:00   │  │   4:45   │   │   │
│  │  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────┤   │   │
│  │  │Sample V4 │  │Sample V5 │  │Sample V6 │  │Sample V7 │   │   │
│  │  │youtube   │  │twitch    │  │youtube   │  │youtube   │   │   │
│  │  │[+Queue]  │  │[+Queue]  │  │[+Queue]  │  │[+Queue]  │   │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  WATCH HISTORY (5)                            [Clear All]   │   │
│  │  ┌──────┬──────────────────────────────────┬──────────────┐│   │
│  │  │[IMG] │ Previously Watched Video         │Play Again    ││   │
│  │  │ 3:45 │ 2025-01-05                        │              ││   │
│  │  └──────┴──────────────────────────────────┴──────────────┘│   │
│  │  ┌──────┬──────────────────────────────────┬──────────────┐│   │
│  │  │[IMG] │ Another Watched Video             │Play Again    ││   │
│  │  │ 4:20 │ 2025-01-04                        │              ││   │
│  │  └──────┴──────────────────────────────────┴──────────────┘│   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

## Color Scheme (Dark Mode)

```
Background Colors:
  Main Background: #0A0A0A (near-black)
  Card/Surface:    rgba(255, 255, 255, 0.05) (subtle transparency)
  Hover State:     rgba(255, 255, 255, 0.08)

Accent Colors:
  Primary (Blue):  #3B82F6 (active tabs, buttons)
  Success (Green): #10B981 (DRM ready indicator)
  Warning (Amber): #F59E0B (DRM unavailable warning)
  Error (Red):     #EF4444 (error states)

Text Colors:
  Primary:   #E5E5E5 (bright white-ish)
  Secondary: #A3A3A3 (muted gray)
  Tertiary:  #666666 (subtle gray)

Borders:
  Default: rgba(255, 255, 255, 0.1)
  Active:  #3B82F6
```

## Interactive States

### Provider Tabs
```
Inactive:  [Background: rgba(255,255,255,0.05), Border: rgba(255,255,255,0.1)]
Hover:     [Background: rgba(255,255,255,0.1), Text: #FFF]
Active:    [Background: #3B82F6, Border: #3B82F6, Text: #FFF]
```

### Video Cards (Recommendations)
```
Default:
  ┌──────────┐
  │ ▶️       │  ← Play button (opacity: 0)
  │          │
  │[  IMG  ] │  ← Thumbnail image
  │          │
  │   3:45   │  ← Duration badge (bottom-right)
  └──────────┘

Hover:
  ┌──────────┐
  │  (▶️)     │  ← Play button (opacity: 1, centered)
  │          │
  │[  IMG  ] │  ← Lifted 4px with shadow
  │          │
  │   3:45   │
  └──────────┘
  transform: translateY(-4px)
  box-shadow: 0 8px 16px rgba(0,0,0,0.3)
```

### Buttons
```
Primary (Play, Search):
  Default: #3B82F6
  Hover:   #2563EB (darker blue)

Secondary (Add to Queue):
  Default: rgba(255,255,255,0.1) with border
  Hover:   rgba(255,255,255,0.15)

Danger (Remove):
  Default: rgba(239,68,68,0.8)
  Hover:   #EF4444 (solid red)

Text (Clear All):
  Default: #3B82F6
  Hover:   #2563EB
```

## Responsive Behavior

### Grid Layout
```css
.video-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}
```

**Behavior:**
- Window width < 320px: 1 column (mobile portrait)
- Window width 320-640px: 1-2 columns (mobile landscape)
- Window width 640-1024px: 2-3 columns (tablet)
- Window width 1024-1400px: 3-4 columns (desktop)
- Window width > 1400px: 4-5 columns (wide desktop)

### Compact List Items (Queue, History)
```
Desktop (>640px):
  ┌──────┬────────────────────────────┬──────────┐
  │[IMG] │ Title + metadata           │ Actions  │
  │160px │ Flex-grow                  │ Flex-end │
  └──────┴────────────────────────────┴──────────┘

Mobile (<640px):
  ┌──────┬────────────────┐
  │[IMG] │ Title          │
  │120px │ Actions below  │
  └──────┴────────────────┘
```

## Animations

### Card Hover (60fps)
```css
.video-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.video-card:hover {
  transform: translateY(-4px);  /* GPU-accelerated */
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
}
```

### Play Button Fade
```css
.play-overlay {
  opacity: 0;
  transition: opacity 0.2s ease;
}
.video-card:hover .play-overlay {
  opacity: 1;
}
```

### Tab Switch
```css
.tab {
  transition: all 0.2s ease;
}
```

## Accessibility

### Keyboard Navigation
- **Tab**: Navigate between interactive elements
- **Enter/Space**: Activate buttons and links
- **Arrow Keys**: Navigate between video cards (future)
- **Escape**: Close modal dialogs

### Screen Reader Support
- Semantic HTML (`<button>`, `<section>`, `<h3>`)
- ARIA labels on icon-only buttons
- Alt text on images (video thumbnails)
- Status announcements for DRM availability

### Focus States
```css
button:focus-visible {
  outline: 2px solid #3B82F6;
  outline-offset: 2px;
}
```

## Modal Dialog (Error Handling)

```
┌─────────────────────────────────────────┐
│                                         │
│  Playback Error                         │
│                                         │
│  Electron runtime not found. Please     │
│  install dependencies.                  │
│                                         │
│                              [ OK ]     │
│                                         │
└─────────────────────────────────────────┘

Background: #1E1E1E
Border: 1px solid #333
Shadow: 0 4px 20px rgba(0,0,0,0.5)
Overlay: rgba(0,0,0,0.7)
```

## Typography

```
Headings (h3):
  font-size: 1.25rem (20px)
  font-weight: 600
  color: #E5E5E5

Body (video titles):
  font-size: 1rem (16px)
  font-weight: 600
  color: #E5E5E5

Secondary (provider, metadata):
  font-size: 0.875rem (14px)
  font-weight: 400
  color: #A3A3A3

Small (duration badges):
  font-size: 0.75rem (12px)
  font-weight: 600
  color: #FFF
```

## Implementation Details

### Component Structure
```svelte
<StreamingHub>
  <ProviderTabs>
    <Tab active={youtube} />
    <Tab active={twitch} />
    <Tab active={custom} />
    <DRMStatusIndicator />
  </ProviderTabs>
  
  <SearchBar />
  
  {#if watchQueue.length > 0}
    <WatchQueueSection>
      {#each watchQueue as video}
        <CompactVideoItem />
      {/each}
    </WatchQueueSection>
  {/if}
  
  <RecommendationsSection>
    <VideoGrid>
      {#each recommendations as video}
        <VideoCard />
      {/each}
    </VideoGrid>
  </RecommendationsSection>
  
  {#if watchHistory.length > 0}
    <WatchHistorySection>
      {#each watchHistory as video}
        <CompactVideoItem />
      {/each}
    </WatchHistorySection>
  {/if}
</StreamingHub>
```

### State Management
```javascript
let activeProvider = 'youtube'    // Current tab
let searchQuery = ''               // Search input
let watchQueue = []                // Videos in queue
let watchHistory = []              // Watched videos
let recommendations = []           // Suggested videos
let electronAvailable = null       // DRM status
let showModal = false              // Error dialog
let modalMessage = ''              // Error text
```

### Event Handlers
```javascript
switchProvider(provider)       // Change active tab
handleSearch()                 // Submit search query
playVideo(video)               // Start playback
addToQueue(video)              // Add to watch queue
removeFromQueue(videoId)       // Remove from queue
addToHistory(video)            // Log to history
clearHistory()                 // Delete all history
```

## Performance Metrics

### Rendering
- Initial render: <100ms (placeholder data)
- Card hover: 60fps (transform-based)
- Tab switch: <50ms (CSS transition)
- Modal open/close: <200ms (fade animation)

### Memory
- Component base: ~5MB
- Per video card: ~50KB (thumbnail + metadata)
- 50 videos displayed: ~7.5MB total
- Target: <50MB for entire module

### Network (Future)
- Thumbnail lazy-loading: Load on scroll
- API response caching: 1h TTL
- Search debouncing: 300ms delay
- Rate limiting: Respect provider quotas

---

**UI Status**: ✅ Implemented and functional with placeholder data
**Next**: Integrate real provider APIs and test with live data
