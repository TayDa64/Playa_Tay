# Phase 2.2 – Agent-Ready Tickets (Detection & Registry)

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Run independent tickets in parallel; sequence those with explicit dependencies. Keep PRs focused, with passing build/tests and updated docs.

Conventions:
- Primary runtime: Tauri v2 (Rust backend, system webview); leverage the Tauri plugin pattern
- New Rust crates live under `crates/`; new JS/TS packages under `packages/`
- TypeScript: no `any` (prefer `unknown` + guards); strict mode on
- Security-by-default: signature verification mandatory, least-privileged FS access
- CI: GitHub Actions, include unit/integration tests where possible

---

## [Phase 2.2][Task 1] Module Registry Crate (SQLite + LRU Cache)

- Title: Implement `module-registry` crate with schema, DAO, and LRU cache
- Goal: Provide a fast, embedded registry service for detected/verified modules

Scope:
- Create crate `crates/module-registry/` with:
  - SQLite schema and migrations (sqlx)
  - DAO layer (insert/update/get/list/search)
  - LRU read-through cache for hot lookups
  - Basic metrics (hit/miss, query latency)

Out of scope:
- Signature verification (Task 2)
- Tauri commands / UI integration (Tasks 3–4)

Paths:
- New: `crates/module-registry/`
- New: `crates/module-registry/migrations/*.sql`
- Update: workspace `Cargo.toml` members

Implementation notes:
- Use `sqlx` with offline mode; add feature flag for `bundled` sqlite if needed
- Cache keyed by `(module_id, version)` and by `module_id` latest
- Provide `Registry` trait with async methods: `upsert_module`, `get_module`, `list_modules`, `search`, `mark_orphaned`

Definition of Done:
- `cargo test` green with DAO + cache tests
- Bench: cached `get_module` <= 1ms on dev laptop (informal)
- Migrations applied automatically on init

Tests:
- Migration test (create, migrate, rollback)
- Cache eviction test (capacity N → evict LRU)
- CRUD + search tests with sample data

Manual validation:
- Run a tiny example binary in the crate that seeds and queries modules; verify output

Dependencies: none

Estimate: 3 days

Labels: phase-2.2, rust, sqlite, performance

Acceptance criteria:
- Stable API; persistent storage; measurable cache hit/miss metrics

Risk/Mitigation:
- Locking/contention → use single-writer pattern + connections pool; wrap in `Arc`

---

## [Phase 2.2][Task 2] Signature Verifier Crate (Ed25519 + Trust Store)

- Title: Implement `signature-verifier` crate with Ed25519 verification and trust store
- Goal: Cryptographically validate module packages/manifests

Scope:
- New crate with:
  - Detached signature verification for module archives and manifests
  - Trust store (public keys) with versioning and key rotation
  - Revocation list support
  - API for verifying bytes/paths; returns structured result and reasons

Out of scope:
- Registry persistence (Task 1)
- UI integration (Task 4)

Paths:
- New: `crates/signature-verifier/`
- New: `crates/signature-verifier/README.md`

Implementation notes:
- Use `ed25519-dalek` and `sha2` for hashing
- Trust store loaded from app data dir or env override; format: JSON w/ kid, algorithm, issued_at, expires_at
- Provide `Verifier::verify_archive(archive_path, sig_path) -> VerifyResult`

Definition of Done:
- Unit tests cover valid, invalid, tampered, expired, revoked keys
- Bench: verification overhead documented (ms range)

Tests:
- Golden vectors; adversarial cases; key rotation

Manual validation:
- CLI demo in examples verifies a sample archive/signature pair

Dependencies: none

Estimate: 2 days

Labels: phase-2.2, security, crypto

Acceptance criteria:
- Deterministic verification with actionable error messages and codes

Risk/Mitigation:
- Time-based expiry drift → allow N-min skew; document

---

## [Phase 2.2][Task 3] Tauri Plugin: Registry & Detection Commands

- Title: Create `tauri-plugin-registry` exposing detection/registry commands and events
- Goal: Provide app-facing API via Tauri commands and event streams

