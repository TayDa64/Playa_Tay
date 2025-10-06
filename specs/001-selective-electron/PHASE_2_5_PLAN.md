# Phase 2.5: Marketplace & Discovery

**Status:** Planning
**Duration:** 12 weeks (January - March 2027)
**Dependencies:** Phase 2.4 (Distribution & CDN) must be complete
**Budget Estimate:** $280K - $340K

---

## 📋 Executive Summary

Phase 2.5 creates the user-facing marketplace experience that transforms the module ecosystem from a technical infrastructure into an accessible, discoverable platform. Users can browse available modules, read reviews, view compatibility information, and install modules with confidence. This phase focuses on user experience, community engagement, and ecosystem growth.

**Core Deliverables:**
1. Web-based marketplace UI with search and filtering
2. In-app module browser integrated into Tauri applications
3. Module metadata system (descriptions, screenshots, reviews)
4. Search and recommendation engine
5. User ratings and review system
6. Developer portal for module submission

---

## 🎯 Objectives

### Primary Goals
- **Discoverability**: Users easily find relevant modules
- **Trust & Transparency**: Clear information on compatibility, security, reviews
- **Seamless Integration**: In-app browsing feels native to Tauri apps
- **Community Engagement**: Ratings, reviews, and feedback mechanisms
- **Developer Friendly**: Simple submission and update process

### Success Metrics
- Search latency <200ms
- User can find relevant module in <30 seconds
- Module installation rate >40% after viewing details
- User satisfaction >4.2/5 for marketplace UX
- Developer submission-to-approval time <48 hours
- Review moderation time <24 hours

---

## 🏗️ Architecture Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                    Marketplace Architecture                       │
└──────────────────────────────────────────────────────────────────┘

                    ┌───────────────────────┐
                    │   Web Marketplace     │
                    │   (marketplace.app)   │
                    │  - Browse modules     │
                    │  - Search & filter    │
                    │  - Reviews & ratings  │
                    └──────────┬────────────┘
                               │
                               │ HTTPS API
                               │
        ┌──────────────────────▼───────────────────────┐
        │        Marketplace API Server                │
        │  - Module catalog                            │
        │  - Search indexing (Elasticsearch)           │
        │  - Review management                         │
        │  - Developer submissions                     │
        │  - Recommendation engine                     │
        └──────────────────────┬───────────────────────┘
                               │
            ┌──────────────────┼──────────────────┐
            │                  │                  │
    ┌───────▼────────┐  ┌──────▼──────┐  ┌──────▼──────┐
    │   PostgreSQL   │  │Elasticsearch│  │    Redis    │
    │  - Modules     │  │ - Search    │  │  - Cache    │
    │  - Reviews     │  │ - Analytics │  │  - Sessions │
    │  - Users       │  │ - Logs      │  │  - Queue    │
    └────────────────┘  └─────────────┘  └─────────────┘

                               │
                               │ Plugin API
                               │
        ┌──────────────────────▼───────────────────────┐
        │     Tauri Plugin (In-App Browser)            │
        │  - Native module browser window              │
        │  - Search & filter UI                        │
        │  - One-click installation                    │
        │  - Review submission                         │
        └──────────────────────────────────────────────┘

                               │
                               │
        ┌──────────────────────▼───────────────────────┐
        │         Developer Portal                     │
        │  - Module submission                         │
        │  - Update management                         │
        │  - Analytics dashboard                       │
        │  - Documentation                             │
        └──────────────────────────────────────────────┘
```

---

## 📦 Core Components

### 1. Marketplace API Server

**Purpose:** Backend API for module discovery, search, reviews, and developer management

**Implementation:**
```rust
// crates/marketplace-server/src/main.rs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use elasticsearch::Elasticsearch;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    search: Arc<SearchEngine>,
    cache: Arc<CacheManager>,
    recommender: Arc<RecommendationEngine>,
}

#[tokio::main]
async fn main() {
    let db = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let search = Arc::new(SearchEngine::new().await);
    let cache = Arc::new(CacheManager::new());
    let recommender = Arc::new(RecommendationEngine::new(db.clone()));

    let state = AppState {
        db,
        search,
        cache,
        recommender,
    };

    let app = Router::new()
        // Module discovery
        .route("/api/v1/modules", get(list_modules))
        .route("/api/v1/modules/:id", get(get_module_details))
        .route("/api/v1/modules/search", get(search_modules))
        .route("/api/v1/modules/featured", get(get_featured_modules))
        .route("/api/v1/modules/recommended", get(get_recommended_modules))

        // Reviews & ratings
        .route("/api/v1/modules/:id/reviews", get(get_reviews))
        .route("/api/v1/modules/:id/reviews", post(submit_review))
        .route("/api/v1/reviews/:id", put(update_review))
        .route("/api/v1/reviews/:id", delete(delete_review))
        .route("/api/v1/reviews/:id/vote", post(vote_review))

        // Developer portal
        .route("/api/v1/developer/modules", post(submit_module))
        .route("/api/v1/developer/modules/:id", put(update_module))
        .route("/api/v1/developer/modules/:id/analytics", get(get_module_analytics))

        // Categories & tags
        .route("/api/v1/categories", get(get_categories))
        .route("/api/v1/tags", get(get_tags))

        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    println!("Marketplace server listening on http://0.0.0.0:4000");

    axum::serve(listener, app).await.unwrap();
}

// List modules with pagination and filtering
#[derive(Deserialize)]
struct ModuleListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    category: Option<String>,
    tag: Option<String>,
    sort: Option<String>, // "popular", "recent", "rating"
}

