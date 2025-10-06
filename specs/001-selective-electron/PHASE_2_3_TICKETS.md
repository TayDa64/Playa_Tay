# Phase 2.3 – Agent-Ready Tickets (Installation & Updates)

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Run independent tickets in parallel; sequence those with explicit dependencies. Keep PRs focused, with passing build/tests and updated docs.

Conventions:
- Primary runtime: Tauri v2 (Rust backend, system webview)
- Crates under `crates/`, packages under `packages/`, examples under `examples/`
- No `any` in TypeScript; strict typing in Rust and TS
- Integrity: strong hashing, signature verification before install

---

## [Phase 2.3][Task 1] HTTP Downloader Crate with Resume and Integrity

- Title: Implement `module-downloader` crate with resumable HTTP and integrity checks
- Goal: Reliably download large module archives with resume, rate-limit, and hash verification

Scope:
- New crate: `crates/module-downloader/`
- Features: HTTP range resume, checksum verification (SHA-256), max throughput limiter, retry with backoff, progress callback, temp-file semantics

Out of scope:
- Signature verification (Phase 2.2 crate handles this)

Paths:
- New: `crates/module-downloader/`
- Update: workspace `Cargo.toml`

Implementation notes:
- Use `reqwest` with streaming; persist partial `.part` files
- Hash as you stream; validate against expected checksum before finalize
- Public API returns a typed `DownloadResult` with stats

Definition of Done:
- Handles network interruption and resume; validates checksum; cleans up temp files

Tests:
- Simulate network drop; ensure resume works
- Mismatched checksum → error
- Rate limit honored (time-based assertion)

Manual validation:
- Download a large file from a test server; pause/resume; confirm integrity

Dependencies: none

Estimate: 3 days

Labels: phase-2.3, rust, networking

Acceptance criteria:
- Deterministic integrity and robust resume; no orphaned temp files

---

## [Phase 2.3][Task 2] Atomic Installer Crate with Staging and Rollback Guard

- Title: Implement `module-installer` with atomic staging directory and rollback
- Goal: Ensure installs are atomic, consistent, and recoverable

Scope:
- New crate: `crates/module-installer/`
- Staging area: extract to staging, verify, then atomic rename into place
- Rollback guard: snapshot previous version, revert on failure
- Pre/post install hooks (safe, constrained)

Out of scope:
- Delta application (Task 3)

Paths:
- New: `crates/module-installer/`

Implementation notes:
- Use platform-appropriate atomic moves; lock with file-based advisory lock
- Keep N previous versions; GC older safely

Definition of Done:
- Crash-safe mid-install; next run either fully old or fully new

Tests:
- Inject failure mid-extract → rollback to previous
- Lock contention test; parallel install blocked
- Hook timeout and failure handling

Manual validation:
- Install same module twice; see versioned directories and GC behavior

Dependencies: none

Estimate: 4 days

Labels: phase-2.3, rust, filesystem

Acceptance criteria:
- No partial installs; clean rollback; idempotent re-runs

---

## [Phase 2.3][Task 3] Delta Update Engine (BSDiff/Libbps) + Fallback Full

- Title: Implement `delta-engine` to generate and apply binary patches
- Goal: Reduce bandwidth/time via deltas with correct fallback

Scope:
- New crate: `crates/delta-engine/`
- Generate/apply deltas between versions; verify final checksum; fallback to full download on failure

Out of scope:
- CDN integration (Phase 2.4)

Paths:
- New: `crates/delta-engine/`

Implementation notes:
- Bindings to `bsdiff`/`bspatch` or pure-Rust equivalent; memory-safe wrapper
- Patch manifest with block checksums; streaming apply where possible

Definition of Done:
- Patch apply speed and integrity validated; safe fallback

Tests:
- Apply valid delta → final hash match
- Corrupted delta → fallback path chosen and succeeds

Manual validation:
- Measure delta size vs full; record metrics in README

Dependencies: none

Estimate: 4 days

Labels: phase-2.3, rust, performance, updates

Acceptance criteria:
- Correctness > performance; never leaves broken install

---

## [Phase 2.3][Task 4] Update Manager Orchestrator (Plan → Download → Verify → Install)

- Title: Implement `update-manager` crate orchestrating the full update flow
- Goal: Provide a single entry point to perform updates with progress and rollback

Scope:
- New crate: `crates/update-manager/`
- Orchestrates: query server → choose delta/full → download (with resume) → signature verify → install via atomic installer → rollback on failure
- Progress events and structured logs

Out of scope:
- UI; CDN specifics

Paths:
- New: `crates/update-manager/`

Implementation notes:
- Compose `module-downloader`, `signature-verifier`, `delta-engine`, `module-installer`
- Expose async API and a channel-based progress stream

Definition of Done:
- Handles all error paths; never leaves broken install; provides clear results

Tests:
- Happy path full + delta
- Simulated network fail with resume
- Verification failure → abort before install
- Mid-install failure → rollback

Manual validation:
- Use sample server/fixtures to run end-to-end update

Dependencies: Tasks 1–3

Estimate: 4 days

Labels: phase-2.3, orchestrator, rust

Acceptance criteria:
- Safe, idempotent, well-logged update flow

---

## [Phase 2.3][Task 5] Tauri Plugin: Updates (Commands + Events)

- Title: Expose update manager via Tauri plugin with typed commands/events
- Goal: Frontend can request and observe updates safely

Scope:
- New crate: `crates/tauri-plugin-updates/`
- Commands: `checkUpdates`, `downloadUpdate`, `applyUpdate`, `cancelUpdate`
- Events: `updateProgress`, `updateAvailable`, `updateError`

Out of scope:
- UI components (Task 7)

Paths:
- New: `crates/tauri-plugin-updates/`
- New: `crates/tauri-plugin-updates/README.md`

Implementation notes:
- Map errors to stable codes; guard concurrent operations; cancellation token support

Definition of Done:
- Example under `examples/updates/` can check, download, and apply

Tests:
- Command tests with mocks; event stream behavior; cancellation

Manual validation:
- Run example; observe progress and final state

Dependencies: Task 4

Estimate: 3 days

Labels: phase-2.3, tauri, plugin

Acceptance criteria:
- Stable command API; example works end-to-end

---

## [Phase 2.3][Task 6] Update UI Components (React/TS) + SDK

- Title: Implement frontend components and SDK wrappers for update flows
- Goal: Provide reusable UI to initiate and observe updates

Scope:
- New package: `packages/updates-sdk/`
- Components: `UpdateBanner`, `UpdateModal`, `ProgressBar`, `ChangelogView`
- Hooks: `useUpdateManager()` for state machine + events

Out of scope:
- Backend plugins; this is UI layer only

Paths:
- New: `packages/updates-sdk/{src,package.json,tsconfig.json}`

Implementation notes:
- Strong typing; persistent state across reloads; debounce progress events

Definition of Done:
- Storybook stories (optional) or example page demonstrating flows

Tests:
- Hook unit tests; component rendering; event handling

Manual validation:
- Demo page shows update check → download → apply → restart prompt

Dependencies: Task 5

Estimate: 3 days

Labels: phase-2.3, typescript, react, ui

Acceptance criteria:
- Clean UX; resilient to event storms; no `any`

---

## [Phase 2.3][Task 7] Rollback and Version Pinning Policies

- Title: Add policy support for rollback triggers and version pinning
- Goal: Control update behavior via config/policy

Scope:
- Extend `update-manager` with:
  - Auto-rollback on crash count or health-check failure
  - Optional version pin constraints
- Expose config via plugin and SDK

Out of scope:
- Enterprise org policies (Phase 2.6)

Paths:
- Update: `crates/update-manager/src/*`
- Update: `crates/tauri-plugin-updates/src/*`
- Update: `packages/updates-sdk/src/*`

Implementation notes:
- Store last-known-good; detect startup crashes count via marker files

Definition of Done:
- Configurable thresholds; verified by tests

Tests:
- Simulated crash loops → rollback
- Version pin violates → update skipped with clear reason

Manual validation:
- Toggle pin; attempt update to blocked version; see correct behavior

Dependencies: Tasks 4–6

Estimate: 2 days

Labels: phase-2.3, policy, reliability

Acceptance criteria:
- Predictable rollback; clear policy enforcement messaging

---

## [Phase 2.3][Task 8] Telemetry and Logs for Update Pipeline

- Title: Add structured logs and telemetry across downloader, installer, delta, and manager
- Goal: Diagnose issues and measure success/failure rates

Scope:
- Add tracing spans and fields (download bytes, resume count, hash ms, patch ms)
- Optional opt-in telemetry events batched and sent to server (stub)

Out of scope:
- Production analytics backend (Phase 2.4)

Paths:
- Update: `crates/*` involved in updates to include `tracing`/`tracing-subscriber`
- New: shared `crates/update-telemetry/` (optional)

Implementation notes:
- Respect privacy; redact PII; feature flag to disable

Definition of Done:
- Logs are structured and helpful; telemetry stubs compile

Tests:
- Assert logs include fields in integration tests (where feasible)

Manual validation:
- Run with RUST_LOG=info; inspect spans and fields

Dependencies: Tasks 1–4

Estimate: 2 days

Labels: phase-2.3, observability

Acceptance criteria:
- Actionable logs; minimal overhead when disabled

---

## Parallelization and Ordering

- Start in parallel: Tasks 1 (Downloader), 2 (Installer), 3 (Delta)
- Then: Task 4 (Manager)
- Then: Task 5 (Plugin) and 8 (Telemetry)
- Then: Task 6 (UI)
- Finally: Task 7 (Policies)

## Global Definition of Done for Phase 2.3
- Downloader resumes and verifies; Installer is atomic with rollback; Delta engine accurate with safe fallback
- Update Manager orchestrates end-to-end; Plugin exposes stable API; UI kit enables UX integration
- Policies for rollback/version pinning; telemetry/logs for diagnosis; all tests green in CI