Scope:
- New crate `crates/tauri-plugin-registry/` with Tauri v2 plugin:
  - Commands: `registry_list`, `registry_get`, `registry_search`, `module_verify`, `registry_refresh`
  - Event: `module_detected` (emitted when new module discovered)
  - Uses `module-registry` + `signature-verifier`

Out of scope:
- Frontend SDK (Task 4)
- Background scheduling (Task 5)

Paths:
- New: `crates/tauri-plugin-registry/`
- New: `crates/tauri-plugin-registry/src/lib.rs`
- New: `crates/tauri-plugin-registry/README.md`
- Update: workspace `Cargo.toml`

Implementation notes:
- Commands strictly typed; validate inputs; map errors to well-defined codes
- Emit `module_detected` only after successful signature verify + registry upsert
- Add an example app under `examples/registry/` showing basic usage

Definition of Done:
- Plugin compiles; example runs and can list/refresh registry

Tests:
- Command-level tests with mocked registry/verifier

Manual validation:
- Run example; trigger refresh pointing to a sample directory; see event + listing

Dependencies: Tasks 1–2

Estimate: 3 days

Labels: phase-2.2, tauri, plugin, rust

Acceptance criteria:
- Stable commands; consistent error mapping; example demonstrates end-to-end

Risk/Mitigation:
- Blocking IO on command thread → spawn tasks; use async FS reads

---

## [Phase 2.2][Task 4] Frontend TypeScript SDK + React Hooks

- Title: Implement `registry-sdk` TS package wrapping Tauri commands with types
- Goal: Provide ergonomic, typed frontend access with React helpers

Scope:
- New package `packages/registry-sdk/` with:
  - Typed functions: `listModules`, `getModule`, `searchModules`, `verifyModule`, `refreshRegistry`
  - React hooks: `useRegistry`, `useModule(id)`, `useRegistryEvents()`
  - Strict types; no `any`; runtime guards for `unknown`

Out of scope:
- App UI pages (later phases)

Paths:
- New: `packages/registry-sdk/{src,index.ts,tsconfig.json,package.json}`
- New: `packages/registry-sdk/src/types.ts`
- New: `packages/registry-sdk/src/hooks.ts`
- Update: `pnpm-workspace.yaml`

Implementation notes:
- Use `@tauri-apps/api` for bridge calls
- Event subscription to `module_detected` with cleanup

Definition of Done:
- Package builds; d.ts emitted; eslint clean

Tests:
- Unit tests for type guards and hook behavior (mock Tauri API)

Manual validation:
- Sample app page calling SDK, rendering list; verify events update state

Dependencies: Task 3

Estimate: 2 days

Labels: phase-2.2, typescript, sdk, react

Acceptance criteria:
- Stable API; hooks update on events; no `any`

Risk/Mitigation:
- Event storms → debounce in hook; document pattern

---

## [Phase 2.2][Task 5] Background Scanner & Reconciliation Job

- Title: Implement background scanner that discovers modules and reconciles registry
- Goal: Keep registry fresh by scanning configured directories periodically

Scope:
- Within `tauri-plugin-registry` add:
  - Configurable scan roots
  - Manifest parsing (`module.json` or similar), hash, version extraction
  - Signature verification callout; upsert to registry
  - Reconciliation: mark orphaned, cleanup stale entries

Out of scope:
- Auto-install/update (later phases)

Paths:
- Update: `crates/tauri-plugin-registry/src/*`
- New: `crates/tauri-plugin-registry/src/scanner.rs`
- New: config struct with serialization

Implementation notes:
- Schedule via tokio interval; backoff on errors
- Concurrency cap; avoid scanning user home recursively; opt-in directories only

Definition of Done:
- Scanner runs; logs metrics; respects cancellation on app quit

Tests:
- Temp directory scenario: add/remove modules; verify registry reflects state

Manual validation:
- Point scanner at a sample dir; observe events and registry updates

Dependencies: Tasks 1–3

Estimate: 3 days

Labels: phase-2.2, rust, background-jobs

Acceptance criteria:
- Accurate detection with verified signatures and consistent registry state

Risk/Mitigation:
- Large directories → ignore patterns; depth limits; concurrency control

---

## [Phase 2.2][Task 6] Registry CLI (Inspect, Import Keys, Refresh)

