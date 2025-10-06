# Phase 2.1 – Agent-Ready Tickets (Foundation & Packaging)

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Run independent tickets in parallel; sequence those with explicit dependencies. Keep PRs focused, with passing build/tests and updated docs.

Conventions:
- Primary runtime: Tauri (Rust backend, system webview) with Electron compatibility where required in Pattern B
- TypeScript: avoid `any`; prefer `unknown` with guards; strict mode on
- Security: no secrets committed; use GitHub Encrypted Secrets
- CI: GitHub Actions, matrix builds, quality gates before deploy

---

## [Phase 2.1][Task 1] Electron Module Template and Builder Config

- Title: Create base Electron module template and cross-platform builder config
- Goal: Provide a secure, reproducible template for optional modules with Electron where applicable

Scope:
- Create new template package and scaffold with secure defaults
- Electron Builder config for macOS/Windows/Linux
- Minimal sample module entry points and assets

Out of scope:
- Code signing, notarization, or CI integration (covered in tasks 3–5)
- Update server or CDN wiring (tasks 6–7)

Paths:
- New: `packages/electron-module-template/`
- New: `packages/electron-module-template/electron-builder.yml`
- New: `packages/electron-module-template/src/{main.ts,preload.ts}`
- New: `packages/electron-module-template/src/security/{flags.ts,csp.ts,ipc-auth.ts}`
- New: `packages/electron-module-template/build/{entitlements.mac.plist,notarize.js}`
- Update: `pnpm-workspace.yaml` to include new package

Implementation notes:
- Enforce BrowserWindow webPreferences per plan (contextIsolation, sandbox, no nodeIntegration)
- Provide strict CSP helper; IPC token auth helper
- Provide npm scripts: build:mac, build:win, build:linux
- README with usage and security notes

Definition of Done:
- Template builds locally on all platforms (CI comes later)
- Security flags enforced by helper; CSP middleware exported
- Lint passes; no `any` types

Tests:
- TS unit tests for security helpers (flag enforcement, IPC token expiry)
- Minimal smoke test of preload exposure contract

Manual validation:
- Build DMG/NSIS/DEB artifacts locally; launch; verify devtools locked down per flags

Dependencies: none

Estimate: 2–3 days

Labels: phase-2.1, electron, packaging, security

Risk/Mitigation: Electron config drifts—lock via helper enforcing flags; add tests

Acceptance criteria:
- Reproducible builds; security helpers included; README published

---

## [Phase 2.1][Task 2] Platform Packaging Matrix and Scripts

- Title: Add cross-platform packaging scripts and matrix-ready npm tasks
- Goal: Prepare template for CI matrix without running CI yet

Scope:
- Add platform scripts and artifact output structure
- Validate platform-specific config keys in builder YAML

Out of scope:
- Signing/notarization and CI pipelines

Paths:
- Update: `packages/electron-module-template/package.json` (scripts)
- Update: `packages/electron-module-template/electron-builder.yml`
- New: `packages/electron-module-template/scripts/validate-config.ts`

Implementation notes:
- Provide `pnpm run build:{mac,win,linux}` and `build:all`
- Add config validator to ensure mandatory keys present per OS

Definition of Done:
- Local build produces artifacts in `dist/`
- Validator fails on missing critical config

Tests:
- Unit test for validator against fixtures

Manual validation:
- Run `build:all`; verify three artifact types exist

Dependencies: Task 1

Estimate: 1–2 days

Labels: phase-2.1, packaging

Acceptance criteria:
- Passing validator; artifacts built for all OS

---

## [Phase 2.1][Task 3] Code Signing Secrets and Local Tooling (Docs + Scripts)

- Title: Prepare signing key import/export scripts and documentation
- Goal: Enable secure acquisition and handling of signing materials without storing secrets

Scope:
- Scripts to import signing certs/keys during CI runtime (macOS p12, Windows Azure SignTool profile, Linux GPG)
- Documentation for obtaining certs and configuring GitHub Secrets

Out of scope:
- CI workflow wiring (Task 5)
- Actual cert procurement

Paths:
- New: `docs/signing/README.md`
- New: `tools/signing/{import-mac.sh, import-gpg.sh}`
- New: `tools/signing/azure-signing.json.example`

Implementation notes:
- macOS: keychain create/import; Windows: reference Azure SignTool inputs; Linux: GPG import with `--pinentry-mode loopback`
- Provide expected secret names for GitHub Actions

Definition of Done:
- Scripts run locally against dummy artifacts; docs complete

Tests:
- Dry-run mode for scripts validates environment variables

Manual validation:
- Run import scripts with placeholder files; confirm no secrets written to repo

Dependencies: none

Estimate: 1–2 days

Labels: phase-2.1, security, signing, docs

Acceptance criteria:
- Clear, tested scripts and docs; zero plaintext secret storage

---

## [Phase 2.1][Task 4] CI Signing Integration (macOS, Windows, Linux)

- Title: Wire signing steps into CI jobs
- Goal: Produce signed artifacts in CI across platforms

Scope:
- macOS: codesign + notarization step
- Windows: Azure Code Signing action integration
- Linux: GPG signing of packages/repo metadata

Out of scope:
- Broad CI matrix and quality gates (Task 8)

Paths:
- New: `.github/workflows/module-signing.yml`
- Update: `packages/electron-module-template/package.json` (sign scripts)
- Update: `tools/signing/*` if needed

Implementation notes:
- Use GitHub environments for prod vs staging secrets
- Upload signed artifacts to workflow artifacts (distribution later)

Definition of Done:
- CI run on tag builds produces signed artifacts for all OS

