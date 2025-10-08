# Phase 2.4–2.6 Delegation Setup - Summary

## What Was Done

This PR sets up the infrastructure to delegate Phase 2.4, 2.5, and 2.6 Task 1 implementations to coding agents.

### 1. Created YOLO Open Issues Workflow
**File**: `.github/workflows/yolo-open-issues.yml`

A GitHub Actions workflow that:
- Parses `PHASE_2_{4,5,6}_TICKETS.md` files
- Extracts individual tasks from each phase
- Creates GitHub issues with appropriate titles, bodies, and labels
- Automatically adds the `yolo` label for YOLO Supervisor tracking
- Supports dry-run mode to preview what would be created
- Checks for duplicate issues before creating

**Usage**:
```
1. Go to Actions → YOLO Open Issues
2. Click "Run workflow"
3. Enter phases: "2.4,2.5,2.6"
4. Set dry_run to "false"
5. Click "Run workflow"
```

This will create 3 issues (one for each Task 1):
- Phase 2.4 Task 1: Update Server API (Axum) – Core Endpoints
- Phase 2.5 Task 1: Marketplace API Service (Catalog, Reviews, Submit)
- Phase 2.6 Task 1: Enterprise Management Server (Axum) – Multi-tenancy & RBAC

### 2. Enhanced PR Template
**File**: `.github/PULL_REQUEST_TEMPLATE.md`

Added required sections:
- **Linked Issue**: Enforces `Closes #123` format
- **Model Used**: Documents which AI model was used (optional but recommended)
- **Test Plan**: Requires description of testing performed
- **Checklist**: Standard items plus Phase 2.4–2.6 specific checks

### 3. Created PR Guard Workflow
**File**: `.github/workflows/pr-guard.yml`

A validation workflow that runs on every PR and checks:
- ✅ Linked issue exists and is referenced correctly
- ✅ Test Plan section is present and has content
- ✅ Checklist section is present
- ⚠️ Model Used section (warning if missing for YOLO PRs)
- ⚠️ PR title follows conventional commit or phase format
- ⚠️ Phase PRs have `yolo` label

Posts a validation report as a comment and fails the check if errors are found.

### 4. Created Delegation Guide
**File**: `specs/001-selective-electron/DELEGATION_GUIDE.md`

Comprehensive guide covering:
- How to create issues using the workflow
- Manual issue creation steps (if workflow unavailable)
- PR requirements and structure
- Implementation details for each Task 1
- CI and review process
- Model recommendations
- Verification checklist

## What Needs to Happen Next

### Step 1: Create the Issues
Run the YOLO Open Issues workflow as described above. This will create 3 GitHub issues.

### Step 2: Create 3 Separate PRs

For each issue, create a new PR in a separate branch:

#### PR 1: Phase 2.4 Task 1
**Branch**: `phase-2.4-task-1`  
**Implementation**: Create `crates/update-server/` with:
- Core REST API endpoints for module metadata and downloads
- Token-based authentication middleware
- Storage abstraction trait (R2/S3/local)
- Rate limiting
- Comprehensive tests

#### PR 2: Phase 2.5 Task 1
**Branch**: `phase-2.5-task-1`  
**Implementation**: Create `crates/marketplace-api/` with:
- Catalog endpoints (list, get, versions)
- Review and submission endpoints
- Postgres schema and migrations
- Validation and rate limiting
- Comprehensive tests

#### PR 3: Phase 2.6 Task 1
**Branch**: `phase-2.6-task-1`  
**Implementation**: Create `crates/enterprise-server/` with:
- Multi-tenant organization model
- RBAC (roles, permissions, bindings)
- Policy engine scaffolding
- Audit log middleware
- Organization management endpoints
- Postgres migrations
- Comprehensive tests

### Step 3: Each PR Must
1. Reference the corresponding issue: `Closes #{issue_number}`
2. Fill out the PR template completely
3. Add the `yolo` label
4. Pass all CI checks:
   - `check formatting`
   - `lint rust`
   - `CI (Selective Electron)` - includes headless launch test with screenshot
   - `PR Guard` - validates PR template compliance
5. Include comprehensive tests
6. Follow repository conventions (see `.github/copilot-instructions.md`)

### Step 4: YOLO Supervisor Will
- Monitor workflow runs
- Auto-rerun once on first failure
- Add `needs-investigation` label and guidance on failures
- Add `ready-for-review` label when all checks pass
- Auto-merge if `automerge` label is present

## CI/CD Pipeline

The existing CI pipeline (`.github/workflows/ci.yml`) already includes:
- ✅ Building the example API app
- ✅ Running integration tests (headless-safe)
- ✅ Building with Tauri CLI
- ✅ **Headless launch test with Xvfb**
- ✅ **Screenshot artifact upload** (`headless-screenshot`)
- ✅ Download smoke test

The YOLO Supervisor monitors these workflows and reacts to failures.

## Architecture Notes

These are **backend services** (Rust crates under `crates/`), not Tauri app features:
- Use **Axum** for HTTP servers
- Use **sqlx** + Postgres for databases
- Use **tokio** for async runtime
- Follow Tauri monorepo conventions
- NO Electron runtime dependencies
- These services will eventually be used by Tauri apps via HTTP APIs

## Validation Checklist

After all 3 PRs are merged:
- [ ] Issue #X (Phase 2.4 Task 1) is closed
- [ ] Issue #Y (Phase 2.5 Task 1) is closed
- [ ] Issue #Z (Phase 2.6 Task 1) is closed
- [ ] `crates/update-server/` exists and `cargo build -p update-server` succeeds
- [ ] `crates/marketplace-api/` exists and `cargo build -p marketplace-api` succeeds
- [ ] `crates/enterprise-server/` exists and `cargo build -p enterprise-server` succeeds
- [ ] All tests pass: `cargo test -p update-server`, `cargo test -p marketplace-api`, `cargo test -p enterprise-server`
- [ ] Documentation exists: Each crate has a comprehensive README.md

## References

- **Ticket Files**: 
  - `specs/001-selective-electron/PHASE_2_4_TICKETS.md`
  - `specs/001-selective-electron/PHASE_2_5_TICKETS.md`
  - `specs/001-selective-electron/PHASE_2_6_TICKETS.md`
- **Delegation Guide**: `specs/001-selective-electron/DELEGATION_GUIDE.md`
- **Workflows**: `.github/workflows/yolo-*.yml`, `.github/workflows/pr-guard.yml`
- **Repository Guidelines**: `.github/copilot-instructions.md`
- **PR Template**: `.github/PULL_REQUEST_TEMPLATE.md`

## Model Recommendations

Based on task complexity and ticket specifications:
- **Phase 2.4 Task 1**: `openai/gpt-4.1` or `openai/gpt-5` (REST API, auth, storage abstraction)
- **Phase 2.5 Task 1**: `openai/gpt-4.1` or `openai/gpt-5` (REST API, database, migrations)
- **Phase 2.6 Task 1**: `openai/o3` or `openai/gpt-5` (complex RBAC, audit, multi-tenancy)
