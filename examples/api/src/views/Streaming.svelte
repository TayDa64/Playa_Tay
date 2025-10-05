<script>
  import { onMount } from 'svelte'
  import SearchBar from '../components/streaming/SearchBar.svelte'
  import ContentCard from '../components/streaming/ContentCard.svelte'
  import VideoPlayer from '../components/streaming/VideoPlayer.svelte'
  import WatchQueue from '../components/streaming/WatchQueue.svelte'
  import * as streamingApi from '../lib/streaming/api'

  let { onMessage } = $props()

  let searchResults = $state([])
  let currentVideo = $state(null)
  let queueItems = $state([])
  let loading = $state(false)
  let error = $state(null)
  let consentGiven = $state(false)
  let showConsentDialog = $state(false)

  onMount(async () => {
    // Check if user has given consent for watch history
    try {
      consentGiven = await streamingApi.checkStreamingConsent('watch_history')
      
      // Show consent dialog on first use if not given
      if (!consentGiven) {
        showConsentDialog = true
      }
      
      // Load watch queue
      await loadQueue()
    } catch (err) {
      console.error('Failed to check consent:', err)
    }
  })

  async function handleSearch(event) {
    const query = event.detail
    loading = true
    error = null

    try {
      searchResults = await streamingApi.searchYouTube(query, 12)
      if (searchResults.length === 0) {
        error = 'No results found'
      }
      onMessage({ status: 'ok', action: 'search', query, results: searchResults.length })
    } catch (err) {
      error = err.message || 'Failed to search videos'
      onMessage({ status: 'error', action: 'search', error: err.message })
    } finally {
      loading = false
    }
  }

  async function handlePlay(event) {
    currentVideo = event.detail
    onMessage({ status: 'ok', action: 'play', video: currentVideo.id })
  }

  async function handleAddToQueue(event) {
    const content = event.detail
    
    try {
      await streamingApi.addToQueue(content)
      await loadQueue()
      onMessage({ status: 'ok', action: 'addToQueue', video: content.id })
    } catch (err) {
      error = 'Failed to add to queue: ' + err.message
      onMessage({ status: 'error', action: 'addToQueue', error: err.message })
    }
  }

  async function handlePlayFromQueue(event) {
    currentVideo = event.detail
    onMessage({ status: 'ok', action: 'playFromQueue', video: currentVideo.id })
  }

  async function handleRemoveFromQueue(event) {
    const queueId = event.detail
    
    try {
      await streamingApi.removeFromQueue(queueId)
      await loadQueue()
      onMessage({ status: 'ok', action: 'removeFromQueue', queueId })
    } catch (err) {
      error = 'Failed to remove from queue: ' + err.message
      onMessage({ status: 'error', action: 'removeFromQueue', error: err.message })
    }
  }

  async function handleProgress(event) {
    if (!consentGiven) return

    const { content, progress, duration } = event.detail
    
    try {
      await streamingApi.saveWatchProgress(content, progress, duration)
    } catch (err) {
      console.error('Failed to save progress:', err)
    }
  }

  async function loadQueue() {
    try {
      queueItems = await streamingApi.getWatchQueue()
    } catch (err) {
      console.error('Failed to load queue:', err)
    }
  }

  async function handleConsentAccept() {
    try {
      await streamingApi.setStreamingConsent('watch_history', true)
      consentGiven = true
      showConsentDialog = false
      onMessage({ status: 'ok', action: 'consentAccepted' })
    } catch (err) {
      console.error('Failed to set consent:', err)
    }
  }

  async function handleConsentDecline() {
    try {
      await streamingApi.setStreamingConsent('watch_history', false)
      consentGiven = false
      showConsentDialog = false
      onMessage({ status: 'ok', action: 'consentDeclined' })
    } catch (err) {
      console.error('Failed to set consent:', err)
    }
  }

  function closePlayer() {
    currentVideo = null
  }
</script>

