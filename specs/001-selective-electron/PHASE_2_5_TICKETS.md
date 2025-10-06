# Phase 2.5 – Agent-Ready Tickets (Marketplace & Discovery)

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Run independent tickets in parallel; sequence those with explicit dependencies. Keep PRs focused, with passing build/tests and updated docs.

Conventions:
- Backend services (Axum/Rust) under `crates/`, frontend (React/TS) under `packages/`
- Search: Elasticsearch (or OpenSearch) client; Redis optional cache
- Security: signature verification remains mandatory for install
- Observability: `tracing` with request IDs; scrub PII

---

## [Phase 2.5][Task 1] Marketplace API Service (Catalog, Reviews, Submit)

- Title: Implement `marketplace-api` with core endpoints for modules, reviews, and submissions
- Goal: Provide backend marketplace catalog service

Scope:
- Endpoints:
  - GET `/v1/modules` (filter/sort/paginate)
  - GET `/v1/modules/{id}`
  - GET `/v1/modules/{id}/versions`
  - POST `/v1/modules/{id}/reviews`
  - POST `/v1/submissions` (developer submit)
- Postgres schema + migrations; validation; rate limits

Out of scope:
- Search index (Task 2); recommendations (Task 3)

Paths:
- New: `crates/marketplace-api/`
- New: `crates/marketplace-api/src/{routes,models,db}.rs`
- New: `crates/marketplace-api/migrations/*.sql`

Implementation notes:
- Strong types; DTOs; error codes; auth stub (JWT/opaque token)

Definition of Done:
- Service runs; endpoints return expected JSON; migrations apply

Tests:
- Route tests; validation errors; pagination

Manual validation:
- Seed script; curl module list/detail; post a review

Dependencies: none

Estimate: 4 days

Labels: phase-2.5, backend, rust

Acceptance criteria:
- Stable API contract and DB schema with constraints

---

## [Phase 2.5][Task 2] Search Engine Integration (Elasticsearch)

- Title: Index modules and wire search endpoints
- Goal: Provide fast, relevant discovery

Scope:
- Index mapping; indexer job on module changes
- Search endpoint: GET `/v1/search?q=...&filters=...`
- Fuzzy match, boosts, synonym support; pagination

Out of scope:
- Recommendations (Task 3)

Paths:
- Update: `crates/marketplace-api/src/routes/search.rs`
- New: `crates/marketplace-api/src/search/{indexer,client}.rs`

Implementation notes:
- Use official ES client; backoff on cluster errors; circuit breaker

Definition of Done:
- Search returns relevant results with filters and scores

Tests:
- Index/refresh; query relevancy fixtures

Manual validation:
- Seed modules; try varied queries; assess results

Dependencies: Task 1

Estimate: 3 days

Labels: phase-2.5, search, elastic

Acceptance criteria:
- Relevance and performance within expected bounds

---

## [Phase 2.5][Task 3] Recommendation Engine (Collaborative + Content-based)

- Title: Basic recommendation service and API endpoints
- Goal: Improve discovery via personalized and popularity-based suggestions

Scope:
- Signals: downloads, ratings, categories
- Algorithms: item-item similarity + popularity fallback
- Endpoints: GET `/v1/recommendations?userId=...`

Out of scope:
- Advanced ML models

Paths:
- New: `crates/recommendations/`
- Update: `crates/marketplace-api/src/routes/recommendations.rs`

Implementation notes:
- Batch job to compute similarities; cache in Redis (optional)

Definition of Done:
- Deterministic recommendations with sensible fallbacks

Tests:
- Small synthetic dataset; verify expected neighbors

Manual validation:
- Seed actions; call endpoint; sanity-check outputs

Dependencies: Task 1

Estimate: 3 days

Labels: phase-2.5, recommendations

Acceptance criteria:
- Reasonable recommendations with stable latency

---

## [Phase 2.5][Task 4] Tauri Plugin: Marketplace Integration

- Title: Expose marketplace catalog/search/recommendations in app
- Goal: Make marketplace consumable from the desktop app

Scope:
- New crate `crates/tauri-plugin-marketplace/`
- Commands: `marketplace_list`, `marketplace_get`, `marketplace_search`, `marketplace_recommend`

Out of scope:
- Install/update (Phase 2.3 plugin handles that)

Paths:
- New: `crates/tauri-plugin-marketplace/`

Implementation notes:
- Paginated results; error mapping; timeouts

