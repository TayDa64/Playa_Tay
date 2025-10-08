# Phase 2.4–2.6 Task 1 - Getting Started

This directory contains the ticket files and delegation infrastructure for Phases 2.4, 2.5, and 2.6.

## Quick Start

### Step 1: Create Issues

Run the YOLO Open Issues workflow to create GitHub issues for Task 1 of each phase:

1. Go to **Actions** → **YOLO Open Issues**
2. Click **Run workflow**
3. Set parameters:
   - `phases`: `2.4,2.5,2.6`
   - `dry_run`: `false`
4. Click **Run workflow**

This will create 3 issues:
- `[Phase 2.4][Task 1] Update Server API (Axum) – Core Endpoints`
- `[Phase 2.5][Task 1] Marketplace API Service (Catalog, Reviews, Submit)`
- `[Phase 2.6][Task 1] Enterprise Management Server (Axum) – Multi-tenancy & RBAC`

### Step 2: Implement Task 1 (Per Phase)

For each issue created, create a separate PR in its own branch:

#### Phase 2.4 Task 1
```bash
git checkout -b phase-2.4-task-1 dev
# Implement crates/update-server/
# Add tests, documentation
git commit -am "feat: implement update-server crate"
git push origin phase-2.4-task-1
# Open PR with title: [Phase 2.4][Task 1] Update Server API (Axum) – Core Endpoints
# Add yolo label
```

#### Phase 2.5 Task 1
```bash
git checkout -b phase-2.5-task-1 dev
# Implement crates/marketplace-api/
# Add tests, documentation
git commit -am "feat: implement marketplace-api crate"
git push origin phase-2.5-task-1
# Open PR with title: [Phase 2.5][Task 1] Marketplace API Service
# Add yolo label
```

#### Phase 2.6 Task 1
```bash
git checkout -b phase-2.6-task-1 dev
# Implement crates/enterprise-server/
# Add tests, documentation
git commit -am "feat: implement enterprise-server crate"
git push origin phase-2.6-task-1
# Open PR with title: [Phase 2.6][Task 1] Enterprise Management Server
# Add yolo label
```

### Step 3: Fill PR Template

Each PR must include:
- **Linked Issue**: `Closes #{issue_number}`
- **Model Used**: e.g., `openai/gpt-4.1`
- **Test Plan**: How you tested the implementation
- **Checklist**: Complete all items

### Step 4: Monitor CI

The YOLO Supervisor will:
- Auto-rerun once on first failure
- Add `needs-investigation` label and guidance if failing
- Add `ready-for-review` label when all checks pass
- Automerge if `automerge` label is present

## Files in This Directory

- **PHASE_2_4_TICKETS.md** - All tasks for Phase 2.4 (Distribution & CDN)
- **PHASE_2_5_TICKETS.md** - All tasks for Phase 2.5 (Marketplace & Discovery)
- **PHASE_2_6_TICKETS.md** - All tasks for Phase 2.6 (Enterprise & Governance)
- **DELEGATION_GUIDE.md** - Complete delegation process documentation
- **PHASE_2_X_PLAN.md** - Implementation plans for each phase

## Implementation Details

### Phase 2.4 Task 1: Update Server API
- Create `crates/update-server/` with Axum HTTP server
- Endpoints: modules list, releases, downloads, changelog, signature verification
- Token-based auth, rate limiting, storage abstraction
- Estimate: 4 days

### Phase 2.5 Task 1: Marketplace API
- Create `crates/marketplace-api/` with Axum HTTP server
- Endpoints: catalog, reviews, submissions
- Postgres database with migrations
- Estimate: 4 days

### Phase 2.6 Task 1: Enterprise Management Server
- Create `crates/enterprise-server/` with Axum HTTP server
- Multi-tenant org model, RBAC, policies, audit logs
- Postgres database with migrations
- Estimate: 4 days

## Architecture Notes

These are **Rust backend services**, not Tauri app features:
- Use **Axum** for HTTP servers
- Use **sqlx** + Postgres for databases
- Use **tokio** for async runtime
- NO Electron runtime dependencies
- Services will be consumed by Tauri apps via HTTP APIs

## Model Recommendations

Based on task complexity:
- **Phase 2.4**: `openai/gpt-4.1` or `openai/gpt-5`
- **Phase 2.5**: `openai/gpt-4.1` or `openai/gpt-5`
- **Phase 2.6**: `openai/o3` or `openai/gpt-5` (complex RBAC and multi-tenancy)

## References

- **Setup Summary**: `/SETUP_SUMMARY.md` (repository root)
- **Delegation Guide**: `DELEGATION_GUIDE.md` (this directory)
- **Workflows**: `.github/workflows/yolo-*.yml`, `.github/workflows/pr-guard.yml`
- **PR Template**: `.github/PULL_REQUEST_TEMPLATE.md`
- **Repository Guidelines**: `.github/copilot-instructions.md`

## Need Help?

See the comprehensive guides:
- `DELEGATION_GUIDE.md` - Step-by-step process
- `/SETUP_SUMMARY.md` - Overview and validation checklist
