# Non-Regression Invariants

This document captures behaviors and contracts that **must not be broken** by future changes to the selective Electron integration.

## Command Contract (Backend API)

### `open_electron_feature(url: String) -> Result<(), String>`
Stable UI contract for opening features that may require Electron.

**Error codes** (returned as string with format `code: message`):
- `not_installed: <message>` — Electron runtime not available
- `spawn_error: <message>` — Failed to launch Electron process

**Success**: Returns `Ok(())` when Electron window launches successfully.

**Implementation location**: `examples/api/src-tauri/src/cmd.rs`

**UI integration**: Button in `examples/api/src/views/Welcome.svelte` labeled "Open Electron Feature (Pattern A/B)"

### Supporting Commands

#### `is_electron_available() -> Result<bool, String>`
Returns `true` if Electron binary is detected, `false` otherwise.

#### `ensure_electron_sidecar() -> Result<(), String>`
Dev-only helper. Returns `Ok(())` if binary exists, `Err("not_installed")` otherwise.

#### `launch_electron(url: String) -> Result<(), String>`
Internal Pattern A implementation. Same error codes as `open_electron_feature`.

## Security Posture

### Electron Window Configuration (Pattern A Sidecar)
The following security flags **must** be enforced in all Electron windows:
- `contextIsolation: true`
- `nodeIntegration: false`
- `sandbox: true`
- `devTools: false` in production (enabled only when `NODE_ENV !== 'production'`)

**Implementation location**: `packages/electron-shell/src/main.ts`

### Content Security Policy
A restrictive CSP must be applied via response headers:
```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data: https:;
connect-src 'self' https:;
```

### IPC and Authentication
- IPC between Tauri and Electron must use localhost-only connections
- Ephemeral authentication tokens passed via environment variables (`PLAYA_AUTH_TOKEN`)
- Unauthenticated requests must be rejected

## Build and Packaging

### Resource Inclusion
The sidecar entry point **must** be copied to and referenced from:
- **Path**: `examples/api/src-tauri/resources/electron-shell/main.js`
- **Config**: Listed in `bundle.resources` array in `tauri.conf.json`

### Build Hooks
- **Dev**: `beforeDevCommand` builds sidecar and copies to resources before starting dev server
- **Production**: `beforeBuildCommand` builds sidecar and copies to resources before Tauri build
- **CI**: Uses `tauri.ci.json` without build hooks; artifacts prebuilt in workflow steps

### CI Pipeline (`.github/workflows/ci.yml`)
The following steps must remain intact:
1. Install and build `@tauri-apps/api`
2. Install and build `packages/electron-shell`
3. Copy sidecar dist to `examples/api/src-tauri/resources/electron-shell/`
4. Build example frontend (`pnpm --filter api build`)
5. Run `cargo check` on example
6. Build with Rust Tauri CLI using `tauri.ci.json` (no-bundle mode)
7. Upload zipped artifacts: `dist` and `target`

## Workspace Constraints

### Cargo Workspace
- `Cargo.toml` must exclude problematic examples (e.g., `examples/multiwindow`) to keep `cargo metadata` stable

### Toolchain Pins
- Node: 22
- pnpm: 10.16.0
- Rust: ≥1.77.2

## UI Flow

### Error Handling
When `not_installed` is returned:
- Frontend must handle gracefully without crashing the host app
- UI should display a helpful prompt with guidance (e.g., install instructions)
- Optional retry mechanism after attempting `ensure_electron_sidecar()`

**Implementation location**: `examples/api/src/lib/electronFeature.js`

## Testing Requirements

### Minimum Validation
Before any release or major change:
1. Click "Open Electron Feature" button → Window launches (when Electron available)
2. Missing Electron → Returns `not_installed`, UI handles gracefully
3. `cargo check` passes for example app
4. Frontend builds without errors
5. CI workflow completes and produces artifacts
6. Security flags verified in sidecar window

### Integration Test (Planned)
A scripted test must verify:
- Invoking `open_electron_feature('https://example.com')` returns `Ok(())` or `not_installed`
- No test requires actual window rendering (CI may be headless)

## Violation Escalation
If any change requires breaking these invariants:
1. Document the reason and migration path in a spec amendment
2. Update this document with new invariants
3. Update root `spec.md` with the change
4. Ensure backward compatibility or provide clear upgrade instructions
