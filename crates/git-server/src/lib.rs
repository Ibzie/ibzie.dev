//! Git Server - Axum HTTP server for ibzie.dev

pub mod auth;
pub mod routes;
pub mod state;

use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    /// The address to bind to
    pub host: String,
    /// The port to bind to
    pub port: u16,
    /// The directory where git repos are stored
    pub repo_base_dir: String,
    /// The API token for authentication
    pub api_token: String,
    /// Path to SQLite database
    pub db_path: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("IBZIE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("IBZIE_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("Invalid IBZIE_PORT"),
            repo_base_dir: std::env::var("IBZIE_REPO_DIR")
                .unwrap_or_else(|_| "./repos".to_string()),
            api_token: std::env::var("IBZIE_API_TOKEN").expect("IBZIE_API_TOKEN is required"),
            db_path: std::env::var("IBZIE_DB_PATH").unwrap_or_else(|_| "./ibzie.db".to_string()),
        }
    }
}

/// Root handler
async fn root() -> &'static str {
    "ibzie.dev git server"
}

/// Create the Axum application
pub fn create_app(state: state::AppState) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .route("/", get(root))
        .nest("/api", routes::router())
        .layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware))
        .layer(cors)
        .with_state(state)
}

/// Run the server
pub async fn run(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = state::AppState::new(&config).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Starting server on http://{}", addr);

    axum::serve(listener, create_app(state)).await?;

    Ok(())
}
