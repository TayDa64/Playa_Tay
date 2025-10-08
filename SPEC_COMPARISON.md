# Spec Alignment Comparison

## Overview
This document shows the before and after state of the spec alignment issue.

## Root `spec.md` - Already Properly Aligned ✅

### After Comment Line
```markdown
### playa Tay's application specification plan, implement without losing already established functionalities.

This document is the top-level specification for Playa Tay. It articulates the vision, 
locks in current invariants so we don't regress, and points to detailed feature specs 
and plans maintained under `specs/`.

## Overview
- Tauri-first desktop app with selective Electron usage for niche capabilities (e.g., DRM) 
  without bloating or weakening the base app.
- Minimal, secure core; advanced modules are opt-in and isolated.
- CI-driven builds ensure determinism on Linux, with portability for macOS/Windows.
```

**Status**: ✅ Properly focused on application-level vision and invariants

---

## Feature `specs/001-selective-electron/spec.md` - Fixed ✅

### BEFORE (Lines 145-200) - Misaligned ❌
```markdown
### playa Tay's application specification plan, implement without losing already established functionalities.
Vision and scope alignment
You're building a living system, not a static app — persistent, secure, efficient, beautiful, and deeply personal...

•	Primary outcome: What's the single most important user outcome this app must deliver...
•	User profile: Who's the first target persona? Power user like you, or a broader consumer?
•	Platforms: Electron desktop first on Windows/macOS/Linux...

Core architecture decisions
•	OAuth2 providers: Which providers do you need at launch (Google, GitHub, Apple, custom OIDC)?
•	Session model: Persisted refresh tokens vs. short-lived tokens with silent reauth?
•	Module isolation: Should sub-apps run as sandboxed BrowserViews...

Security and privacy foundation
•	Threat model: What are your top risks (token leakage, rogue integrations, data exfil...)?
•	Network boundaries: Should each module have its own network policy...?

Data model and persistence
•	Primary store: SQLite + Prisma/TypeORM, Postgres, or a replicated KV/log (LiteFS/CRDT)?
•	Event schema: What events will you persist (login, nav, stream selections...)?

UX flows and design system
•	Design references: Can you share assets for "apple-tvs-new," "liku_prototype_landingpage"...?
•	Design system: Do you want a bespoke system (tokens for color/space/type)...?

Integrations and feature modules
•	Streaming: Which providers and protocols (HLS/DASH/RTMP)? DRM constraints?
•	Terminal/CLI: Embedded PTY with profiles, hotkey panes, command palettes...?
•	Social elicitation/data aggregation: Which networks/APIs?
•	Financial research: Data sources (Yahoo Finance, Alpha Vantage, Tiingo...)?

AI tailoring and agent orchestration
•	Model strategy: Local models, cloud LLMs, or hybrid?
•	Agent roles: Planner, researcher, summarizer, recommender...

Performance, reliability, and operations
•	Startup budget: What's your acceptable cold start time and memory footprint?
•	Caching: Do you want per-module caches with eviction policies?
```

**Problem**: This content was asking application-wide architecture questions, NOT focused on the selective Electron integration feature.

### AFTER - Properly Aligned ✅
```markdown
### playa Tay's application specification plan, implement without losing already established functionalities.

This feature specification focuses on implementing selective Electron integration (Pattern A/B) 
for Playa Tay without losing already established functionalities. The specification above defines:

- **Core functionality**: Tauri-first desktop app with optional Electron sidecar for niche 
  web capabilities (e.g., DRM/Widevine)
- **Security posture**: Enforced isolation flags (contextIsolation, sandbox, nodeIntegration=false) 
  and strict CSP
- **Packaging strategy**: Pattern A (bundled sidecar) with Pattern B (separate module) groundwork
- **Non-regression invariants**: Preserved existing UI triggers, backend contracts, build pipeline, 
  and workspace constraints
- **Testing requirements**: Integration tests for spawn validation, preflight checks, and error handling

All implementation work has been completed (see PROGRESS.md) with:
- ✅ Backend contract hardening with structured error types
- ✅ Security flag enforcement in Electron windows
- ✅ CI deterministic builds with artifact uploads
- ✅ Non-regression invariants documentation
- ✅ Modal UI for not_installed flow with retry mechanism
- ✅ Preflight availability check with button state management
- ✅ Headless CI guidance and testing strategy
- ✅ Integration test suite (4/4 tests passing)
- ✅ Pattern B README with comprehensive architecture and 6-phase plan
- ✅ Branching detection logic with TODO markers
- ✅ Spec clarifications documented and deferred with resolution process

This feature maintains alignment with the Playa_Tay Constitution (v1.0.0) principles:
- **P1**: Tauri-first with selective Electron via Pattern A/B
- **P2**: Security-by-default with strict isolation
- **P3**: Reproducible monorepo builds
- **P4**: Tests and quality gates (all passing)
- **P5**: Modularity and minimal packaging
```

**Status**: ✅ Now properly focused on the selective Electron integration feature only

---

## Summary of Changes

### What Was Removed (56 lines)
- Application-wide vision questions (OAuth2, session model, module isolation)
- Design system and UX planning questions (design references, navigation, motion)
- Integration module questions (streaming, terminal/CLI, social, financial)
- AI orchestration planning (model strategy, agent roles, signals)
- Performance and operations questions (startup budget, caching, metrics)

### What Was Added
- Clear statement that this is a **feature specification** for selective Electron integration
- Summary of what the feature spec covers (core functionality, security, packaging, invariants, testing)
- Documentation of completed implementation work (11/11 tasks, 100% complete)
- Confirmation of alignment with Constitution v1.0.0 principles (P1-P5)

### Result
Both specs now have clear, distinct scopes:
- **Root spec.md**: Application-level vision, goals, and cross-cutting concerns
- **Feature spec.md**: Selective Electron integration feature implementation details

✅ **Alignment achieved**: Both specs follow the instruction to "implement without losing already established functionalities"
