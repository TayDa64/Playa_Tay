# Playa Tay — Vision & Architecture Blueprint

**Status**: Living Document
**Last Updated**: October 5, 2025
**Owner**: TayDa64

---

## Executive Summary

Playa Tay is a **living personal system** — not a static app — designed to be persistent, secure, efficient, beautiful, and deeply personal. It serves as a unified command center for streaming, research, automation, and AI-powered personalization, built on a foundation of user control, privacy-by-design, and modular extensibility.

**Core Philosophy**: Zero-trust architecture with opt-in modules, explicit consent, and transparent control. Every feature earns its place; nothing is imposed.

---

## Vision and Scope Alignment

### Primary Outcome (v1)
**Personalized streaming hub with integrated research and automation capabilities.**

The single most important user outcome: A unified interface where users can:
1. **Stream content** from multiple providers with intelligent recommendations
2. **Research topics** across financial, social, and web sources in real-time
3. **Automate workflows** through an embedded terminal and CLI orchestration
4. **Personalize experience** via AI agents that learn from explicit consent and feedback

### User Profile

**Primary Persona: "The Power Integrator"**
- **Profile**: Tech-savvy professional (developer, researcher, trader, content creator)
- **Needs**: Efficiency, control, privacy, deep customization
- **Pain Points**: Context-switching between 10+ apps, data silos, lack of personalization control
- **Expectations**: Fast startup (<2s), keyboard-driven navigation, transparent data usage

**Secondary Persona: "The Technical Evaluator"**
- **Profile**: Early adopter testing new platforms
- **Needs**: Stability, clear migration paths, export capabilities
- **Pain Points**: Vendor lock-in, opaque data practices
- **Expectations**: Open standards, local-first data, opt-in cloud sync

**Accessibility Targets**:
- Keyboard navigation (100% coverage)
- Screen reader support (ARIA labels, semantic HTML)
- High contrast mode support
- Configurable font sizes and spacing

**Device Constraints**:
- Desktop-first: Windows 11+, macOS 13+, Linux (Ubuntu 22.04+)
- Minimum: 4GB RAM, 2GB storage, 1280x720 display
- Optimal: 16GB RAM, 10GB storage, 1920x1080+ multi-monitor

### Platform Strategy

**Phase 1 (v1.0 — Current Focus)**:
- Tauri desktop on Windows/macOS/Linux
- Selective Electron for DRM/Widevine capabilities (Pattern A)

**Phase 2 (v1.5 — Post-Launch)**:
- Pattern B: Separately distributed modules with signed updates
- Mobile companion app (React Native) for notifications and quick actions

**Phase 3 (v2.0 — Future)**:
- TV app for streaming-focused interface (Apple TV, Android TV)
- Kiosk mode for dedicated streaming stations
- Multi-display orchestration (control panels + content displays)

**Out of Scope**:
- Web-based version (desktop-native is core value prop)
- Browser extensions (conflicts with app-centric model)
- Embedded systems or IoT devices

---

## Core Architecture Decisions

### Identity & Authentication

**OAuth2 Providers (v1)**:
1. **Google** — Primary (YouTube, Gmail, Drive)
2. **GitHub** — For developer integrations
3. **Apple** — iOS/macOS users
4. **Custom OIDC** — Enterprise self-hosted instances

**Enterprise IdP (v1.5+)**:
- Okta, Azure AD support for enterprise deployments
- SAML 2.0 bridge for legacy systems

**Session Model**:
- **Persisted refresh tokens** stored in OS keychain (Keychain/DPAPI)
- **Short-lived access tokens** (15min) with automatic silent refresh
- **Offline mode**: Cached credentials valid for 7 days
- **Multi-account support**: Up to 5 profiles per user

### Module Isolation Strategy

**Architecture**: Hybrid multi-process with IPC

1. **Core Host** (Tauri/Rust)
   - Main process, window management, system integration
   - Authentication broker, secrets manager
   - Module lifecycle orchestrator

2. **Module Types**:
   - **BrowserView Modules** (light, embedded): Settings, search, AI chat
   - **Separate Windows** (heavy, isolated): Streaming players, terminal, financial charts
   - **External Processes** (sandboxed): Background agents, data indexers
   - **Native Add-ons** (optional): Hardware acceleration, codec support

