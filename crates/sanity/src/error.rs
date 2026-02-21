//! Error types for sanity

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SanityError {
    #[error("config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),
}