async fn list_modules(
    Query(params): Query<ModuleListQuery>,
    State(state): State<AppState>,
) -> Result<Json<ModuleListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let order_by = match params.sort.as_deref() {
        Some("popular") => "download_count DESC",
        Some("recent") => "published_at DESC",
        Some("rating") => "average_rating DESC",
        _ => "download_count DESC",
    };

    let mut query = format!(
        "SELECT id, name, description, icon_url, version, average_rating,
                review_count, download_count, category, tags, published_at
         FROM modules
         WHERE published = true"
    );

    if let Some(category) = &params.category {
        query.push_str(&format!(" AND category = '{}'", category));
    }

    if let Some(tag) = &params.tag {
        query.push_str(&format!(" AND '{}' = ANY(tags)", tag));
    }

    query.push_str(&format!(" ORDER BY {} LIMIT {} OFFSET {}", order_by, per_page, offset));

    let modules = sqlx::query_as::<_, ModuleListItem>(&query)
        .fetch_all(&state.db)
        .await?;

    let total_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM modules WHERE published = true"
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ModuleListResponse {
        modules,
        total: total_count,
        page,
        per_page,
        total_pages: (total_count as f64 / per_page as f64).ceil() as i64,
    }))
}

// Get detailed module information
async fn get_module_details(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ModuleDetails>, ApiError> {
    // Check cache first
    if let Some(cached) = state.cache.get::<ModuleDetails>(&format!("module:{}", id)).await {
        return Ok(Json(cached));
    }

    let module = sqlx::query_as!(
        ModuleDetailsRecord,
        r#"
        SELECT
            m.id, m.name, m.description, m.long_description,
            m.icon_url, m.screenshots, m.version, m.size_bytes,
            m.category, m.tags, m.author_name, m.author_url,
            m.homepage_url, m.repository_url, m.license,
            m.average_rating, m.review_count, m.download_count,
            m.published_at, m.updated_at,
            m.min_tauri_version, m.compatibility_notes,
            ARRAY_AGG(v.version ORDER BY v.released_at DESC) as versions
        FROM modules m
        LEFT JOIN module_versions v ON m.id = v.module_id
        WHERE m.id = $1 AND m.published = true
        GROUP BY m.id
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound)?;

    let details: ModuleDetails = module.into();

    // Cache for 5 minutes
    state.cache.set(&format!("module:{}", id), &details, 300).await;

    Ok(Json(details))
}

// Search modules
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<i64>,
    per_page: Option<i64>,
    category: Option<String>,
}

async fn search_modules(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, ApiError> {
    let results = state.search.search(
        &params.q,
        params.page.unwrap_or(1),
        params.per_page.unwrap_or(20),
        params.category.as_deref(),
    ).await?;

    Ok(Json(results))
}

// Get featured modules
async fn get_featured_modules(
    State(state): State<AppState>,
) -> Result<Json<Vec<ModuleListItem>>, ApiError> {
    if let Some(cached) = state.cache.get::<Vec<ModuleListItem>>("featured").await {
        return Ok(Json(cached));
    }

    let modules = sqlx::query_as!(
        ModuleListItem,
        r#"
        SELECT id, name, description, icon_url, version, average_rating,
               review_count, download_count, category, tags, published_at
        FROM modules
        WHERE published = true AND featured = true
        ORDER BY featured_order, download_count DESC
        LIMIT 10
        "#
    )
    .fetch_all(&state.db)
    .await?;

    state.cache.set("featured", &modules, 600).await;

    Ok(Json(modules))
}

// Get personalized recommendations
#[derive(Deserialize)]
struct RecommendationQuery {
    user_id: Option<String>,
    installed_modules: Option<String>, // comma-separated
    limit: Option<i64>,
}

async fn get_recommended_modules(
    Query(params): Query<RecommendationQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ModuleListItem>>, ApiError> {
    let installed: Vec<String> = params.installed_modules
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_default();

    let recommendations = state.recommender
        .recommend(params.user_id.as_deref(), &installed, params.limit.unwrap_or(10))
        .await?;

    Ok(Json(recommendations))
}

// Get reviews for a module
#[derive(Deserialize)]
struct ReviewQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    sort: Option<String>, // "recent", "helpful", "rating"
}

async fn get_reviews(
    Path(module_id): Path<String>,
    Query(params): Query<ReviewQuery>,
    State(state): State<AppState>,
) -> Result<Json<ReviewListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * per_page;

    let order_by = match params.sort.as_deref() {
        Some("recent") => "created_at DESC",
        Some("helpful") => "helpful_count DESC",
        Some("rating") => "rating DESC",
        _ => "helpful_count DESC, created_at DESC",
    };

    let reviews = sqlx::query_as!(
        ReviewRecord,
        &format!(
            r#"
            SELECT id, module_id, user_id, username, rating, title, content,
                   helpful_count, unhelpful_count, created_at, updated_at
            FROM reviews
            WHERE module_id = $1 AND moderated = true
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            order_by
        ),
        module_id,
        per_page,
        offset
    )
    .fetch_all(&state.db)
    .await?;

    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM reviews WHERE module_id = $1 AND moderated = true",
        module_id
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    Ok(Json(ReviewListResponse {
        reviews: reviews.into_iter().map(Into::into).collect(),
        total,
        page,
        per_page,
    }))
}

// Submit a new review
#[derive(Deserialize)]
struct SubmitReviewRequest {
    user_id: String,
    username: String,
    rating: i32,
    title: String,
    content: String,
}

