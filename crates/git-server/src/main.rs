//! Main entry point for git-server

use git_server::{run, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::from_env();
    run(config).await
}
