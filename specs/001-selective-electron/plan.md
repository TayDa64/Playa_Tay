# Checklist Vigilance & Completion Instructions

Before marking any phase or item as complete, follow this process:

1. **Locate All Relevant Checklists:**
	- Check for progress tracking, review, and acceptance checklists in all related files (e.g., `plan.md`, `spec.md`, `tasks.md`, and agent-specific docs).
	- Review both markdown checkboxes and any checklist tables or lists.

2. **Verify Deliverables:**
	- For each checklist item, confirm that all required deliverables (contracts, data models, quickstart guides, code, tests, etc.) are present and meet the acceptance criteria.
	- Cross-reference acceptance scenarios and functional requirements to ensure nothing is missed.

3. **Document Evidence:**
	- When possible, link to or summarize the location of each deliverable (file path, section, or commit).
	- Note any open questions, TODOs, or deferred clarifications.

4. **Only Check Off When Complete:**
	- Do not mark a checklist item as complete until all evidence is reviewed and deliverables are verified.
	- If in doubt, leave the item unchecked and document what is missing or unclear.

5. **Communicate Status:**
	- When updating checklists, communicate the status to the team or coding agent, especially if blockers or ambiguities remain.

**Be vigilant:** Checklists may exist in multiple files and sections. Always search and review all relevant documentation before updating progress.
# Implementation Plan: Selective Electron Integration (Pattern A/B)

**Branch**: `[001-selective-electron]` | **Date**: 2025-10-03 | **Spec**: `./spec.md`
**Input**: Feature specification from `/specs/001-selective-electron/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
3. Fill the Constitution Check section based on the constitution document.
4. Evaluate Constitution Check section below
5. Execute Phase 0 → research.md
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific file
7. Re-evaluate Constitution Check section
8. Plan Phase 2 → Describe task generation approach
9. STOP - Ready for /tasks command
```

## Summary
Tauri-first with selective Electron. Pattern A: Electron sidecar launched on-demand from `packages/electron-shell`; bundled as a minimal resource. Pattern B: optional separate module, installed on demand with signed updates. A single backend command `open_electron_feature` keeps the UI stable.

## Technical Context
**Language/Version**: Rust 1.77.2, Node 22, pnpm 10.16.0
**Primary Dependencies**: Tauri (workspace crates), Electron (sidecar), @tauri-apps/api
**Storage**: N/A
**Testing**: cargo test, Vite/Svelte build checks; integration tests for Tauri↔Electron spawn
**Target Platform**: Linux/macOS/Windows (CI initially Ubuntu)
**Project Type**: monorepo (crates/, packages/, examples/)
**Performance Goals**: Minimal overhead; launch sidecar only on demand
**Constraints**: Tauri-first; security flags enforced; small base installer
**Scale/Scope**: Sidecar only for niche capabilities (e.g., DRM/Widevine)

## Constitution Check
- Tauri‑first: Electron only via Pattern A/B with a sidecar/module. PASS
- Security posture: contextIsolation=true, nodeIntegration=false, sandbox=true; CSP allowlists; devtools off in prod. PASS
- Monorepo coherence: keep `crates/`, `packages/`, `examples/`; pinned pnpm and Rust toolchain. PASS
- Tests & gates: add sidecar spawn integration test; lint/format gates. PARTIAL (tests TODO)
- Packaging: base installer minimal; sidecar as resource in Pattern A. PASS

## Project Structure
```
specs/001-selective-electron/
├── spec.md
└── plan.md

examples/api/
├── src/
├── src-tauri/
│   ├── src/cmd.rs               # open_electron_feature + launch_electron
│   ├── tauri.conf.json          # dev/prod config with copy hook
│   └── tauri.ci.json            # CI config (no hook)
packages/electron-shell/         # Pattern A sidecar
packages/electron-drm-shell/     # Pattern B outline
```

**Structure Decision**: Monorepo with Tauri host (examples/api) and a sidecar package (packages/electron-shell). CI uses a separate config to avoid workspace CLI build.

## Phase 0: Outline & Research
- Unknowns: DRM/Widevine licensing and distribution [NEEDS CLARIFICATION]
- Best practices: Electron sandboxing, CSP, localhost IPC; Tauri resource packaging
 - Deliverable: See `./research.md` for Phase 0 research outcomes and evidence

## Phase 1: Design & Contracts
- Entities: ElectronFeature (url, pattern, status), SidecarAuth (token). See `./data-model.md`.
- Contracts: Tauri invoke `open_electron_feature(url) -> Result<(), code)`. See `./contracts/open_electron_feature.md`.
- Quickstart: Run host app; click UI button; Electron window appears only on demand. See `./quickstart.md`.
 - Manual checks: See `./VERIFICATION.md` for step-by-step acceptance validation.

## Phase 2: Task Planning Approach
- Derive tasks from contracts and data model; tests before implementation

## Phase 3+: Future Implementation
- Task generation then execution; e2e validation

## Complexity Tracking
| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |

## Progress Tracking
- [x] Phase 0: Research complete (see `./research.md`)
- [ ] Phase 1: Design complete
- [ ] Phase 2: Task planning complete
- [ ] Phase 3: Tasks generated
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Based on Constitution v1.0.0 - See `/memory/constitution.md`**
