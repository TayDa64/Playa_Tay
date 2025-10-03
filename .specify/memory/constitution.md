<!--
Sync Impact Report
Version change: 0.0.0 → 1.0.0
Modified principles: Template placeholders → Concrete principles P1–P5
Added sections: "Additional Constraints & Implementation Guide", "Development Workflow & Review Gates"
Removed sections: None
Templates requiring updates:
	✅ .specify/templates/plan-template.md (Constitution Check + version/path reference)
	✅ .specify/templates/spec-template.md (reviewed, no changes needed)
	✅ .specify/templates/tasks-template.md (reviewed, no changes needed)
	N/A .specify/templates/commands/* (directory not present)
Deferred TODOs:
	- TODO(RATIFICATION_DATE): original adoption date unknown; set when governance approves.
-->

# Playa_Tay Constitution

## Core Principles

### P1. Tauri‑First with Selective Electron
Playa_Tay applications MUST default to a Tauri/WebView runtime. Electron
is permitted only for narrowly scoped, renderer‑only capabilities using one of
two sanctioned patterns:
- Pattern A (Sidecar Window): A minimal Electron shell lives under
	`packages/electron-shell` and is spawned as a child process by the Tauri host
	only on demand. IPC is restricted to localhost with an ephemeral auth token.
- Pattern B (Optional Module): A separately packaged Electron micro‑shell is
	distributed and updated independently. The Tauri host detects presence,
	authenticates a localhost channel, and enables the feature at runtime.

Rationale: Preserve Tauri’s small footprint, security model, and performance
while enabling niche web capabilities (e.g., DRM/Widevine, exotic extension
APIs) without bloating the base app.

Acceptance rules:
- The base app MUST be fully usable without Electron present.
- Electron processes MUST be launched only through explicit Tauri commands with
	explicit URLs and a per‑session token.
- No direct Electron renderer Node APIs; `nodeIntegration=false`,
	`contextIsolation=true`, `sandbox=true`.
- Do not embed Electron except as in Pattern A resources or Pattern B module.

### P2. Security‑by‑Default (Tauri and Electron)
Both runtimes MUST enforce strong isolation and least privilege:
- Renderers: `contextIsolation=true`, `nodeIntegration=false`, `sandbox=true`,
	devtools disabled in production, remote module disabled.
- Content Security Policy: strict allowlist; no `eval`; no remote content
	unless explicitly justified and reviewed.
- IPC: localhost only with ephemeral token or mTLS; reject unauthenticated
	traffic; rotate tokens per session; validate origin and intent.
- Supply chain: lockfiles checked in; `cargo audit`/Dependabot/Renovate enabled;
	prompt CVE updates for Electron/WebView/OS SDKs.

Rationale: Constrain attack surface across native and web layers.

### P3. Reproducible Monorepo Builds
Preserve the tauri‑style monorepo layout and deterministic builds:
- Structure MUST remain under `crates/`, `packages/`, `examples/` with
	workspace manifests managed at the root.
- Node tooling: pnpm 10.16.0 via Corepack activation; no global postinstall
	side effects; scripts run with `pnpm -w/-r`.
- Rust toolchain: Rust ≥ 1.77.2 as specified by the workspace; `cargo` builds
	remain cacheable; no unpinned native build steps.
- CI and local builds produce identical artifacts given the same lockfiles.

Rationale: Ensure predictable builds and simple CI.

### P4. Tests and Quality Gates (Non‑Negotiable)
- New features MUST include tests proportionate to risk: unit for logic,
	integration for Tauri↔Electron sidecar flows, and e2e when user journeys are
	affected.
- Lint/format gates MUST pass (`prettier`, ESLint, `cargo fmt`, `cargo clippy`).
- Constitution checks MUST be performed in planning and post‑design (see
	Development Workflow section).

Rationale: Prevent regressions and maintain reliability.

### P5. Modularity, Packaging, and Versioning
- Packaging options: (A) embed Electron sidecar assets as Tauri resources, or
	(B) distribute Electron module separately with its own signed updater.
- Keep the base installer minimal. Pattern B is preferred when the feature is
	truly optional.
- Use semantic versioning for modules. Breaking changes to sidecar IPC contracts
	require a coordinated rollout and migration notes.

Rationale: Keep footprint small and upgrades safe.

## Additional Constraints & Implementation Guide

- Where to put things in this repo:
	- `packages/electron-shell`: minimal Electron sidecar package (Pattern A)
	- Tauri host for dev: `examples/api` or `examples/streaming`
	- Root scripts may include:
		- `dev:host`: `pnpm --filter api tauri dev`
		- `dev:electron`: `pnpm --filter @playa/electron-shell dev`
		- `build:electron`: `pnpm --filter @playa/electron-shell build`
- Workspace coherence:
	- Install/build with `pnpm -r install` and `pnpm -r build`.
	- Align Node tooling via Corepack; respect pinned pnpm `10.16.0` and Rust
		`1.77.2+`.
- Electron usage guidelines (2025 posture):
	- Keep Electron sandboxed, disable remote module, use strict protocols, avoid
		`file://` loads from untrusted paths, and promptly update for CVEs.
- IPC contract (Pattern A/B):
	- Localhost only, ephemeral auth token, per‑session rotation, and explicit
		allowlist of commands.
- Optional DRM/Widevine note: If required, confine DRM playback to the Electron
	shell, launched only for that page, with strict isolation. The base app MUST
	remain Widevine‑free.

## Development Workflow & Review Gates

Constitution Checks (apply in plans and reviews):
- Tauri‑first: No direct Electron dependency in the host renderer; Electron only
	via Pattern A/B.
- Security: Isolation flags set; CSP enforced; no Node exposure to remote
	content; devtools off in production.
- Monorepo: Paths and manifests stay within `crates/`, `packages/`, `examples/`;
	pnpm and Rust versions respected.
- Tests: Sidecar IPC covered by integration tests; format/lint gates pass.
- Packaging: Base installer remains minimal; Pattern A resources or Pattern B
	module documented.

Review Process:
- Design docs must list which pattern (A or B) is used and why.
- Any exceptions require a written rationale under Complexity Tracking in the
	plan and an approval from maintainers.

## Governance

This constitution supersedes prior informal practices for runtime selection and
packaging. Amendments require:
- A proposal PR updating this file with version bump and rationale (MAJOR/MINOR/
	PATCH based on semantic impact).
- Approval from maintainers and a migration note for any affected modules.
- LAST_AMENDED_DATE set to the merge date; RATIFICATION_DATE preserves the
	original adoption date.

Compliance:
- All PRs and reviews MUST verify Constitution Checks in planning and design.
- CI MUST enforce format/lint and run tests for changed areas. Electron changes
	MUST pass platform builds on supported OSes.

Versioning Policy:
- MAJOR: Incompatible governance changes or removal/redefinition of principles.
- MINOR: New principles/sections or materially expanded guidance.
- PATCH: Clarifications, wording, or non‑semantic refinements.

**Version**: 1.0.0 | **Ratified**: TODO(RATIFICATION_DATE): unknown; set upon adoption | **Last Amended**: 2025-10-03
