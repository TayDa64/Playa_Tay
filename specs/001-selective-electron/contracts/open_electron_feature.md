# Contract: open_electron_feature(url)

Stable backend command for launching Electron feature on demand.

## Signature
- Rust (Tauri command): `open_electron_feature(url: String) -> Result<(), String>`
- Frontend helper: `openElectronFeature(url: string, options?: { retry?: boolean }) => Promise<{ ok: boolean; code?: string; message?: string }>`

## Error Codes (string format `code: message`)
- `not_installed: ...` — Electron runtime not available
- `spawn_error: ...` — Spawn failed

## Behavior
- Decides Pattern A vs B internally (v1: Pattern A only, B scaffolded)
- On success: opens Electron window loading the provided URL
- On error: returns structured code; frontend surfaces prompt without crashing host

## Sources
- Implementation: `examples/api/src-tauri/src/cmd.rs`
- Registration: `examples/api/src-tauri/src/lib.rs`
- Frontend integration: `examples/api/src/lib/electronFeature.ts|.js`, `examples/api/src/views/Welcome.svelte`

## Related Commands
- `launch_electron(url: String)` — internal Pattern A sidecar spawn
- `is_electron_available() -> Result<bool, String>`
- `ensure_electron_sidecar() -> Result<(), String>` (dev helper)

## Tests
- `examples/api/src-tauri/tests/electron_integration.rs` (spawn test skips headless)
