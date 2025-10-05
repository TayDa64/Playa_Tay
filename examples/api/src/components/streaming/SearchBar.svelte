<script>
  import { createEventDispatcher } from 'svelte'

  let { placeholder = 'Search videos...', loading = false } = $props()
  const dispatch = createEventDispatcher()

  let query = $state('')
  let debounceTimer = null

  function handleInput(event) {
    query = event.target.value
    
    // Clear existing timer
    if (debounceTimer) {
      clearTimeout(debounceTimer)
    }

    // Debounce search by 300ms
    debounceTimer = setTimeout(() => {
      if (query.trim()) {
        dispatch('search', query.trim())
      }
    }, 300)
  }

  function handleSubmit(event) {
    event.preventDefault()
    if (debounceTimer) {
      clearTimeout(debounceTimer)
    }
    if (query.trim()) {
      dispatch('search', query.trim())
    }
  }
</script>

<form class="search-bar" onsubmit={handleSubmit}>
  <div class="search-input-container">
    <span class="search-icon i-ph-magnifying-glass"></span>
    <input
      type="text"
      class="search-input"
      {placeholder}
      value={query}
      oninput={handleInput}
      disabled={loading}
    />
    {#if loading}
      <span class="loading-spinner i-ph-spinner animate-spin"></span>
    {/if}
  </div>
</form>

<style>
  .search-bar {
    width: 100%;
    max-width: 600px;
  }

  .search-input-container {
    position: relative;
    display: flex;
    align-items: center;
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 8px;
    transition: border-color 0.2s ease;
  }

  .search-input-container:focus-within {
    border-color: #3b82f6;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    color: #666;
    font-size: 1.2rem;
  }

  .search-input {
    width: 100%;
    padding: 12px 40px;
    background: transparent;
    border: none;
    outline: none;
    color: #e5e5e5;
    font-size: 0.95rem;
  }

  .search-input::placeholder {
    color: #666;
  }

  .search-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-spinner {
    position: absolute;
    right: 12px;
    color: #3b82f6;
    font-size: 1.2rem;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
