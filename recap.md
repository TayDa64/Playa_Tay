# Project Recap and Contributor Onboarding

This document provides a detailed summary of the work implemented in this repository to date. It’s intended to quickly orient maintainers, contributors, and AI agents to the repo’s current architecture, automation, CI, and active initiatives.

If you’re new here, skim the sections in order; if you’re returning, jump to the “What changed recently” and “Next steps” sections.

## What this repo is

- Framework: Tauri v2 monorepo (Rust backend + system webview frontend via WRY/TAO).
- Language mix: Rust for core/CLI/plugins; TypeScript/JS for tooling and frontend APIs.
- Primary runtime: Tauri. Electron packages are present for legacy/testing contexts only.
- Workspace: Managed via Cargo (Rust) and pnpm (Node). Root `package.json` provides helpful scripts.

Key root references:
- `crates/tauri` (core), `tauri-runtime(-wry)`, `tauri-utils` et al.
- Tooling: `crates/tauri-cli`, `packages/cli`, `packages/api` (JS API), bundler/build helpers.
- Examples: `examples/api` is used for CI smoke tests and headless runs.
- CI/automation: `.github/workflows/*` and `scripts/`.

## Phase 2 program: scope and status

We planned and executed a phased implementation for “Selective Electron” development and enterprise-readiness.

- Phase 2.1–2.3: Core automation and validation
  - Issue automation (YOLO) to open agent-ready issues from curated ticket files.
  - Supervisor to guide PRs/workflows, with re-run-once safeguards and labeling.
  - CI validation: build + run headless; artifact capture for visibility.
- Phase 2.4–2.6: Enterprise features & hardening (in progress)
  - RBAC, SSO, policy, audit/compliance, airgap patterns, admin flows.

Reference tickets live under `specs/001-selective-electron/PHASE_2_*_TICKETS.md` and the consolidated plan in `PHASE_2_6_PLAN.md`.

## Automation: YOLO workflows and helpers

Implemented a lightweight but powerful automation suite to enable “hands-off” contribution loops while remaining safe and traceable.

- Issue opener (workflow + script)
  - File: `.github/workflows/yolo-open-issues.yml`
  - Script: `scripts/yolo-open-issues.js`
  - Reads Phase ticket files and opens issues with required labels, model hints, and an embedded checklist. Supports dry runs and selective phase filters.

- Guidance comment bot
  - File: `.github/workflows/yolo-guidance-comment.yml`
  - Posts concise instructions and guardrails on newly opened YOLO-labeled issues. Helps agents conform to repo expectations (PR template, license headers, CI norms).

- PR Guard
  - File: `.github/workflows/pr-guard.yml`
  - Enforces PR body completeness using the repo’s PR template (linked issue, test plan, model info, checklist). Warns on unusually large diffs. Blocks merges until compliant.

- YOLO Supervisor (event-driven)
  - File: `.github/workflows/yolo-supervisor.yml`
  - Trigger: `workflow_run` on core CI/lint workflows and manual dispatch. No cron; purely event-driven for predictable behavior.
  - Behavior:
    - On failed runs: one-time auto-rerun, label with `needs-investigation`, and post guidance comments (including on linked issues when applicable).
    - On passing PRs: undraft to “Ready for review” and optionally automerge when the `automerge` label is present.
    - Applies helpful labels to aid triage and visibility.

## CI/CD: selective build, headless validation, artifacts

The main CI workflow focuses on validating the example app and proving the Tauri stack runs headless in CI, with tight dependencies and minimal network assumptions.

