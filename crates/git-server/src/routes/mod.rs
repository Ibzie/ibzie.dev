//! Route modules

mod analyze;
mod branches;
mod commits;
mod diff;
mod files;
mod repos;

use crate::state::AppState;
use axum::Router;

/// Create the API router
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/repos", repos::router())
        .nest("/repos/:repo/commits", commits::router())
        .nest("/repos/:repo/branches", branches::router())
        .nest("/repos/:repo/diff", diff::router())
        .nest("/repos/:repo/files", files::router())
        .nest("/repos/:repo/analyze", analyze::router())
}
