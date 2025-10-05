<script>
  import { invoke } from '@tauri-apps/api/core'
  import { convertFileSrc } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'
  import { openElectronFeature, isElectronAvailable } from '../lib/electronFeature.js'

  let { onMessage } = $props()

  // State management
  let electronAvailable = $state(null)
  let watchQueue = $state([])
  let currentVideo = $state(null)
  let isPlaying = $state(false)
  let showModal = $state(false)
  let modalMessage = $state('')
  let videoUrl = $state('')

  // Mock video library for demonstration
  const videoLibrary = [
    {
      id: 1,
      title: 'Example Video 1',
      thumbnail: 'https://via.placeholder.com/320x180?text=Video+1',
      duration: '10:30',
      provider: 'local'
    },
    {
      id: 2,
      title: 'Example Video 2',
      thumbnail: 'https://via.placeholder.com/320x180?text=Video+2',
      duration: '15:45',
      provider: 'local'
    },
    {
      id: 3,
      title: 'DRM Protected Content',
      thumbnail: 'https://via.placeholder.com/320x180?text=DRM+Video',
      duration: '20:00',
      provider: 'widevine',
      requiresDRM: true
    }
  ]

  onMount(async () => {
    // Check if Electron is available for DRM content
    electronAvailable = await isElectronAvailable()
    
    // Initialize watch queue from localStorage
    const savedQueue = localStorage.getItem('watchQueue')
    if (savedQueue) {
      try {
        watchQueue = JSON.parse(savedQueue)
      } catch (e) {
        console.error('Failed to parse saved queue:', e)
      }
    }

    onMessage({ status: 'info', action: 'streamingHubMounted', electronAvailable })
  })

  function addToQueue(video) {
    if (!watchQueue.find(v => v.id === video.id)) {
      watchQueue = [...watchQueue, video]
      localStorage.setItem('watchQueue', JSON.stringify(watchQueue))
      onMessage({ status: 'ok', action: 'addToQueue', video: video.title })
    }
  }

  function removeFromQueue(videoId) {
    watchQueue = watchQueue.filter(v => v.id !== videoId)
    localStorage.setItem('watchQueue', JSON.stringify(watchQueue))
    onMessage({ status: 'ok', action: 'removeFromQueue', videoId })
  }

  async function playVideo(video) {
    if (video.requiresDRM) {
      // DRM content requires Electron with Widevine
      if (!electronAvailable) {
        modalMessage = 'DRM-protected content requires Electron runtime with Widevine support.'
        showModal = true
        return
      }

      // Launch Electron player for DRM content
      const drmUrl = `https://example.com/drm-player?video=${video.id}`
      const res = await openElectronFeature(drmUrl)
      
      if (res.ok) {
        onMessage({ 
          status: 'ok', 
          action: 'playDRMVideo', 
          video: video.title,
          provider: video.provider 
        })
      } else {
        modalMessage = res.message || 'Failed to launch DRM player'
        showModal = true
        onMessage({ 
          status: 'error', 
          action: 'playDRMVideo', 
          video: video.title,
          error: res.code 
        })
      }
    } else {
      // Regular content can be played in-app
      currentVideo = video
      isPlaying = true
      onMessage({ 
        status: 'ok', 
        action: 'playVideo', 
        video: video.title,
        provider: video.provider 
      })
    }
  }

  function closeModal() {
    showModal = false
  }

  async function registerStreamingProtocol() {
    try {
      await invoke('register_streaming_protocol')
      onMessage({ status: 'ok', action: 'registerStreamingProtocol' })
    } catch (e) {
      onMessage({ 
        status: 'error', 
        action: 'registerStreamingProtocol', 
        error: e.toString() 
      })
    }
  }
</script>

