# Phase 1 — Manual Verification Checklist

Use this checklist to validate Phase 1 acceptance scenarios end-to-end before marking the phase complete.

## How to use
- Run steps on a developer machine with GUI; for CI/headless cases, see HEADLESS_CI.md and adjust accordingly.
- Record outcomes in the log section with date, environment, and result.

## Environment
- OS: Linux/macOS/Windows
- Rust: >= 1.77.2
- Node: 22; pnpm: 10.16.0
- Electron runtime deps (Linux): see HEADLESS_CI.md

## Scenarios

1) Launch on demand
- Action: Start dev host (Tauri). Click "Open Electron Feature (Pattern A/B)".
- Expected: Electron window opens loading configured URL when Electron is available.
- Evidence: No host crash; helper returns `{ ok: true }`.

2) Not installed handling
- Action: Ensure Electron sidecar is unavailable; invoke feature.
- Expected: Backend returns `not_installed`; UI shows helpful prompt and remains stable; optional retry path.

3) Packaging resource inclusion
- Action: Build using CI config or local equivalent with prebuilt sidecar.
- Expected: Resource exists at `examples/api/src-tauri/resources/electron-shell/main.js` and included in `bundle.resources`.

4) Security flags & CSP in production
- Action: Inspect `packages/electron-shell/src/main.ts` and run with NODE_ENV=production.
- Expected: `contextIsolation=true`, `nodeIntegration=false`, `sandbox=true`, devTools disabled, CSP header present.

## Optional: Headless validation
- Follow `HEADLESS_CI.md` for skipping spawn or using Xvfb.

## Log Template
- Date:
- Environment (OS, tool versions):
- Step 1 result:
- Step 2 result:
- Step 3 result:
- Step 4 result:
- Notes:
