# Tasks: Selective Electron Integration (Pattern A/B)

Derived from `./spec.md` and `./plan.md` with product details (personas, platforms, modules).

## Conventions
- Status: [NS]=Not Started, [IP]=In Progress, [DN]=Done
- Priority: P0 (blocker), P1 (important), P2 (nice-to-have)
- Owning Area: FE (frontend), BE (tauri host), Sidecar (electron-shell), CI, Docs, QA

## P0 — Core functionality and safety
1. [NS][P0][BE] Harden backend command contract
   - Freeze `open_electron_feature(url: String)` error codes (`ok`, `not_installed`, `spawn_error`) and document in README.
   - Add structured error mapping in `examples/api/src-tauri/src/cmd.rs`.
   - Add minimal unit tests for error code mapping.
2. [NS][P0][Sidecar] Enforce security flags in electron-shell
   - Ensure BrowserWindow has `contextIsolation: true`, `sandbox: true`, `nodeIntegration: false`; disable devtools when `NODE_ENV=production`.
   - Add a basic CSP header/meta in the loaded page (if applicable).
3. [NS][P0][CI] Deterministic CI build (already present — finalize)
   - Keep `tauri.ci.json` build path; ensure artifacts upload includes `dist` and `target`.
   - Add cache steps for pnpm and cargo to speed up builds.
4. [NS][P0][Docs] Non-regression invariants doc
   - Mirror items from root `spec.md` into a `NON_REGRESSION.md` section under `specs/001-selective-electron/` or README.

## P1 — Developer experience and guardrails
5. [NS][P1][FE] Improve UI flow on not_installed
   - Show a modal with guidance (install sidecar deps; link to docs); retry action.
   - Telemetry placeholder: local log only.
6. [NS][P1][BE] Add `is_electron_available()` usage to preflight
   - Frontend calls availability check at startup (optional) to grey-out action if missing.
7. [NS][P1][CI] Headless guidance
   - Add docs for Xvfb or skip-UI strategy; optional job matrix variant enabling Xvfb.
8. [NS][P1][QA] Spawn integration test
   - Scripted test: invoke `open_electron_feature('https://example.com')` and assert `Ok(())` or `not_installed` in CI (no window assertion required).

## P2 — Pattern B groundwork
9. [NS][P2][Sidecar] Create `packages/electron-drm-shell` README and placeholder main
   - Document Pattern B scope, installer/update expectations, and security posture.
10. [NS][P2][BE] Design branching for Pattern B in `open_electron_feature`
    - Add stub detection for external module and TODO comment linking to spec.
11. [NS][P2][Docs] Update `spec.md` to resolve [NEEDS CLARIFICATION] placeholders as decisions arrive.

## Acceptance checklist mapping
- Button launches Electron window in dev/prod where available → Tasks 1, 2, 8
- Not installed flow handled gracefully → Tasks 1, 5, 6
- Packaged resource unaffected → Task 3
- Security flags enforced → Task 2
- CI artifacts available → Task 3

## Stretch (optional)
- Add an example DRM-safe page or provider stub under `packages/electron-shell` for demonstration-only purposes.
- Provide platform-specific notes (Windows/macOS) in docs for Electron prerequisites.