3. **Security Boundaries**:
   - Each module has its own V8 context with contextIsolation=true
   - IPC via authenticated channels with ephemeral tokens
   - Network policies enforced per module (allowlist domains)

### Runtime Orchestration

**Message Bus Architecture**:
- **Primary**: Event-sourced command log with RxJS streams
- **Protocol**: JSON-RPC 2.0 over localhost with TLS
- **Type Safety**: Shared TypeScript/Rust type definitions via code generation
- **Replay**: Event log for debugging and state reconstruction

**Key Channels**:
- `auth.events` — Login, logout, token refresh
- `module.lifecycle` — Start, stop, crash, upgrade
- `data.sync` — Cross-module data sharing (with consent)
- `ai.signals` — User events for personalization
- `network.status` — Connectivity, rate limits, errors

### Configuration Strategy

**Layered Config**:
1. **System Defaults** (bundled JSON schema)
2. **User Preferences** (JSON in `~/.config/playa-tay/config.json`)
3. **Runtime Overrides** (in-memory, session-only)

**Secrets Management**:
- **Encrypted Vault**: SQLite with SQLCipher for tokens/keys
- **OS Keychain**: Fallback for master encryption key
- **External Vault** (optional): 1Password/HashiCorp Vault integration

**Feature Flags**:
- Database-driven toggles for gradual rollouts
- Per-user and per-module granularity
- Hot-reloadable without restart

---

## Security and Privacy Foundation

### Threat Model

**Top Risks** (prioritized):
1. **Token Leakage** — OAuth tokens stolen via malicious module or memory dump
2. **Rogue Integrations** — User installs malicious third-party module
3. **Data Exfiltration** — Module sends user data to unauthorized endpoints
4. **Prompt Injection** — AI model manipulated to leak secrets or execute harmful actions
5. **Side-Channel Tracking** — Behavioral fingerprinting without consent

### Network Boundaries

**Module Network Policies**:
- Each module declares allowlist domains in manifest
- Per-module API keys issued by auth broker
- Network requests routed through proxy with logging
- User can revoke network access per module

**Proxy Configuration**:
- System proxy detection and honor
- Custom proxy with authentication for corporate environments
- Split tunneling for VPN scenarios

### Secrets Handling

**Storage Hierarchy** (order of preference):
1. **OS Keychain** — macOS Keychain, Windows DPAPI, Linux Secret Service
2. **Encrypted SQLite** — SQLCipher with user-derived key
3. **External Vault** — 1Password/HashiCorp (enterprise only)

**Key Rotation**:
- Automatic rotation every 90 days
- User-triggered rotation on suspected compromise
- Graceful degradation if vault unavailable (read-only mode)

### Sandboxing

**Enforcement**:
- `contextIsolation=true` — Separate JS contexts for renderer and preload
- `nodeIntegration=false` — No direct Node.js access from renderer
- `sandbox=true` — OS-level process sandboxing
- `webSecurity=true` — CORS, CSP enforcement
- `allowRunningInsecureContent=false` — HTTPS-only

**WebAssembly & Native Modules**:
- WASM allowed for compute-heavy tasks (embeddings, codecs)
- Native modules require explicit user approval
- Binary signature verification before loading

### Privacy UX

**Data Diary**:
- Real-time log of all collected data with timestamps
- Per-event purpose and retention period displayed
- "Purge" action per data type with confirmation
- Export to JSON for portability

**Consent Management**:
- Per-integration toggleable scopes (e.g., YouTube: watch history ON, subscriptions OFF)
- Granular permissions checklist on module install
- Opt-in for any cloud sync or telemetry

---

## Data Model and Persistence

### Primary Store

**Technology**: SQLite + Prisma ORM
- **Local-first**: All data stored on device
- **Edge Sync** (v2.0): CRDTs via LiteFS for multi-device
- **Backup**: Encrypted backups to user-chosen cloud storage

**Alternative Considered**:
- Postgres: Too heavy for desktop
- IndexedDB: Insufficient query capabilities
- LiteFS/CRDT: Deferred to v2.0 for complexity

### Event Schema

