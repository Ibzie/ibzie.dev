//! Repository operations

use crate::error::GitError;
use crate::types::Map;
use std::path::Path;

fn validate_repo_path(base_dir: &Path, name: &str) -> Result<std::path::PathBuf, GitError> {
    let resolved = base_dir.join(name);
    let canonical_base = base_dir
        .canonicalize()
        .map_err(|_| GitError::PathTraversal)?;

    if resolved.exists() {
        let canonical_resolved = resolved
            .canonicalize()
            .map_err(|_| GitError::PathTraversal)?;
        if !canonical_resolved.starts_with(&canonical_base) {
            return Err(GitError::PathTraversal);
        }
    } else if let Some(parent) = resolved.parent() {
        let canonical_parent = parent.canonicalize().map_err(|_| GitError::PathTraversal)?;
        if !canonical_parent.starts_with(&canonical_base) {
            return Err(GitError::PathTraversal);
        }
    }

    Ok(resolved)
}

fn get_default_branch(repo: &git2::Repository) -> String {
    repo.find_branch("main", git2::BranchType::Local)
        .map(|_| "main".to_string())
        .or_else(|_| {
            repo.find_branch("master", git2::BranchType::Local)
                .map(|_| "master".to_string())
        })
        .unwrap_or_else(|_| "main".to_string())
}

/// Initialize a new repository
pub fn init_repo(base_dir: &Path, name: &str) -> Result<Map, GitError> {
    let repo_path = validate_repo_path(base_dir, name)?;

    if let Some(parent) = repo_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let repo = git2::Repository::init_bare(&repo_path)?;

    let default_branch = get_default_branch(&repo);
    let is_empty = repo.is_empty()?;

    Ok(Map {
        name: name.to_string(),
        description: None,
        is_bare: true,
        is_empty,
        default_branch,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    })
}

/// Open an existing repository
pub fn open_repo(base_dir: &Path, name: &str) -> Result<Map, GitError> {
    let repo_path = validate_repo_path(base_dir, name)?;

    if !repo_path.exists() {
        return Err(GitError::RepoNotFound {
            name: name.to_string(),
        });
    }

    let repo = git2::Repository::open(&repo_path)?;

    let default_branch = get_default_branch(&repo);
    let is_empty = repo.is_empty()?;

    Ok(Map {
        name: name.to_string(),
        description: None,
        is_bare: repo.is_bare(),
        is_empty,
        default_branch,
        created_at: None,
        updated_at: Some(chrono::Utc::now()),
    })
}

/// List all repositories in the base directory
pub fn list_repos(base_dir: &Path) -> Result<Vec<Map>, GitError> {
    let mut repos = Vec::new();

    if !base_dir.exists() {
        return Ok(repos);
    }

    let _ = base_dir.canonicalize();

    for entry in std::fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Ok(repo) = git2::Repository::open(&path) {
                let name = entry.file_name().to_string_lossy().to_string();
                let default_branch = get_default_branch(&repo);
                let is_empty = repo.is_empty().unwrap_or(true);

                repos.push(Map {
                    name,
                    description: None,
                    is_bare: repo.is_bare(),
                    is_empty,
                    default_branch,
                    created_at: None,
                    updated_at: Some(chrono::Utc::now()),
                });
            }
        }
    }

    Ok(repos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_traversal_blocked() {
        let temp_dir = TempDir::new().unwrap();
        let result = init_repo(temp_dir.path(), "../../../etc");
        assert!(matches!(result, Err(GitError::PathTraversal)));
    }

    #[test]
    fn test_init_and_open_repo() {
        let temp_dir = TempDir::new().unwrap();
        let name = "test-repo";

        let info = init_repo(temp_dir.path(), name).unwrap();
        assert_eq!(info.name, name);
        assert!(info.is_empty);

        let info2 = open_repo(temp_dir.path(), name).unwrap();
        assert_eq!(info2.name, name);
    }

    #[test]
    fn test_open_nonexistent_repo() {
        let temp_dir = TempDir::new().unwrap();
        let result = open_repo(temp_dir.path(), "nonexistent");
        assert!(matches!(result, Err(GitError::RepoNotFound { .. })));
    }
}
