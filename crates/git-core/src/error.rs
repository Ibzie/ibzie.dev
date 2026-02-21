//! Error types for git-core

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("repository not found: {name}")]
    RepoNotFound { name: String },

    #[error("path traversal attempt detected")]
    PathTraversal,

    #[error("ref not found: {r#ref}")]
    RefNotFound { r#ref: String },

    #[error("object not found: {hash}")]
    ObjectNotFound { hash: String },

    #[error("branch already exists: {name}")]
    BranchExists { name: String },

    #[error("libgit2 error: {0}")]
    Git2(#[from] git2::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
