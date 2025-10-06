# Phase 2.6 – Agent-Ready Tickets (Enterprise Features & Governance)

Note to all agents (mandatory): Before starting, read and follow `.github/copilot-instructions.md` in this repo. Honor Tauri-first architecture, avoid `any` in TS, and do not use `sleep` in terminal commands. Prefer Rust for backend services; Electron is for compatibility only.

This file contains copy/paste-ready tickets for “Delegate to coding agent.” Each ticket is sized for a single PR. Parallelize independent tickets; sequence those with explicit dependencies.

---

## [Phase 2.6][Task 1] Enterprise Management Server (Axum) – Multi-tenancy & RBAC

- Title: Create `enterprise-server` with org isolation, RBAC, policies, and audit log foundation
- Goal: Central control plane for enterprise organizations

Scope:
- New crate: `crates/enterprise-server/`
- Features:
  - Multi-tenant org model; per-org settings
  - RBAC roles/permissions; user-role bindings
  - Policy engine scaffolding (allowlist/blocklist/approval/version-pin types)
  - Audit log schema and middleware (request/user/action)
  - Org CRUD, users/roles endpoints, policies CRUD

Out of scope:
- SSO (Task 3), private registry storage (Task 4), compliance reports (Task 6)

Paths:
- New: `crates/enterprise-server/src/{main.rs,rbac.rs,policy.rs,audit.rs,models.rs,routes/*.rs}`
- New: `crates/enterprise-server/migrations/*.sql`
- Update: workspace `Cargo.toml`

Implementation notes:
- Use Postgres via `sqlx`; add middleware for org scoping and request IDs
- Emit structured audit events on mutations

Definition of Done:
- Server runs; org/role/policy CRUD works; audit entries created

Tests:
- RBAC checks; policy serialization; audit insertion; tenancy scoping

Manual validation:
- Create org → roles → policies; perform action; check audit log row

Dependencies: none

Estimate: 4 days

Labels: phase-2.6, backend, rbac, audit

Acceptance criteria:
- Strong tenancy isolation; stable endpoints; comprehensive audit coverage

---

## [Phase 2.6][Task 2] Private Module Registry per Organization

- Title: Implement per-org private registry API and storage policy enforcement
- Goal: Allow enterprises to host private modules with access control

Scope:
- Endpoints: list/upload/delete private modules per org
- Storage backends: S3/R2/local with per-org prefixes and ACL
- Access control integrated with RBAC

Out of scope:
- CDN setup (Phase 2.4 covers distribution)

Paths:
- Update: `crates/enterprise-server/src/routes/registry.rs`
- New: `crates/enterprise-server/src/storage/{mod.rs,s3.rs,local.rs}`

Implementation notes:
- Validate signatures on upload; write metadata into enterprise DB
- Quotas and size limits in org settings

Definition of Done:
- Upload and list work with RBAC checks; objects stored under org namespace

Tests:
- Upload happy path; unauthorized access blocked; quota exceeded error

Manual validation:
- Upload sample module; verify metadata row and object in bucket

Dependencies: Task 1

Estimate: 3 days

Labels: phase-2.6, storage, security

Acceptance criteria:
- Isolated per-org storage with enforced permissions and signatures

---

## [Phase 2.6][Task 3] SSO Integration (SAML 2.0 + OIDC) with MFA Hooks

- Title: Add SSO providers and session management to enterprise-server
- Goal: Authenticate enterprise users via IdPs

Scope:
- Add OIDC (Auth Code + PKCE) and SAML 2.0
- Session cookie + CSRF protection; MFA-required policy bit
- IdP config per org (metadata, client IDs, certs)

Out of scope:
- Fine-grained device posture checks

Paths:
- Update: `crates/enterprise-server/src/routes/auth.rs`
- New: `crates/enterprise-server/src/auth/{oidc.rs,saml.rs,session.rs}`

Implementation notes:
- Use secure cookies; rotate session secrets; short-lived sessions
- Map IdP claims to roles optionally via attribute mapping

Definition of Done:
- Login flows complete with Okta/Azure AD; sessions created; logout works

Tests:
- Mock IdP flows; invalid assertion/replay attacks rejected

Manual validation:
- Configure dev IdP; complete login; protected route returns data

Dependencies: Task 1

Estimate: 4 days

Labels: phase-2.6, security, sso

Acceptance criteria:
- Reliable SSO with major IdPs; MFA gate honored by policy

---

## [Phase 2.6][Task 4] Air-Gapped Deployment Tools (Bundle, Mirror, Install)

- Title: Create `airgap-tools` CLI for offline bundles and registry mirrors
- Goal: Support disconnected enterprise environments

Scope:
- Commands: `bundle`, `mirror`, `install`
- Manifest generation; signature verification during install

Out of scope:
- GUI; only CLI in this ticket

Paths:
- New: `crates/airgap-tools/`

Implementation notes:
- Tar.gz bundles with manifest; verify all signatures before install

Definition of Done:
- CLI builds; commands function against fixtures

Tests:
- Bundle/Install round-trip; mirror creation with catalog JSON

Manual validation:
- Produce a bundle from public/private modules; install to target dir

Dependencies: Task 2 (for private modules), Phase 2.2 (signature-verifier)

Estimate: 3 days

