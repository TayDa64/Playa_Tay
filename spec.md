# Playa Tay – Application Specification Plan

This document is the top-level specification for Playa Tay. It articulates current implementation status, locks in non-regression invariants, and points to detailed feature specs and plans maintained under `specs/`.

**For the broader application vision and architecture blueprint, see [VISION.md](./VISION.md).**

## Overview
- Tauri-first desktop app with selective Electron usage for niche capabilities (e.g., DRM) without bloating or weakening the base app.
- Minimal, secure core; advanced modules are opt-in and isolated.
- CI-driven builds ensure determinism on Linux, with portability for macOS/Windows.

## Non‑regression invariants (must remain working)
These behaviors and assets exist in the repo today and must not be broken by future changes.

- UI trigger
  - A button in `examples/api/src/views/Welcome.svelte` labeled “Open Electron Feature (Pattern A/B)” invokes the backend to open the feature.
- Stable backend contract (Tauri commands)
  - Commands defined and registered in `examples/api/src-tauri/src/cmd.rs` and `examples/api/src-tauri/src/lib.rs`:
    - `open_electron_feature(url: String)` – stable UI contract
    - `launch_electron(url: String)` – Pattern A sidecar spawner
    - `is_electron_available()` – availability check
    - `ensure_electron_sidecar()` – dev helper for missing runtime
- Sidecar packaging
  - Sidecar entry copied to `examples/api/src-tauri/resources/electron-shell/main.js` and referenced in `examples/api/src-tauri/tauri.conf.json` under `bundle.resources`.
- Build & CI
  - CI workflow `.github/workflows/ci.yml` prebuilds `@tauri-apps/api`, builds `packages/electron-shell`, copies sidecar resources, builds the example frontend, runs `cargo check`, then builds using Rust Tauri CLI with `examples/api/src-tauri/tauri.ci.json` and uploads zipped artifacts.
- Workspace constraints
  - `Cargo.toml` workspace excludes problematic examples (e.g., `examples/multiwindow`) to keep `cargo metadata` and builds stable in CI.
- Security posture (Pattern A sidecar)
  - Electron windows launched by the sidecar enforce `nodeIntegration=false`, `contextIsolation=true`, and `sandbox=true` with devtools disabled in production.

If any change risks these invariants, capture it in a dedicated spec with migration notes and update this list explicitly.

## Scope and goals (v1)
- Deliver a minimal but polished host app proving selective Electron integration via one user-facing feature.
- Preserve small installer footprint; keep Electron optional.
- Establish clear contracts (invoke commands, IPC, resource packaging) and a CI pipeline producing artifacts.

## Out of scope (v1)
- Full Pattern B (separately installed Electron module with signed updates) beyond scaffolding.
- Cross-platform installers and code signing.
- Rich settings, identity, and data persistence beyond demo needs.

## Acceptance criteria
- Clicking the “Open Electron Feature (Pattern A/B)” button launches an Electron window loading the configured URL.
- If Electron is unavailable in dev, the backend returns a `not_installed` code and the UI handles it gracefully without crashing the host.
- Packaging includes the minimal sidecar asset without pulling Electron into the main renderer bundle.
- CI builds pass and produce zipped artifacts of the example build outputs.
- Security flags listed above are enforced for the Electron window.

## Detailed specs and plans
- Selective Electron (Pattern A/B)
  - Spec: `specs/001-selective-electron/spec.md`
  - Plan: `specs/001-selective-electron/plan.md`

## Open questions
- DRM/Widevine distribution and licensing model for production.
- Pattern B installer/update channel, signing, and hosting.
- Headless Linux strategies (Xvfb/Wayland) for CI e2e tests of Electron windows.

## Next steps
- Refine `specs/001-selective-electron/spec.md` by resolving `[NEEDS CLARIFICATION]` items.
- Generate/align `plan.md` tasks and derive a `tasks.md` for execution.
- Add minimal integration tests to validate sidecar spawn and non-regression of the UI command contract.
