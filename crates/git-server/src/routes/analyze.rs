//! Analyze routes - PR sanity scoring

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use sanity::{load_config, SanityConfig, SanityReport};

/// Analyze request body
#[derive(serde::Deserialize)]
pub struct AnalyzeRequest {
    /// Base ref (branch or commit hash)
    base: String,
    /// Head ref (branch or commit hash)
    head: String,
    /// Optional TOML config (if not provided, uses defaults)
    config: Option<String>,
    /// Include patch content in diff (required for some metrics)
    #[serde(default)]
    include_patch: bool,
}

/// Analyze response
#[derive(serde::Serialize)]
struct AnalyzeResponse {
    score: i32,
    raw_score: i32,
    passed: bool,
    metrics: Vec<MetricResponse>,
}

/// Metric result response
#[derive(serde::Serialize)]
struct MetricResponse {
    name: String,
    delta: i32,
    detail: String,
}

impl From<sanity::MetricResult> for MetricResponse {
    fn from(m: sanity::MetricResult) -> Self {
        Self {
            name: m.name.to_string(),
            delta: m.delta,
            detail: m.detail,
        }
    }
}

impl From<SanityReport> for AnalyzeResponse {
    fn from(r: SanityReport) -> Self {
        Self {
            score: r.score,
            raw_score: r.raw_score,
            passed: r.passed,
            metrics: r.metrics.into_iter().map(MetricResponse::from).collect(),
        }
    }
}

/// Run sanity analysis on a diff
async fn analyze_pr(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, String)> {
    // Get the diff
    let base_dir = state.base_dir.clone();
    let base = payload.base.clone();
    let head = payload.head.clone();
    let include_patch = payload.include_patch;

    let diff = tokio::task::spawn_blocking(move || {
        // Try interpreting as commits first, then as branches
        let result = git_core::diff_commits(&base_dir, &repo, &base, &head, include_patch);
        if let Ok(diff) = result {
            return Ok::<_, git_core::error::GitError>(diff);
        }

        // Try as branches
        git_core::diff_branch_from_base(&base_dir, &repo, &base, &head, include_patch)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Load config or use defaults
    let config = if let Some(config_str) = payload.config {
        load_config(&config_str).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    } else {
        SanityConfig::default()
    };

    // Run analysis (pure function, no async needed)
    let report = sanity::analyze(&diff, &config);

    Ok(Json(AnalyzeResponse::from(report)))
}

/// Create the analyze router
pub fn router() -> Router<AppState> {
    Router::new().route("/", post(analyze_pr))
}
