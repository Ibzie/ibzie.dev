//! Git Core - wraps libgit2 for repository operations

pub mod error;
pub mod types;
mod repo;
mod commit;
mod diff;
mod branch;
mod object;

// Re-export types
pub use types::*;

// Re-export diff types
pub use diff::{DiffSummary, FileDiff, FileStatus};
pub use branch::BranchInfo;

// Repository operations
pub fn init_repo(base_dir: &std::path::Path, name: &str) -> Result<Map, error::GitError> {
    repo::init_repo(base_dir, name)
}

pub fn open_repo(base_dir: &std::path::Path, name: &str) -> Result<Map, error::GitError> {
    repo::open_repo(base_dir, name)
}

pub fn list_repos(base_dir: &std::path::Path) -> Result<Vec<Map>, error::GitError> {
    repo::list_repos(base_dir)
}

// Commit operations
pub fn get_commit(
    base_dir: &std::path::Path,
    repo: &str,
    hash: &str,
) -> Result<Commit, error::GitError> {
    commit::get_commit(base_dir, repo, hash)
}

pub fn log(
    base_dir: &std::path::Path,
    repo: &str,
    branch: &str,
    limit: usize,
) -> Result<Vec<Commit>, error::GitError> {
    commit::log(base_dir, repo, branch, limit)
}

// Diff operations
pub fn diff_commits(
    base_dir: &std::path::Path,
    repo: &str,
    base_oid: &str,
    head_oid: &str,
    include_patch: bool,
) -> Result<DiffSummary, error::GitError> {
    diff::diff_commits(base_dir, repo, base_oid, head_oid, include_patch)
}

pub fn diff_branch_from_base(
    base_dir: &std::path::Path,
    repo: &str,
    base_branch: &str,
    head_branch: &str,
    include_patch: bool,
) -> Result<DiffSummary, error::GitError> {
    diff::diff_branch_from_base(base_dir, repo, base_branch, head_branch, include_patch)
}

// Branch operations
pub fn list_branches(
    base_dir: &std::path::Path,
    repo: &str,
) -> Result<Vec<BranchInfo>, error::GitError> {
    branch::list_branches(base_dir, repo)
}

pub fn create_branch(
    base_dir: &std::path::Path,
    repo: &str,
    name: &str,
    from_oid: &str,
) -> Result<BranchInfo, error::GitError> {
    branch::create_branch(base_dir, repo, name, from_oid)
}

pub fn delete_branch(base_dir: &std::path::Path, repo: &str, name: &str) -> Result<(), error::GitError> {
    branch::delete_branch(base_dir, repo, name)
}

// Object operations
pub fn get_blob(
    base_dir: &std::path::Path,
    repo: &str,
    hash: &str,
) -> Result<Blob, error::GitError> {
    object::get_blob(base_dir, repo, hash)
}

pub fn get_tree(
    base_dir: &std::path::Path,
    repo: &str,
    hash: &str,
) -> Result<Tree, error::GitError> {
    object::get_tree(base_dir, repo, hash)
}

pub fn get_tag(base_dir: &std::path::Path, repo: &str, name: &str) -> Result<Tag, error::GitError> {
    object::get_tag(base_dir, repo, name)
}
