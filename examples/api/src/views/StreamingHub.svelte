<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { onMessage } = $props()

  // State
  let currentStream = $state(null)
  let watchQueue = $state([])
  let watchHistory = $state([])
  let recommendations = $state([])
  let isPlaying = $state(false)
  let showQueue = $state(false)
  let selectedProvider = $state('all')

  // Sample streaming data (in production, this would come from backend/API)
  const sampleStreams = [
    {
      id: 'yt-1',
      title: 'Big Buck Bunny',
      url: 'https://www.youtube.com/watch?v=aqz-KE-bpKQ',
      thumbnail: 'https://i.ytimg.com/vi/aqz-KE-bpKQ/hqdefault.jpg',
      provider: 'youtube',
      duration: 596,
      watched_progress: 0
    },
    {
      id: 'yt-2',
      title: 'Sintel',
      url: 'https://www.youtube.com/watch?v=eRsGyueVLvQ',
      thumbnail: 'https://i.ytimg.com/vi/eRsGyueVLvQ/hqdefault.jpg',
      provider: 'youtube',
      duration: 888,
      watched_progress: 0
    },
    {
      id: 'hls-1',
      title: 'HLS Stream Demo',
      url: 'https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8',
      thumbnail: null,
      provider: 'hls',
      duration: null,
      watched_progress: 0
    },
    {
      id: 'twitch-1',
      title: 'Twitch Sample Stream',
      url: 'https://www.twitch.tv/directory',
      thumbnail: null,
      provider: 'twitch',
      duration: null,
      watched_progress: 0
    }
  ]

  let availableStreams = $state(sampleStreams)
  let filteredStreams = $derived(
    selectedProvider === 'all'
      ? availableStreams
      : availableStreams.filter((s) => s.provider === selectedProvider)
  )

  onMount(async () => {
    // Load watch history
    try {
      const history = await invoke('get_watch_history', { limit: 10 })
      watchHistory = history
    } catch (err) {
      console.error('Failed to load history:', err)
    }

    // Load recommendations
    try {
      const recs = await invoke('get_recommendations')
      recommendations = recs
    } catch (err) {
      console.error('Failed to load recommendations:', err)
    }
  })

  async function playStream(stream) {
    try {
      await invoke('play_stream', { item: stream })
      currentStream = stream
      isPlaying = true
      onMessage({ status: 'playing', stream: stream.title })
    } catch (err) {
      onMessage({ status: 'error', error: err })
      console.error('Failed to play stream:', err)
    }
  }

  async function addToQueue(stream) {
    try {
      await invoke('add_to_queue', { item: stream })
      watchQueue = [...watchQueue, stream]
      onMessage({ status: 'queued', stream: stream.title })
    } catch (err) {
      console.error('Failed to add to queue:', err)
    }
  }

  async function saveProgress(streamId, progress) {
    try {
      await invoke('save_watch_progress', { id: streamId, progress })
    } catch (err) {
      console.error('Failed to save progress:', err)
    }
  }

  function formatDuration(seconds) {
    if (!seconds) return 'Live'
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  function getProviderColor(provider) {
    const colors = {
      youtube: '#FF0000',
      twitch: '#9146FF',
      hls: '#3B82F6',
      dash: '#10B981',
      widevine: '#F59E0B'
    }
    return colors[provider] || '#6B7280'
  }

  function stopPlaying() {
    isPlaying = false
    if (currentStream) {
      // Save progress before stopping
      saveProgress(currentStream.id, 0)
    }
    currentStream = null
  }
</script>

<div class="streaming-hub">
  <div class="hub-header">
    <h2 class="hub-title">Streaming Hub</h2>
    <div class="hub-controls">
      <select class="provider-filter" bind:value={selectedProvider}>
        <option value="all">All Providers</option>
        <option value="youtube">YouTube</option>
        <option value="twitch">Twitch</option>
        <option value="hls">HLS/DASH</option>
      </select>
      <button class="btn-queue" onclick={() => (showQueue = !showQueue)}>
        Queue ({watchQueue.length})
      </button>
    </div>
  </div>

  {#if currentStream && isPlaying}
    <div class="now-playing">
      <div class="player-info">
        <h3>Now Playing: {currentStream.title}</h3>
        <span
          class="provider-badge"
          style="background-color: {getProviderColor(currentStream.provider)}"
        >
          {currentStream.provider.toUpperCase()}
        </span>
      </div>
      <button class="btn-stop" onclick={stopPlaying}>Stop</button>
    </div>
  {/if}

  <div class="content-grid">
    {#if showQueue && watchQueue.length > 0}
      <div class="queue-section">
        <h3>Watch Queue</h3>
        <div class="queue-list">
          {#each watchQueue as item, index}
            <div class="queue-item">
              <span class="queue-number">{index + 1}</span>
              <span class="queue-title">{item.title}</span>
              <button class="btn-play-small" onclick={() => playStream(item)}>
                ▶
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="streams-section">
      <h3>Available Streams</h3>
      <div class="stream-grid">
        {#each filteredStreams as stream}
          <div class="stream-card">
            <div class="stream-thumbnail">
              {#if stream.thumbnail}
                <img src={stream.thumbnail} alt={stream.title} />
              {:else}
                <div class="thumbnail-placeholder">
                  <span class="placeholder-icon">🎬</span>
                </div>
              {/if}
              {#if stream.duration}
                <span class="duration-badge">{formatDuration(stream.duration)}</span>
              {/if}
            </div>
            <div class="stream-info">
              <h4 class="stream-title">{stream.title}</h4>
              <span
                class="provider-badge"
                style="background-color: {getProviderColor(stream.provider)}"
              >
                {stream.provider}
              </span>
            </div>
            <div class="stream-actions">
              <button class="btn-play" onclick={() => playStream(stream)}>
                Play
              </button>
              <button class="btn-queue-add" onclick={() => addToQueue(stream)}>
                + Queue
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  {#if watchHistory.length > 0}
    <div class="history-section">
      <h3>Watch History</h3>
      <div class="history-list">
        {#each watchHistory as item}
          <div class="history-item">
            <span>{item.title}</span>
            <button class="btn-play-small" onclick={() => playStream(item)}>▶</button>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .streaming-hub {
    padding: 1rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .hub-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #333;
  }

  .hub-title {
    font-size: 1.75rem;
    font-weight: 600;
    margin: 0;
    color: #e5e5e5;
  }

  .hub-controls {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .provider-filter {
    padding: 0.5rem 1rem;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 4px;
    color: #e5e5e5;
    cursor: pointer;
  }

  .btn-queue {
    padding: 0.5rem 1rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  .btn-queue:hover {
    background: #2563eb;
  }

  .now-playing {
    background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 100%);
    border: 1px solid #3b82f6;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .player-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .player-info h3 {
    margin: 0;
    font-size: 1.25rem;
    color: #e5e5e5;
  }

  .provider-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    color: white;
    text-transform: uppercase;
  }

  .btn-stop {
    padding: 0.5rem 1.5rem;
    background: #ef4444;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  .btn-stop:hover {
    background: #dc2626;
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .queue-section,
  .streams-section,
  .history-section {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .queue-section h3,
  .streams-section h3,
  .history-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: #e5e5e5;
  }

  .stream-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .stream-card {
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 8px;
    overflow: hidden;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .stream-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .stream-thumbnail {
    position: relative;
    width: 100%;
    height: 160px;
    background: #1a1a1a;
    overflow: hidden;
  }

  .stream-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .thumbnail-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 100%);
  }

  .placeholder-icon {
    font-size: 3rem;
  }

  .duration-badge {
    position: absolute;
    bottom: 0.5rem;
    right: 0.5rem;
    background: rgba(0, 0, 0, 0.8);
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .stream-info {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .stream-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 500;
    color: #e5e5e5;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .stream-actions {
    display: flex;
    gap: 0.5rem;
    padding: 0 1rem 1rem 1rem;
  }

  .btn-play,
  .btn-queue-add {
    flex: 1;
    padding: 0.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-play {
    background: #3b82f6;
    color: white;
  }

  .btn-play:hover {
    background: #2563eb;
  }

  .btn-queue-add {
    background: #444;
    color: #e5e5e5;
  }

  .btn-queue-add:hover {
    background: #555;
  }

  .queue-list,
  .history-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .queue-item,
  .history-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
  }

  .queue-number {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #3b82f6;
    color: white;
    border-radius: 50%;
    font-size: 0.875rem;
    font-weight: 600;
    flex-shrink: 0;
  }

  .queue-title {
    flex: 1;
    color: #e5e5e5;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .btn-play-small {
    padding: 0.25rem 0.75rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    flex-shrink: 0;
  }

  .btn-play-small:hover {
    background: #2563eb;
  }

  .history-item span {
    flex: 1;
    color: #e5e5e5;
  }
</style>