async fn submit_review(
    Path(module_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<SubmitReviewRequest>,
) -> Result<Json<Review>, ApiError> {
    // Validate rating
    if !(1..=5).contains(&payload.rating) {
        return Err(ApiError::BadRequest("Rating must be 1-5".to_string()));
    }

    // Check if user already reviewed
    let existing = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM reviews WHERE module_id = $1 AND user_id = $2",
        module_id,
        payload.user_id
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0);

    if existing > 0 {
        return Err(ApiError::BadRequest("Already reviewed".to_string()));
    }

    // Insert review
    let review = sqlx::query_as!(
        ReviewRecord,
        r#"
        INSERT INTO reviews
        (module_id, user_id, username, rating, title, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING id, module_id, user_id, username, rating, title, content,
                  helpful_count, unhelpful_count, created_at, updated_at
        "#,
        module_id,
        payload.user_id,
        payload.username,
        payload.rating,
        payload.title,
        payload.content
    )
    .fetch_one(&state.db)
    .await?;

    // Update module average rating
    sqlx::query!(
        r#"
        UPDATE modules
        SET average_rating = (
            SELECT AVG(rating)::float FROM reviews
            WHERE module_id = $1 AND moderated = true
        ),
        review_count = (
            SELECT COUNT(*) FROM reviews
            WHERE module_id = $1 AND moderated = true
        )
        WHERE id = $1
        "#,
        module_id
    )
    .execute(&state.db)
    .await?;

    Ok(Json(review.into()))
}

// Developer: Submit new module
#[derive(Deserialize)]
struct ModuleSubmission {
    name: String,
    description: String,
    long_description: String,
    version: String,
    category: String,
    tags: Vec<String>,
    homepage_url: Option<String>,
    repository_url: String,
    license: String,
    min_tauri_version: String,
}

async fn submit_module(
    State(state): State<AppState>,
    Json(payload): Json<ModuleSubmission>,
) -> Result<Json<ModuleSubmissionResponse>, ApiError> {
    // Validate submission
    validate_module_submission(&payload)?;

    // Create pending submission
    let submission = sqlx::query!(
        r#"
        INSERT INTO module_submissions
        (name, description, long_description, version, category, tags,
         homepage_url, repository_url, license, min_tauri_version,
         status, submitted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'pending', NOW())
        RETURNING id
        "#,
        payload.name,
        payload.description,
        payload.long_description,
        payload.version,
        payload.category,
        &payload.tags,
        payload.homepage_url,
        payload.repository_url,
        payload.license,
        payload.min_tauri_version
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ModuleSubmissionResponse {
        submission_id: submission.id,
        status: "pending".to_string(),
        estimated_review_time_hours: 48,
    }))
}

