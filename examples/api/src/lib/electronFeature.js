import { invoke } from '@tauri-apps/api/core'

/**
 * Check if Electron runtime is available
 * @returns {Promise<boolean>}
 */
export async function isElectronAvailable() {
  try {
    return await invoke('is_electron_available')
  } catch (e) {
    console.error('Failed to check Electron availability:', e)
    return false
  }
}

/**
 * Attempt to ensure Electron sidecar is installed (dev-only helper)
 * @returns {Promise<{ok: boolean, message?: string}>}
 */
export async function ensureElectronSidecar() {
  try {
    await invoke('ensure_electron_sidecar')
    return { ok: true }
  } catch (e) {
    const msg = typeof e === 'string' ? e : (e && e.toString ? e.toString() : 'error')
    return { ok: false, message: msg }
  }
}

/**
 * Open a feature that requires Electron (Pattern A/B)
 * @param {string} url - Target URL to load
 * @param {object} options - Options
 * @param {boolean} options.retry - Whether to retry after ensure attempt
 * @returns {Promise<{ok: boolean, code?: string, message?: string}>}
 */
export async function openElectronFeature(url, options = { retry: true }) {
  try {
    await invoke('open_electron_feature', { url })
    return { ok: true }
  } catch (e) {
    const msg = typeof e === 'string' ? e : (e && e.toString ? e.toString() : 'error')

    // Parse structured error codes from backend
    if (msg.includes('not_installed')) {
      if (options.retry) {
        // Attempt ensure step (dev-only helper)
        const ensureResult = await ensureElectronSidecar()
        if (ensureResult.ok) {
          // Retry once without further retries
          return await openElectronFeature(url, { retry: false })
        }
      }
      return {
        ok: false,
        code: 'not_installed',
        message: 'Electron runtime not found. Please install dependencies.'
      }
    }

    if (msg.includes('spawn_error')) {
      return { ok: false, code: 'spawn_error', message: 'Failed to launch Electron process.' }
    }

    if (msg.includes('denied')) {
      return { ok: false, code: 'denied', message: 'Permission denied.' }
    }

    return { ok: false, code: 'unknown', message: msg }
  }
}
