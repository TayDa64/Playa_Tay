# Phase 2.4 – Agent-Ready Tickets (Distribution & CDN)

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Run independent tickets in parallel; sequence those with explicit dependencies. Keep PRs focused, with passing build/tests and updated docs.

Conventions:
- Rust backend services under `crates/`; infra scripts under `infrastructure/` or `workers/`
- Use Axum for APIs, Cloudflare Workers for edge, and R2/S3-compatible storage
- Strong integrity guarantees: checksums + signatures are mandatory
- Observability with `tracing` and structured logs

---

## [Phase 2.4][Task 1] Update Server API (Axum) – Core Endpoints

- Title: Implement `update-server` crate with production-ready endpoints and auth
- Goal: Serve metadata, deltas, and downloads via authenticated API

Scope:
- Endpoints:
  - GET `/v1/modules` (list, filters)
  - GET `/v1/modules/{id}/releases/latest`
  - GET `/v1/modules/{id}/releases/{version}`
  - GET `/v1/modules/{id}/releases/{version}/download`
  - GET `/v1/modules/{id}/changelog`
  - POST `/v1/verify/signature`
- Auth: token-based (server-to-server), rate limiting, request ID headers
- Storage abstraction for object stores (R2/S3/local)

Out of scope:
- Admin UI; developer upload portal (future phases)

Paths:
- New: `crates/update-server/` (upgrade skeleton from 2.1 if present)
- New: `crates/update-server/src/{routes,storage,auth}.rs`
- New: `crates/update-server/README.md`

Implementation notes:
- Add CORS for app; gzip/br; ETag for caching
- Structured errors with codes; OpenAPI spec (utoipa) optional

Definition of Done:
- Server runs locally; endpoints return correct JSON; download proxy works

Tests:
- Handler tests; auth middleware; rate limit behavior

Manual validation:
- Curl endpoints; verify headers (ETag, Cache-Control) and auth rejection path

Dependencies: none

Estimate: 4 days

Labels: phase-2.4, backend, rust, server

Acceptance criteria:
- Stable endpoints; storage behind trait; consistent error model

---

## [Phase 2.4][Task 2] Multi-CDN Manager with Failover

- Title: Implement `cdn-manager` crate to resolve best CDN URL and failover
- Goal: Resilient download link resolution across CloudFront/Fastly/R2

Scope:
- Strategy to choose CDN based on region, health, and policy
- Health checks; circuit breaker; backoff and fallback order
- Signed URL generation if required per provider

Out of scope:
- Actual provider account setup (manual/ops)

Paths:
- New: `crates/cdn-manager/`

Implementation notes:
- Provider trait: `resolve_url(object_key) -> Url`
- Cache health state; expose metrics

Definition of Done:
- Unit-tested resolution/failover logic; pluggable providers

Tests:
- Provider down → fallback; health recovers → primary restored

Manual validation:
- Simulate failures via mock provider; observe chosen URL

Dependencies: Task 1 (API uses manager)

Estimate: 3 days

Labels: phase-2.4, networking, rust

Acceptance criteria:
- Deterministic failover; minimal added latency

---

## [Phase 2.4][Task 3] Delta Artifact Generation Pipeline

- Title: Generate delta patches for new releases and publish to storage
- Goal: Reduce bandwidth with precomputed deltas

Scope:
- Batch job or CLI that computes deltas between consecutive versions
- Writes patch artifacts + manifest to object storage
- Hooks for update-server to advertise delta availability

Out of scope:
- Delta application (Phase 2.3)

Paths:
- New: `crates/delta-publisher/`
- New: `crates/delta-publisher/src/main.rs`
- Update: `crates/update-server` to surface delta info in responses

Implementation notes:
- Integrate with `delta-engine` crate from Phase 2.3
- Concurrency control; skip if patch size > threshold of full

Definition of Done:
- Running the publisher outputs patches and manifest; server shows deltas

Tests:
- Fixture versions produce non-empty patch; size threshold logic works

Manual validation:
- Run against sample versions; inspect storage for patch + manifest

Dependencies: Phase 2.3 (delta-engine)

Estimate: 3 days

Labels: phase-2.4, updates, pipeline

Acceptance criteria:
- Safe, idempotent batch generation; clear logging and exit codes

---

## [Phase 2.4][Task 4] Cloudflare Worker: Download/Verify Edge Layer

- Title: Implement edge worker routing downloads and optional signature pre-check
- Goal: Edge acceleration with basic verification and analytics capture

Scope:
- Routes: `/download/*`, `/verify/*`
- Fetch from origin/storage; set cache headers; log metrics; rate-limit
- Optional lightweight signature check proxying to update-server

Out of scope:
- Complex business logic (keep thin)

Paths:
- New: `workers/cloudflare/worker.ts`
- New: `workers/cloudflare/wrangler.toml.example`
- Update: `workers/cloudflare/README.md`

Implementation notes:
- Use KV for counters; consider Durable Objects for per-IP tokens if needed

Definition of Done:
- `wrangler dev` serves local; handlers return expected status and headers

Tests:
- Unit tests with miniflare for caching and rate-limit logic

