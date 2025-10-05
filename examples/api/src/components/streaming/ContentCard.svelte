<script>
  import { createEventDispatcher } from 'svelte'

  let { content } = $props()
  const dispatch = createEventDispatcher()

  function formatDuration(seconds) {
    if (!seconds) return ''
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = seconds % 60
    
    if (hours > 0) {
      return `${hours}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
    }
    return `${minutes}:${String(secs).padStart(2, '0')}`
  }

  function formatViewCount(count) {
    if (!count) return ''
    if (count >= 1000000) {
      return `${(count / 1000000).toFixed(1)}M views`
    }
    if (count >= 1000) {
      return `${(count / 1000).toFixed(1)}K views`
    }
    return `${count} views`
  }

  function handlePlay() {
    dispatch('play', content)
  }

  function handleAddToQueue() {
    dispatch('queue', content)
  }
</script>

<div class="content-card">
  <div class="thumbnail-container" onclick={handlePlay}>
    {#if content.thumbnail}
      <img src={content.thumbnail} alt={content.title} class="thumbnail" />
    {:else}
      <div class="thumbnail-placeholder">
        <span class="i-ph-play-circle text-4xl"></span>
      </div>
    {/if}
    {#if content.duration}
      <span class="duration-badge">{formatDuration(content.duration)}</span>
    {/if}
    <div class="play-overlay">
      <span class="i-ph-play-circle-fill text-6xl"></span>
    </div>
  </div>
  
  <div class="content-info">
    <h3 class="content-title" title={content.title}>{content.title}</h3>
    <div class="content-meta">
      {#if content.creator}
        <span class="creator">{content.creator}</span>
      {/if}
      {#if content.view_count}
        <span class="views">{formatViewCount(content.view_count)}</span>
      {/if}
    </div>
    
    <div class="actions">
      <button class="btn btn-sm btn-primary" onclick={handlePlay}>
        <span class="i-ph-play"></span>
        Play
      </button>
      <button class="btn btn-sm btn-secondary" onclick={handleAddToQueue}>
        <span class="i-ph-queue"></span>
        Queue
      </button>
    </div>
  </div>
</div>

<style>
  .content-card {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    cursor: pointer;
  }

  .content-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
  }

  .thumbnail-container {
    position: relative;
    width: 100%;
    padding-top: 56.25%; /* 16:9 aspect ratio */
    background: #0a0a0a;
    overflow: hidden;
  }

  .thumbnail {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .thumbnail-placeholder {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #666;
  }

  .duration-badge {
    position: absolute;
    bottom: 8px;
    right: 8px;
    background: rgba(0, 0, 0, 0.8);
    color: #fff;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .play-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.4);
    opacity: 0;
    transition: opacity 0.2s ease;
    color: #fff;
  }

  .thumbnail-container:hover .play-overlay {
    opacity: 1;
  }

  .content-info {
    padding: 12px;
  }

  .content-title {
    font-size: 0.95rem;
    font-weight: 600;
    margin: 0 0 8px 0;
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #e5e5e5;
  }

  .content-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 12px;
    font-size: 0.85rem;
    color: #a3a3a3;
  }

  .creator {
    font-weight: 500;
  }

  .actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .btn-sm {
    padding: 4px 10px;
    font-size: 0.8rem;
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
</style>
