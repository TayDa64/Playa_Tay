import { invoke } from '@tauri-apps/api/core'

/**
 * Check if Electron runtime is available
 */
export async function isElectronAvailable(): Promise<boolean> {
  try {
    return await invoke('is_electron_available')
  } catch (e: unknown) {
    console.error('Failed to check Electron availability:', e)
    return false
  }
}

/**
 * Attempt to ensure Electron sidecar is installed (dev-only helper)
 */
export async function ensureElectronSidecar(): Promise<{ ok: boolean; message?: string }> {
  try {
    await invoke('ensure_electron_sidecar')
    return { ok: true }
  } catch (e: unknown) {
    const msg = typeof e === 'string' ? e : (e && typeof e === 'object' && 'toString' in e && typeof e.toString === 'function' ? e.toString() : 'error')
    return { ok: false, message: msg }
  }
}

export type OpenElectronFeatureResult = { ok: boolean; code?: string; message?: string }

export interface OpenElectronFeatureOptions {
  retry?: boolean
}

/**
 * Open a feature that requires Electron (Pattern A/B)
 */
export async function openElectronFeature(
  url: string, 
  options: OpenElectronFeatureOptions = { retry: true }
): Promise<OpenElectronFeatureResult> {
  try {
    await invoke('open_electron_feature', { url })
    return { ok: true }
  } catch (e: unknown) {
    const msg = typeof e === 'string' ? e : (e && typeof e === 'object' && 'toString' in e && typeof e.toString === 'function' ? e.toString() : 'error')

    // Parse structured error codes from backend
    if (msg.includes('not_installed')) {
      if (options.retry) {
        // Attempt ensure step (dev-only helper)
        const ensureResult = await ensureElectronSidecar()
        if (ensureResult.ok) {
          // Retry once without further retries
          const retryResult = await openElectronFeature(url, { retry: false })
          return retryResult
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
