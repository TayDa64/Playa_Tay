<script>
  import { onMount, onDestroy, createEventDispatcher } from 'svelte'

  let { content } = $props()
  const dispatch = createEventDispatcher()

  let videoElement = $state(null)
  let playing = $state(false)
  let currentTime = $state(0)
  let duration = $state(0)
  let volume = $state(1)
  let fullscreen = $state(false)
  let progressSaveInterval = null

  onMount(() => {
    if (videoElement) {
      videoElement.volume = volume
      
      // Save progress every 5 seconds
      progressSaveInterval = setInterval(() => {
        if (playing && currentTime > 0) {
          dispatch('progress', {
            content,
            progress: currentTime,
            duration
          })
        }
      }, 5000)
    }
  })

  onDestroy(() => {
    if (progressSaveInterval) {
      clearInterval(progressSaveInterval)
    }
  })

  function togglePlay() {
    if (videoElement) {
      if (playing) {
        videoElement.pause()
      } else {
        videoElement.play()
      }
    }
  }

  function handleTimeUpdate() {
    if (videoElement) {
      currentTime = videoElement.currentTime
    }
  }

  function handleLoadedMetadata() {
    if (videoElement) {
      duration = videoElement.duration
    }
  }

  function handlePlay() {
    playing = true
  }

  function handlePause() {
    playing = false
  }

  function handleSeek(event) {
    if (videoElement) {
      const rect = event.currentTarget.getBoundingClientRect()
      const x = event.clientX - rect.left
      const percentage = x / rect.width
      videoElement.currentTime = percentage * duration
    }
  }

  function handleVolumeChange(event) {
    volume = parseFloat(event.target.value)
    if (videoElement) {
      videoElement.volume = volume
    }
  }

  function toggleFullscreen() {
    if (videoElement) {
      if (!document.fullscreenElement) {
        videoElement.requestFullscreen()
        fullscreen = true
      } else {
        document.exitFullscreen()
        fullscreen = false
      }
    }
  }

  function formatTime(seconds) {
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = Math.floor(seconds % 60)
    
    if (hours > 0) {
      return `${hours}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
    }
    return `${minutes}:${String(secs).padStart(2, '0')}`
  }

  function getYouTubeEmbedUrl(content) {
    // Extract video ID from YouTube URL
    const urlParams = new URLSearchParams(new URL(content.url).search)
    const videoId = urlParams.get('v') || content.id
    return `https://www.youtube.com/embed/${videoId}?autoplay=1`
  }
</script>

<div class="video-player">
  <div class="video-container">
    {#if content.provider === 'youtube'}
      <!-- Use YouTube iframe embed for now -->
      <iframe
        src={getYouTubeEmbedUrl(content)}
        title={content.title}
        frameborder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
        allowfullscreen
        class="video-iframe"
      ></iframe>
    {:else}
      <!-- Standard HTML5 video for other sources -->
      <video
        bind:this={videoElement}
        src={content.url}
        ontimeupdate={handleTimeUpdate}
        onloadedmetadata={handleLoadedMetadata}
        onplay={handlePlay}
        onpause={handlePause}
        class="video-element"
      >
        <track kind="captions" />
      </video>

      <div class="controls">
        <button class="control-btn" onclick={togglePlay}>
          {#if playing}
            <span class="i-ph-pause-fill"></span>
          {:else}
            <span class="i-ph-play-fill"></span>
          {/if}
        </button>

        <div class="progress-bar" onclick={handleSeek}>
          <div class="progress-fill" style="width: {(currentTime / duration) * 100}%"></div>
        </div>

        <span class="time-display">
          {formatTime(currentTime)} / {formatTime(duration)}
        </span>

        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={volume}
          oninput={handleVolumeChange}
          class="volume-slider"
        />

        <button class="control-btn" onclick={toggleFullscreen}>
          <span class="i-ph-arrows-out"></span>
        </button>
      </div>
    {/if}
  </div>

  <div class="video-info">
    <h2 class="video-title">{content.title}</h2>
    {#if content.creator}
      <p class="video-creator">{content.creator}</p>
    {/if}
    {#if content.description}
      <p class="video-description">{content.description}</p>
    {/if}
  </div>
</div>

<style>
  .video-player {
    width: 100%;
    background: #0a0a0a;
    border-radius: 8px;
    overflow: hidden;
  }

  .video-container {
    position: relative;
    width: 100%;
    padding-top: 56.25%; /* 16:9 aspect ratio */
    background: #000;
  }

  .video-element,
  .video-iframe {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }

  .controls {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
  }

  .control-btn {
    background: none;
    border: none;
    color: #fff;
    font-size: 1.5rem;
    cursor: pointer;
    transition: opacity 0.2s ease;
  }

  .control-btn:hover {
    opacity: 0.8;
  }

  .progress-bar {
    flex: 1;
    height: 6px;
    background: rgba(255, 255, 255, 0.3);
    border-radius: 3px;
    cursor: pointer;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    background: #3b82f6;
    border-radius: 3px;
    transition: width 0.1s ease;
  }

  .time-display {
    color: #fff;
    font-size: 0.85rem;
    font-weight: 500;
    min-width: 100px;
    text-align: center;
  }

  .volume-slider {
    width: 80px;
  }

  .video-info {
    padding: 16px;
  }

  .video-title {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0 0 8px 0;
    color: #e5e5e5;
  }

  .video-creator {
    font-size: 0.95rem;
    font-weight: 500;
    margin: 0 0 12px 0;
    color: #a3a3a3;
  }

  .video-description {
    font-size: 0.9rem;
    line-height: 1.6;
    color: #ccc;
    margin: 0;
    white-space: pre-wrap;
  }
</style>
