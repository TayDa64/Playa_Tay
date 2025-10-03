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

## Phase 1: Design & Contracts
- Entities: ElectronFeature (url, pattern, status), SidecarAuth (token)
- Contracts: Tauri invoke `open_electron_feature(url) -> Result<(), code>`
- Quickstart: Run host app; click UI button; Electron window appears only on demand

## Phase 2: Task Planning Approach
- Derive tasks from contracts and data model; tests before implementation

## Phase 3+: Future Implementation
- Task generation then execution; e2e validation

## Complexity Tracking
| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |

## Progress Tracking
- [ ] Phase 0: Research complete
- [ ] Phase 1: Design complete
- [ ] Phase 2: Task planning complete
- [ ] Phase 3: Tasks generated
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Based on Constitution v1.0.0 - See `/memory/constitution.md`**
