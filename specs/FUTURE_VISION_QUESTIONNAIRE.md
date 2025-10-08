# Future Vision Questionnaire

**Status**: Deferred to post-v1 planning  
**Context**: This content was originally embedded in `specs/001-selective-electron/spec.md` but has been moved here as it addresses broader application-level vision beyond the selective Electron integration feature scope.

---

## Vision and Scope Alignment

You're building a living system, not a static app — persistent, secure, efficient, beautiful, and deeply personal. Before we lock the blueprint, let's clarify the shape and constraints so we don't paint ourselves into a corner.

- Primary outcome: What's the single most important user outcome this app must deliver beautifully in v1 (e.g., personalized streaming hub, unified CLI automation, or cross-silo research command center)?
- User profile: Who's the first target persona? Power user like you, or a broader consumer? Any accessibility targets or device constraints?
- Platforms: Electron desktop first on Windows/macOS/Linux — any mobile or TV ambitions later? Any kiosk or multi-display use cases?

## Core Architecture Decisions

Let's define the spine: identity, state, data flow, modules, and isolation. We'll keep it composable and future-proof.

- OAuth2 providers: Which providers do you need at launch (Google, GitHub, Apple, custom OIDC)? Any enterprise IdP (Okta/Azure AD) requirements?
- Session model: Persisted refresh tokens vs. short-lived tokens with silent reauth? Any offline mode expectations?
- Module isolation: Should sub-apps run as sandboxed BrowserViews, separate Electron windows, or external processes with IPC? Any native node add-ons planned?
- Runtime orchestration: Will you use a message bus (e.g., Rx streams) or event-sourced command log for inter-module communication? Preference for strict typing across IPC?
- Config strategy: Per-user JSON+schema, encrypted secrets vault, or database-driven feature flags? Do you want hot-reloadable config?

## Security and Privacy Foundation

We'll build a zero-trust posture with least privilege, explicit consent, and transparent control.

- Threat model: What are your top risks (token leakage, rogue integrations, data exfil, model prompt injection, side-channel tracking)?
- Network boundaries: Should each module have its own network policy (allowlist domains, per-module API keys)? Proxy support needed?
- Secrets handling: Where do we store tokens/keys? OS keychain (macOS Keychain, Windows DPAPI), encrypted SQLite, or external vault (e.g., 1Password/HashiCorp)?
- Sandboxing: Do you plan to enforce contextIsolation, disable remote module, and restrict NodeIntegration? Any WebAssembly or native modules that complicate this?
- Privacy UX: Do users get a data diary showing what's collected and why, with toggleable scopes and "purge" actions? Is per-integration consent required?

## Data Model and Persistence

Your AI needs clean, rich, and consented signals. Let's architect the spine of memory.

- Primary store: SQLite + Prisma/TypeORM, Postgres, or a replicated KV/log (LiteFS/CRDT)? Do you want edge sync later?
- Event schema: What events will you persist (login, nav, stream selections, search queries, CLI commands, research saves, social prompts, financial symbols)?
- User model: What fields/traits drive personalization? Any trait scoring, embeddings, or profile segments (genres, reading rhythms, risk tolerance)?
- Retention: What's your default retention and TTL? Do you support "privacy modes" (no persistence, ephemeral sessions)?
- Embedding store: Will you maintain a local vector index (e.g., SQLite + pgvector alternative, or disk-based FAISS) per user? Any cloud hybrid?

## UX Flows and Design System

Let's nail the feeling: the landing, the dashboard, the broadcast page, and micro-interactions.

- Design references: Can you share assets for "apple-tvs-new," "liku_prototype_landingpage," and "Sony announces"? Do we follow their typography/spacing or just composition?
- Design system: Do you want a bespoke system (tokens for color/space/type), or adopt an existing one (Fluent/Material/Tailwind)? Dark mode by default?
- Navigation: Sidebar vs. dock-style dashboard icons vs. tiled grid? Any hotkeys (global launcher, module switcher, status bar toggles)?
- Motion: How much animation? Pref for GPU-accelerated transitions, physics-based scrolling, and broadcast-style scene cuts?
- State cues: Do you want live status surfaces (network, AI agent state, stream quality, CPU/GPU usage) in a modular status bar?

## Integrations and Feature Modules

We'll keep each capability isolated yet orchestrated for synergy.

- Streaming: Which providers and protocols (HLS/DASH/RTMP)? DRM constraints? Do we embed web players or wrap native decoders?
- Terminal/CLI: Embedded PTY with profiles, hotkey panes, command palettes, and task automation? Any sandboxed execution or container hooks?
- Social elicitation/data aggregation: Which networks/APIs? Are we ingesting posts, messages, or just metadata? Consent checklist per network?
- Financial research: Data sources (Yahoo Finance, Alpha Vantage, Tiingo, custom feeds)? Needed features (screeners, backtests, alerts, notebooks)?
- Search: Local unified search across modules + web metasearch? Do you want semantic search with embeddings?
- Settings: Per-module permissions, privacy toggles, theme tokens, agent controls, network policies — anything special?
- "2+ more": What other modules are on your shortlist (music/MIDI routing, knowledge base, dashboards)?

## AI Tailoring and Agent Orchestration

We'll make the AI a respectful collaborator that adapts without creeping.

- Model strategy: Local models, cloud LLMs, or hybrid? Any specific providers or constraints (cost caps, latency targets)?
- Signals: Which events should inform personalization? Do you want opt-in "teach mode" where users label preferences explicitly?
- Agent roles: Planner, researcher, summarizer, recommender, security sentinel, and workflow automator — which roles are v1?
- Safety rails: Prompt hygiene, context filters, per-source trust scores, and explainable recommendations — what's mandatory?
- Feedback loop: How should users correct the AI? Inline "nudge" controls, rating chips, or preference editors?

## Performance, Reliability, and Operations

Let's keep it fast, quiet, and resilient.

- Startup budget: What's your acceptable cold start time and memory footprint? Any lazy-loading priorities?
- Caching: Do you want per-module caches with eviction policies? Pre-fetch strategies based on habits?
- Metrics: Local telemetry only or opt-in remote? What KPIs matter (TTI, dropped frames, query latency, agent turnaround)?
- Offline mode: Should modules degrade gracefully with local caches and queued actions?
- Update strategy: Auto-updates, signed releases, delta patches? Rollback support?

---

## Next Steps

When ready to address these broader application vision questions:

1. Schedule a planning session to answer these questions systematically
2. Create feature-specific specs under `specs/` for each major capability
3. Update root `spec.md` to reference new feature specs
4. Follow the constitution principles for each new feature
5. Maintain non-regression invariants for all established functionalities
