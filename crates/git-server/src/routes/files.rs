//! File routes

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

/// Tree entry response
#[derive(serde::Serialize)]
struct TreeEntryResponse {
    hash: String,
    name: String,
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
}

/// Get file tree at a ref
async fn get_tree(
    State(state): State<AppState>,
    Path((repo, ref_or_hash)): Path<(String, String)>,
) -> Result<Json<Vec<TreeEntryResponse>>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();
    let repo_for_tree = repo.clone();

    // First resolve the ref to a hash
    let tree_hash = tokio::task::spawn_blocking(move || {
        // Try to find the commit first
        if let Ok(commit) = git_core::get_commit(&base_dir, &repo, &ref_or_hash) {
            return Ok::<_, git_core::error::GitError>(commit.tree_hash);
        }

        // Try as a branch
        let branches = git_core::list_branches(&base_dir, &repo)?;
        if let Some(branch) = branches.into_iter().find(|b| b.name == ref_or_hash) {
            return Ok(branch.target_oid);
        }

        // Assume it's already a hash
        Ok(ref_or_hash)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    // Get the tree
    let base_dir = state.base_dir.clone();
    let tree = tokio::task::spawn_blocking(move || {
        git_core::get_tree(&base_dir, &repo_for_tree, &tree_hash)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    let entries: Vec<TreeEntryResponse> = tree
        .entries
        .into_iter()
        .map(|e| {
            let entry_type = match e.mode {
                git_core::FileMode::Blob | git_core::FileMode::BlobExecutable => "blob",
                git_core::FileMode::Tree => "tree",
                git_core::FileMode::Link => "link",
            };
            TreeEntryResponse {
                hash: e.hash,
                name: e.name.clone(),
                path: e.name,
                entry_type: entry_type.to_string(),
            }
        })
        .collect();

    Ok(Json(entries))
}

/// Get blob content
async fn get_blob(
    State(state): State<AppState>,
    Path((repo, hash)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();

    let blob = tokio::task::spawn_blocking(move || git_core::get_blob(&base_dir, &repo, &hash))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    // Check if binary
    if blob.is_binary {
        return Ok((
            StatusCode::OK,
            [("Content-Type", "application/octet-stream")],
            blob.data,
        ));
    }

    // Return as text
    let content = String::from_utf8(blob.data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid UTF-8".to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        [("Content-Type", "text/plain; charset=utf-8")],
        content.into_bytes(),
    ))
}

/// Create the files router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:ref", get(get_tree))
        .route("/:ref/*path", get(get_tree))
        .route("/blob/:hash", get(get_blob))
}