**Persisted Events** (with user consent):
- `auth.login` — Timestamp, provider, success/failure
- `nav.route` — Page/module transitions, dwell time
- `stream.play` — Content ID, provider, watch duration, completion %
- `search.query` — Query text, source, results clicked
- `cli.command` — Command executed, success/failure, runtime
- `research.save` — Article/stock/post saved with tags
- `social.prompt` — AI prompt submitted, response received
- `finance.symbol` — Symbol viewed, chart interactions, alerts set

**Event Metadata**:
- `user_id`, `session_id`, `timestamp`, `module_id`
- `consent_level` (0=none, 1=local, 2=cloud, 3=personalization)

### User Model

**Core Fields**:
```typescript
User {
  id: UUID
  profiles: Profile[] // Multi-account support
  preferences: Preferences
  traits: UserTraits
  consent: ConsentSettings
}

UserTraits {
  genres: string[] // Inferred from watch history
  reading_pace: number // WPM average
  risk_tolerance: number // 0-100 scale
  activity_rhythm: TimePattern // Peak hours, weekday/weekend
  embeddings: float[] // 768-dim for semantic search
}
```

**Privacy Levels**:
- **Ghost Mode**: No persistence, ephemeral session
- **Local Only**: All data stays on device
- **Opt-in Sync**: Encrypted cloud backup with E2EE
- **Personalization**: AI uses data for recommendations

### Retention & TTL

**Default Retention**:
- Auth events: 90 days
- Navigation: 30 days
- Stream history: 365 days
- CLI commands: 30 days (sensitive, user-configurable)
- Research saves: Indefinite (user-managed)
- AI prompts: 7 days (auto-purge)

**Privacy Modes**:
- **Incognito**: No persistence, events logged but not saved
- **Ephemeral Session**: Session-only data, purged on exit
- **Minimal**: Only essential events, shortest TTL

### Embedding Store

**Implementation**: SQLite with extension + HNSW index
- **Technology**: sqlite-vec (native extension)
- **Dimensions**: 768 (BERT-style embeddings)
- **Index**: Hierarchical Navigable Small World (HNSW) for fast ANN
- **Capacity**: Up to 1M vectors per user
- **Cloud Hybrid** (v2.0): Offload cold embeddings to S3-compatible storage

---

## UX Flows and Design System

### Design References

**Inspirations**:
1. **Apple TV** — Clean cards, focus states, smooth transitions
2. **Liku Prototype** — Minimal chrome, content-first layout
3. **Sony Announces** — Bold typography, layered depth

**Design Principles**:
- **Content-First**: Maximize content, minimize chrome
- **Focus-Driven**: Clear visual hierarchy, keyboard-navigable
- **Adaptive**: Layouts adjust to window size and content type
- **Consistent**: Unified design language across modules

### Design System

**Foundation**: Bespoke system with Tailwind utilities
- **Tokens**: JSON-defined colors, spacing, typography
- **Components**: Shared React/Svelte component library
- **Theme**: Dark mode by default, light mode optional
- **Customization**: User-editable color accents

**Color Palette** (Dark Mode):
- Background: `#0A0A0A` (near-black)
- Surface: `#1A1A1A` (cards, panels)
- Primary: `#3B82F6` (blue, accent)
- Success: `#10B981` (green)
- Warning: `#F59E0B` (amber)
- Error: `#EF4444` (red)
- Text: `#E5E5E5` (primary), `#A3A3A3` (secondary)

**Typography**:
- Heading: Inter (system-ui fallback)
- Body: Inter
- Mono: JetBrains Mono

### Navigation

**Pattern**: Hybrid sidebar + command palette
- **Sidebar** (left, collapsible): Persistent module icons
- **Command Palette** (Cmd+K): Quick actions, search, navigation
- **Status Bar** (bottom): Contextual info, notifications

**Hotkeys**:
- `Cmd/Ctrl+K` — Command palette
- `Cmd/Ctrl+1-9` — Switch to module 1-9
- `Cmd/Ctrl+T` — Toggle terminal
- `Cmd/Ctrl+,` — Settings
- `Cmd/Ctrl+Shift+P` — Privacy diary
- `Esc` — Back/close modal

### Motion & Animation

