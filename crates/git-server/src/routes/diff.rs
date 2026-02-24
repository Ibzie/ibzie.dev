//! Diff routes

use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use git_core::{DiffSummary, FileDiff, FileStatus};
use serde::Deserialize;

/// Query parameters for diff
#[derive(Deserialize, Default)]
pub struct DiffQuery {
    /// Base ref (branch or commit hash)
    base: Option<String>,
    /// Head ref (branch or commit hash)
    head: Option<String>,
    /// Include patch content in response
    #[serde(default)]
    include_patch: bool,
}

/// Diff file response
#[derive(serde::Serialize)]
struct FileDiffResponse {
    old_path: Option<String>,
    new_path: Option<String>,
    status: String,
    insertions: usize,
    deletions: usize,
    patch: Option<String>,
}

impl From<FileDiff> for FileDiffResponse {
    fn from(f: FileDiff) -> Self {
        let status = match f.status {
            FileStatus::Added => "added",
            FileStatus::Deleted => "deleted",
            FileStatus::Modified => "modified",
            FileStatus::Renamed => "renamed",
            FileStatus::Copied => "copied",
        };
        Self {
            old_path: f.old_path,
            new_path: f.new_path,
            status: status.to_string(),
            insertions: f.insertions,
            deletions: f.deletions,
            patch: f.patch,
        }
    }
}

/// Diff response type
#[derive(serde::Serialize)]
struct DiffResponse {
    files_changed: Vec<FileDiffResponse>,
    insertions: usize,
    deletions: usize,
}

impl From<DiffSummary> for DiffResponse {
    fn from(d: DiffSummary) -> Self {
        Self {
            files_changed: d
                .files_changed
                .into_iter()
                .map(FileDiffResponse::from)
                .collect(),
            insertions: d.insertions,
            deletions: d.deletions,
        }
    }
}

/// Get diff between two refs
async fn get_diff(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    Query(query): Query<DiffQuery>,
) -> Result<Json<DiffResponse>, (StatusCode, String)> {
    let base = query
        .base
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "base is required".to_string()))?;
    let head = query
        .head
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "head is required".to_string()))?;

    let base_dir = state.base_dir.clone();
    let include_patch = query.include_patch;

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

    Ok(Json(DiffResponse::from(diff)))
}

/// Create the diff router
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_diff))
}
