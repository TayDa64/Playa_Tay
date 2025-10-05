<script>
  import { invoke } from '@tauri-apps/api/core'
  import { openElectronFeature, isElectronAvailable } from '../lib/electronFeature.js'
  import { onMount } from 'svelte'

  let { onMessage } = $props()

  // State
  let electronAvailable = $state(null) // null=checking, true=available, false=not available
  let activeProvider = $state('youtube')
  let searchQuery = $state('')
  let watchQueue = $state([])
  let watchHistory = $state([])
  let recommendations = $state([])
  let currentVideo = $state(null)
  let showModal = $state(false)
  let modalMessage = $state('')

  // Sample data structure for videos
  const sampleVideos = [
    {
      id: 'sample1',
      title: 'Sample Video 1',
      provider: 'youtube',
      url: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
      thumbnail: 'https://via.placeholder.com/320x180?text=Video+1',
      duration: '3:45'
    },
    {
      id: 'sample2',
      title: 'Sample Video 2',
      provider: 'youtube',
      url: 'https://www.youtube.com/watch?v=9bZkp7q19f0',
      thumbnail: 'https://via.placeholder.com/320x180?text=Video+2',
      duration: '4:20'
    },
    {
      id: 'sample3',
      title: 'Live Stream Example',
      provider: 'twitch',
      url: 'https://www.twitch.tv/directory',
      thumbnail: 'https://via.placeholder.com/320x180?text=Live+Stream',
      duration: 'LIVE'
    }
  ]

  onMount(async () => {
    // Check if Electron is available for DRM content
    electronAvailable = await isElectronAvailable()
    
    // Load sample recommendations
    recommendations = [...sampleVideos]
    
    // Try to load watch history and queue from backend
    try {
      const history = await invoke('get_watch_history')
      watchHistory = history || []
    } catch (e) {
      // History not yet implemented, use empty array
      watchHistory = []
    }
    
    try {
      const queue = await invoke('get_watch_queue')
      watchQueue = queue || []
    } catch (e) {
      // Queue not yet implemented, use empty array
      watchQueue = []
    }
  })

  async function playVideo(video) {
    onMessage({ action: 'playVideo', video })
    
    // For DRM content, use Electron sidecar
    if (video.requiresDRM && electronAvailable) {
      const res = await openElectronFeature(video.url)
      if (res.ok) {
        currentVideo = video
        addToHistory(video)
        onMessage({ status: 'ok', action: 'playVideo', url: video.url })
      } else {
        modalMessage = res.message || 'Failed to play DRM-protected content.'
        showModal = true
      }
    } else {
      // For non-DRM content, open in default browser or embedded player
      currentVideo = video
      addToHistory(video)
      await invoke('open_video', { url: video.url }).catch(() => {
        // Fallback: open in external browser
        window.open(video.url, '_blank')
      })
    }
  }

  function addToQueue(video) {
    if (!watchQueue.find(v => v.id === video.id)) {
      watchQueue = [...watchQueue, video]
      onMessage({ action: 'addToQueue', video })
      
      // Persist to backend
      invoke('add_to_watch_queue', { video }).catch(e => {
        console.error('Failed to persist queue:', e)
      })
    }
  }

  function removeFromQueue(videoId) {
    watchQueue = watchQueue.filter(v => v.id !== videoId)
    invoke('remove_from_watch_queue', { videoId }).catch(e => {
      console.error('Failed to remove from queue:', e)
    })
  }

  function addToHistory(video) {
    const historyEntry = { ...video, watchedAt: new Date().toISOString() }
    watchHistory = [historyEntry, ...watchHistory.filter(v => v.id !== video.id)].slice(0, 50)
    
    // Persist to backend
    invoke('add_to_watch_history', { video: historyEntry }).catch(e => {
      console.error('Failed to persist history:', e)
    })
  }

  function clearHistory() {
    watchHistory = []
    invoke('clear_watch_history').catch(e => {
      console.error('Failed to clear history:', e)
    })
  }

  function switchProvider(provider) {
    activeProvider = provider
    onMessage({ action: 'switchProvider', provider })
  }

  function handleSearch() {
    if (searchQuery.trim()) {
      onMessage({ action: 'search', query: searchQuery, provider: activeProvider })
      // Backend search implementation would go here
      invoke('search_videos', { query: searchQuery, provider: activeProvider })
        .then(results => {
          recommendations = results
        })
        .catch(() => {
          // Fallback to sample data
          recommendations = sampleVideos.filter(v => 
            v.title.toLowerCase().includes(searchQuery.toLowerCase())
          )
        })
    }
  }

  function closeModal() {
    showModal = false
  }