- File: `.github/workflows/ci.yml`
- Highlights:
  - Environment setup: Node + pnpm, Rust stable. Installs system dependencies including `libxdo-dev` (fixes `-lxdo` link), `libasound2t64` with fallback to `libasound2` for Ubuntu image incompatibilities, GTK/WebKit deps, and X11 tools.
  - JS build: builds `@tauri-apps/api` and a small Electron sidecar used for selective testing.
  - Frontend build: builds example `examples/api` frontend and wires assets via `tauri.ci.json`.
  - Rust checks: `cargo check` and targeted integration tests.
  - Tauri build: runs a no-bundle build to produce a runnable app for validation.
  - Headless run: launches the app under Xvfb, verifies main window events via logs, and captures a screenshot artifact using `xwd | convert` (ImageMagick). Artifact is uploaded for inspection.
  - Local download smoke test: serves a local file to avoid external networking and validates download behavior.

Outcomes:
- Reproducible headless launch verified in CI and in Codespaces.
- Screenshot artifact available on each CI run for visual regression sanity checks.

## Repository hygiene and contributor guardrails

- Link checks: `.github/copilot-instructions.md` updated to avoid markdown link validation failures by using plain URLs.
- PR Template: `.github/pull_request_template.md` requires:
  - Linked Issue (e.g., “Closes #123”)
  - Model used and prompt summary (for AI-driven changes)
  - Test Plan with explicit steps/logs/artifacts
  - Checklist (license headers, docs, lint/format, etc.)
- License headers: Ensure SPDX headers are present on new files in the appropriate comment style.

## Local development

- Short dev command
  - Dev run from repo root: `pnpm dev:host` (filters to `examples/api` project and runs `tauri dev`).
- Build command
  - Release bundle for example app: `pnpm --filter api tauri build`.
- Binary run (after build)
  - `./examples/api/src-tauri/target/release/api`.
- Disk management in Codespaces: heavy build caches redirected to `/tmp` to reduce workspace pressure. CI likewise avoids unnecessary cache bloat.

## Electron note

Electron packages (`packages/electron-shell`, `packages/electron-drm-shell`) exist for compatibility and testing. The primary runtime is Tauri; prefer Tauri-native implementations unless Electron is explicitly required for the task or comparison.

## Recent changes (high level)

- Planned Phases 2.1–2.6 and authored agent-ready tickets for each phase.
- Automated issue creation (YOLO) with dry-run and selective opening by phase.
- Event-driven YOLO Supervisor with one-time reruns, labels, guidance, and auto-undraft/automerge mechanics.
- Strengthened CI: added headless Xvfb launch, local download smoke test, and screenshot artifact upload.
- Fixed CI/system issues: `libxdo-dev` install for linking, `libasound2t64` fallback, and link-check errors from markdown.
- Validated example app runs headless both locally and in CI.
- Kicked off delegations for Phases 2.4–2.6 initial tasks (PRs expected to reference corresponding issues and satisfy PR Guard).

## What’s in progress / next steps

- Ensure issues for Phases 2.4–2.6 are open and correctly labeled. Use YOLO opener with `PHASES=2.4,2.5,2.6` if needed.
- Monitor incoming PRs for PR Guard and license header compliance; rely on Supervisor’s auto-rerun/guidance for transient failures.
- Implement enterprise features from Phase 2.6 plan: RBAC, SSO, policy-driven controls, admin audit & compliance, airgap-safe patterns.
- Expand tests around headless flows and optionally capture structured runtime logs as artifacts.

## Quick links and references

- Project site and docs: https://tauri.app and https://v2.tauri.app
- TAO (windowing): https://github.com/tauri-apps/tao
- WRY (webview): https://github.com/tauri-apps/wry
- Plugins: https://v2.tauri.app/develop/plugins/

## Contact and conventions

- Default branch: `dev`
- Use PR template religiously; PR Guard will block otherwise.
- Prefer Tauri-native solutions; Electron usage is for testing/compat only.
- Add SPDX headers to new files.

---

If you’re an AI agent: read `.github/copilot-instructions.md` first, follow the PR template, and include a minimal, reproducible test plan with logs or artifacts (e.g., the CI screenshot). If something fails in CI, the Supervisor will rerun once and then tag with `needs-investigation` with guidance. Use linked issues to maintain traceability.
