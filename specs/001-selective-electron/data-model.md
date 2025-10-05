# Data Model — Selective Electron Integration (Phase 1)

## Entities

### ElectronFeature
Represents a feature that requires Electron.
- url: string — target URL to load
- pattern: "A" | "B" — Pattern A (sidecar) or Pattern B (module)
- status: "available" | "not_installed" | "running"

Notes:
- Pattern defaults to "A" in v1; Pattern B is scaffolded only.
- Status is derived from backend availability checks and runtime state.

### SidecarAuth
Ephemeral token shared across Tauri/Electron sessions for localhost IPC authorization.
- token: string — short-lived random value
- issuedAt: number (epoch ms)
- ttlMs: number — time-to-live

Notes:
- Pass via environment (e.g., PLAYA_AUTH_TOKEN) to the sidecar process.
- Reject unauthenticated requests.

## Source of Truth (current implementation)
- Backend commands and spawn logic: `examples/api/src-tauri/src/cmd.rs`
- Command registration: `examples/api/src-tauri/src/lib.rs`
- Frontend integration: `examples/api/src/views/Welcome.svelte`
- Helper module: `examples/api/src/lib/electronFeature.ts|.js`
- Sidecar security flags: `packages/electron-shell/src/main.ts`

## Derivations and Validation
- status = available → `is_electron_available()` returns true
- status = not_installed → `is_electron_available()` returns false (or ensure fails)
- status = running → `open_electron_feature(url)` returned Ok(()) and process is active

## Open Questions (deferred)
- Persistence of last-run feature state (out of scope for v1)
- Token rotation and multi-window scope (deferred)