</script>

<div class="streaming-hub">
  <!-- Header with provider tabs -->
  <div class="provider-tabs">
    <button 
      class="tab {activeProvider === 'youtube' ? 'active' : ''}"
      onclick={() => switchProvider('youtube')}
    >
      YouTube
    </button>
    <button 
      class="tab {activeProvider === 'twitch' ? 'active' : ''}"
      onclick={() => switchProvider('twitch')}
    >
      Twitch
    </button>
    <button 
      class="tab {activeProvider === 'custom' ? 'active' : ''}"
      onclick={() => switchProvider('custom')}
    >
      Custom HLS/DASH
    </button>
    
    {#if electronAvailable === false}
      <div class="drm-warning">
        ⚠️ DRM content unavailable
      </div>
    {:else if electronAvailable === true}
      <div class="drm-status">
        ✓ DRM ready
      </div>
    {/if}
  </div>

  <!-- Search bar -->
  <div class="search-bar">
    <input 
      type="text" 
      bind:value={searchQuery}
      onkeydown={(e) => e.key === 'Enter' && handleSearch()}
      placeholder="Search {activeProvider}..."
      class="search-input"
    />
    <button class="search-button" onclick={handleSearch}>
      Search
    </button>
  </div>

  <!-- Watch Queue Section -->
  {#if watchQueue.length > 0}
    <section class="queue-section">
      <h3>Watch Queue ({watchQueue.length})</h3>
      <div class="video-list">
        {#each watchQueue as video}
          <div class="video-item-small">
            <div class="thumbnail-small">
              <img src={video.thumbnail} alt={video.title} />
              <span class="duration">{video.duration}</span>
            </div>
            <div class="video-info-small">
              <p class="video-title">{video.title}</p>
              <div class="video-actions">
                <button class="btn-small" onclick={() => playVideo(video)}>Play</button>
                <button class="btn-small btn-remove" onclick={() => removeFromQueue(video.id)}>Remove</button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- Recommendations Section -->
  <section class="recommendations-section">
    <h3>Recommendations</h3>
    <div class="video-grid">
      {#each recommendations as video}
        <div class="video-card">
          <div class="thumbnail">
            <img src={video.thumbnail} alt={video.title} />
            <span class="duration">{video.duration}</span>
            <button class="play-overlay" onclick={() => playVideo(video)}>
              ▶️
            </button>
          </div>
          <div class="video-info">
            <p class="video-title">{video.title}</p>
            <p class="video-provider">{video.provider}</p>
            <button class="btn-secondary" onclick={() => addToQueue(video)}>
              Add to Queue
            </button>
          </div>
        </div>
      {/each}
    </div>
  </section>

  <!-- Watch History Section -->
  {#if watchHistory.length > 0}
    <section class="history-section">
      <div class="section-header">
        <h3>Watch History ({watchHistory.length})</h3>
        <button class="btn-text" onclick={clearHistory}>Clear All</button>
      </div>
      <div class="video-list">
        {#each watchHistory.slice(0, 10) as video}
          <div class="video-item-small">
            <div class="thumbnail-small">
              <img src={video.thumbnail} alt={video.title} />
              <span class="duration">{video.duration}</span>
            </div>
            <div class="video-info-small">
              <p class="video-title">{video.title}</p>
              <p class="video-meta">{new Date(video.watchedAt).toLocaleDateString()}</p>
              <button class="btn-small" onclick={() => playVideo(video)}>Play Again</button>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</div>

{#if showModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={closeModal}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h3 class="modal-title">Playback Error</h3>
      <p class="modal-message">{modalMessage}</p>
      <div class="modal-actions">
        <button class="btn btn-primary" onclick={closeModal}>OK</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .streaming-hub {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  /* Provider Tabs */
  .provider-tabs {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    padding-bottom: 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .tab {
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ccc;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tab:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .tab.active {
    background: #3b82f6;
    border-color: #3b82f6;
    color: #fff;
  }

  .drm-warning {
    margin-left: auto;
    padding: 0.25rem 0.75rem;
    background: rgba(245, 158, 11, 0.2);
    border: 1px solid #f59e0b;
    border-radius: 4px;
    color: #f59e0b;
    font-size: 0.875rem;
  }

  .drm-status {
    margin-left: auto;
    padding: 0.25rem 0.75rem;
    background: rgba(16, 185, 129, 0.2);
    border: 1px solid #10b981;
    border-radius: 4px;
    color: #10b981;
    font-size: 0.875rem;
  }

  /* Search Bar */
  .search-bar {
    display: flex;
    gap: 0.5rem;
  }

  .search-input {
    flex: 1;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #fff;
    font-size: 1rem;
  }

  .search-input:focus {
    outline: none;
    border-color: #3b82f6;
    background: rgba(255, 255, 255, 0.08);
  }

  .search-button {
    padding: 0.75rem 1.5rem;
    background: #3b82f6;
    border: none;
    border-radius: 6px;
    color: #fff;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .search-button:hover {
    background: #2563eb;
  }

  /* Sections */
  section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #e5e5e5;
    margin: 0;
  }

  /* Video Grid (Card View) */
  .video-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .video-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    overflow: hidden;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    cursor: pointer;
  }

  .video-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  }

  .thumbnail {
    position: relative;
    width: 100%;
    aspect-ratio: 16/9;
    overflow: hidden;
    background: #1a1a1a;
  }

  .thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .duration {
    position: absolute;
    bottom: 0.5rem;
    right: 0.5rem;
    padding: 0.25rem 0.5rem;
    background: rgba(0, 0, 0, 0.8);
    border-radius: 4px;
    color: #fff;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .play-overlay {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 60px;
    height: 60px;
    background: rgba(59, 130, 246, 0.9);
    border: none;
    border-radius: 50%;
    color: #fff;
    font-size: 1.5rem;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .video-card:hover .play-overlay {
    opacity: 1;
  }

  .video-info {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .video-title {
    font-weight: 600;
    color: #e5e5e5;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .video-provider {
    font-size: 0.875rem;
    color: #a3a3a3;
    margin: 0;
  }

  /* Video List (Compact View) */
  .video-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .video-item-small {
    display: flex;
    gap: 1rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    padding: 0.75rem;
    transition: background 0.2s ease;
  }

  .video-item-small:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .thumbnail-small {
    position: relative;
    width: 160px;
    aspect-ratio: 16/9;
    flex-shrink: 0;
    border-radius: 6px;
    overflow: hidden;
    background: #1a1a1a;
  }

  .thumbnail-small img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .video-info-small {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    justify-content: center;
  }

  .video-meta {
    font-size: 0.75rem;
    color: #a3a3a3;
    margin: 0;
  }

  .video-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Buttons */
  .btn-secondary {
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    color: #e5e5e5;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .btn-small {
    padding: 0.25rem 0.75rem;
    background: #3b82f6;
    border: none;
    border-radius: 4px;
    color: #fff;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .btn-small:hover {
    background: #2563eb;
  }

  .btn-remove {
    background: rgba(239, 68, 68, 0.8);
  }

  .btn-remove:hover {
    background: #ef4444;
  }

  .btn-text {
    padding: 0.25rem 0.5rem;
    background: transparent;
    border: none;
    color: #3b82f6;
    font-size: 0.875rem;
    cursor: pointer;
    transition: color 0.2s ease;
  }

  .btn-text:hover {
    color: #2563eb;
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 24px;
    max-width: 500px;
    width: 90%;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .modal-title {
    margin: 0 0 16px 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #ff6b6b;
  }

  .modal-message {
    margin: 0 0 16px 0;
    color: #ccc;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 20px;
  }

  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
    transition: background 0.2s ease;
  }

  .btn-primary {
    background: #3b82f6;
    color: #fff;
  }

  .btn-primary:hover {
    background: #2563eb;
  }
</style>
