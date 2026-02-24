//! Branch routes

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};

/// Branch response type
#[derive(serde::Serialize)]
struct BranchResponse {
    name: String,
    target_hash: String,
    is_head: bool,
}

impl From<git_core::BranchInfo> for BranchResponse {
    fn from(b: git_core::BranchInfo) -> Self {
        Self {
            name: b.name,
            target_hash: b.target_oid,
            is_head: b.is_head,
        }
    }
}

/// Create branch request
#[derive(serde::Deserialize)]
pub struct CreateBranchRequest {
    name: String,
    from_oid: String,
}

/// List branches
async fn list_branches(
    State(state): State<AppState>,
    Path(repo): Path<String>,
) -> Result<Json<Vec<BranchResponse>>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();

    let branches = tokio::task::spawn_blocking(move || git_core::list_branches(&base_dir, &repo))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(
        branches.into_iter().map(BranchResponse::from).collect(),
    ))
}

/// Create a branch
async fn create_branch(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    Json(payload): Json<CreateBranchRequest>,
) -> Result<Json<BranchResponse>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();

    let branch = tokio::task::spawn_blocking(move || {
        git_core::create_branch(&base_dir, &repo, &payload.name, &payload.from_oid)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(BranchResponse::from(branch)))
}

/// Delete a branch
async fn delete_branch(
    State(state): State<AppState>,
    Path((repo, name)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();

    tokio::task::spawn_blocking(move || git_core::delete_branch(&base_dir, &repo, &name))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Create the branches router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_branches))
        .route("/", post(create_branch))
        .route("/:name", delete(delete_branch))
}