**Animation Budget**: 60fps minimum, GPU-accelerated
- **Transitions**: 200ms ease-out for state changes
- **Scrolling**: Physics-based momentum scrolling
- **Scene Cuts**: Broadcast-style fades for major transitions (300ms)
- **Focus States**: Subtle scale (1.05x) + shadow

**Reduced Motion**: Honor `prefers-reduced-motion` media query

### State Cues

**Modular Status Bar**:
- **Network**: Online/offline, latency, rate limit warnings
- **AI Agent**: Idle/thinking/ready, confidence level
- **Stream Quality**: Bitrate, buffering, codec info
- **System**: CPU/GPU usage, memory pressure

**Notification System**:
- **Toast**: Transient (3s), bottom-right
- **Persistent**: Requires action, top-right
- **Critical**: Modal overlay with sound

---

## Integrations and Feature Modules

### Module Catalog (v1)

**M1: Streaming Hub** (P0)
- **Providers**: YouTube, Twitch, custom HLS/DASH
- **Protocols**: HLS, DASH, WebRTC (for live)
- **DRM**: Widevine via Electron sidecar (Pattern A)
- **Features**: Watch queue, history, recommendations
- **UI**: Card grid, inline player, PiP mode

**M2: Terminal/CLI** (P1)
- **Technology**: Embedded PTY (xterm.js + node-pty)
- **Profiles**: Bash, Zsh, Fish, PowerShell
- **Features**: Split panes, tabs, hotkey launcher
- **Automation**: Command palette with saved scripts
- **Sandboxing**: Allowlist executables, no sudo by default

**M3: Social Aggregator** (P1)
- **Networks**: Twitter/X, Reddit, Hacker News, RSS
- **Data**: Posts, metadata only (no private messages)
- **Consent**: Per-network opt-in with scope selector
- **Features**: Unified feed, keyword filters, sentiment analysis

**M4: Financial Research** (P2)
- **Data Sources**: Yahoo Finance (free), Alpha Vantage (API key)
- **Features**: Stock screeners, watchlists, price alerts, basic charts
- **Real-time**: Delayed quotes (15min) for free tier
- **Advanced**: TradingView widgets (embedded iframe)

**M5: Unified Search** (P1)
- **Local**: Full-text search across all modules
- **Web**: Metasearch (Google, DuckDuckGo, Brave)
- **Semantic**: Embedding-based similarity search
- **Shortcuts**: `/web query`, `/local query`, `/ai query`

**M6: Settings** (P0)
- **Sections**: Modules, privacy, theme, hotkeys, network, AI
- **Features**: Import/export config, reset to defaults
- **Validation**: Schema-based with inline error messages

**Future Modules** (v2.0+):
- Music/MIDI routing
- Knowledge base (note-taking with backlinks)
- Custom dashboards (widget builder)
- Calendar & reminders

---

## AI Tailoring and Agent Orchestration

### Model Strategy

**Hybrid Approach**:
1. **Local Models** (privacy-sensitive, low-latency):
   - Text embeddings: `all-MiniLM-L6-v2` (384-dim)
   - Sentiment analysis: `distilbert-base-uncased-finetuned-sst-2`
   - Runs on CPU, <1GB RAM

2. **Cloud LLMs** (complex reasoning, user opt-in):
   - Primary: OpenAI GPT-4o (cost caps: $10/month)
   - Fallback: Anthropic Claude Sonnet
   - Latency target: <3s for responses

3. **Cost Controls**:
   - Token budgets per user per month
   - Cache common prompts locally
   - Batch non-urgent queries

### Signals for Personalization

**Opt-in Events**:
- Stream watch duration + completion rate
- Search queries and clicked results
- Saved articles and tagged items
- CLI command frequency
- Explicit thumbs up/down ratings

**Teach Mode** (explicit preference labeling):
- "More like this" / "Less like this" buttons
- Genre preference sliders (0-100)
- Per-content feedback chips

### Agent Roles (v1)

1. **Recommender** (P0):
   - Suggests streams based on watch history
   - Surfaces related articles during research
   - Confidence scores displayed

2. **Summarizer** (P1):
   - Generates TL;DR for long articles
   - Creates daily digest of social feeds
   - CLI command help with context

3. **Researcher** (P2):
   - Gathers multi-source info on topics
   - Cross-references claims with sources
   - Generates annotated bibliographies

