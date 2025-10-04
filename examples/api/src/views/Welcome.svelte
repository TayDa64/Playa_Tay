<script>
  import { invoke } from '@tauri-apps/api/core'
  import { openElectronFeature, isElectronAvailable } from '../lib/electronFeature.js'
  import {
    getName,
    getVersion,
    getTauriVersion,
    getBundleType
  } from '@tauri-apps/api/app'
  import { onMount } from 'svelte'

  let { onMessage } = $props()

  let version = $state('1.0.0')
  let tauriVersion = $state('1.0.0')
  let appName = $state('Unknown')
  let bundleType = $state('Unknown')
  let electronAvailable = $state(null) // null=checking, true=available, false=not available
  let showModal = $state(false)
  let modalMessage = $state('')
  let isRetrying = $state(false)

  getName().then((n) => {
    appName = n
  })
  getVersion().then((v) => {
    version = v
  })
  getTauriVersion().then((v) => {
    tauriVersion = v
  })
  getBundleType().then((b) => {
    if (b) {
      bundleType = b
    }
  })

  onMount(async () => {
    // Preflight check: is Electron available?
    electronAvailable = await isElectronAvailable()
  })

  function contextMenu() {
    invoke('plugin:app-menu|popup')
  }

  async function openElectron() {
    const url = 'https://example.com'
    const res = await openElectronFeature(url)
    if (res.ok) {
      onMessage({ status: 'ok', action: 'openElectronFeature', url })
    } else {
      const code = 'code' in res ? res.code : undefined
      const message = 'message' in res ? res.message : undefined

      // Show modal for not_installed errors
      if (code === 'not_installed') {
        modalMessage = message || 'Electron runtime is not installed.'
        showModal = true
      }

      onMessage({ status: 'error', action: 'openElectronFeature', url, code, message })
    }
  }

  async function handleRetry() {
    isRetrying = true
    showModal = false

    // Re-check availability
    electronAvailable = await isElectronAvailable()

    if (electronAvailable) {
      // Try opening again
      await openElectron()
    } else {
      modalMessage = 'Electron is still not available. Please install dependencies manually.'
      showModal = true
    }

    isRetrying = false
  }

  function closeModal() {
    showModal = false
  }
</script>

<div class="grid gap-8 justify-items-start">
  <p>
    This is a demo of Tauri's API capabilities using the <code
      >@tauri-apps/api</code
    > package. It's used as the main validation app, serving as the test bed of our
    development process. In the future, this app will be used on Tauri's integration
    tests.
  </p>
  <pre>
    App name: <code>{appName}</code>
    App version: <code>{version}</code>
    Tauri version: <code>{tauriVersion}</code>
    Bundle type: <code>{bundleType}</code>
  </pre>

  <button class="btn" onclick={contextMenu}>Context menu</button>
  <button
    class="btn"
    onclick={openElectron}
    disabled={electronAvailable === false}
    title={electronAvailable === false ? 'Electron runtime not available' : ''}
  >
    Open Electron Feature (Pattern A/B)
    {#if electronAvailable === null}
      <span class="text-xs">(checking...)</span>
    {:else if electronAvailable === false}
      <span class="text-xs">(unavailable)</span>
    {/if}
  </button>
</div>

{#if showModal}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={closeModal}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h3 class="modal-title">Electron Runtime Not Available</h3>
      <p class="modal-message">{modalMessage}</p>
      <div class="modal-instructions">
        <p><strong>To install dependencies:</strong></p>
        <code>pnpm -F @playa/electron-shell install</code>
        <p class="text-sm mt-2">Then click Retry below.</p>
      </div>
      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={closeModal} disabled={isRetrying}>Close</button>
        <button class="btn btn-primary" onclick={handleRetry} disabled={isRetrying}>
          {isRetrying ? 'Retrying...' : 'Retry'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
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

  .btn-secondary {
    background: #444;
    color: #fff;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #555;
  }

  .btn-primary {
    background: #3b82f6;
    color: #fff;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .text-xs {
    font-size: 0.75rem;
    opacity: 0.7;
  }

  .text-sm {
    font-size: 0.875rem;
  }

  .mt-2 {
    margin-top: 0.5rem;
  }
</style>