<div class="streaming-hub">
  <div class="header">
    <h2>Streaming Hub (M1)</h2>
    <p class="subtitle">Multi-provider video streaming with DRM support</p>
    
    <div class="status-bar">
      <span class="status-item">
        DRM Support: 
        {#if electronAvailable === null}
          <span class="checking">Checking...</span>
        {:else if electronAvailable}
          <span class="available">Available ✓</span>
        {:else}
          <span class="unavailable">Unavailable ✗</span>
        {/if}
      </span>
      <span class="status-item">
        Queue: {watchQueue.length} video{watchQueue.length !== 1 ? 's' : ''}
      </span>
    </div>
  </div>

  <div class="content-grid">
    <!-- Video Library -->
    <section class="video-library">
      <h3>Video Library</h3>
      <div class="video-grid">
        {#each videoLibrary as video}
          <div class="video-card">
            <div class="video-thumbnail">
              <img src={video.thumbnail} alt={video.title} />
              {#if video.requiresDRM}
                <span class="drm-badge">DRM</span>
              {/if}
              <span class="duration">{video.duration}</span>
            </div>
            <div class="video-info">
              <h4>{video.title}</h4>
              <p class="provider">{video.provider}</p>
            </div>
            <div class="video-actions">
              <button class="btn btn-primary" onclick={() => playVideo(video)}>
                Play
              </button>
              <button class="btn btn-secondary" onclick={() => addToQueue(video)}>
                + Queue
              </button>
            </div>
          </div>
        {/each}
      </div>
    </section>

    <!-- Watch Queue -->
    <aside class="watch-queue">
      <h3>Watch Queue</h3>
      {#if watchQueue.length === 0}
        <p class="empty-message">No videos in queue</p>
      {:else}
        <div class="queue-list">
          {#each watchQueue as video}
            <div class="queue-item">
              <img src={video.thumbnail} alt={video.title} class="queue-thumbnail" />
              <div class="queue-info">
                <h4>{video.title}</h4>
                <p>{video.duration}</p>
              </div>
              <div class="queue-actions">
                <button class="btn-icon" onclick={() => playVideo(video)}>▶</button>
                <button class="btn-icon" onclick={() => removeFromQueue(video.id)}>✕</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </aside>
  </div>

  <!-- Current Player (for non-DRM content) -->
  {#if currentVideo && isPlaying}
    <div class="player-overlay">
      <div class="player-container">
        <div class="player-header">
          <h3>{currentVideo.title}</h3>
          <button class="btn-close" onclick={() => { isPlaying = false; currentVideo = null }}>✕</button>
        </div>
        <div class="player-content">
          <video controls autoplay width="100%">
            <source src={currentVideo.thumbnail} type="video/mp4" />
            Your browser does not support the video tag.
          </video>
        </div>
      </div>
    </div>
  {/if}
</div>

{#if showModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={closeModal}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h3 class="modal-title">DRM Support Required</h3>
      <p class="modal-message">{modalMessage}</p>
      <div class="modal-instructions">
        <p><strong>To enable DRM support:</strong></p>
        <code>pnpm -F @playa/electron-shell install</code>
        <p class="text-sm mt-2">This will install Electron with Widevine support.</p>
      </div>
      <div class="modal-actions">
        <button class="btn btn-primary" onclick={closeModal}>Close</button>
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
  }

  .header {
    border-bottom: 1px solid #333;
    padding-bottom: 1rem;
  }

  .header h2 {
    margin: 0;
    font-size: 1.5rem;
    color: #3b82f6;
  }

  .subtitle {
    margin: 0.5rem 0 0 0;
    color: #a3a3a3;
    font-size: 0.875rem;
  }

  .status-bar {
    display: flex;
    gap: 2rem;
    margin-top: 1rem;
    font-size: 0.875rem;
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .checking { color: #f59e0b; }
  .available { color: #10b981; }
  .unavailable { color: #ef4444; }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 2rem;
  }

  .video-library h3,
  .watch-queue h3 {
    margin: 0 0 1rem 0;
    font-size: 1.125rem;
  }

  .video-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .video-card {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .video-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2);
  }

  .video-thumbnail {
    position: relative;
    aspect-ratio: 16 / 9;
    overflow: hidden;
  }

  .video-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .drm-badge {
    position: absolute;
    top: 8px;
    left: 8px;
    background: #ef4444;
    color: white;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .duration {
    position: absolute;
    bottom: 8px;
    right: 8px;
    background: rgba(0, 0, 0, 0.8);
    color: white;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.75rem;
  }

  .video-info {
    padding: 12px;
  }

  .video-info h4 {
    margin: 0;
    font-size: 1rem;
    color: #e5e5e5;
  }

  .provider {
    margin: 4px 0 0 0;
    font-size: 0.75rem;
    color: #a3a3a3;
    text-transform: uppercase;
  }

  .video-actions {
    display: flex;
    gap: 8px;
    padding: 0 12px 12px 12px;
  }

  .btn {
    flex: 1;
    padding: 8px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-primary {
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover {
    background: #2563eb;
  }

  .btn-secondary {
    background: #444;
    color: white;
  }

  .btn-secondary:hover {
    background: #555;
  }

  .watch-queue {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1rem;
    max-height: 600px;
    overflow-y: auto;
  }

  .empty-message {
    color: #a3a3a3;
    text-align: center;
    padding: 2rem 0;
  }

  .queue-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .queue-item {
    display: flex;
    gap: 12px;
    background: #0a0a0a;
    border: 1px solid #333;
    border-radius: 4px;
    padding: 8px;
    align-items: center;
  }

  .queue-thumbnail {
    width: 80px;
    height: 45px;
    object-fit: cover;
    border-radius: 4px;
  }

  .queue-info {
    flex: 1;
    min-width: 0;
  }

  .queue-info h4 {
    margin: 0;
    font-size: 0.875rem;
    color: #e5e5e5;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .queue-info p {
    margin: 4px 0 0 0;
    font-size: 0.75rem;
    color: #a3a3a3;
  }

  .queue-actions {
    display: flex;
    gap: 4px;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
    border: none;
    background: #444;
    color: white;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s;
  }

  .btn-icon:hover {
    background: #555;
  }

  .player-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.95);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }

  .player-container {
    width: 90%;
    max-width: 1200px;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
  }

  .player-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: #0a0a0a;
    border-bottom: 1px solid #333;
  }

  .player-header h3 {
    margin: 0;
    color: #e5e5e5;
  }

  .btn-close {
    width: 32px;
    height: 32px;
    border: none;
    background: #444;
    color: white;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1.25rem;
    transition: background 0.2s;
  }

  .btn-close:hover {
    background: #ef4444;
  }

  .player-content {
    padding: 1rem;
  }

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
    color: #3b82f6;
  }

  .modal-message {
    margin: 0 0 16px 0;
    color: #ccc;
  }

  .modal-instructions {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 12px;
    margin: 16px 0;
  }

  .modal-instructions code {
    display: block;
    background: #1a1a1a;
    padding: 8px;
    border-radius: 4px;
    margin: 8px 0;
    font-family: 'Courier New', monospace;
    color: #4ade80;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 20px;
  }

  .text-sm {
    font-size: 0.875rem;
  }

  .mt-2 {
    margin-top: 0.5rem;
  }
</style>