4. **Security Sentinel** (P1):
   - Monitors for suspicious network activity
   - Flags prompt injection attempts
   - Alerts on token expiration

5. **Workflow Automator** (P2):
   - Suggests CLI automations based on patterns
   - Creates hotkeys for frequent actions
   - Optimizes routine tasks

### Safety Rails

**Mandatory Protections**:
- **Prompt Hygiene**: Strip PII before sending to cloud
- **Context Filters**: No prompts containing auth tokens or keys
- **Per-Source Trust Scores**: Weight by domain reputation
- **Explainable Recommendations**: "Why this?" link on every suggestion

**Rate Limits**:
- Max 100 prompts/day per user
- Max 10 concurrent requests
- Exponential backoff on API errors

### Feedback Loop

**User Correction Mechanisms**:
- **Inline Nudge Controls**: Thumbs up/down on recommendations
- **Rating Chips**: 1-5 stars on summaries
- **Preference Editor**: Visual sliders for genre weights
- **Correction Log**: History of user adjustments for model tuning

---

## Performance, Reliability, and Operations

### Startup Budget

**Cold Start Target**: <2 seconds
- Main window visible: 500ms
- Modules loaded: 1.5s
- Full interactivity: 2s

**Optimization Strategy**:
- Lazy-load modules on first access
- Pre-warm critical paths during splash
- Defer non-critical init to background threads

**Memory Footprint**:
- Idle: <200MB (host + one module)
- Active (3 modules): <800MB
- Peak (5 modules + AI): <1.5GB

### Caching

**Per-Module Caches**:
- **LRU Eviction**: Max 500MB per module
- **TTL Policies**: Configurable per content type (images: 7d, API: 1h)
- **Pre-fetch**: Based on usage patterns (e.g., pre-load watch queue thumbnails)

**Shared Cache**:
- Auth tokens (15min TTL)
- User embeddings (24h TTL)
- API responses (1h TTL)

### Metrics

**Local Telemetry** (always on):
- Startup time, module load time
- Memory usage, CPU%, GPU%
- Dropped frames, ANR (app not responding) events
- Error rates, crash logs

**Opt-in Remote** (user consent required):
- Aggregated usage stats (daily active users, feature adoption)
- Performance benchmarks (P50, P95, P99 latencies)
- Error telemetry (sanitized stack traces)

**KPIs** (v1 success metrics):
- TTI (Time to Interactive): <2s (P95)
- Module switch latency: <300ms (P95)
- Search query latency: <500ms (P95)
- AI agent turnaround: <3s (P95)
- Crash-free rate: >99.5%

### Offline Mode

**Degraded Functionality**:
- Streaming: Play cached/downloaded content only
- Terminal: Fully functional (local shell)
- Social: Read cached feeds, queue posts for later sync
- Financial: Show cached quotes with staleness warning
- Search: Local index only, no web metasearch
- AI: Local models only, summarizer and embeddings work

**Queue Actions**:
- Posts, likes, saves queued and synced when online
- Retry with exponential backoff
- User notification on sync failure

### Update Strategy

**Mechanism**: Tauri's built-in updater with Sparkle/Squirrel
- **Auto-updates**: Enabled by default, check on launch
- **Signed Releases**: Code signing with EV certificates
- **Delta Patches**: Binary diffs for small updates (<10MB)
- **Rollback**: Automatic on crash loop (3 consecutive crashes)

**Channels**:
- **Stable**: Monthly releases, battle-tested
- **Beta**: Weekly releases, early access
- **Nightly**: Daily builds, bleeding edge (opt-in)

**User Controls**:
- Disable auto-updates (manual check only)
- Rollback to previous version
- View changelog before installing

---

## Development & Testing Strategy

### Toolchain

**Core Stack**:
- **Host**: Tauri 2.x (Rust 1.77+)
- **Frontend**: Svelte 5 (runes) + Vite 7
- **Styles**: UnoCSS (Tailwind-compatible)
- **State**: Zustand (shared state across modules)
- **IPC**: tRPC (type-safe RPC over Tauri commands)

**Monorepo**:
- **Manager**: pnpm 10.16.0
- **Structure**: `packages/` (shared libs), `modules/` (features), `examples/` (demos)

