//! Commit routes

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;

/// Query parameters for listing commits
#[derive(Deserialize, Default)]
pub struct CommitQuery {
    /// Branch name (defaults to "main")
    #[serde(default = "default_branch")]
    branch: String,
    /// Maximum number of commits to return
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_limit() -> usize {
    50
}

/// Commit response type
#[derive(serde::Serialize)]
struct CommitResponse {
    hash: String,
    message: String,
    author: git_core::Signature,
    committer: git_core::Signature,
    parent_hashes: Vec<String>,
    tree_hash: String,
}

impl From<git_core::Commit> for CommitResponse {
    fn from(c: git_core::Commit) -> Self {
        Self {
            hash: c.hash,
            message: c.message,
            author: c.author,
            committer: c.committer,
            parent_hashes: c.parent_hashes,
            tree_hash: c.tree_hash,
        }
    }
}

/// List commits for a repo
async fn list_commits(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    Query(query): Query<CommitQuery>,
) -> Result<Json<Vec<CommitResponse>>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();
    let branch = if query.branch.is_empty() {
        "main".to_string()
    } else {
        query.branch
    };
    let limit = if query.limit == 0 { 50 } else { query.limit };

    let commits =
        tokio::task::spawn_blocking(move || git_core::log(&base_dir, &repo, &branch, limit))
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(
        commits.into_iter().map(CommitResponse::from).collect(),
    ))
}

/// Get a single commit
async fn get_commit(
    State(state): State<AppState>,
    Path((repo, hash)): Path<(String, String)>,
) -> Result<Json<CommitResponse>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();

    let commit = tokio::task::spawn_blocking(move || git_core::get_commit(&base_dir, &repo, &hash))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(CommitResponse::from(commit)))
}

/// Create the commits router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_commits))
        .route("/:hash", get(get_commit))
}
