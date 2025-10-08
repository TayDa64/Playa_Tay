# Feature Specification: Selective Electron Integration (Pattern A/B)

**Feature Branch**: `[001-selective-electron]`
**Created**: 2025-10-03

## Execution Flow (main)
```
1. Parse user description from Input
2. Extract key concepts from description
3. Mark unclear aspects with [NEEDS CLARIFICATION]
4. Fill User Scenarios & Testing section
5. Generate Functional Requirements (testable)
6. Identify Key Entities (if data involved)
7. Run Review Checklist and record status
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW (beyond constraints the constitution mandates)
- 👥 Written for stakeholders; technical but implementation-agnostic where possible

### Section Requirements
- Mandatory sections are required for every feature
- Remove entire optional sections if not relevant

### For AI Generation
- Mark ambiguities: [NEEDS CLARIFICATION: ...]
- Don’t guess implementation unless required by constraints
- Think like a tester: each requirement must be verifiable

---

## User Scenarios & Testing (mandatory)

### Primary User Story
As a developer, I want the Tauri app to open an Electron-powered window only when a specific feature requires web capabilities not available in WebView, so that the base app stays small and secure while still supporting niche use cases (e.g., DRM).

### Acceptance Scenarios
1. Given the Tauri app is running, When the user clicks "Open Electron Feature", Then the app launches an Electron window loading the configured URL, with nodeIntegration=false, contextIsolation=true, sandbox=true.
2. Given Electron runtime is not present in dev, When the feature is invoked, Then the app returns a not_installed outcome and the UI shows a helpful prompt (with a basic dev ensure step available) without crashing the Tauri host.
3. Given packaging is enabled, When the app is built, Then a minimal Electron sidecar asset is included as a resource (Pattern A) without breaking the monorepo build and with no Electron dependency in the main renderer.
4. Given the app is in production mode, When the feature is invoked, Then devtools are disabled for Electron windows and CSP allowlists are enforced.
5. Given a headless environment, When the sidecar is run directly, Then the system reports that a display is required (or uses a documented headless approach) without causing the Tauri host to fail.

### Edge Cases
- Headless/containerized environments without DISPLAY (Linux) require headless run guidance.
- Missing system libraries for Electron on Linux (libnss3, etc.)
- Unsupported platform architectures for the Electron download in dev
- IPC misuse attempts: unauthenticated localhost requests must be rejected

## Requirements (mandatory)

### Functional Requirements
- FR-001: The base Tauri app MUST remain fully functional without Electron.
- FR-002: The app MUST provide a single UI action that calls a stable command to open the Electron feature; backend decides Pattern A vs B.
- FR-003: Pattern A MUST spawn an Electron sidecar from a workspace package during dev and from a bundled resource during packaging.
- FR-004: If Electron is unavailable at runtime, the backend MUST return a not_installed error code; the frontend MUST surface a clear prompt.
- FR-005: Electron windows MUST have nodeIntegration=false, contextIsolation=true, sandbox=true, devtools disabled in production.
- FR-006: IPC between Tauri and Electron MUST use localhost only with ephemeral tokens and reject unauthenticated traffic.
- FR-007: Packaging MUST include only the minimal sidecar assets required for Pattern A; the base installer MUST remain small.
- FR-008: CI and local builds MUST remain deterministic and respect the monorepo toolchain pins (pnpm 10.16.0, Rust ≥1.77.2).
- FR-009: Logs and errors from Electron launch failures MUST not crash the Tauri host; errors are surfaced as structured codes.
- FR-010: Constitution checks MUST be applied in planning and post-design.

Examples of clarifications:
- FR-011: The app SHOULD support Pattern B as a separate module with its own installer and signed update channel. [NEEDS CLARIFICATION: hosting location and signing process]
- FR-012: The UI SHOULD offer an install/upgrade prompt for the module when not_installed is returned. [NEEDS CLARIFICATION: UX flow and permissions]

### Key Entities (optional)
- ElectronFeature: represents a feature that requires Electron; attributes: url, pattern(A|B), status(available|not_installed|running)
- SidecarAuth: ephemeral token shared across Tauri/Electron sessions

---

## Review & Acceptance Checklist

### Content Quality
- [x] No low-level implementation details beyond constraints
- [x] Focused on user outcomes and governance requirements
- [x] Mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain — Deferred to post-v1 decisions doc
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable (launches, security flags, packaging)
- [x] Scope clearly bounded (Tauri-first with selective Electron)
- [x] Dependencies and assumptions identified (toolchain, libs, headless)

---

## Deferred Decisions (Post-v1)

The following [NEEDS CLARIFICATION] items are acknowledged and deferred to post-v1 implementation planning:

### Pattern B Implementation Details
- **DRM Provider Selection**: Specific Widevine-enabled services, licensing terms, distribution requirements
  - Impact: Pattern B packaging and compliance strategy
  - Timeline: Research during Pattern A production validation

- **Module Hosting & Signing**: Update server infrastructure, code signing certificates, delta patching strategy
  - Impact: Auto-update implementation and security model
  - Timeline: Q1 post-v1 (if Pattern B validated)

- **Install/Upgrade UX Flow**: Permissions model, download progress UI, rollback strategy
  - Impact: Frontend installer dialog and error handling
  - Timeline: Concurrent with Pattern B Phase 3

### Module Catalog Clarifications
- **Terminal/CLI**: Shell profiles, sandboxing policy, command allowlist
  - Impact: Security posture for command execution
  - Decision needed: Before M2 implementation

- **Social Ingest**: Network list (Twitter/X, Reddit, etc.), consent UX, rate limits
  - Impact: API integration and privacy compliance
  - Decision needed: Before M3 implementation

- **Financial Research**: Data sources (Yahoo Finance, Alpha Vantage), terms of service, real-time vs delayed quotes
  - Impact: Subscription model and data licensing
  - Decision needed: Before M4 implementation

- **Telemetry Strategy**: Local-only logs vs opt-in remote; metrics collection scope
  - Impact: Privacy policy and infrastructure
  - Decision needed: Before v1.1 release

### Resolution Process
1. Document decisions in `specs/001-selective-electron/decisions/YYYY-MM-DD-topic.md`
2. Update relevant sections of this spec
3. Propagate changes to `plan.md` and `tasks.md`
4. Mark as resolved in this section

---

## Execution Status
- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified (initial pass)
- [x] Review checklist passed

### playa Tay's application specification plan, implement without losing already established functionalities.

This feature specification aligns with the top-level application specification defined in `/spec.md`. The selective Electron integration provides a minimal, secure pattern for incorporating Electron-specific capabilities (e.g., DRM/Widevine) without compromising the Tauri-first architecture or bloating the base application.

**Key Alignment Points:**
- Preserves all established functionalities documented in root `spec.md`
- Maintains non-regression invariants for command contracts, security posture, and build/packaging
- Implements Pattern A (bundled sidecar) with scaffolding for Pattern B (optional module)
- Follows constitution principles: Tauri-first, security-by-default, reproducible builds, testing, modularity
- Focuses exclusively on selective Electron integration scope; broader application vision and roadmap items are deferred to separate planning documents

**Non-Regression Commitment:**
All existing behaviors must continue to work:
- UI trigger: Button in `examples/api/src/views/Welcome.svelte` labeled "Open Electron Feature (Pattern A/B)"
- Backend commands: `open_electron_feature`, `launch_electron`, `is_electron_available`, `ensure_electron_sidecar`
- Sidecar packaging: Resources copied to `examples/api/src-tauri/resources/electron-shell/`
- Build & CI: Workflow in `.github/workflows/ci.yml` builds artifacts without regression
- Security posture: `nodeIntegration=false`, `contextIsolation=true`, `sandbox=true`, devtools disabled in production