### Testing

**Unit Tests** (95% coverage target):
- Rust: `cargo test` (tokio runtime)
- TypeScript: Vitest (fast, Vite-native)

**Integration Tests**:
- Tauri commands end-to-end
- Module IPC communication
- Auth flows (mocked OAuth)

**E2E Tests** (Playwright):
- Critical user journeys (login, stream, search)
- Headless-compatible (Xvfb on Linux)
- Visual regression (Percy/Chromatic)

**Performance Tests**:
- Lighthouse CI (startup, TTI)
- Memory profiling (Chrome DevTools)
- Load testing (concurrent module spawns)

### CI/CD

**Pipeline** (GitHub Actions):
1. Lint (ESLint, Clippy, Prettier)
2. Test (unit, integration)
3. Build (all platforms)
4. E2E (smoke tests)
5. Package (installers, notarization)
6. Deploy (to release channels)

**Artifacts**:
- Signed installers (.dmg, .exe, .AppImage)
- Checksums (SHA256)
- Release notes (auto-generated from commits)

---

## Roadmap & Milestones

### v1.0 — Foundation (Q4 2025)
- ✅ Selective Electron integration (Pattern A)
- ✅ M1: Streaming Hub (YouTube, Twitch)
- [ ] M2: Terminal/CLI
- [ ] M5: Unified Search
- [ ] M6: Settings + Privacy Diary
- [ ] Auth: Google, GitHub OAuth2
- [ ] AI: Local embeddings + cloud summarizer
- [ ] Security: Full sandboxing, encrypted secrets
- [ ] Performance: <2s startup, <800MB memory

### v1.5 — Expansion (Q1 2026)
- [ ] Pattern B: Separate Electron module with auto-updates
- [ ] M3: Social Aggregator (Twitter/X, Reddit)
- [ ] M4: Financial Research (stocks, crypto)
- [ ] Multi-account support (up to 5 profiles)
- [ ] Mobile companion app (notifications, remote control)
- [ ] Enterprise SSO (Okta, Azure AD)

### v2.0 — Intelligence (Q2 2026)
- [ ] Multi-device sync (CRDTs via LiteFS)
- [ ] Advanced AI agents (researcher, automator)
- [ ] Custom dashboards (widget builder)
- [ ] TV apps (Apple TV, Android TV)
- [ ] Knowledge base module (note-taking with backlinks)
- [ ] Performance: <1s startup, <500MB memory

---

## Open Questions & Decision Log

### Awaiting Decisions

1. **DRM Licensing** (blocker for M1 advanced):
   - Widevine licensing terms for indie app
   - Distribution requirements (CDM delivery)
   - Cost model (per-install, flat fee, revenue share?)

2. **Cloud Infrastructure** (needed for v1.5 sync):
   - Hosting provider (AWS, Cloudflare, self-hosted?)
   - CDN for updates and assets
   - Backup storage for encrypted user data

3. **Monetization** (v2.0):
   - Freemium model (free tier + paid features)?
   - One-time purchase vs. subscription?
   - Enterprise licensing for multi-seat deployments?

### Resolved Decisions

_(Will be populated as questions are answered)_

---

## References & Resources

### External Docs
- [Tauri Documentation](https://tauri.app/v2/guides/)
- [Electron Security Best Practices](https://www.electronjs.org/docs/latest/tutorial/security)
- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [OWASP Desktop App Security](https://owasp.org/www-project-desktop-app-security-top-10/)

### Internal Specs
- [Feature 001: Selective Electron Integration](./specs/001-selective-electron/spec.md)
- [Non-Regression Invariants](./spec.md)
- [Pattern B Implementation Plan](./specs/001-selective-electron/plan.md)

### Design Assets
- Figma: `[Link TBD]`
- Component Library: `packages/ui/README.md`
- Brand Guidelines: `docs/brand/guidelines.md`

---

## Changelog

### 2025-10-05
- Initial vision document created
- Defined v1.0, v1.5, v2.0 roadmap
- Specified module catalog and AI strategy
- Documented security, privacy, and performance targets

---

**Next Steps**: Review with stakeholders, prioritize v1.0 features, begin M1 (Streaming Hub) implementation.