Manual validation:
- Download path proxies to local server; analytics counter increments

Dependencies: Task 1 (API contract), Task 2 (URL resolution)

Estimate: 3 days

Labels: phase-2.4, edge, cloudflare

Acceptance criteria:
- Low-latency proxying; safe guardrails; deployable config

---

## [Phase 2.4][Task 5] Download Analytics Collector Service

- Title: Implement a lightweight ingestion service for download/update events
- Goal: Enable dashboards on adoption, failures, and regional performance

Scope:
- New crate: `crates/analytics-collector/` (Axum)
- Endpoint to ingest events (JSON batch)
- Writes to Postgres (or ClickHouse optional) with partitioning

Out of scope:
- Fancy dashboards (basic SQL + placeholders)

Paths:
- New: `crates/analytics-collector/`
- New: `crates/analytics-collector/src/{routes,models}.rs`

Implementation notes:
- Define event schema (download_started/completed, bytes, duration, region, error)
- Backpressure and rate limiting

Definition of Done:
- Service accepts and stores events; simple query endpoints return aggregates

Tests:
- Ingest happy/invalid payloads; partitioning verified

Manual validation:
- Post sample batch; query aggregates; confirm counts

Dependencies: none (but integrates with Worker/Update server later)

Estimate: 3 days

Labels: phase-2.4, analytics, rust

Acceptance criteria:
- Reliable ingestion; basic aggregates reproducible

---

## [Phase 2.4][Task 6] Update Server ↔ CDN/Edge Wiring

- Title: Connect update-server to CDN manager and expose signed/failover URLs
- Goal: Server responses carry optimal, resilient download URLs

Scope:
- Use `cdn-manager` in download endpoints
- Add signed URL support with TTL; include regional hints
- Ensure headers support CDN caching (ETag, Cache-Control)

Out of scope:
- Provider account provisioning

Paths:
- Update: `crates/update-server/src/routes/*.rs`
- Update: `crates/cdn-manager/` if needed

Implementation notes:
- Include alt URLs array for clients that want their own retry strategy

Definition of Done:
- Responses contain primary + alternates; TTL respected; signatures valid

Tests:
- Unit tests for response shapes; integration test with mock providers

Manual validation:
- Curl response and validate URL reachability (mock)

Dependencies: Tasks 1–2

Estimate: 2 days

Labels: phase-2.4, distribution, resiliency

Acceptance criteria:
- Clear, robust URL strategy from server to clients

---

## [Phase 2.4][Task 7] Delta Generation CI Job and Release Hooks

- Title: Add CI job to generate/publish deltas on release tags
- Goal: Automate patch creation and publishing as part of release

Scope:
- GitHub Actions workflow invoking `delta-publisher`
- Upload artifacts to storage; invalidate CDN paths if needed

Out of scope:
- Full release orchestration beyond deltas

Paths:
- New: `.github/workflows/delta-publish.yml`
- Update: `crates/delta-publisher/README.md`

Implementation notes:
- Secrets and buckets via env; retries; notifications on failure

Definition of Done:
- Tag push triggers delta build; artifacts present in storage; logs attached

Tests:
- Workflow run on a dry-run repo fixture (if feasible)

Manual validation:
- Push a test tag; verify patches in bucket

Dependencies: Task 3

Estimate: 2 days

Labels: phase-2.4, ci, updates

Acceptance criteria:
- Reliable CI step with clear diagnostics and rollback plan

---

## [Phase 2.4][Task 8] Observability: Tracing, Metrics, Dashboards

- Title: Add tracing/metrics across update-server, CDN manager, worker, and analytics
- Goal: End-to-end visibility into distribution health and performance

Scope:
- Add `tracing` spans; metrics via Prometheus exporter (or OpenTelemetry)
- Prebuilt Grafana dashboards JSON and example docker-compose

Out of scope:
- Production monitoring setup (ops)

Paths:
- Update: affected crates to emit traces/metrics
- New: `observability/docker-compose.yml`
- New: `observability/grafana/dashboards/*.json`

Implementation notes:
- Correlate with `X-Request-Id`; include CDN selection metrics and error rates

Definition of Done:
- Local docker-compose shows metrics; dashboards render key panels

Tests:
- Basic scrape test; unit tests assert metrics increment

Manual validation:
- Run stack; induce failures; see metrics reflect reality

Dependencies: Tasks 1–6

Estimate: 3 days

Labels: phase-2.4, observability, metrics

Acceptance criteria:
- Actionable dashboards; low overhead when disabled

---

## Parallelization and Ordering

- Start: Task 1 (Update Server), Task 2 (CDN Manager), Task 5 (Analytics Collector)
- Then: Task 4 (Worker), Task 6 (Server ↔ CDN wiring)
- Then: Task 3 (Delta Publisher), Task 7 (CI Job)
- Finally: Task 8 (Observability)

## Global Definition of Done for Phase 2.4
- Update server exposes stable endpoints with auth and caching; CDN manager provides resilient links
- Cloudflare Worker accelerates downloads with guardrails; delta artifacts generated automatically on release
- Analytics collector ingests events; observability stack surfaces health and performance
- CI workflows green; docs updated where applicable
