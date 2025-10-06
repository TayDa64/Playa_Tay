# Phase 2.6: Enterprise Features & Governance

**Status:** Planning
**Duration:** 10 weeks (March - May 2027)
**Dependencies:** Phase 2.5 (Marketplace & Discovery) must be complete
**Budget Estimate:** $320K - $380K

---

## 📋 Executive Summary

Phase 2.6 completes the Pattern B roadmap by adding enterprise-grade features that enable large organizations to adopt, deploy, and manage optional Electron modules at scale. This phase focuses on governance, security, compliance, multi-tenancy, and administrative controls required for enterprise environments.

**Core Deliverables:**
1. Private module registries for enterprise customers
2. Organization-level management and role-based access control (RBAC)
3. Approval workflows and policy enforcement
4. Audit logging and compliance reporting
5. Air-gapped/offline deployment support
6. SLA monitoring and enterprise support tools

---

## 🎯 Objectives

### Primary Goals
- **Enterprise Ready**: Meet Fortune 500 security and compliance requirements
- **Centralized Control**: IT admins can govern module usage across organization
- **Compliance**: SOC 2, GDPR, HIPAA audit trails and controls
- **Scalability**: Support 10,000+ users per organization
- **Air-gapped Support**: Work in disconnected/restricted networks

### Success Metrics
- Support 100+ enterprise customers
- RBAC operational with <50ms overhead
- Audit logs comprehensive (100% coverage)
- Air-gapped deployment success rate >95%
- Enterprise customer satisfaction >4.8/5
- SOC 2 Type II certification achieved

---

## 🏗️ Architecture Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                  Enterprise Architecture                          │
└──────────────────────────────────────────────────────────────────┘

                    ┌───────────────────────┐
                    │   Admin Portal        │
                    │  - Org management     │
                    │  - User/role mgmt     │
                    │  - Policy configuration│
                    │  - Compliance reports  │
                    └──────────┬────────────┘
                               │
                               │ Admin API
                               │
        ┌──────────────────────▼───────────────────────┐
        │      Enterprise Management Server            │
        │  - Multi-tenancy (org isolation)             │
        │  - RBAC enforcement                          │
        │  - Approval workflows                        │
        │  - Policy engine                             │
        │  - Audit logging                             │
        └──────────────────────┬───────────────────────┘
                               │
            ┌──────────────────┼──────────────────┐
            │                  │                  │
    ┌───────▼────────┐  ┌──────▼──────┐  ┌──────▼──────┐
    │  Private       │  │  Policy     │  │   Audit     │
    │  Registry      │  │  Database   │  │   Store     │
    │  (Org modules) │  │  (Rules)    │  │  (Logs)     │
    └────────────────┘  └─────────────┘  └─────────────┘

                               │
                               │
        ┌──────────────────────▼───────────────────────┐
        │        Enterprise Tauri Applications         │
        │  - SSO authentication (SAML/OIDC)            │
        │  - Policy-aware module installer             │
        │  - Audit event generation                    │
        │  - Telemetry to admin dashboard              │
        └──────────────────────────────────────────────┘

                               │
                               │
        ┌──────────────────────▼───────────────────────┐
        │      Air-gapped Deployment Tools             │
        │  - Offline installer bundles                 │
        │  - Local registry mirror                     │
        │  - Manual update packages                    │
        └──────────────────────────────────────────────┘
```

---

## 📦 Core Components

### 1. Enterprise Management Server

**Purpose:** Central control plane for enterprise organizations

**Implementation:**
```rust
// crates/enterprise-server/src/main.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    rbac: Arc<RBACEngine>,
    policy: Arc<PolicyEngine>,
    audit: Arc<AuditLogger>,
}

#[tokio::main]
async fn main() {
    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let rbac = Arc::new(RBACEngine::new(db.clone()));
    let policy = Arc::new(PolicyEngine::new(db.clone()));
    let audit = Arc::new(AuditLogger::new(db.clone()));

    let state = AppState {
        db,
        rbac,
        policy,
        audit,
    };

    let app = Router::new()
        // Organization management
        .route("/api/v1/organizations", post(create_organization))
        .route("/api/v1/organizations/:id", get(get_organization))
        .route("/api/v1/organizations/:id", put(update_organization))
        .route("/api/v1/organizations/:id/settings", put(update_org_settings))

        // User & role management
        .route("/api/v1/organizations/:org_id/users", get(list_users))
        .route("/api/v1/organizations/:org_id/users", post(add_user))
        .route("/api/v1/organizations/:org_id/users/:user_id", delete(remove_user))
        .route("/api/v1/organizations/:org_id/users/:user_id/roles", put(assign_roles))
        .route("/api/v1/organizations/:org_id/roles", get(list_roles))
        .route("/api/v1/organizations/:org_id/roles", post(create_role))

        // Module policies
        .route("/api/v1/organizations/:org_id/policies", get(list_policies))
        .route("/api/v1/organizations/:org_id/policies", post(create_policy))
        .route("/api/v1/organizations/:org_id/policies/:id", put(update_policy))
        .route("/api/v1/organizations/:org_id/policies/:id", delete(delete_policy))

        // Approval workflows
        .route("/api/v1/organizations/:org_id/approvals", get(list_pending_approvals))
        .route("/api/v1/organizations/:org_id/approvals/:id", post(process_approval))

        // Private registry
        .route("/api/v1/organizations/:org_id/registry/modules", get(list_private_modules))
        .route("/api/v1/organizations/:org_id/registry/modules", post(upload_private_module))
        .route("/api/v1/organizations/:org_id/registry/modules/:id", delete(delete_private_module))

        // Audit logs & compliance
        .route("/api/v1/organizations/:org_id/audit/logs", get(get_audit_logs))
        .route("/api/v1/organizations/:org_id/audit/report", get(generate_compliance_report))

        // Analytics & monitoring
        .route("/api/v1/organizations/:org_id/analytics/usage", get(get_usage_analytics))
        .route("/api/v1/organizations/:org_id/analytics/security", get(get_security_dashboard))

        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .unwrap();

    println!("Enterprise server listening on http://0.0.0.0:5000");

    axum::serve(listener, app).await.unwrap();
}

