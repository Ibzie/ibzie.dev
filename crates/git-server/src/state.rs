//! Application state

use crate::ServerConfig;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state shared across all requests
#[derive(Clone)]
pub struct AppState {
    /// Base directory where git repos are stored
    pub base_dir: PathBuf,
    /// SQLite connection pool (wrapped in Arc<Mutex> for async safety)
    pub db: Arc<Mutex<Connection>>,
    /// API token for authentication
    pub api_token: String,
}

#[derive(thiserror::Error, Debug)]
pub enum StateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Failed to create repo directory: {0}")]
    CreateRepoDir(String),
}

impl AppState {
    /// Create new application state from config
    pub async fn new(config: &ServerConfig) -> Result<Self, StateError> {
        // Ensure repo base directory exists
        let base_dir = PathBuf::from(&config.repo_base_dir);
        if !base_dir.exists() {
            std::fs::create_dir_all(&base_dir)
                .map_err(|e| StateError::CreateRepoDir(e.to_string()))?;
        }

        // Open SQLite database
        let db = Connection::open(&config.db_path)?;
        init_db(&db)?;

        Ok(Self {
            base_dir,
            db: Arc::new(Mutex::new(db)),
            api_token: config.api_token.clone(),
        })
    }
}

/// Initialize the SQLite database schema
fn init_db(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS repos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            default_branch TEXT NOT NULL DEFAULT 'main',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    // Index on repo name for faster lookups
    db.execute(
        "CREATE INDEX IF NOT EXISTS idx_repos_name ON repos(name)",
        [],
    )?;

    Ok(())
}