Labels: phase-2.6, cli, offline

Acceptance criteria:
- Reliable offline flows with integrity guarantees

---

## [Phase 2.6][Task 5] Admin Portal (React) – Users, Roles, Policies, Audit Viewer

- Title: Implement admin portal UI with core governance flows
- Goal: Enterprise admins can manage org and view compliance events

Scope:
- Pages: Dashboard, Users/Roles, Policies, Approvals, Audit Logs
- Components: tables/forms; RBAC-gated buttons

Out of scope:
- Deep analytics (Task 7 covers org analytics)

Paths:
- New: `packages/admin-portal/`

Implementation notes:
- Type-safe API client; pagination; optimistic updates where safe

Definition of Done:
- Connects to enterprise-server; CRUD flows and audit viewer work

Tests:
- Component tests; permissions rendering; basic e2e path

Manual validation:
- Create/edit role; add policy; see audit entry after an action

Dependencies: Task 1

Estimate: 4 days

Labels: phase-2.6, web, admin

Acceptance criteria:
- Usable admin UI with core governance flows, no `any`

---

## [Phase 2.6][Task 6] Compliance Reports (SOC 2, GDPR, HIPAA stubs)

- Title: Generate downloadable compliance reports from audit data
- Goal: Support audit readiness and reporting

Scope:
- Endpoints: `/v1/organizations/{org}/compliance/report?type=...&start=...&end=...`
- SOC 2 Type II control coverage; GDPR/HIPAA stubs with TODOs

Out of scope:
- Full legal review; provide stub templates and data wiring

Paths:
- Update: `crates/enterprise-server/src/routes/compliance.rs`

Implementation notes:
- Produce JSON and PDF via template engine (optional); track generation in audit

Definition of Done:
- Report endpoint returns structured data; includes control findings list

Tests:
- Query window validation; empty-range behavior; sample finding present

Manual validation:
- Generate SOC 2 report for seeded data; download; verify contents

Dependencies: Task 1 (audit events)

Estimate: 2 days

Labels: phase-2.6, compliance, reporting

Acceptance criteria:
- Actionable report artifacts; correct audit trail

---

## [Phase 2.6][Task 7] Organization Analytics & SLA Monitoring

- Title: Aggregate org-wide usage, install/update rates, and error budgets
- Goal: Provide operational visibility and SLA tracking

Scope:
- Metrics: active users, installed modules, update success rate, mean download time
- SLA alerts stubs: threshold breach markers
- Simple dashboards in admin portal

Out of scope:
- Pager/alerting integrations

Paths:
- Update: `crates/analytics-collector` to emit org-scoped metrics (Phase 2.4)
- Update: `packages/admin-portal` to visualize charts

Implementation notes:
- Pre-aggregate by day; expose `/analytics/*` endpoints in enterprise-server

Definition of Done:
- Charts render with seeded data; SLA state visible

Tests:
- Aggregation correctness; edge cases (no data)

Manual validation:
- Seed events; dashboard reflects metrics/sla flags

Dependencies: Task 1, Phase 2.4 (collector)

Estimate: 3 days

Labels: phase-2.6, analytics, sla

Acceptance criteria:
- Useful org analytics with clear SLA visualization

---

## [Phase 2.6][Task 8] Enterprise Packaging & On-Prem Deployment Docs

- Title: Author comprehensive enterprise deployment and operations docs
- Goal: Unblock enterprise adoption, including air-gapped and on-prem

Scope:
- Docs: SSO setup, RBAC model, policy definitions, private registry, air-gap, backups, DR
- On-prem reference: docker-compose and Helm skeletons (optional)

Out of scope:
- Full production Helm charts

Paths:
- New: `docs/enterprise/README.md`
- New: `docs/enterprise/{sso.md,rbac.md,policies.md,airgap.md,ops.md}`
- New: `deploy/examples/{docker-compose.yml,helm-skeleton/}`

Implementation notes:
- Include troubleshooting and security considerations

Definition of Done:
- Docs are complete, accurate, and pass a fresh-run validation

Tests:
- N/A (docs), but example compose should boot locally

Manual validation:
- Follow docs from clean checkout to working dev/on-prem setup

Dependencies: Tasks 1–7

Estimate: 3 days

Labels: phase-2.6, docs, ops

Acceptance criteria:
- Clear paths for enterprise rollouts, air-gapped installs, and ops

---

## Parallelization and Ordering

- Start: Task 1 (Enterprise server)
- Then: Task 2 (Private registry), Task 3 (SSO)
- Then: Task 4 (Air-gap tools), Task 5 (Admin portal)
- Then: Task 6 (Compliance), Task 7 (Analytics)
- Finally: Task 8 (Docs & packaging)

## Global Definition of Done for Phase 2.6
- Enterprise server provides multi-tenancy, RBAC, policies, and audit logging
- Private registries enforce access and signatures; SSO works with major IdPs
- Air-gapped tooling supports offline bundles and mirrors
- Admin portal delivers core governance workflows; compliance reports downloadable
- Org analytics and SLA monitoring available; docs enable enterprise rollouts

Note: When opening issues in yolo mode, add a checklist item to each issue: “Agent must read `.github/copilot-instructions.md` before starting and follow repo conventions (Tauri-first, TypeScript without `any`, no `sleep` in terminal).”
