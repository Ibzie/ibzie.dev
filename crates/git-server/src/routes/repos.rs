//! Repository routes

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};

/// Repository response type
#[derive(serde::Serialize)]
struct RepoResponse {
    name: String,
    description: Option<String>,
    default_branch: String,
    is_empty: bool,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl From<git_core::Map> for RepoResponse {
    fn from(m: git_core::Map) -> Self {
        Self {
            name: m.name,
            description: m.description,
            default_branch: m.default_branch,
            is_empty: m.is_empty,
            created_at: m.created_at.map(|t| t.to_rfc3339()),
            updated_at: m.updated_at.map(|t| t.to_rfc3339()),
        }
    }
}

/// Create repo request body
#[derive(serde::Deserialize)]
pub struct CreateRepoRequest {
    name: String,
    description: Option<String>,
}

/// List all repos
async fn list_repos(
    State(state): State<AppState>,
) -> Result<Json<Vec<RepoResponse>>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();
    let repos = tokio::task::spawn_blocking(move || git_core::list_repos(&base_dir))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(repos.into_iter().map(RepoResponse::from).collect()))
}

/// Get a single repo
async fn get_repo(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<RepoResponse>, (StatusCode, String)> {
    let base_dir = state.base_dir.clone();
    let repo = tokio::task::spawn_blocking(move || git_core::open_repo(&base_dir, &name))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(RepoResponse::from(repo)))
}

/// Create a new repo
async fn create_repo(
    State(state): State<AppState>,
    Json(payload): Json<CreateRepoRequest>,
) -> Result<(StatusCode, Json<RepoResponse>), (StatusCode, String)> {
    let base_dir = state.base_dir.clone();
    let name = payload.name.clone();
    let description = payload.description.clone();

    let repo = tokio::task::spawn_blocking(move || {
        let mut repo = git_core::init_repo(&base_dir, &name)?;
        if let Some(ref desc) = description {
            repo.description = Some(desc.clone());
        }
        Ok::<_, git_core::error::GitError>(repo)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Store in database
    let db = state.db.lock().await;
    db.execute(
        "INSERT OR IGNORE INTO repos (name, description, default_branch) VALUES (?1, ?2, ?3)",
        rusqlite::params![&repo.name, &repo.description, &repo.default_branch],
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(RepoResponse::from(repo))))
}

/// Delete a repo
async fn delete_repo(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let name_for_check = name.clone();
    let name_for_db = name.clone();
    let base_dir = state.base_dir.clone();

    // Check if repo exists
    let _ = tokio::task::spawn_blocking(move || git_core::open_repo(&base_dir, &name_for_check))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|_| (StatusCode::NOT_FOUND, "Repository not found".to_string()))?;

    // Remove from database first
    let db = state.db.lock().await;
    db.execute(
        "DELETE FROM repos WHERE name = ?1",
        rusqlite::params![&name_for_db],
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    drop(db);

    // Delete the git directory
    let repo_path = state.base_dir.join(&name);
    tokio::task::spawn_blocking(move || std::fs::remove_dir_all(repo_path))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Create the repos router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_repos))
        .route("/", post(create_repo))
        .route("/:name", get(get_repo))
        .route("/:name", delete(delete_repo))
}
