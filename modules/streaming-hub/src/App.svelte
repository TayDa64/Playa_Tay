<script lang="ts">
  // M1 Streaming Hub - Main Application
  // Phase 1 (Foundation): Stub UI with basic structure
  // Performance target: <500ms module initialization

  import { onMount } from 'svelte';
  import { getWatchHistory, getWatchQueue, getRecommendations } from '$lib/ipc/commands';
  import type { WatchHistoryEntry, WatchQueueEntry, Recommendation } from '$lib/ipc/commands';

  // State
  let loading = $state(true);
  let error = $state<string | null>(null);
  let history = $state<WatchHistoryEntry[]>([]);
  let queue = $state<WatchQueueEntry[]>([]);
  let recommendations = $state<Recommendation[]>([]);

  // Initialize module
  onMount(async () => {
    try {
      console.log('Initializing M1 Streaming Hub...');

      // Load initial data in parallel
      const [historyData, queueData, recommendationsData] = await Promise.all([
        getWatchHistory(10),
        getWatchQueue(),
        getRecommendations(),
      ]);

      history = historyData;
      queue = queueData;
      recommendations = recommendationsData;

      console.log(`Loaded: ${history.length} history, ${queue.length} queue, ${recommendations.length} recommendations`);
    } catch (err) {
      console.error('Failed to initialize:', err);
      error = String(err);
    } finally {
      loading = false;
    }
  });
</script>

<main class="h-screen w-screen bg-gray-50 overflow-hidden">
  {#if loading}
    <!-- Loading state -->
    <div class="flex items-center justify-center h-full">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
        <p class="text-gray-600">Loading Streaming Hub...</p>
      </div>
    </div>
  {:else if error}
    <!-- Error state -->
    <div class="flex items-center justify-center h-full">
      <div class="card max-w-md">
        <div class="text-error text-4xl mb-4">⚠️</div>
        <h2 class="text-xl font-bold mb-2">Failed to Initialize</h2>
        <p class="text-gray-600 mb-4">{error}</p>
        <button
          class="btn-primary"
          onclick={() => window.location.reload()}
        >
          Retry
        </button>
      </div>
    </div>
  {:else}
    <!-- Main application layout -->
    <div class="flex flex-col h-full">
      <!-- Header -->
      <header class="bg-white border-b border-gray-200 px-6 py-4">
        <h1 class="text-2xl font-bold text-gray-900">Streaming Hub</h1>
        <p class="text-sm text-gray-500">Phase 1 (Foundation) - Basic Structure</p>
      </header>

      <!-- Content area -->
      <div class="flex-1 overflow-y-auto p-6">
        <div class="max-w-7xl mx-auto space-y-6">
          <!-- Watch Queue Section -->
          <section class="card">
            <h2 class="text-xl font-bold mb-4">Watch Queue ({queue.length})</h2>
            {#if queue.length > 0}
              <div class="space-y-2">
                {#each queue as item}
                  <div class="flex items-center gap-4 p-3 bg-gray-50 rounded-lg">
                    <div class="flex-shrink-0 w-10 h-10 bg-gray-300 rounded flex items-center justify-center">
                      <span class="text-gray-600 font-mono text-sm">{item.position + 1}</span>
                    </div>
                    <div class="flex-1">
                      <p class="font-medium">{item.title}</p>
                      <p class="text-sm text-gray-500">{item.channel} • {item.provider}</p>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-gray-500">Your watch queue is empty</p>
            {/if}
          </section>

          <!-- Recommendations Section -->
          <section class="card">
            <h2 class="text-xl font-bold mb-4">For You ({recommendations.length})</h2>
            {#if recommendations.length > 0}
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {#each recommendations as rec}
                  <div class="border border-gray-200 rounded-lg p-4">
                    <div class="w-full aspect-video bg-gray-300 rounded mb-3"></div>
                    <p class="font-medium mb-1">{rec.title}</p>
                    <p class="text-sm text-gray-500 mb-2">{rec.channel}</p>
                    {#if rec.reasoning}
                      <p class="text-xs text-gray-400 italic">{rec.reasoning}</p>
                    {/if}
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-gray-500">No recommendations yet. Watch some videos to get personalized suggestions!</p>
            {/if}
          </section>

          <!-- Watch History Section -->
          <section class="card">
            <h2 class="text-xl font-bold mb-4">Watch History ({history.length})</h2>
            {#if history.length > 0}
              <div class="space-y-2">
                {#each history as entry}
                  <div class="flex items-center gap-4 p-3 bg-gray-50 rounded-lg">
                    <div class="flex-1">
                      <p class="font-medium">{entry.title}</p>
                      <p class="text-sm text-gray-500">{entry.channel} • {entry.provider}</p>
                      <div class="flex items-center gap-2 mt-1">
                        <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                          <div
                            class="bg-primary h-1.5 rounded-full"
                            style={`width: ${entry.completion_percent}%`}
                          ></div>
                        </div>
                        <span class="text-xs text-gray-500">{entry.completion_percent}%</span>
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-gray-500">No watch history yet</p>
            {/if}
          </section>
        </div>
      </div>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', system-ui, -apple-system, sans-serif;
  }
</style>
