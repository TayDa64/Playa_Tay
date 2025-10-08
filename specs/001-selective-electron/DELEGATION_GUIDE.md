# Phase 2.4–2.6 Task 1 Delegation Guide

## Overview

This guide explains how to delegate the first task of Phases 2.4, 2.5, and 2.6 to coding agents using the YOLO workflows.

## Prerequisites

- The YOLO Open Issues workflow exists at `.github/workflows/yolo-open-issues.yml`
- The YOLO Supervisor workflow exists at `.github/workflows/yolo-supervisor.yml`
- Phase ticket files exist: `PHASE_2_4_TICKETS.md`, `PHASE_2_5_TICKETS.md`, `PHASE_2_6_TICKETS.md`

## Step 1: Create Issues for Task 1

### Using the Workflow

1. Go to GitHub Actions → YOLO Open Issues workflow
2. Click "Run workflow"
3. Enter phases: `2.4,2.5,2.6`
4. Set dry_run to `false`
5. Run the workflow

This will create GitHub issues for each task in the specified phases.

### Manual Creation (if needed)

If the workflow is not available, issues can be created manually with the following structure:

#### Phase 2.4 Task 1
- **Title**: `[Phase 2.4][Task 1] Update Server API (Axum) – Core Endpoints`
- **Labels**: `phase-2.4`, `backend`, `rust`, `server`, `yolo`
- **Body**: (Content from PHASE_2_4_TICKETS.md Task 1 section)

#### Phase 2.5 Task 1
- **Title**: `[Phase 2.5][Task 1] Marketplace API Service (Catalog, Reviews, Submit)`
- **Labels**: `phase-2.5`, `backend`, `rust`, `yolo`
- **Body**: (Content from PHASE_2_5_TICKETS.md Task 1 section)

#### Phase 2.6 Task 1
- **Title**: `[Phase 2.6][Task 1] Enterprise Management Server (Axum) – Multi-tenancy & RBAC`
- **Labels**: `phase-2.6`, `backend`, `rbac`, `audit`, `yolo`
- **Body**: (Content from PHASE_2_6_TICKETS.md Task 1 section)

## Step 2: Create PRs for Each Task

For each issue created, a separate PR should be opened with:

### PR Requirements

1. **Branch naming**: `phase-{phase}-task-{task}` (e.g., `phase-2.4-task-1`)
2. **PR title**: Match the issue title
3. **PR description**: Use the template at `.github/PULL_REQUEST_TEMPLATE.md`
   - **Linked Issue**: `Closes #{issue_number}`
   - **Model used**: Specify the AI model used (e.g., `openai/gpt-4.1`)
   - **Test Plan**: Describe how the implementation was tested
   - **Checklist**: Complete all items
4. **Labels**: Add `yolo` label so the Supervisor monitors it
5. **Implementation**: Follow the task specification from the ticket file

### Phase 2.4 Task 1: Update Server API

Create a new Rust crate under `crates/update-server/` with:
- Core endpoints for module metadata and downloads
- Token-based authentication middleware
- Storage abstraction trait for R2/S3/local backends
- Rate limiting
- Tests for routes, auth, and rate limits

### Phase 2.5 Task 1: Marketplace API Service

Create a new Rust crate under `crates/marketplace-api/` with:
- Catalog endpoints (list, get, versions)
- Review submission endpoint
- Module submission endpoint
- Postgres schema and migrations
- Validation and rate limiting
- Tests for routes and validation

### Phase 2.6 Task 1: Enterprise Management Server

Create a new Rust crate under `crates/enterprise-server/` with:
- Multi-tenant organization model
- RBAC (roles/permissions/bindings)
- Policy engine scaffolding
- Audit log middleware
- Organization and user management endpoints
- Postgres migrations
- Tests for RBAC, policies, and audit logging

## Step 3: CI and Review Process

1. **CI checks**: Each PR will trigger:
   - `check formatting`
   - `lint rust`
   - `CI (Selective Electron)` with headless launch test
   
2. **YOLO Supervisor** will:
   - Monitor workflow runs
   - Auto-rerun once on first failure
   - Add `needs-investigation` label and guidance on failures
   - Add `ready-for-review` label when all checks pass
   - Auto-merge if `automerge` label is present

3. **PR Guard** will verify:
   - Linked issue exists and matches pattern
   - PR template is filled out
   - Test plan is provided
   - Checklist is complete

## Implementation Notes

- **Tauri-first**: These are backend services (Rust), not Tauri app features
- **No Electron**: Electron packages exist for compatibility only; these tasks are pure Rust
- **Database**: Use `sqlx` with Postgres for all database needs
- **Authentication**: Stub auth for now; production auth comes in later phases
- **Testing**: Write integration tests; avoid mocks where possible
- **Documentation**: Include README.md in each crate with setup and usage instructions

## Suggested Models

From the ticket files, recommended models for Copilot agents:
- **Phase 2.4**: `openai/gpt-4.1` or `openai/gpt-5` (backend services, REST APIs)
- **Phase 2.5**: `openai/gpt-4.1` or `openai/gpt-5` (backend services, database)
- **Phase 2.6**: `openai/o3` or `openai/gpt-5` (complex RBAC, audit, multi-tenancy)

## Verification

After all 3 PRs are merged:
- [ ] Issue for Phase 2.4 Task 1 is closed
- [ ] Issue for Phase 2.5 Task 1 is closed
- [ ] Issue for Phase 2.6 Task 1 is closed
- [ ] Crate `update-server` exists and builds
- [ ] Crate `marketplace-api` exists and builds
- [ ] Crate `enterprise-server` exists and builds
- [ ] All tests pass in CI
- [ ] Documentation is complete for each crate

## References

- Ticket files: `specs/001-selective-electron/PHASE_2_{4,5,6}_TICKETS.md`
- YOLO workflows: `.github/workflows/yolo-*.yml`
- PR template: `.github/PULL_REQUEST_TEMPLATE.md`
- Repository guidelines: `.github/copilot-instructions.md`