Tests:
- CI job logs show successful verify steps per-OS

Manual validation:
- Download artifacts; run `codesign --verify` / `signtool verify` / `dpkg-sig --verify`

Dependencies: Task 3

Estimate: 2–3 days

Labels: phase-2.1, ci, signing

Acceptance criteria:
- Signed artifacts verified; secrets confined to CI environment

---

## [Phase 2.1][Task 5] Base Module Security Framework Package

- Title: Implement security helpers as reusable package and integrate in template
- Goal: Enforce security posture consistently across modules

Scope:
- Implement `flags.ts`, `csp.ts`, `ipc-auth.ts` as a small internal library
- Integrate into template’s main/preload

Out of scope:
- App-level policy engine (later phases)

Paths:
- New: `packages/electron-security-kit/` (internal package)
- Update: `packages/electron-module-template/src/*` to use the kit
- Update: `pnpm-workspace.yaml` to include the kit

Implementation notes:
- No `any`; provide narrow IPC surfaces; short TTL tokens; revoke on window close

Definition of Done:
- Template compiles and uses security kit; lints pass

Tests:
- Unit tests for token expiry/revoke; CSP header injection

Manual validation:
- Launch template; confirm restricted prefs, CSP applied, IPC gated by token

Dependencies: Task 1

Estimate: 2 days

Labels: phase-2.1, security, typescript

Acceptance criteria:
- Security kit published internally; adopted by template

---

## [Phase 2.1][Task 6] Update Server Skeleton (Rust/Axum)

- Title: Create Rust update server crate with initial endpoints and metadata schema
- Goal: Lay backend foundation aligned with Tauri-first architecture

Scope:
- New crate with Axum server exposing:
  - GET `/api/modules/list`
  - GET `/api/modules/{id}/latest`
  - GET `/api/modules/{id}/download`
  - GET `/api/modules/{id}/changelog`
  - POST `/api/verification/signature`
- In-memory storage + JSON files; wiring for future DB
- Module metadata schema per plan

Out of scope:
- CDN, Cloudflare Worker, analytics (Task 7)

Paths:
- New: `crates/update-server/` (Rust crate)
- Update: workspace `Cargo.toml` to include crate
- New: `crates/update-server/README.md`

Implementation notes:
- Strong typing with serde; input validation; health endpoint `/healthz`
- Feature flags for future persistence

Definition of Done:
- `cargo run` serves endpoints; JSON schema validated; basic unit tests

Tests:
- Handler tests for list/latest/changelog; signature verify stub

Manual validation:
- Curl endpoints; verify JSON shape and status codes

Dependencies: none

Estimate: 3 days

Labels: phase-2.1, backend, rust, server

Acceptance criteria:
- Running server with defined endpoints and sample metadata

---

## [Phase 2.1][Task 7] CDN & Edge Worker Scaffolding (Cloudflare)

- Title: Provide edge worker scaffold and R2 integration stubs
- Goal: Prepare distribution layer without full production wiring

Scope:
- Cloudflare Worker skeleton with routes:
  - `/download/*` and `/verify/*`
- R2 client placeholders; logging and rate-limit hooks
- Config examples for regions

Out of scope:
- Production CDN accounts and secrets

Paths:
- New: `workers/cloudflare/worker.ts`
- New: `workers/cloudflare/wrangler.toml.example`
- New: `workers/cloudflare/README.md`

Implementation notes:
- Minimal KV for rate-limit; structured logs; return 404/429 correctly

Definition of Done:
- Local wrangler dev works; routes return placeholder responses

Tests:
- Unit tests for route handlers (miniflare)

Manual validation:
- `wrangler dev` and curl endpoints; confirm status behavior

Dependencies: Task 6 (for API shape alignment)

Estimate: 2 days

Labels: phase-2.1, cdn, edge, cloudflare

Acceptance criteria:
- Edge worker compiles and serves placeholder responses; docs included

---

## [Phase 2.1][Task 8] CI/CD Pipeline with Quality Gates

- Title: Add GitHub Actions workflows for build, test, sign, and gated deploy
- Goal: End-to-end automated pipeline with approvals and smoke tests

Scope:
- Build matrix (mac/win/linux, Node 20)
- Unit/integration/security scans
- License check; artifact signing; staged deploy job

Out of scope:
- Production CDN/upload (future phase 2.3/2.4)

Paths:
- New: `.github/workflows/build-modules.yml`
- New: `.github/workflows/test-and-scan.yml`
- Update: `packages/electron-module-template/package.json` (ci scripts)

Implementation notes:
- Use required checks; separate jobs for clarity; cache pnpm
- Manual approval gate for deploy

Definition of Done:
- All workflows green; artifacts attached; quality gates enforced

Tests:
- CI runs on PR and tag; failing tests block merge

Manual validation:
- Trigger workflow; verify artifacts, logs, and required checks

Dependencies: Tasks 2, 4, 5

Estimate: 2–3 days

Labels: phase-2.1, ci, quality-gates

Acceptance criteria:
- Reliable, repeatable pipeline with approvals and smoke tests

---

## Parallelization and Ordering

- Start in parallel: Tasks 1, 3, 6
- Then: Tasks 2, 5, 7 (dependent on earlier artifacts/interfaces)
- Last: Task 4, 8 (CI wiring and quality gates)

## Global Definition of Done for Phase 2.1
- Template builds for mac/win/linux with enforced security
- Signed artifacts produced in CI; verification steps pass
- Update server skeleton running with defined endpoints
- Edge worker scaffold ready for CDN integration
- CI pipelines green with quality gates and approvals
- Docs complete for signing and developer onboarding
