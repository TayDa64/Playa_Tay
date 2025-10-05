# Phase 0 — Outline & Research: Selective Electron Integration (Pattern A/B)

## Objective
Establish feasibility, constraints, unknowns, and best practices for a Tauri-first app with selective Electron usage. Produce references and risks to inform Phase 1 contracts and the data model.

## Scope
- Pattern A: Electron sidecar launched on-demand; bundled as minimal resource in packaging
- Pattern B: Optional separate module installed on demand (deferred for v1)
- Host app remains fully functional without Electron

## Key Unknowns and Open Questions
- DRM/Widevine provider, licensing, and distribution requirements (Pattern B)
- Module hosting, code signing, and update channels (Pattern B)
- Install/upgrade UX permissions and rollback strategy (Pattern B)
- Headless Linux e2e validation strategy for Electron spawn
- Platform-specific runtime deps for Electron (Linux packages, Windows/macOS notes)
- IPC auth hardening (token lifetime, rotation, scope)

See deferred decisions in: `specs/001-selective-electron/spec.md` (Deferred Decisions section).

## Best Practices and Constraints
- Security posture for Electron windows:
  - contextIsolation=true
  - nodeIntegration=false
  - sandbox=true
  - devTools disabled in production
- Restrictive Content Security Policy for Electron sessions
- IPC via localhost only with ephemeral token; reject unauthenticated traffic
- Tauri resource packaging: copy sidecar entry to `examples/api/src-tauri/resources/electron-shell/`
- Deterministic CI: prebuild sidecar, no Electron dependency in host renderer, use `tauri.ci.json` for build
- Headless guidance: prefer skipping UI rendering in CI; use Xvfb optionally

## References (local)
- `specs/001-selective-electron/NON_REGRESSION.md`
- `specs/001-selective-electron/HEADLESS_CI.md`
- `specs/001-selective-electron/PROGRESS.md`
- `packages/electron-shell/` (Pattern A)
- `packages/electron-drm-shell/README.md` (Pattern B outline)

## References (external)
- https://www.electronjs.org/docs/latest/tutorial/security
- https://v2.tauri.app/

## Evidence Collected (Phase 0 outcomes)
- Non-regression invariants documented (command contracts, security posture, CI, packaging)
- Headless CI strategy documented with Linux deps and Xvfb guidance
- Initial `PROGRESS.md` tracks P0/P1/P2 tasks; P0 tasks completed

## Risks and Mitigations
- DRM licensing unclear → defer Pattern B; document decisions before implementation
- CI flakiness with displays → skip UI rendering by default; optional Xvfb job
- Security regressions → invariants doc; automated checks where possible (lint/tests)

## Outcome
Phase 0 deliverables produced and referenced. Ready to proceed to Phase 1 (Design & Contracts): define entities, finalize backend command contract, and prepare quickstart.