Definition of Done:
- Example app shows listing/search/recommendations

Tests:
- Command tests with mocks

Manual validation:
- Run example and validate data flows

Dependencies: Tasks 1–3

Estimate: 2 days

Labels: phase-2.5, tauri, plugin

Acceptance criteria:
- Stable commands; example app compiles and runs

---

## [Phase 2.5][Task 5] Web Marketplace UI (React/TS)

- Title: Implement public marketplace frontend
- Goal: Search, browsable catalog, module detail with reviews and versions

Scope:
- Pages: Home, Search, Module detail, Submit module
- Components: ModuleCard, FilterBar, ReviewList, Pagination

Out of scope:
- Developer portal analytics (can be later)

Paths:
- New: `packages/marketplace-web/`

Implementation notes:
- Use registry/install plugins for deep links into app

Definition of Done:
- Builds; basic accessibility; client-side routing

Tests:
- Component tests; e2e smoke (Playwright optional)

Manual validation:
- Browse catalog; search; view a module; submit dummy

Dependencies: Tasks 1–2

Estimate: 4 days

Labels: phase-2.5, web, react

Acceptance criteria:
- Functional marketplace shell with core flows

---

## [Phase 2.5][Task 6] In-app Browser / UI Surfaces

- Title: Integrate marketplace in-app surfaces in desktop
- Goal: Provide discovery without leaving the app

Scope:
- UI surfaces: sidebar entry, marketplace view, module detail drawer
- Deep links from web to app via custom protocol

Out of scope:
- Purchase/monetization

Paths:
- Update: desktop app frontend (path TBD), add views/components

Implementation notes:
- Reuse marketplace plugin; handle auth state gracefully

Definition of Done:
- In-app browsing of catalog and detail

Tests:
- UI behavior tests; protocol handler test

Manual validation:
- Navigate to marketplace from app; click through to details

Dependencies: Task 4

Estimate: 3 days

Labels: phase-2.5, ui, integration

Acceptance criteria:
- Smooth in-app experience; no broken back/forward flows

---

## [Phase 2.5][Task 7] Moderation and Content Flags

- Title: Add review moderation, content flags, and abuse reporting
- Goal: Keep marketplace healthy and compliant

Scope:
- Admin endpoints: flag review/module; approve/reject submissions
- UI: admin moderation queue; user report button

Out of scope:
- Full enterprise policies (Phase 2.6)

Paths:
- Update: `crates/marketplace-api` (admin routes, models)
- Update: `packages/marketplace-web` (admin pages)

Implementation notes:
- Soft-delete patterns; audit fields; pagination

Definition of Done:
- Moderation flows working; state transitions logged

Tests:
- Admin actions; visibility changes; abuse reports

Manual validation:
- Flag/approve/reject from UI; verify state in DB

Dependencies: Tasks 1, 5

Estimate: 3 days

Labels: phase-2.5, moderation, compliance

Acceptance criteria:
- Clear, auditable moderation with least privilege

---

## [Phase 2.5][Task 8] Developer Portal: Submissions & Analytics (MVP)

- Title: Create basic developer portal for submissions and simple analytics
- Goal: Empower developers to submit and monitor modules

Scope:
- Pages: My Modules, Submit New, Basic Analytics (installs, ratings)
- API integration for submissions and metrics

Out of scope:
- Billing/monetization

Paths:
- New: `packages/dev-portal/`
- Update: `crates/marketplace-api` endpoints as needed

Implementation notes:
- Auth stub (JWT/session); role checks

Definition of Done:
- Developer can submit, view module status, and see basic charts

Tests:
- Component/unit tests; basic e2e happy path

Manual validation:
- Walk through submission to listing with moderation approval

Dependencies: Tasks 1, 2, 7

Estimate: 4 days

Labels: phase-2.5, portal, ux

Acceptance criteria:
- End-to-end developer flow working with minimal friction

---

## Parallelization and Ordering

- Start: Task 1 (API), Task 2 (Search)
- Then: Task 3 (Recs), Task 4 (Plugin)
- Then: Task 5 (Web UI), Task 7 (Moderation)
- Then: Task 6 (In-app UI), Task 8 (Dev Portal)

## Global Definition of Done for Phase 2.5
- Marketplace backend provides catalog, search, recommendations, moderation, and submissions
- Tauri plugin and in-app UI surfaces enable discovery inside the desktop app
- Public web marketplace and dev portal deliver core user flows
- Tests green, docs updated, and endpoints stable
