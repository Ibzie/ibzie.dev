//! Authentication middleware

use crate::state::AppState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

/// Authentication middleware that validates the API token
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get API token from request headers
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    // Check for Bearer token
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate token against the one in state
    if token != state.api_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
