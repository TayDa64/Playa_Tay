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
Vision and scope alignment
You’re building a living system, not a static app — persistent, secure, efficient, beautiful, and deeply personal. Before we lock the blueprint, let’s clarify the shape and constraints so we don’t paint ourselves into a corner.
•	Primary outcome: What’s the single most important user outcome this app must deliver beautifully in v1 (e.g., personalized streaming hub, unified CLI automation, or cross-silo research command center)?
•	User profile: Who’s the first target persona? Power user like you, or a broader consumer? Any accessibility targets or device constraints?
•	Platforms: Electron desktop first on Windows/macOS/Linux — any mobile or TV ambitions later? Any kiosk or multi-display use cases?
Core architecture decisions
Let’s define the spine: identity, state, data flow, modules, and isolation. We’ll keep it composable and future-proof.
•	OAuth2 providers: Which providers do you need at launch (Google, GitHub, Apple, custom OIDC)? Any enterprise IdP (Okta/Azure AD) requirements?
•	Session model: Persisted refresh tokens vs. short-lived tokens with silent reauth? Any offline mode expectations?
•	Module isolation: Should sub-apps run as sandboxed BrowserViews, separate Electron windows, or external processes with IPC? Any native node add-ons planned?
•	Runtime orchestration: Will you use a message bus (e.g., Rx streams) or event-sourced command log for inter-module communication? Preference for strict typing across IPC?
•	Config strategy: Per-user JSON+schema, encrypted secrets vault, or database-driven feature flags? Do you want hot-reloadable config?
Security and privacy foundation
We’ll build a zero-trust posture with least privilege, explicit consent, and transparent control.
•	Threat model: What are your top risks (token leakage, rogue integrations, data exfil, model prompt injection, side-channel tracking)?
•	Network boundaries: Should each module have its own network policy (allowlist domains, per-module API keys)? Proxy support needed?
•	Secrets handling: Where do we store tokens/keys? OS keychain (macOS Keychain, Windows DPAPI), encrypted SQLite, or external vault (e.g., 1Password/HashiCorp)?
•	Sandboxing: Do you plan to enforce contextIsolation, disable remote module, and restrict NodeIntegration? Any WebAssembly or native modules that complicate this?
•	Privacy UX: Do users get a data diary showing what’s collected and why, with toggleable scopes and “purge” actions? Is per-integration consent required?
Data model and persistence
Your AI needs clean, rich, and consented signals. Let’s architect the spine of memory.
•	Primary store: SQLite + Prisma/TypeORM, Postgres, or a replicated KV/log (LiteFS/CRDT)? Do you want edge sync later?
•	Event schema: What events will you persist (login, nav, stream selections, search queries, CLI commands, research saves, social prompts, financial symbols)?
•	User model: What fields/traits drive personalization? Any trait scoring, embeddings, or profile segments (genres, reading rhythms, risk tolerance)?
•	Retention: What’s your default retention and TTL? Do you support “privacy modes” (no persistence, ephemeral sessions)?
•	Embedding store: Will you maintain a local vector index (e.g., SQLite + pgvector alternative, or disk-based FAISS) per user? Any cloud hybrid?
UX flows and design system
Let’s nail the feeling: the landing, the dashboard, the broadcast page, and micro-interactions.
•	Design references: Can you share assets for “apple-tvs-new,” “liku_prototype_landingpage,” and “Sony announces”? Do we follow their typography/spacing or just composition?
•	Design system: Do you want a bespoke system (tokens for color/space/type), or adopt an existing one (Fluent/Material/Tailwind)? Dark mode by default?
•	Navigation: Sidebar vs. dock-style dashboard icons vs. tiled grid? Any hotkeys (global launcher, module switcher, status bar toggles)?
•	Motion: How much animation? Pref for GPU-accelerated transitions, physics-based scrolling, and broadcast-style scene cuts?
•	State cues: Do you want live status surfaces (network, AI agent state, stream quality, CPU/GPU usage) in a modular status bar?
Integrations and feature modules
We’ll keep each capability isolated yet orchestrated for synergy.
•	Streaming: Which providers and protocols (HLS/DASH/RTMP)? DRM constraints? Do we embed web players or wrap native decoders?
•	Terminal/CLI: Embedded PTY with profiles, hotkey panes, command palettes, and task automation? Any sandboxed execution or container hooks?
•	Social elicitation/data aggregation: Which networks/APIs? Are we ingesting posts, messages, or just metadata? Consent checklist per network?
•	Financial research: Data sources (Yahoo Finance, Alpha Vantage, Tiingo, custom feeds)? Needed features (screeners, backtests, alerts, notebooks)?
•	Search: Local unified search across modules + web metasearch? Do you want semantic search with embeddings?
•	Settings: Per-module permissions, privacy toggles, theme tokens, agent controls, network policies — anything special?
•	“2+ more”: What other modules are on your shortlist (music/MIDI routing, knowledge base, dashboards)?
AI tailoring and agent orchestration
We’ll make the AI a respectful collaborator that adapts without creeping.
•	Model strategy: Local models, cloud LLMs, or hybrid? Any specific providers or constraints (cost caps, latency targets)?
•	Signals: Which events should inform personalization? Do you want opt-in “teach mode” where users label preferences explicitly?
•	Agent roles: Planner, researcher, summarizer, recommender, security sentinel, and workflow automator — which roles are v1?
•	Safety rails: Prompt hygiene, context filters, per-source trust scores, and explainable recommendations — what’s mandatory?
•	Feedback loop: How should users correct the AI? Inline “nudge” controls, rating chips, or preference editors?
Performance, reliability, and operations
Let’s keep it fast, quiet, and resilient.
•	Startup budget: What’s your acceptable cold start time and memory footprint? Any lazy-loading priorities?
•	Caching: Do you want per-module caches with eviction policies? Pre-fetch strategies based on habits?
•	Metrics: Local telemetry only or opt-in remote? What KPIs matter (TTI, dropped frames, query latency, agent turnaround)?
•	Offline mode: Should modules degrade gracefully with local caches and queued actions?
•	Update strategy: Auto-updates, signed releases, delta patches? Rollback support?















---