- Title: Create `registry-cli` for admin/dev workflows
- Goal: Provide headless controls: inspect registry, import keys, trigger refresh

Scope:
- New crate `crates/registry-cli/` with commands:
  - `list`, `get <id>`, `search <query>`
  - `import-key <path>` to trust store
  - `refresh [--path <dir>]`

Out of scope:
- Organization-level policies (Phase 2.6)

Paths:
- New: `crates/registry-cli/`

Implementation notes:
- Use `clap`; JSON output option for scripting
- Reuse crates: `module-registry`, `signature-verifier`

Definition of Done:
- Binary builds; help text clear; returns correct exit codes

Tests:
- CLI tests with temp dirs and fixtures

Manual validation:
- Run `registry-cli list` after scanner populates registry; verify output

Dependencies: Tasks 1–2

Estimate: 2 days

Labels: phase-2.2, cli, tooling

Acceptance criteria:
- Useful for CI and local debugging; stable JSON schema

Risk/Mitigation:
- Cross-platform FS paths → use dirs-next for app data locations

---

## [Phase 2.2][Task 7] End-to-End Integration Tests (Detect → Verify → Persist)

- Title: Add E2E tests covering the detection/verification/registry flow
- Goal: Ensure the complete pipeline works under realistic conditions

Scope:
- Workspace-level tests using a sample module fixture
- Start plugin (or call library functions) to simulate detection
- Verify signature pass/fail propagation and registry state

Out of scope:
- UI rendering tests

Paths:
- New: `tests/phase_2_2/` with Rust integration tests
- New: `tests/phase_2_2/fixtures/` sample module + signatures

Implementation notes:
- Use temp dirs; isolate sqlite db per test
- Useful logs on failure; assert metrics expectations

Definition of Done:
- E2E tests passing in CI reliably (< 5 min)

Tests:
- Valid module → present in registry
- Tampered module → rejected, error logged
- Key rotation → old signature rejected, new accepted

Manual validation:
- Run the tests locally; inspect generated logs/db when failing

Dependencies: Tasks 1–5

Estimate: 3 days

Labels: phase-2.2, testing, integration

Acceptance criteria:
- Deterministic outcomes; flake-free in CI; helpful failure output

Risk/Mitigation:
- Flakiness → retry small IO sections; deterministic fixtures

---

## [Phase 2.2][Task 8] Documentation and Examples

- Title: Write developer docs and example app for registry/detection
- Goal: Ensure easy adoption and contributor onboarding

Scope:
- Docs covering: registry schema, trust store, plugin commands, SDK usage
- Example app under `examples/registry/` showing list/search/verify flows

Out of scope:
- Marketplace UI (Phase 2.5)

Paths:
- New: `docs/registry/README.md`
- Update: `crates/tauri-plugin-registry/README.md`
- New: `examples/registry/` (minimal Tauri app sample)

Implementation notes:
- Include troubleshooting (permissions, missing keys)
- Provide telemetry notes and privacy considerations

Definition of Done:
- Docs accurate and actionable; example runs end-to-end locally

Tests:
- N/A (docs), but example should compile in CI

Manual validation:
- Follow docs from clean checkout; complete in < 30 minutes

Dependencies: Tasks 3–5

Estimate: 2 days

Labels: phase-2.2, docs, examples

Acceptance criteria:
- Clear getting-started; working example; diagrams included where helpful

Risk/Mitigation:
- Docs drift → link to code; keep snippets tested via example build

---

## Parallelization and Ordering

- Start in parallel: Tasks 1 (Registry), 2 (Verifier)
- Then: Task 3 (Plugin) uses 1–2; Task 6 (CLI) uses 1–2
- Then: Task 5 (Scanner) builds on 3; Task 4 (SDK) on 3
- Finally: Task 7 (E2E), Task 8 (Docs/Examples)

## Global Definition of Done for Phase 2.2
- Registry crate persists and serves module data with cache and metrics
- Signature verifier validates modules with robust error reporting
- Tauri plugin exposes typed commands/events; example app demonstrates usage
- Background scanner keeps registry current with verified modules only
- CLI enables headless workflows; SDK enables frontend integration
- E2E tests green in CI; docs and examples enable onboarding
