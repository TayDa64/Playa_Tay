import { invoke } from '@tauri-apps/api/core'

export type OpenElectronFeatureResult =
  | { ok: true }
  | { ok: false; code: 'not_installed' | 'spawn_failed' | 'auth_failed' | 'denied' | 'unknown'; message?: string }

export async function openElectronFeature(url: string): Promise<OpenElectronFeatureResult> {
  try {
    await invoke('open_electron_feature', { url })
    return { ok: true }
  } catch (e: any) {
    const msg = typeof e === 'string' ? e : (e?.toString?.() ?? 'error')
    // Naive mapping for now; backend can specialize later
    if (msg.includes('not found')) return { ok: false, code: 'not_installed', message: msg }
    if (msg.includes('denied')) return { ok: false, code: 'denied', message: msg }
    if (msg.includes('auth')) return { ok: false, code: 'auth_failed', message: msg }
    if (msg.includes('spawn')) return { ok: false, code: 'spawn_failed', message: msg }
    return { ok: false, code: 'unknown', message: msg }
  }
}