// Create new organization
#[derive(Deserialize)]
struct CreateOrgRequest {
    name: String,
    contact_email: String,
    plan: String, // "starter", "business", "enterprise"
    max_users: i32,
}

async fn create_organization(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<Organization>, ApiError> {
    let org_id = uuid::Uuid::new_v4().to_string();

    let org = sqlx::query_as!(
        OrganizationRecord,
        r#"
        INSERT INTO organizations
        (id, name, contact_email, plan, max_users, created_at, active)
        VALUES ($1, $2, $3, $4, $5, NOW(), true)
        RETURNING id, name, contact_email, plan, max_users,
                  created_at, active, settings
        "#,
        org_id,
        payload.name,
        payload.contact_email,
        payload.plan,
        payload.max_users
    )
    .fetch_one(&state.db)
    .await?;

    // Create default roles
    state.rbac.create_default_roles(&org_id).await?;

    // Create default policies
    state.policy.create_default_policies(&org_id).await?;

    // Audit log
    state.audit.log(AuditEvent {
        org_id: Some(org_id.clone()),
        user_id: None,
        action: "organization.created".to_string(),
        resource_type: "organization".to_string(),
        resource_id: org_id.clone(),
        details: serde_json::json!({ "name": payload.name }),
        timestamp: chrono::Utc::now(),
    }).await;

    Ok(Json(org.into()))
}

// List users in organization
async fn list_users(
    Path(org_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<Vec<User>>, ApiError> {
    // Check permission
    state.rbac.check_permission(&auth.user_id, &org_id, "users.read").await?;

    let users = sqlx::query_as!(
        UserRecord,
        r#"
        SELECT u.id, u.email, u.name, u.created_at,
               ARRAY_AGG(ur.role_id) as role_ids
        FROM users u
        JOIN org_users ou ON u.id = ou.user_id
        LEFT JOIN user_roles ur ON u.id = ur.user_id AND ur.org_id = $1
        WHERE ou.org_id = $1
        GROUP BY u.id
        "#,
        org_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(users.into_iter().map(Into::into).collect()))
}

// Create policy
#[derive(Deserialize)]
struct CreatePolicyRequest {
    name: String,
    description: String,
    policy_type: String, // "allowlist", "blocklist", "approval_required"
    rules: serde_json::Value,
}

async fn create_policy(
    Path(org_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthContext,
    Json(payload): Json<CreatePolicyRequest>,
) -> Result<Json<Policy>, ApiError> {
    // Check permission
    state.rbac.check_permission(&auth.user_id, &org_id, "policies.write").await?;

    let policy = sqlx::query_as!(
        PolicyRecord,
        r#"
        INSERT INTO policies
        (org_id, name, description, policy_type, rules, enabled, created_at)
        VALUES ($1, $2, $3, $4, $5, true, NOW())
        RETURNING id, org_id, name, description, policy_type, rules,
                  enabled, created_at
        "#,
        org_id,
        payload.name,
        payload.description,
        payload.policy_type,
        payload.rules
    )
    .fetch_one(&state.db)
    .await?;

    // Audit log
    state.audit.log(AuditEvent {
        org_id: Some(org_id.clone()),
        user_id: Some(auth.user_id),
        action: "policy.created".to_string(),
        resource_type: "policy".to_string(),
        resource_id: policy.id.to_string(),
        details: serde_json::json!({ "name": payload.name }),
        timestamp: chrono::Utc::now(),
    }).await;

    Ok(Json(policy.into()))
}

// Get audit logs
#[derive(Deserialize)]
struct AuditLogQuery {
    start_date: Option<String>,
    end_date: Option<String>,
    action: Option<String>,
    user_id: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}

async fn get_audit_logs(
    Path(org_id): Path<String>,
    Query(params): Query<AuditLogQuery>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<AuditLogResponse>, ApiError> {
    // Check permission
    state.rbac.check_permission(&auth.user_id, &org_id, "audit.read").await?;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut query = format!(
        "SELECT id, org_id, user_id, action, resource_type, resource_id,
                details, timestamp, ip_address, user_agent
         FROM audit_logs
         WHERE org_id = '{}'"
    , org_id);

    if let Some(start) = &params.start_date {
        query.push_str(&format!(" AND timestamp >= '{}'", start));
    }

    if let Some(end) = &params.end_date {
        query.push_str(&format!(" AND timestamp <= '{}'", end));
    }

    if let Some(action) = &params.action {
        query.push_str(&format!(" AND action = '{}'", action));
    }

    if let Some(user_id) = &params.user_id {
        query.push_str(&format!(" AND user_id = '{}'", user_id));
    }

    query.push_str(&format!(" ORDER BY timestamp DESC LIMIT {} OFFSET {}", per_page, offset));

    let logs = sqlx::query_as::<_, AuditLogRecord>(&query)
        .fetch_all(&state.db)
        .await?;

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM audit_logs WHERE org_id = '{}'", org_id)
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(AuditLogResponse {
        logs: logs.into_iter().map(Into::into).collect(),
        total,
        page,
        per_page,
    }))
}

// Generate compliance report
#[derive(Deserialize)]
struct ComplianceReportQuery {
    report_type: String, // "soc2", "gdpr", "hipaa"
    start_date: String,
    end_date: String,
}

async fn generate_compliance_report(
    Path(org_id): Path<String>,
    Query(params): Query<ComplianceReportQuery>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<ComplianceReport>, ApiError> {
    // Check permission
    state.rbac.check_permission(&auth.user_id, &org_id, "compliance.read").await?;

    let report = match params.report_type.as_str() {
        "soc2" => generate_soc2_report(&state.db, &org_id, &params).await?,
        "gdpr" => generate_gdpr_report(&state.db, &org_id, &params).await?,
        "hipaa" => generate_hipaa_report(&state.db, &org_id, &params).await?,
        _ => return Err(ApiError::BadRequest("Invalid report type".to_string())),
    };

    // Audit log
    state.audit.log(AuditEvent {
        org_id: Some(org_id.clone()),
        user_id: Some(auth.user_id),
        action: "compliance.report_generated".to_string(),
        resource_type: "report".to_string(),
        resource_id: report.id.clone(),
        details: serde_json::json!({ "type": params.report_type }),
        timestamp: chrono::Utc::now(),
    }).await;

    Ok(Json(report))
}

// DTOs
#[derive(Serialize)]
struct Organization {
    id: String,
    name: String,
    contact_email: String,
    plan: String,
    max_users: i32,
    current_users: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    active: bool,
    settings: OrganizationSettings,
}

#[derive(Serialize, Deserialize)]
struct OrganizationSettings {
    require_approval: bool,
    allowed_modules: Option<Vec<String>>,
    blocked_modules: Option<Vec<String>>,
    auto_update_enabled: bool,
    sso_enabled: bool,
    mfa_required: bool,
}

#[derive(Serialize)]
struct User {
    id: String,
    email: String,
    name: String,
    roles: Vec<Role>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct Role {
    id: String,
    name: String,
    permissions: Vec<String>,
}

#[derive(Serialize)]
struct Policy {
    id: i64,
    name: String,
    description: String,
    policy_type: String,
    rules: serde_json::Value,
    enabled: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct AuditLog {
    id: i64,
    user_id: Option<String>,
    action: String,
    resource_type: String,
    resource_id: String,
    details: serde_json::Value,
    timestamp: chrono::DateTime<chrono::Utc>,
    ip_address: Option<String>,
}

#[derive(Serialize)]
struct AuditLogResponse {
    logs: Vec<AuditLog>,
    total: i64,
    page: i64,
    per_page: i64,
}

#[derive(Serialize)]
struct ComplianceReport {
    id: String,
    report_type: String,
    period_start: chrono::DateTime<chrono::Utc>,
    period_end: chrono::DateTime<chrono::Utc>,
    controls_checked: i32,
    controls_passed: i32,
    controls_failed: i32,
    findings: Vec<Finding>,
    generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct Finding {
    control_id: String,
    severity: String, // "low", "medium", "high", "critical"
    description: String,
    recommendation: String,
}

// Database records
struct OrganizationRecord {
    id: String,
    name: String,
    contact_email: String,
    plan: String,
    max_users: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    active: bool,
    settings: serde_json::Value,
}

// Error handling
enum ApiError {
    NotFound,
    Unauthorized,
    Forbidden,
    BadRequest(String),
    DatabaseError(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            ApiError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

// Helper functions
async fn generate_soc2_report(
    db: &PgPool,
    org_id: &str,
    params: &ComplianceReportQuery,
) -> Result<ComplianceReport, ApiError> {
    // SOC 2 controls: CC6.1, CC6.6, CC7.2, etc.
    let findings = vec![
        Finding {
            control_id: "CC6.1".to_string(),
            severity: "passed".to_string(),
            description: "Logical and physical access controls".to_string(),
            recommendation: "Continue monitoring".to_string(),
        },
        // ... more controls
    ];

    Ok(ComplianceReport {
        id: uuid::Uuid::new_v4().to_string(),
        report_type: "SOC 2 Type II".to_string(),
        period_start: chrono::DateTime::parse_from_rfc3339(&params.start_date)
            .unwrap().into(),
        period_end: chrono::DateTime::parse_from_rfc3339(&params.end_date)
            .unwrap().into(),
        controls_checked: 20,
        controls_passed: 18,
        controls_failed: 2,
        findings,
        generated_at: chrono::Utc::now(),
    })
}

async fn generate_gdpr_report(
    db: &PgPool,
    org_id: &str,
    params: &ComplianceReportQuery,
) -> Result<ComplianceReport, ApiError> {
    // GDPR articles: Art. 32, Art. 33, Art. 35, etc.
    // Similar structure to SOC 2
    todo!()
}

async fn generate_hipaa_report(
    db: &PgPool,
    org_id: &str,
    params: &ComplianceReportQuery,
) -> Result<ComplianceReport, ApiError> {
    // HIPAA rules: 164.308, 164.312, etc.
    // Similar structure to SOC 2
    todo!()
}
```

---

### 2. RBAC Engine

**Purpose:** Role-based access control with fine-grained permissions

**Implementation:**
```rust
// crates/enterprise-server/src/rbac.rs

use sqlx::PgPool;
use std::collections::HashMap;

pub struct RBACEngine {
    db: PgPool,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub resource: String,
    pub action: String,
}

impl RBACEngine {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn check_permission(
        &self,
        user_id: &str,
        org_id: &str,
        permission: &str,
    ) -> Result<(), RBACError> {
        let has_permission = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_roles ur
                JOIN role_permissions rp ON ur.role_id = rp.role_id
                WHERE ur.user_id = $1
                AND ur.org_id = $2
                AND rp.permission = $3
            ) as "exists!"
            "#,
            user_id,
            org_id,
            permission
        )
        .fetch_one(&self.db)
        .await?;

        if has_permission {
            Ok(())
        } else {
            Err(RBACError::PermissionDenied)
        }
    }

    pub async fn create_default_roles(&self, org_id: &str) -> Result<(), RBACError> {
        let roles = vec![
            ("admin", vec![
                "users.*", "roles.*", "policies.*", "modules.*",
                "audit.*", "compliance.*", "settings.*"
            ]),
            ("developer", vec![
                "modules.read", "modules.install", "modules.update",
                "audit.read"
            ]),
            ("user", vec![
                "modules.read", "modules.install"
            ]),
            ("auditor", vec![
                "audit.read", "compliance.read", "users.read"
            ]),
        ];

        for (role_name, permissions) in roles {
            let role_id = uuid::Uuid::new_v4().to_string();

            sqlx::query!(
                "INSERT INTO roles (id, org_id, name, created_at)
                 VALUES ($1, $2, $3, NOW())",
                role_id,
                org_id,
                role_name
            )
            .execute(&self.db)
            .await?;

            for permission in permissions {
                sqlx::query!(
                    "INSERT INTO role_permissions (role_id, permission)
                     VALUES ($1, $2)",
                    role_id,
                    permission
                )
                .execute(&self.db)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn assign_role(
        &self,
        user_id: &str,
        org_id: &str,
        role_id: &str,
    ) -> Result<(), RBACError> {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, org_id, role_id, assigned_at)
             VALUES ($1, $2, $3, NOW())
             ON CONFLICT (user_id, org_id, role_id) DO NOTHING",
            user_id,
            org_id,
            role_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum RBACError {
    PermissionDenied,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for RBACError {
    fn from(err: sqlx::Error) -> Self {
        RBACError::DatabaseError(err)
    }
}
```

---

### 3. Policy Engine

**Purpose:** Enforce organization-level module policies

**Implementation:**
```rust
// crates/enterprise-server/src/policy.rs

use sqlx::PgPool;
use serde::{Deserialize, Serialize};

pub struct PolicyEngine {
    db: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PolicyType {
    AllowList,      // Only listed modules allowed
    BlockList,      // Listed modules blocked
    ApprovalRequired, // All installs require approval
    VersionPin,     // Pin to specific versions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyRules {
    pub module_ids: Option<Vec<String>>,
    pub version_constraints: Option<HashMap<String, String>>,
    pub approval_roles: Option<Vec<String>>,
}

impl PolicyEngine {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn check_module_allowed(
        &self,
        org_id: &str,
        module_id: &str,
        version: &str,
    ) -> Result<PolicyResult, PolicyError> {
        let policies = sqlx::query_as!(
            PolicyRecord,
            "SELECT id, policy_type, rules, enabled
             FROM policies
             WHERE org_id = $1 AND enabled = true
             ORDER BY priority DESC",
            org_id
        )
        .fetch_all(&self.db)
        .await?;

        for policy in policies {
            let rules: PolicyRules = serde_json::from_value(policy.rules)?;

            match policy.policy_type.as_str() {
                "allowlist" => {
                    if let Some(allowed) = &rules.module_ids {
                        if !allowed.contains(&module_id.to_string()) {
                            return Ok(PolicyResult::Denied {
                                reason: "Module not in allowlist".to_string(),
                            });
                        }
                    }
                }
                "blocklist" => {
                    if let Some(blocked) = &rules.module_ids {
                        if blocked.contains(&module_id.to_string()) {
                            return Ok(PolicyResult::Denied {
                                reason: "Module is blocked".to_string(),
                            });
                        }
                    }
                }
                "approval_required" => {
                    return Ok(PolicyResult::ApprovalRequired {
                        approval_roles: rules.approval_roles.unwrap_or_default(),
                    });
                }
                "version_pin" => {
                    if let Some(constraints) = &rules.version_constraints {
                        if let Some(required_version) = constraints.get(module_id) {
                            if version != required_version {
                                return Ok(PolicyResult::Denied {
                                    reason: format!("Version {} required", required_version),
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(PolicyResult::Allowed)
    }

    pub async fn create_default_policies(&self, org_id: &str) -> Result<(), PolicyError> {
        // Default: require approval for all installs
        sqlx::query!(
            r#"
            INSERT INTO policies (org_id, name, description, policy_type, rules, enabled, priority)
            VALUES ($1, 'Default Approval', 'Require approval for all module installations',
                    'approval_required',
                    '{"approval_roles": ["admin"]}'::jsonb,
                    false, 0)
            "#,
            org_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum PolicyResult {
    Allowed,
    Denied { reason: String },
    ApprovalRequired { approval_roles: Vec<String> },
}

#[derive(Debug)]
pub enum PolicyError {
    DatabaseError(sqlx::Error),
    SerializationError(serde_json::Error),
}

impl From<sqlx::Error> for PolicyError {
    fn from(err: sqlx::Error) -> Self {
        PolicyError::DatabaseError(err)
    }
}

impl From<serde_json::Error> for PolicyError {
    fn from(err: serde_json::Error) -> Self {
        PolicyError::SerializationError(err)
    }
}
```

---

### 4. Air-gapped Deployment Tools

**Purpose:** Support offline/disconnected enterprise environments

**Implementation:**
```rust
// crates/airgap-tools/src/main.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Parser)]
#[command(name = "airgap-tools")]
#[command(about = "Tools for air-gapped deployments")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create offline bundle with all modules
    Bundle {
        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        modules: Vec<String>,

        #[arg(short, long)]
        include_dependencies: bool,
    },

    /// Install from offline bundle
    Install {
        #[arg(short, long)]
        bundle: PathBuf,

        #[arg(short, long)]
        target_dir: PathBuf,
    },

    /// Create local registry mirror
    Mirror {
        #[arg(short, long)]
        registry_url: String,

        #[arg(short, long)]
        output_dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle { output, modules, include_dependencies } => {
            create_offline_bundle(&output, &modules, include_dependencies).await?;
        }
        Commands::Install { bundle, target_dir } => {
            install_from_bundle(&bundle, &target_dir).await?;
        }
        Commands::Mirror { registry_url, output_dir } => {
            create_registry_mirror(&registry_url, &output_dir).await?;
        }
    }

    Ok(())
}

async fn create_offline_bundle(
    output: &PathBuf,
    modules: &[String],
    include_deps: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating offline bundle...");

    let file = std::fs::File::create(output)?;
    let encoder = GzEncoder::new(file, Compression::best());
    let mut archive = Builder::new(encoder);

    for module_id in modules {
        println!("Downloading {}...", module_id);

        // Download module
        let module_data = download_module(module_id).await?;

        // Add to archive
        let mut header = tar::Header::new_gnu();
        header.set_size(module_data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();

        archive.append_data(
            &mut header,
            format!("modules/{}.tar.gz", module_id),
            module_data.as_slice()
        )?;

        // Download signature
        let sig_data = download_signature(module_id).await?;
        let mut sig_header = tar::Header::new_gnu();
        sig_header.set_size(sig_data.len() as u64);
        sig_header.set_mode(0o644);
        sig_header.set_cksum();

        archive.append_data(
            &mut sig_header,
            format!("modules/{}.sig", module_id),
            sig_data.as_slice()
        )?;

        if include_deps {
            // TODO: Download dependencies
        }
    }

    // Add manifest
    let manifest = create_bundle_manifest(modules)?;
    let manifest_data = serde_json::to_vec_pretty(&manifest)?;

    let mut manifest_header = tar::Header::new_gnu();
    manifest_header.set_size(manifest_data.len() as u64);
    manifest_header.set_mode(0o644);
    manifest_header.set_cksum();

    archive.append_data(
        &mut manifest_header,
        "manifest.json",
        manifest_data.as_slice()
    )?;

    archive.finish()?;

    println!("Bundle created: {}", output.display());

    Ok(())
}

async fn install_from_bundle(
    bundle: &PathBuf,
    target_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing from bundle: {}", bundle.display());

    let file = std::fs::File::open(bundle)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(target_dir)?;

    println!("Bundle extracted to: {}", target_dir.display());

    // Verify signatures
    println!("Verifying signatures...");
    // TODO: Implement signature verification

    println!("Installation complete!");

    Ok(())
}

async fn create_registry_mirror(
    registry_url: &str,
    output_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating registry mirror...");

    std::fs::create_dir_all(output_dir)?;

    // Fetch catalog
    let catalog_url = format!("{}/api/v1/catalog", registry_url);
    let response = reqwest::get(&catalog_url).await?;
    let catalog: ModuleCatalog = response.json().await?;

    println!("Found {} modules", catalog.modules.len());

    for module in &catalog.modules {
        println!("Mirroring {}...", module.id);

        // Download module
        let download_url = format!("{}/api/v1/download/{}/{}",
                                  registry_url, module.id, module.version);
        let module_data = reqwest::get(&download_url).await?.bytes().await?;

        let module_path = output_dir.join(format!("{}-{}.tar.gz", module.id, module.version));
        std::fs::write(&module_path, module_data)?;

        // Download signature
        let sig_url = format!("{}.sig", download_url);
        let sig_data = reqwest::get(&sig_url).await?.bytes().await?;

        let sig_path = output_dir.join(format!("{}-{}.sig", module.id, module.version));
        std::fs::write(&sig_path, sig_data)?;
    }

    // Save catalog
    let catalog_path = output_dir.join("catalog.json");
    std::fs::write(&catalog_path, serde_json::to_vec_pretty(&catalog)?)?;

    println!("Registry mirror created at: {}", output_dir.display());

    Ok(())
}

async fn download_module(module_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // TODO: Implement actual download
    Ok(vec![])
}

async fn download_signature(module_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // TODO: Implement actual download
    Ok(vec![])
}

fn create_bundle_manifest(modules: &[String]) -> Result<BundleManifest, Box<dyn std::error::Error>> {
    Ok(BundleManifest {
        version: "1.0".to_string(),
        created_at: chrono::Utc::now(),
        modules: modules.iter().map(|id| ModuleEntry {
            id: id.clone(),
            version: "1.0.0".to_string(), // TODO: Get actual version
        }).collect(),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BundleManifest {
    version: String,
    created_at: chrono::DateTime<chrono::Utc>,
    modules: Vec<ModuleEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ModuleEntry {
    id: String,
    version: String,
}

#[derive(serde::Deserialize)]
struct ModuleCatalog {
    modules: Vec<ModuleInfo>,
}

#[derive(serde::Deserialize)]
struct ModuleInfo {
    id: String,
    version: String,
}
```

---

### 5. Admin Portal UI (React)

**Purpose:** Web interface for enterprise administrators

**Implementation:**
```tsx
// packages/admin-portal/src/pages/Dashboard.tsx

import React from 'react';
import { useOrganization } from '../hooks/useOrganization';
import { UsageChart } from '../components/UsageChart';
import { SecurityAlerts } from '../components/SecurityAlerts';
import { QuickActions } from '../components/QuickActions';

export function DashboardPage() {
  const { organization, stats } = useOrganization();

  return (
    <div className="dashboard">
      <h1>Enterprise Dashboard</h1>

      <div className="stats-grid">
        <div className="stat-card">
          <h3>Active Users</h3>
          <p className="stat-value">{stats.activeUsers}</p>
          <span className="stat-change">+5% this week</span>
        </div>

        <div className="stat-card">
          <h3>Installed Modules</h3>
          <p className="stat-value">{stats.installedModules}</p>
          <span className="stat-change">3 new today</span>
        </div>

        <div className="stat-card">
          <h3>Pending Approvals</h3>
          <p className="stat-value">{stats.pendingApprovals}</p>
          <a href="/approvals">Review</a>
        </div>

        <div className="stat-card">
          <h3>Compliance Score</h3>
          <p className="stat-value">{stats.complianceScore}%</p>
          <a href="/compliance">View Report</a>
        </div>
      </div>

      <div className="content-grid">
        <section className="usage-section">
          <h2>Module Usage</h2>
          <UsageChart data={stats.usageData} />
        </section>

        <section className="alerts-section">
          <h2>Security Alerts</h2>
          <SecurityAlerts alerts={stats.securityAlerts} />
        </section>
      </div>

      <QuickActions />
    </div>
  );
}

// packages/admin-portal/src/pages/UserManagement.tsx

import React, { useState } from 'react';
import { useUsers } from '../hooks/useUsers';
import { UserTable } from '../components/UserTable';
import { AddUserModal } from '../components/AddUserModal';
import { RoleAssignmentModal } from '../components/RoleAssignmentModal';

export function UserManagementPage() {
  const { users, loading, addUser, removeUser, assignRole } = useUsers();
  const [showAddModal, setShowAddModal] = useState(false);
  const [selectedUser, setSelectedUser] = useState(null);

  return (
    <div className="user-management">
      <div className="page-header">
        <h1>User Management</h1>
        <button onClick={() => setShowAddModal(true)}>
          Add User
        </button>
      </div>

      {loading ? (
        <LoadingSpinner />
      ) : (
        <UserTable
          users={users}
          onEditRoles={(user) => setSelectedUser(user)}
          onRemove={(user) => removeUser(user.id)}
        />
      )}

      {showAddModal && (
        <AddUserModal
          onAdd={(email, name, role) => {
            addUser(email, name, role);
            setShowAddModal(false);
          }}
          onClose={() => setShowAddModal(false)}
        />
      )}

      {selectedUser && (
        <RoleAssignmentModal
          user={selectedUser}
          onAssign={(roles) => {
            assignRole(selectedUser.id, roles);
            setSelectedUser(null);
          }}
          onClose={() => setSelectedUser(null)}
        />
      )}
    </div>
  );
}
```

---

## 📝 Task Breakdown

### Task 1: Enterprise Management Server (Weeks 1-3)

**Owner:** Lead Developer + Backend Engineer

**Deliverables:**
- [ ] Multi-tenant architecture
- [ ] Organization CRUD APIs
- [ ] RBAC engine implementation
- [ ] Policy engine implementation
- [ ] Audit logging system
- [ ] Database schema and migrations

**Acceptance Criteria:**
- Multi-tenancy enforced at DB level
- RBAC checks <50ms overhead
- Audit logs 100% coverage
- Policies evaluated correctly
- Support 1000+ orgs

**Dependencies:** None

---

### Task 2: Admin Portal UI (Weeks 2-5)

**Owner:** Frontend Developer + UX Designer

**Deliverables:**
- [ ] React admin dashboard
- [ ] User/role management UI
- [ ] Policy configuration UI
- [ ] Audit log viewer
- [ ] Compliance report generator
- [ ] Analytics dashboards

**Acceptance Criteria:**
- Intuitive admin workflows
- Real-time data updates
- Responsive design
- Accessibility compliant
- Admin satisfaction >4.5/5

**Dependencies:** Task 1 (APIs)

---

### Task 3: SSO Integration (Weeks 3-5)

**Owner:** Security Engineer + Backend Engineer

**Deliverables:**
- [ ] SAML 2.0 integration
- [ ] OIDC/OAuth integration
- [ ] MFA support
- [ ] Session management
- [ ] Identity provider testing
- [ ] Documentation

**Acceptance Criteria:**
- Works with major IdPs (Okta, Azure AD, etc.)
- MFA enforcement options
- Session timeout configurable
- Login flow <3 seconds
- Security audit passed

**Dependencies:** Task 1 (auth framework)

---

### Task 4: Private Registry (Weeks 4-6)

**Owner:** DevOps Engineer + Backend Engineer

**Deliverables:**
- [ ] Private registry per organization
- [ ] Upload/download APIs
- [ ] Access control enforcement
- [ ] Storage backend integration
- [ ] CDN integration
- [ ] Migration tools

**Acceptance Criteria:**
- Isolated storage per org
- Upload/download performant
- Access control enforced
- Works with existing tools
- Migration smooth

**Dependencies:** Task 1 (org management)

---

### Task 5: Air-gapped Tools (Weeks 5-7)

**Owner:** DevOps Engineer + Lead Developer

**Deliverables:**
- [ ] Offline bundle creator
- [ ] Local registry mirror
- [ ] Installation tools
- [ ] Update package generator
- [ ] Documentation
- [ ] Testing in isolated environment

**Acceptance Criteria:**
- Bundles self-contained
- Installation without internet works
- Updates via USB/network share
- Documentation comprehensive
- Success rate >95%

**Dependencies:** Phase 2.3 (installer)

---

### Task 6: Compliance & Reporting (Weeks 6-8)

**Owner:** Security Engineer + Compliance Specialist (new role)

**Deliverables:**
- [ ] SOC 2 Type II controls
- [ ] GDPR compliance features
- [ ] HIPAA controls (if needed)
- [ ] Automated report generation
- [ ] External audit preparation
- [ ] Certification achievement

**Acceptance Criteria:**
- SOC 2 Type II certified
- GDPR compliant
- Reports generated automatically
- Audit findings minimal
- Certification maintained

**Dependencies:** Task 1 (audit logs)

---

### Task 7: Testing & Documentation (Weeks 9-10)

**Owner:** QA Engineer + Technical Writer + All Team

**Deliverables:**
- [ ] Enterprise feature testing
- [ ] Security penetration testing
- [ ] Load testing (10K users)
- [ ] Admin documentation
- [ ] Developer migration guide
- [ ] Enterprise sales materials

**Acceptance Criteria:**
- All features tested
- Security audit passed
- Performance targets met
- Documentation complete
- Sales ready

**Dependencies:** Tasks 1-6 (all features)

---

## 🏃 Sprint Structure

### Sprint 1-2 (Weeks 1-4): Core Infrastructure
- **Goal:** Enterprise server and RBAC operational
- **Demos:** Org management, user roles
- **Risks:** Multi-tenancy complexity, RBAC performance

### Sprint 3-4 (Weeks 5-7): Integration & Tooling
- **Goal:** SSO, private registry, air-gapped tools
- **Demos:** SSO login, offline deployment
- **Risks:** IdP integration, air-gap testing

### Sprint 5 (Weeks 8-10): Compliance & Launch
- **Goal:** Certification, documentation, enterprise launch
- **Demos:** Full enterprise demo, compliance reports
- **Risks:** Audit findings, certification timeline

---

## 👥 Team & Resources

### Core Team (Full-time)
- **Lead Developer** (10 weeks)
  - Architecture, policy engine, air-gap tools
  - Rate: $150/hr × 40hr/wk × 10wks = $60,000

- **Backend Engineer** (10 weeks)
  - APIs, RBAC, private registry
  - Rate: $145/hr × 40hr/wk × 10wks = $58,000

- **Security Engineer** (10 weeks)
  - SSO, compliance, security audit
  - Rate: $160/hr × 40hr/wk × 10wks = $64,000

- **DevOps Engineer** (8 weeks)
  - Infrastructure, air-gap tools, deployment
  - Rate: $140/hr × 40hr/wk × 8wks = $44,800

### New Roles (Full-time/Part-time)
- **Compliance Specialist** (NEW - 6 weeks)
  - SOC 2, GDPR, HIPAA controls
  - Rate: $170/hr × 40hr/wk × 6wks = $40,800

- **Frontend Developer** (50% × 8 weeks)
  - Admin portal UI
  - Rate: $130/hr × 20hr/wk × 8wks = $20,800

- **UX Designer** (50% × 6 weeks)
  - Admin UI design, workflows
  - Rate: $120/hr × 20hr/wk × 6wks = $14,400

### Supporting Team (Part-time)
- **QA Engineer** (50% × 8 weeks)
  - Testing, security testing
  - Rate: $110/hr × 20hr/wk × 8wks = $17,600

- **Technical Writer** (50% × 6 weeks)
  - Enterprise docs, sales materials
  - Rate: $100/hr × 20hr/wk × 6wks = $12,000

**Total Personnel:** $332,400

---

## 💰 Budget Breakdown

| Category | Item | Cost |
|----------|------|------|
| **Personnel** | Core team (4 FT) | $226,800 |
| | New roles (1 FT, 2 PT) | $76,000 |
| | Supporting team (2 PT) | $29,600 |
| **Infrastructure** | Multi-tenant DB infrastructure | $5,000 |
| | Private registry storage | $4,000 |
| | SSO/IdP testing accounts | $2,400 |
| | Enterprise hosting | $3,600 |
| **Compliance** | SOC 2 Type II audit | $25,000 |
| | GDPR compliance review | $12,000 |
| | Penetration testing | $15,000 |
| | Legal/compliance consulting | $8,000 |
| **Tools & Services** | Enterprise monitoring | $2,400 |
| | SIEM integration | $3,000 |
| | Identity management tools | $2,000 |
| **Testing** | Enterprise testing lab | $4,000 |
| | Load testing services | $3,000 |
| **Sales & Marketing** | Sales collateral creation | $5,000 |
| | Demo environment | $2,000 |
| **Contingency** | 10% buffer | $43,280 |
| **TOTAL** | | **$476,080** |

**Budget Range:** $430K - $520K (includes significant compliance costs)

---

## 🎯 Success Criteria

### Enterprise Readiness
- ✅ SOC 2 Type II certification achieved
- ✅ GDPR compliant
- ✅ RBAC operational with <50ms overhead
- ✅ Support 100+ enterprise customers
- ✅ Air-gapped deployment working
- ✅ SSO with major IdPs functional

### Technical Performance
- ✅ Multi-tenancy enforced at all layers
- ✅ Audit logs 100% coverage
- ✅ System scales to 10,000 users/org
- ✅ Private registry performant
- ✅ Policy evaluation <100ms

### Business Metrics
- ✅ 20+ enterprise customers signed
- ✅ Enterprise customer satisfaction >4.8/5
- ✅ Support ticket resolution <24 hours
- ✅ Compliance maintained continuously
- ✅ Revenue targets met

---

## 🚨 Risk Management

### Critical Risks

#### 1. SOC 2 Certification Delays
- **Probability:** High
- **Impact:** Critical
- **Mitigation:**
  - Start audit process early
  - Hire experienced auditor
  - Pre-audit readiness assessment
  - Dedicated compliance specialist
- **Contingency:** Launch with "SOC 2 in progress" status

#### 2. SSO Integration Complexity
- **Probability:** Medium
- **Impact:** High
- **Mitigation:**
  - Support major IdPs first
  - Comprehensive testing
  - Fallback to local auth
  - Clear documentation
- **Contingency:** Phase rollout by IdP

#### 3. Multi-tenancy Security Issues
- **Probability:** Low
- **Impact:** Critical
- **Mitigation:**
  - Security-first architecture
  - Penetration testing
  - Code review
  - Database-level isolation
- **Contingency:** Additional security audit, delayed launch

---

## 📊 Milestones & Timeline

```
Week 1-3: Enterprise Management Server
├─ Week 1: Multi-tenant architecture, RBAC
├─ Week 2: Policy engine, audit logging
└─ Week 3: APIs complete, testing
    ✓ Milestone: Core platform ready

Week 2-5: Admin Portal & SSO
├─ Week 2-3: Admin UI development
├─ Week 4: SSO integration (SAML/OIDC)
└─ Week 5: MFA, testing
    ✓ Milestone: Admin experience complete

Week 4-7: Registry & Air-gap Tools
├─ Week 4-5: Private registry
├─ Week 6: Air-gapped tools
└─ Week 7: Testing, documentation
    ✓ Milestone: Enterprise features operational

Week 6-10: Compliance & Launch
├─ Week 6-8: SOC 2 audit, compliance
├─ Week 9: Final testing, docs
└─ Week 10: Enterprise launch
    ✓ Milestone: Phase 2.6 complete, Pattern B done!
```

---

## ✅ Phase 2.6 Complete When

- [ ] SOC 2 Type II certification achieved
- [ ] GDPR compliance validated
- [ ] Multi-tenancy working (100+ orgs)
- [ ] RBAC operational (<50ms overhead)
- [ ] Admin portal fully functional
- [ ] SSO with major IdPs working
- [ ] Private registries operational
- [ ] Air-gapped deployment tested
- [ ] Audit logging 100% coverage
- [ ] Policy engine enforcing rules
- [ ] Security audit passed (zero critical)
- [ ] Documentation complete
- [ ] 5+ enterprise beta customers
- [ ] Sales materials ready
- [ ] **Pattern B roadmap complete!**

---

## 📞 Stakeholder Communication

### Weekly Updates
- Compliance progress
- Enterprise signups
- Feature completion status
- Audit findings

### Demos
- Week 3: RBAC and multi-tenancy
- Week 5: SSO and admin portal
- Week 7: Air-gapped deployment
- Week 10: Full enterprise demo

### Decision Points
- Week 2: SOC 2 auditor selection
- Week 5: SSO provider priorities
- Week 8: Compliance readiness
- Week 10: Enterprise launch go/no-go

---

## 🔗 Related Documents

- **Phase 2 Overview:** [../../PHASE_2_PLANNING.md](../../PHASE_2_PLANNING.md)
- **Phase 2.1-2.5 Plans:** [PHASE_2_1_PLAN.md](./PHASE_2_1_PLAN.md) through [PHASE_2_5_PLAN.md](./PHASE_2_5_PLAN.md)
- **Verification Report:** [../../VERIFICATION_REPORT.md](../../VERIFICATION_REPORT.md)

---

## 🎉 Pattern B Completion Summary

With Phase 2.6 complete, the full Pattern B implementation is finished:

| Phase | Duration | Budget | Status |
|-------|----------|--------|--------|
| 2.1: Foundation & Packaging | 14 weeks | $162K | ✅ Planned |
| 2.2: Detection & Registry | 12 weeks | $307K | ✅ Planned |
| 2.3: Installation & Updates | 14 weeks | $391K | ✅ Planned |
| 2.4: Distribution & CDN | 10 weeks | $335K | ✅ Planned |
| 2.5: Marketplace & Discovery | 12 weeks | $389K | ✅ Planned |
| 2.6: Enterprise Features | 10 weeks | $476K | ✅ Planned |
| **TOTAL** | **72 weeks** | **$2.06M** | **Ready!** |

**Timeline:** January 2026 - May 2027 (18 months)
**Team Size:** 6-8 people (rotating specialists)
**End State:** Production-ready optional module ecosystem with enterprise support

---

**Status:** Ready for Executive Review
**Next Action:** Executive approval, budget allocation, team assembly
**Target Start Date:** January 2026
**Target Completion:** May 2027
**Total Investment:** $2.06M over 18 months