<div class="streaming-hub">
  <div class="header">
    <h1 class="title">
      <span class="i-ph-play-circle-fill"></span>
      Streaming Hub
    </h1>
    <SearchBar 
      placeholder="Search YouTube videos..." 
      {loading} 
      on:search={handleSearch}
    />
  </div>

  {#if error}
    <div class="error-banner">
      <span class="i-ph-warning-circle"></span>
      {error}
      <button class="close-btn" onclick={() => error = null}>
        <span class="i-ph-x"></span>
      </button>
    </div>
  {/if}

  <div class="content-area">
    <div class="main-content">
      {#if currentVideo}
        <div class="player-section">
          <button class="back-btn" onclick={closePlayer}>
            <span class="i-ph-arrow-left"></span>
            Back to results
          </button>
          <VideoPlayer content={currentVideo} on:progress={handleProgress} />
        </div>
      {:else if searchResults.length > 0}
        <div class="results-grid">
          {#each searchResults as content (content.id)}
            <ContentCard 
              {content} 
              on:play={handlePlay}
              on:queue={handleAddToQueue}
            />
          {/each}
        </div>
      {:else if !loading}
        <div class="empty-state">
          <span class="i-ph-magnifying-glass text-6xl opacity-20"></span>
          <h2>Search for videos</h2>
          <p>Enter a search term to find videos on YouTube</p>
        </div>
      {/if}

      {#if loading}
        <div class="loading-state">
          <span class="i-ph-spinner animate-spin text-4xl"></span>
          <p>Searching...</p>
        </div>
      {/if}
    </div>

    <div class="sidebar">
      <WatchQueue 
        items={queueItems}
        on:play={handlePlayFromQueue}
        on:remove={handleRemoveFromQueue}
      />
    </div>
  </div>
</div>

{#if showConsentDialog}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={(e) => e.target === e.currentTarget && handleConsentDecline()}>
    <div class="modal-content">
      <h3 class="modal-title">
        <span class="i-ph-shield-check"></span>
        Watch History Consent
      </h3>
      <p class="modal-message">
        Would you like to save your watch history and progress? This helps you resume videos 
        where you left off and provides a better experience.
      </p>
      <div class="modal-info">
        <p><strong>What we track:</strong></p>
        <ul>
          <li>Videos you watch and their progress</li>
          <li>Timestamps of when you watched</li>
        </ul>
        <p><strong>Privacy:</strong></p>
        <ul>
          <li>All data stays on your device</li>
          <li>You can clear history anytime</li>
          <li>You can change this setting later</li>
        </ul>
      </div>
      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={handleConsentDecline}>
          No, don't track
        </button>
        <button class="btn btn-primary" onclick={handleConsentAccept}>
          Yes, enable tracking
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .streaming-hub {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #0a0a0a;
    color: #e5e5e5;
  }

  .header {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 24px;
    background: #1a1a1a;
    border-bottom: 1px solid #333;
  }

  .title {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 1.75rem;
    font-weight: 700;
    margin: 0;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 24px;
    background: #fee2e2;
    color: #991b1b;
    border-bottom: 1px solid #fecaca;
  }

  .error-banner .close-btn {
    margin-left: auto;
    background: none;
    border: none;
    color: #991b1b;
    cursor: pointer;
    font-size: 1.2rem;
  }

  .content-area {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    padding: 24px;
    overflow-y: auto;
  }

  .player-section {
    max-width: 1200px;
    margin: 0 auto;
  }

  .back-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    margin-bottom: 16px;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 6px;
    color: #e5e5e5;
    font-size: 0.95rem;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .back-btn:hover {
    background: #2a2a2a;
  }

  .results-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 20px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 80px 24px;
    color: #666;
  }

  .empty-state h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
    color: #a3a3a3;
  }

  .empty-state p {
    margin: 0;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 80px 24px;
    color: #666;
  }

  .sidebar {
    width: 320px;
    min-width: 320px;
    background: #0a0a0a;
    border-left: 1px solid #333;
    overflow-y: auto;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 12px;
    padding: 32px;
    max-width: 500px;
    width: 90%;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7);
  }

  .modal-title {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0 0 16px 0;
    color: #3b82f6;
  }

  .modal-message {
    margin: 0 0 20px 0;
    line-height: 1.6;
    color: #ccc;
  }

  .modal-info {
    background: #0a0a0a;
    border: 1px solid #2a2a2a;
    border-radius: 8px;
    padding: 16px;
    margin: 16px 0;
  }

  .modal-info p {
    margin: 8px 0;
    font-weight: 600;
    color: #e5e5e5;
  }

  .modal-info ul {
    margin: 8px 0;
    padding-left: 24px;
    color: #a3a3a3;
  }

  .modal-info li {
    margin: 4px 0;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 24px;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 6px;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .btn-primary {
    background: #3b82f6;
    color: #fff;
  }

  .btn-primary:hover {
    background: #2563eb;
  }

  .btn-secondary {
    background: #444;
    color: #fff;
  }

  .btn-secondary:hover {
    background: #555;
  }

  .text-4xl {
    font-size: 2.5rem;
  }

  .text-6xl {
    font-size: 4rem;
  }

  .opacity-20 {
    opacity: 0.2;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
