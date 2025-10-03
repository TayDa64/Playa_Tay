import { invoke } from '@tauri-apps/api/core'

export async function openElectronFeature(url) {
  try {
    await invoke('open_electron_feature', { url })
    return { ok: true }
  } catch (e) {
    const msg = typeof e === 'string' ? e : (e && e.toString ? e.toString() : 'error')
    if (msg.includes('not_installed') || msg.includes('not found')) {
      // Attempt a simple ensure step (dev-only helper). Real installer logic would go here for Pattern B.
      try {
        await invoke('ensure_electron_sidecar')
      } catch {}
      // Retry once
      try {
        await invoke('open_electron_feature', { url })
        return { ok: true }
      } catch (e2) {
        const m2 = typeof e2 === 'string' ? e2 : (e2 && e2.toString ? e2.toString() : 'error')
        return { ok: false, code: 'not_installed', message: m2 }
      }
    }
    if (msg.includes('denied')) return { ok: false, code: 'denied', message: msg }
    if (msg.includes('auth')) return { ok: false, code: 'auth_failed', message: msg }
    if (msg.includes('spawn')) return { ok: false, code: 'spawn_failed', message: msg }
    return { ok: false, code: 'unknown', message: msg }
  }
}