// Get module analytics for developers
async fn get_module_analytics(
    Path(module_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ModuleAnalytics>, ApiError> {
    let analytics = sqlx::query_as!(
        ModuleAnalyticsRecord,
        r#"
        SELECT
            m.download_count,
            m.average_rating,
            m.review_count,
            COUNT(DISTINCT ae.user_id) as active_users,
            COUNT(DISTINCT CASE WHEN ae.timestamp > NOW() - INTERVAL '7 days'
                  THEN ae.user_id END) as weekly_active_users,
            COUNT(DISTINCT CASE WHEN ae.timestamp > NOW() - INTERVAL '30 days'
                  THEN ae.user_id END) as monthly_active_users
        FROM modules m
        LEFT JOIN analytics_events ae ON ae.module_id = m.id
        WHERE m.id = $1
        GROUP BY m.id, m.download_count, m.average_rating, m.review_count
        "#,
        module_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(analytics.into()))
}

// DTOs
#[derive(Serialize, sqlx::FromRow)]
struct ModuleListItem {
    id: String,
    name: String,
    description: String,
    icon_url: Option<String>,
    version: String,
    average_rating: Option<f32>,
    review_count: i32,
    download_count: i64,
    category: String,
    tags: Vec<String>,
    published_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct ModuleListResponse {
    modules: Vec<ModuleListItem>,
    total: i64,
    page: i64,
    per_page: i64,
    total_pages: i64,
}

#[derive(Serialize)]
struct ModuleDetails {
    id: String,
    name: String,
    description: String,
    long_description: String,
    icon_url: Option<String>,
    screenshots: Vec<String>,
    version: String,
    size_bytes: i64,
    category: String,
    tags: Vec<String>,
    author_name: String,
    author_url: Option<String>,
    homepage_url: Option<String>,
    repository_url: String,
    license: String,
    average_rating: Option<f32>,
    review_count: i32,
    download_count: i64,
    published_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    min_tauri_version: String,
    compatibility_notes: Option<String>,
    versions: Vec<String>,
}

#[derive(Serialize, sqlx::FromRow)]
struct ReviewRecord {
    id: i64,
    module_id: String,
    user_id: String,
    username: String,
    rating: i32,
    title: String,
    content: String,
    helpful_count: i32,
    unhelpful_count: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct Review {
    id: i64,
    username: String,
    rating: i32,
    title: String,
    content: String,
    helpful_count: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ReviewRecord> for Review {
    fn from(r: ReviewRecord) -> Self {
        Self {
            id: r.id,
            username: r.username,
            rating: r.rating,
            title: r.title,
            content: r.content,
            helpful_count: r.helpful_count,
            created_at: r.created_at,
        }
    }
}

#[derive(Serialize)]
struct ReviewListResponse {
    reviews: Vec<Review>,
    total: i64,
    page: i64,
    per_page: i64,
}

#[derive(Serialize)]
struct ModuleSubmissionResponse {
    submission_id: i64,
    status: String,
    estimated_review_time_hours: i32,
}

#[derive(Serialize)]
struct ModuleAnalytics {
    download_count: i64,
    average_rating: Option<f32>,
    review_count: i32,
    active_users: i64,
    weekly_active_users: i64,
    monthly_active_users: i64,
}

// Error handling
enum ApiError {
    NotFound,
    BadRequest(String),
    DatabaseError(sqlx::Error),
    SearchError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::SearchError(_) => (StatusCode::SERVICE_UNAVAILABLE, "Search error".to_string()),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

fn validate_module_submission(payload: &ModuleSubmission) -> Result<(), ApiError> {
    if payload.name.is_empty() || payload.name.len() > 100 {
        return Err(ApiError::BadRequest("Invalid name".to_string()));
    }

    if payload.description.is_empty() || payload.description.len() > 500 {
        return Err(ApiError::BadRequest("Invalid description".to_string()));
    }

    if semver::Version::parse(&payload.version).is_err() {
        return Err(ApiError::BadRequest("Invalid version".to_string()));
    }

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
```

---

### 2. Search Engine (Elasticsearch)

**Purpose:** Fast, relevant search across module catalog

**Implementation:**
```rust
// crates/marketplace-server/src/search.rs

use elasticsearch::{Elasticsearch, http::transport::Transport, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct SearchEngine {
    client: Elasticsearch,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub rating: Option<f32>,
    pub download_count: i64,
    pub relevance_score: f32,
}

impl SearchEngine {
    pub async fn new() -> Self {
        let transport = Transport::single_node(
            &std::env::var("ELASTICSEARCH_URL").unwrap()
        ).unwrap();

        let client = Elasticsearch::new(transport);

        Self { client }
    }

    pub async fn index_module(&self, module: &ModuleDocument) -> Result<(), SearchError> {
        self.client
            .index(elasticsearch::IndexParts::IndexId("modules", &module.id))
            .body(json!(module))
            .send()
            .await?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        page: i64,
        per_page: i64,
        category: Option<&str>,
    ) -> Result<SearchResponse, SearchError> {
        let from = (page - 1) * per_page;

        let mut must_clauses = vec![
            json!({
                "multi_match": {
                    "query": query,
                    "fields": ["name^3", "description^2", "tags", "category"],
                    "fuzziness": "AUTO"
                }
            })
        ];

        if let Some(cat) = category {
            must_clauses.push(json!({
                "term": { "category": cat }
            }));
        }

        let search_body = json!({
            "from": from,
            "size": per_page,
            "query": {
                "bool": {
                    "must": must_clauses,
                    "should": [
                        { "rank_feature": { "field": "download_count", "boost": 0.5 } },
                        { "rank_feature": { "field": "average_rating", "boost": 0.3 } }
                    ]
                }
            },
            "highlight": {
                "fields": {
                    "name": {},
                    "description": {}
                }
            }
        });

        let response = self.client
            .search(SearchParts::Index(&["modules"]))
            .body(search_body)
            .send()
            .await?;

        let body = response.json::<serde_json::Value>().await?;

        let hits = body["hits"]["hits"].as_array().unwrap();
        let total = body["hits"]["total"]["value"].as_i64().unwrap_or(0);

        let results = hits.iter()
            .filter_map(|hit| {
                let source = &hit["_source"];
                let score = hit["_score"].as_f64().unwrap_or(0.0) as f32;

                Some(SearchResult {
                    id: source["id"].as_str()?.to_string(),
                    name: source["name"].as_str()?.to_string(),
                    description: source["description"].as_str()?.to_string(),
                    category: source["category"].as_str()?.to_string(),
                    tags: source["tags"].as_array()?
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect(),
                    rating: source["average_rating"].as_f64().map(|v| v as f32),
                    download_count: source["download_count"].as_i64()?,
                    relevance_score: score,
                })
            })
            .collect();

        Ok(SearchResponse {
            results,
            total,
            page,
            per_page,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModuleDocument {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub average_rating: Option<f32>,
    pub download_count: i64,
}

#[derive(Debug)]
pub enum SearchError {
    ElasticsearchError(elasticsearch::Error),
}

impl From<elasticsearch::Error> for SearchError {
    fn from(err: elasticsearch::Error) -> Self {
        SearchError::ElasticsearchError(err)
    }
}
```

---

### 3. Recommendation Engine

**Purpose:** Personalized module recommendations based on user behavior

**Implementation:**
```rust
// crates/marketplace-server/src/recommender.rs

use sqlx::PgPool;
use std::collections::HashMap;

pub struct RecommendationEngine {
    db: PgPool,
}

impl RecommendationEngine {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn recommend(
        &self,
        user_id: Option<&str>,
        installed_modules: &[String],
        limit: i64,
    ) -> Result<Vec<ModuleListItem>, RecommendationError> {
        if installed_modules.is_empty() {
            // No installed modules - return popular modules
            return self.get_popular_modules(limit).await;
        }

        // Collaborative filtering: users who installed similar modules also installed...
        let recommendations = self.collaborative_filtering(installed_modules, limit).await?;

        if !recommendations.is_empty() {
            return Ok(recommendations);
        }

        // Fallback: category-based recommendations
        self.category_based_recommendations(installed_modules, limit).await
    }

    async fn collaborative_filtering(
        &self,
        installed_modules: &[String],
        limit: i64,
    ) -> Result<Vec<ModuleListItem>, RecommendationError> {
        // Find users with similar installations
        let recommendations = sqlx::query_as!(
            ModuleListItem,
            r#"
            WITH similar_users AS (
                SELECT DISTINCT user_id
                FROM user_installations
                WHERE module_id = ANY($1)
                AND user_id IS NOT NULL
            ),
            recommended_modules AS (
                SELECT ui.module_id, COUNT(*) as score
                FROM user_installations ui
                WHERE ui.user_id IN (SELECT user_id FROM similar_users)
                AND ui.module_id != ALL($1)
                GROUP BY ui.module_id
                ORDER BY score DESC
                LIMIT $2
            )
            SELECT m.id, m.name, m.description, m.icon_url, m.version,
                   m.average_rating, m.review_count, m.download_count,
                   m.category, m.tags, m.published_at
            FROM modules m
            WHERE m.id IN (SELECT module_id FROM recommended_modules)
            AND m.published = true
            ORDER BY m.download_count DESC
            "#,
            installed_modules,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        Ok(recommendations)
    }

    async fn category_based_recommendations(
        &self,
        installed_modules: &[String],
        limit: i64,
    ) -> Result<Vec<ModuleListItem>, RecommendationError> {
        // Get categories of installed modules
        let categories = sqlx::query_scalar!(
            "SELECT DISTINCT category FROM modules WHERE id = ANY($1)",
            installed_modules
        )
        .fetch_all(&self.db)
        .await?;

        if categories.is_empty() {
            return self.get_popular_modules(limit).await;
        }

        // Recommend popular modules in same categories
        let recommendations = sqlx::query_as!(
            ModuleListItem,
            r#"
            SELECT id, name, description, icon_url, version, average_rating,
                   review_count, download_count, category, tags, published_at
            FROM modules
            WHERE category = ANY($1)
            AND id != ALL($2)
            AND published = true
            ORDER BY download_count DESC, average_rating DESC NULLS LAST
            LIMIT $3
            "#,
            &categories,
            installed_modules,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        Ok(recommendations)
    }

    async fn get_popular_modules(
        &self,
        limit: i64,
    ) -> Result<Vec<ModuleListItem>, RecommendationError> {
        let modules = sqlx::query_as!(
            ModuleListItem,
            r#"
            SELECT id, name, description, icon_url, version, average_rating,
                   review_count, download_count, category, tags, published_at
            FROM modules
            WHERE published = true
            ORDER BY download_count DESC, average_rating DESC NULLS LAST
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        Ok(modules)
    }
}

#[derive(Debug)]
pub enum RecommendationError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for RecommendationError {
    fn from(err: sqlx::Error) -> Self {
        RecommendationError::DatabaseError(err)
    }
}
```

---

### 4. Tauri Plugin (In-App Browser)

**Purpose:** Native module browser embedded in Tauri applications

**Implementation:**
```rust
// crates/tauri-plugin-marketplace/src/lib.rs

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State, Window,
};
use serde::{Deserialize, Serialize};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("marketplace")
        .invoke_handler(tauri::generate_handler![
            open_marketplace,
            search_modules,
            get_module_details,
            install_from_marketplace,
        ])
        .setup(|app| {
            let marketplace_url = std::env::var("MARKETPLACE_URL")
                .unwrap_or_else(|_| "https://marketplace.tauri.app".to_string());

            app.manage(MarketplaceConfig {
                api_url: format!("{}/api/v1", marketplace_url),
                web_url: marketplace_url,
            });

            Ok(())
        })
        .build()
}

struct MarketplaceConfig {
    api_url: String,
    web_url: String,
}

#[tauri::command]
async fn open_marketplace<R: Runtime>(
    window: Window<R>,
    config: State<'_, MarketplaceConfig>,
) -> Result<(), String> {
    // Create new window for marketplace
    tauri::WindowBuilder::new(
        &window.app_handle(),
        "marketplace",
        tauri::WindowUrl::External(config.web_url.parse().unwrap())
    )
    .title("Module Marketplace")
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn search_modules(
    query: String,
    config: State<'_, MarketplaceConfig>,
) -> Result<Vec<ModuleSearchResult>, String> {
    let url = format!("{}/modules/search?q={}", config.api_url,
                     urlencoding::encode(&query));

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;

    let results: SearchResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(results.results)
}

#[tauri::command]
async fn get_module_details(
    module_id: String,
    config: State<'_, MarketplaceConfig>,
) -> Result<ModuleDetails, String> {
    let url = format!("{}/modules/{}", config.api_url, module_id);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;

    let details: ModuleDetails = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(details)
}

#[tauri::command]
async fn install_from_marketplace<R: Runtime>(
    module_id: String,
    version: String,
    window: Window<R>,
) -> Result<(), String> {
    // Use existing installation system from Phase 2.3
    window.emit("marketplace-install-start", InstallEvent {
        module_id: module_id.clone(),
        version: version.clone(),
    }).map_err(|e| e.to_string())?;

    // Delegate to tauri-plugin-electron install command
    window.eval(&format!(
        "window.__TAURI__.invoke('plugin:electron|install_electron_module', {{ moduleId: '{}', version: '{}' }})",
        module_id, version
    )).map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Serialize)]
struct InstallEvent {
    module_id: String,
    version: String,
}

#[derive(Deserialize, Serialize)]
struct ModuleSearchResult {
    id: String,
    name: String,
    description: String,
    rating: Option<f32>,
    download_count: i64,
}

#[derive(Deserialize)]
struct SearchResponse {
    results: Vec<ModuleSearchResult>,
}

#[derive(Deserialize, Serialize)]
struct ModuleDetails {
    id: String,
    name: String,
    description: String,
    long_description: String,
    version: String,
    screenshots: Vec<String>,
    rating: Option<f32>,
    review_count: i32,
}
```

---

### 5. Web Marketplace UI (React)

**Purpose:** Beautiful, responsive web interface for browsing modules

**Implementation:**
```tsx
// packages/marketplace-web/src/pages/Marketplace.tsx

import React, { useState, useEffect } from 'react';
import { SearchBar } from '../components/SearchBar';
import { ModuleGrid } from '../components/ModuleGrid';
import { FilterPanel } from '../components/FilterPanel';
import { useModules } from '../hooks/useModules';

export function MarketplacePage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [category, setCategory] = useState<string | null>(null);
  const [sortBy, setSortBy] = useState('popular');

  const { modules, loading, error, refetch } = useModules({
    search: searchQuery,
    category,
    sort: sortBy,
  });

  return (
    <div className="marketplace-container">
      <header className="marketplace-header">
        <h1>Module Marketplace</h1>
        <SearchBar
          value={searchQuery}
          onChange={setSearchQuery}
          placeholder="Search modules..."
        />
      </header>

      <div className="marketplace-content">
        <aside className="filter-panel">
          <FilterPanel
            selectedCategory={category}
            onCategoryChange={setCategory}
            sortBy={sortBy}
            onSortChange={setSortBy}
          />
        </aside>

        <main className="module-grid-container">
          {loading && <LoadingSpinner />}
          {error && <ErrorMessage error={error} />}
          {modules && (
            <ModuleGrid
              modules={modules}
              onModuleClick={(module) => {
                window.location.href = `/modules/${module.id}`;
              }}
            />
          )}
        </main>
      </div>
    </div>
  );
}

// packages/marketplace-web/src/pages/ModuleDetails.tsx

import React from 'react';
import { useParams } from 'react-router-dom';
import { useModuleDetails } from '../hooks/useModuleDetails';
import { InstallButton } from '../components/InstallButton';
import { ReviewList } from '../components/ReviewList';
import { ScreenshotGallery } from '../components/ScreenshotGallery';

export function ModuleDetailsPage() {
  const { moduleId } = useParams<{ moduleId: string }>();
  const { module, loading, error } = useModuleDetails(moduleId!);

  if (loading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;
  if (!module) return <NotFound />;

  return (
    <div className="module-details">
      <div className="module-header">
        <img src={module.icon_url} alt={module.name} className="module-icon" />
        <div className="module-info">
          <h1>{module.name}</h1>
          <p className="module-description">{module.description}</p>

          <div className="module-meta">
            <span className="rating">
              ⭐ {module.average_rating?.toFixed(1) || 'N/A'}
              <span className="review-count">({module.review_count} reviews)</span>
            </span>
            <span className="downloads">
              📦 {module.download_count.toLocaleString()} downloads
            </span>
            <span className="category">
              🏷️ {module.category}
            </span>
          </div>

          <InstallButton
            moduleId={module.id}
            version={module.version}
            size={module.size_bytes}
          />
        </div>
      </div>

      <div className="module-content">
        <section className="screenshots">
          <h2>Screenshots</h2>
          <ScreenshotGallery screenshots={module.screenshots} />
        </section>

        <section className="description">
          <h2>About</h2>
          <div
            className="long-description"
            dangerouslySetInnerHTML={{ __html: module.long_description }}
          />
        </section>

        <section className="details-grid">
          <div className="detail-item">
            <h3>Version</h3>
            <p>{module.version}</p>
          </div>
          <div className="detail-item">
            <h3>License</h3>
            <p>{module.license}</p>
          </div>
          <div className="detail-item">
            <h3>Author</h3>
            <p>
              <a href={module.author_url} target="_blank" rel="noopener">
                {module.author_name}
              </a>
            </p>
          </div>
          <div className="detail-item">
            <h3>Repository</h3>
            <p>
              <a href={module.repository_url} target="_blank" rel="noopener">
                GitHub
              </a>
            </p>
          </div>
        </section>

        <section className="reviews">
          <h2>Reviews</h2>
          <ReviewList moduleId={module.id} />
        </section>
      </div>
    </div>
  );
}
```

---

## 📝 Task Breakdown

### Task 1: Marketplace API (Weeks 1-3)

**Owner:** Backend Engineer + Lead Developer

**Deliverables:**
- [ ] REST API with all endpoints
- [ ] PostgreSQL schema for modules, reviews, users
- [ ] API authentication and authorization
- [ ] Rate limiting and abuse prevention
- [ ] API documentation (OpenAPI/Swagger)
- [ ] Unit and integration tests

**Acceptance Criteria:**
- API handles 500 req/sec
- Response time <100ms (99th percentile)
- Database queries optimized
- Comprehensive error handling
- API docs complete

**Dependencies:** None

---

### Task 2: Search & Discovery (Weeks 2-4)

**Owner:** Backend Engineer + Data Engineer

**Deliverables:**
- [ ] Elasticsearch integration
- [ ] Search indexing pipeline
- [ ] Recommendation engine
- [ ] Relevance tuning
- [ ] Analytics integration
- [ ] Performance benchmarks

**Acceptance Criteria:**
- Search latency <200ms
- Relevant results in top 5 (>80% cases)
- Recommendations personalized
- Index updates in real-time
- Handles typos and fuzzy matching

**Dependencies:** Task 1 (API structure)

---

### Task 3: Review System (Weeks 3-5)

**Owner:** Backend Engineer + Content Moderator (new role)

**Deliverables:**
- [ ] Review submission and storage
- [ ] Rating calculation system
- [ ] Review moderation tools
- [ ] Helpful/unhelpful voting
- [ ] Spam detection
- [ ] Moderation queue

**Acceptance Criteria:**
- Reviews appear after moderation
- Rating updates in real-time
- Spam filtered automatically
- Moderation time <24 hours
- Users cannot review twice

**Dependencies:** Task 1 (API)

---

### Task 4: Web Marketplace UI (Weeks 4-7)

**Owner:** Frontend Developer + UX Designer

**Deliverables:**
- [ ] React-based marketplace website
- [ ] Module browsing and search UI
- [ ] Module details pages
- [ ] Review submission UI
- [ ] Responsive design (mobile/desktop)
- [ ] Accessibility compliance

**Acceptance Criteria:**
- Fast page loads (<2s initial)
- Smooth animations and transitions
- Works on mobile and desktop
- WCAG 2.1 AA compliant
- User testing feedback positive

**Dependencies:** Tasks 1-3 (API complete)

---

### Task 5: Tauri Plugin Integration (Weeks 6-8)

**Owner:** Lead Developer + Frontend Developer

**Deliverables:**
- [ ] Tauri plugin for in-app marketplace
- [ ] Native window integration
- [ ] One-click installation flow
- [ ] Progress tracking integration
- [ ] TypeScript API bindings
- [ ] Example applications

**Acceptance Criteria:**
- Opens in native window
- Feels native to Tauri app
- Installation works seamlessly
- Progress updates in real-time
- Works on all platforms

**Dependencies:** Task 4 (UI), Phase 2.3 (installer)

---

### Task 6: Developer Portal (Weeks 7-10)

**Owner:** Backend Engineer + Frontend Developer

**Deliverables:**
- [ ] Module submission workflow
- [ ] Analytics dashboard
- [ ] Review/approval process
- [ ] Update management
- [ ] Documentation and guidelines
- [ ] Developer onboarding

**Acceptance Criteria:**
- Submission process clear
- Analytics real-time and actionable
- Approval time <48 hours
- Updates easy to publish
- Documentation comprehensive

**Dependencies:** Tasks 1-4 (core infrastructure)

---

### Task 7: Testing & Launch (Weeks 11-12)

**Owner:** QA Engineer + All Team

**Deliverables:**
- [ ] End-to-end testing
- [ ] Performance testing
- [ ] Security audit
- [ ] Beta user program
- [ ] Launch preparation
- [ ] Monitoring setup

**Acceptance Criteria:**
- All user flows tested
- Performance targets met
- Security audit passed
- Beta feedback positive
- Monitoring comprehensive
- Ready for public launch

**Dependencies:** Tasks 1-6 (all features)

---

## 🏃 Sprint Structure

### Sprint 1-2 (Weeks 1-4): Backend Foundation
- **Goal:** API, search, and reviews operational
- **Demos:** API endpoints, search functionality
- **Risks:** Elasticsearch complexity, relevance tuning

### Sprint 3-4 (Weeks 5-8): Frontend & Integration
- **Goal:** Web UI and Tauri plugin working
- **Demos:** Full browsing experience, in-app marketplace
- **Risks:** UX feedback, integration issues

### Sprint 5-6 (Weeks 9-12): Developer Tools & Launch
- **Goal:** Developer portal, testing, public launch
- **Demos:** Complete system, developer onboarding
- **Risks:** Review process bottlenecks, launch readiness

---

## 👥 Team & Resources

### Core Team (Full-time)
- **Lead Developer** (12 weeks)
  - Architecture, Tauri plugin, integration
  - Rate: $150/hr × 40hr/wk × 12wks = $72,000

- **Backend Engineer** (12 weeks)
  - API, reviews, developer portal
  - Rate: $145/hr × 40hr/wk × 12wks = $69,600

- **Frontend Developer** (10 weeks)
  - React UI, components, responsive design
  - Rate: $130/hr × 40hr/wk × 10wks = $52,000

### New Roles (Full-time)
- **UX Designer** (8 weeks)
  - Marketplace design, user flows, accessibility
  - Rate: $120/hr × 40hr/wk × 8wks = $38,400

- **Data Engineer** (6 weeks)
  - Search, recommendations, analytics
  - Rate: $150/hr × 40hr/wk × 6wks = $36,000

### New Roles (Part-time)
- **Content Moderator** (50% × 12 weeks)
  - Review moderation, spam detection, guidelines
  - Rate: $80/hr × 20hr/wk × 12wks = $19,200

### Supporting Team (Part-time)
- **QA Engineer** (50% × 10 weeks)
  - Testing, automation, quality assurance
  - Rate: $110/hr × 20hr/wk × 10wks = $22,000

- **Technical Writer** (50% × 8 weeks)
  - Developer docs, user guides, API docs
  - Rate: $100/hr × 20hr/wk × 8wks = $16,000

**Total Personnel:** $325,200

---

## 💰 Budget Breakdown

| Category | Item | Cost |
|----------|------|------|
| **Personnel** | Core team (3 FT) | $193,600 |
| | New roles (3 FT/PT) | $93,600 |
| | Supporting team (2 PT) | $38,000 |
| **Infrastructure** | Elasticsearch cluster (3 months) | $4,500 |
| | PostgreSQL database | $2,400 |
| | Redis caching | $1,200 |
| | Web hosting & CDN | $2,000 |
| | Media storage (screenshots) | $1,500 |
| **Tools & Services** | Monitoring & analytics | $1,800 |
| | Email service (notifications) | $600 |
| | Search relevance tools | $1,000 |
| **Design & UX** | UI component library license | $800 |
| | Stock photos/assets | $500 |
| **Testing** | User testing program | $3,000 |
| | Security audit | $8,000 |
| **Contingency** | 10% buffer | $35,490 |
| **TOTAL** | | **$389,390** |

**Budget Range:** $350K - $430K

---

## 🎯 Success Criteria

### User Experience
- ✅ Users find relevant modules in <30 seconds
- ✅ Search latency <200ms
- ✅ Module installation rate >40% after viewing details
- ✅ User satisfaction >4.2/5
- ✅ Mobile experience rated highly

### Developer Experience
- ✅ Module submission time <15 minutes
- ✅ Approval time <48 hours
- ✅ Analytics dashboard comprehensive
- ✅ Documentation clear and complete
- ✅ Developer satisfaction >4.5/5

### Technical Performance
- ✅ API response time <100ms (99th percentile)
- ✅ Search handles 100 concurrent queries
- ✅ Web page load <2 seconds
- ✅ System uptime 99.9%
- ✅ Review moderation time <24 hours

### Business Metrics
- ✅ 100+ modules submitted in first month
- ✅ 10K+ marketplace visits/month
- ✅ 5K+ module installations/month
- ✅ Growing review count (>100/month)
- ✅ Active developer community (>50 contributors)

---

## 🚨 Risk Management

### High-Impact Risks

#### 1. Search Relevance Issues
- **Probability:** High
- **Impact:** High
- **Mitigation:**
  - Extensive relevance testing
  - A/B testing of algorithms
  - User feedback mechanisms
  - Continuous tuning
- **Contingency:** Simpler search, manual curation

#### 2. Low Module Submission Rate
- **Probability:** Medium
- **Impact:** Critical
- **Mitigation:**
  - Clear submission guidelines
  - Developer outreach program
  - Incentives for early submitters
  - Featured developer spotlight
- **Contingency:** Seed with first-party modules, partnerships

#### 3. Review Spam/Abuse
- **Probability:** High
- **Impact:** Medium
- **Mitigation:**
  - Automated spam detection
  - Human moderation
  - Verified purchase requirement
  - Rate limiting
- **Contingency:** Disable reviews temporarily, improve moderation

### Medium-Impact Risks

#### 4. UX Not Intuitive
- **Probability:** Medium
- **Impact:** High
- **Mitigation:**
  - Early user testing
  - UX designer involvement
  - Iterative design process
  - Analytics tracking user behavior
- **Contingency:** Redesign based on feedback

#### 5. Performance Issues
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:**
  - Performance testing early
  - Caching strategies
  - Database optimization
  - CDN for static assets
- **Contingency:** Scale infrastructure, optimize queries

---

## 📊 Milestones & Timeline

```
Week 1-3: Marketplace API
├─ Week 1: Database schema, basic endpoints
├─ Week 2: Complete CRUD operations
└─ Week 3: Authentication, rate limiting
    ✓ Milestone: API operational

Week 2-4: Search & Discovery
├─ Week 2-3: Elasticsearch setup, indexing
└─ Week 4: Recommendation engine, tuning
    ✓ Milestone: Search working

Week 3-5: Review System
├─ Week 3-4: Review API, moderation tools
└─ Week 5: Spam detection, voting
    ✓ Milestone: Reviews functional

Week 4-7: Web Marketplace UI
├─ Week 4-5: Core UI components, layouts
├─ Week 6: Module details, search UI
└─ Week 7: Polish, responsive design
    ✓ Milestone: Web marketplace live

Week 6-8: Tauri Plugin
├─ Week 6-7: Plugin implementation
└─ Week 8: Integration, testing
    ✓ Milestone: In-app browsing works

Week 7-10: Developer Portal
├─ Week 7-8: Submission workflow
├─ Week 9: Analytics dashboard
└─ Week 10: Documentation, onboarding
    ✓ Milestone: Developers can submit

Week 11-12: Testing & Launch
├─ Week 11: Testing, bug fixes, beta program
└─ Week 12: Final prep, public launch
    ✓ Milestone: Phase 2.5 complete
```

### Key Dates
- **Week 1:** Kickoff, design reviews
- **Week 4:** Backend complete demo
- **Week 7:** Web marketplace preview
- **Week 10:** Developer portal beta
- **Week 12:** Public launch

---

## 🔄 Integration Points

### With Phase 2.3 (Installation & Updates)
- Install button triggers Phase 2.3 installer
- Progress updates shown in marketplace UI
- Installation success tracked in analytics

### With Phase 2.4 (Distribution & CDN)
- Module catalog powered by distribution server
- Screenshots and assets served from CDN
- Download stats fed into recommendations

### With Phase 2.6 (Enterprise Features)
- Private marketplace for enterprise customers
- Custom approval workflows
- Organization-level analytics

---

## 📚 Documentation Deliverables

1. **User Guide**
   - Browsing and searching modules
   - Installing modules
   - Writing reviews
   - Managing installations

2. **Developer Guide**
   - Module submission checklist
   - Manifest requirements
   - Approval process
   - Update procedures
   - Analytics interpretation

3. **API Documentation**
   - OpenAPI specification
   - Authentication guide
   - Rate limiting details
   - Example code

4. **Design System**
   - UI components
   - Brand guidelines
   - Accessibility standards
   - Responsive patterns

5. **Moderation Guidelines**
   - Review approval criteria
   - Spam detection rules
   - Escalation procedures
   - Community standards

---

## ✅ Phase 2.5 Complete When

- [ ] Marketplace API fully operational
- [ ] Search returns relevant results (<200ms)
- [ ] Review system with moderation working
- [ ] Web marketplace responsive and accessible
- [ ] Tauri plugin integrated seamlessly
- [ ] Developer portal accepting submissions
- [ ] 50+ modules available at launch
- [ ] Performance targets met
- [ ] Security audit passed
- [ ] Beta user feedback positive
- [ ] Documentation complete
- [ ] Monitoring and analytics operational
- [ ] Team trained on operations
- [ ] Public launch successful

---

## 📞 Stakeholder Communication

### Weekly Updates
- Module submission count
- User engagement metrics
- Search performance stats
- Progress against timeline

### Demos
- Week 4: Backend APIs demo
- Week 7: Web marketplace preview
- Week 10: Developer portal demo
- Week 12: Full system walkthrough

### Decision Points
- Week 3: Search algorithm selection
- Week 6: UI design approval
- Week 9: Developer onboarding process
- Week 11: Launch readiness go/no-go

---

## 🔗 Related Documents

- **Phase 2 Overview:** [../../PHASE_2_PLANNING.md](../../PHASE_2_PLANNING.md)
- **Phase 2.1 Plan:** [PHASE_2_1_PLAN.md](./PHASE_2_1_PLAN.md)
- **Phase 2.2 Plan:** [PHASE_2_2_PLAN.md](./PHASE_2_2_PLAN.md)
- **Phase 2.3 Plan:** [PHASE_2_3_PLAN.md](./PHASE_2_3_PLAN.md)
- **Phase 2.4 Plan:** [PHASE_2_4_PLAN.md](./PHASE_2_4_PLAN.md)
- **Verification Report:** [../../VERIFICATION_REPORT.md](../../VERIFICATION_REPORT.md)

---

**Status:** Ready for Review
**Next Action:** Stakeholder approval, team allocation, UI design kickoff
**Target Start Date:** January 2027
**Target Completion:** March 2027
**Budget:** $389K (includes significant personnel costs)
