# Quickstart — Selective Electron Integration (Phase 1)

This guide validates Pattern A (sidecar) end to end in dev and outlines packaging behavior.

## Prerequisites
- Rust ≥1.77.2
- Node 22, pnpm 10.16.0
- Linux: Electron runtime libs (libnss3, libnspr4, libgbm1, etc.) — see `HEADLESS_CI.md`

## Dev Run (Pattern A)
1. Install workspace deps
2. Build sidecar and start dev
3. Use UI to launch Electron feature on demand

Expected outcomes:
- If Electron available → new Electron window opens with target URL
- If not available → UI shows helpful prompt, host remains stable

## Commands to try (optional)
```bash
pnpm i
pnpm -w --filter @playa/api build
pnpm -w --filter @playa/electron-shell build
pnpm -w tauri dev
```

## Packaging (Pattern A)
- Sidecar entry is copied to `examples/api/src-tauri/resources/electron-shell/main.js`
- `tauri.conf.json` includes path in `bundle.resources`
- Build uses Rust Tauri CLI (see CI config `tauri.ci.json`)

## Troubleshooting
- Headless: See `HEADLESS_CI.md`; spawn tests may skip without DISPLAY
- Missing libs: Install listed packages; re-run
- Errors: Backend returns structured codes (`not_installed`, `spawn_error`)

## Files to inspect
- Backend: `examples/api/src-tauri/src/cmd.rs`, `examples/api/src-tauri/src/lib.rs`
- Frontend: `examples/api/src/views/Welcome.svelte`, `examples/api/src/lib/electronFeature.ts|.js`
- Sidecar: `packages/electron-shell/src/main.ts`
- Config: `examples/api/src-tauri/tauri.conf.json`, `examples/api/src-tauri/tauri.ci.json`
