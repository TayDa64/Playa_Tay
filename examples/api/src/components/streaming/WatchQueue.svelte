<script>
  import { createEventDispatcher } from 'svelte'

  let { items = [] } = $props()
  const dispatch = createEventDispatcher()

  function handlePlay(item) {
    dispatch('play', item.content)
  }

  function handleRemove(item) {
    dispatch('remove', item.id)
  }

  function formatDuration(seconds) {
    if (!seconds) return ''
    const minutes = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${minutes}:${String(secs).padStart(2, '0')}`
  }
</script>

<div class="watch-queue">
  <h3 class="queue-title">
    <span class="i-ph-queue"></span>
    Watch Queue ({items.length})
  </h3>

  {#if items.length === 0}
    <div class="empty-state">
      <span class="i-ph-queue text-4xl opacity-30"></span>
      <p>No videos in queue</p>
      <p class="text-sm">Add videos to watch later</p>
    </div>
  {:else}
    <div class="queue-items">
      {#each items as item (item.id)}
        <div class="queue-item">
          <button class="item-thumbnail" onclick={() => handlePlay(item)}>
            {#if item.content.thumbnail}
              <img src={item.content.thumbnail} alt={item.content.title} />
            {:else}
              <div class="thumbnail-placeholder">
                <span class="i-ph-play-circle"></span>
              </div>
            {/if}
            {#if item.content.duration}
              <span class="duration">{formatDuration(item.content.duration)}</span>
            {/if}
          </button>

          <div class="item-info">
            <h4 class="item-title" onclick={() => handlePlay(item)}>
              {item.content.title}
            </h4>
            {#if item.content.creator}
              <p class="item-creator">{item.content.creator}</p>
            {/if}
          </div>

          <button class="remove-btn" onclick={() => handleRemove(item)}>
            <span class="i-ph-x"></span>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .watch-queue {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 16px;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .queue-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 1.1rem;
    font-weight: 600;
    margin: 0 0 16px 0;
    color: #e5e5e5;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: #666;
    padding: 32px 16px;
  }

  .empty-state p {
    margin: 0;
  }

  .queue-items {
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow-y: auto;
  }

  .queue-item {
    display: flex;
    gap: 12px;
    background: #0a0a0a;
    border: 1px solid #2a2a2a;
    border-radius: 6px;
    padding: 8px;
    transition: background 0.2s ease;
  }

  .queue-item:hover {
    background: #1e1e1e;
  }

  .item-thumbnail {
    position: relative;
    width: 120px;
    min-width: 120px;
    height: 68px;
    background: #000;
    border: none;
    border-radius: 4px;
    overflow: hidden;
    cursor: pointer;
    padding: 0;
  }

  .item-thumbnail img {
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
    color: #666;
    font-size: 2rem;
  }

  .duration {
    position: absolute;
    bottom: 4px;
    right: 4px;
    background: rgba(0, 0, 0, 0.8);
    color: #fff;
    padding: 2px 4px;
    border-radius: 2px;
    font-size: 0.7rem;
    font-weight: 600;
  }

  .item-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow: hidden;
  }

  .item-title {
    font-size: 0.9rem;
    font-weight: 500;
    margin: 0;
    color: #e5e5e5;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    cursor: pointer;
    transition: color 0.2s ease;
  }

  .item-title:hover {
    color: #3b82f6;
  }

  .item-creator {
    font-size: 0.8rem;
    color: #a3a3a3;
    margin: 0;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #666;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 4px;
    transition: color 0.2s ease;
  }

  .remove-btn:hover {
    color: #ef4444;
  }

  .text-sm {
    font-size: 0.85rem;
  }

  .text-4xl {
    font-size: 2.5rem;
  }

  .opacity-30 {
    opacity: 0.3;
  }
</style>
