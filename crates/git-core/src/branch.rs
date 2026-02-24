//! Branch operations

use crate::error::GitError;
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

/// Branch info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub target_oid: String,
    pub is_head: bool,
}

/// List all branches in a repository
pub fn list_branches(base_dir: &Path, repo: &str) -> Result<Vec<BranchInfo>, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let head = git_repo.head().ok();
    let head_name = head.as_ref().and_then(|h| h.shorthand().map(String::from));

    let mut branches = Vec::new();

    for branch_result in git_repo.branches(Some(git2::BranchType::Local))? {
        let (branch, _) = branch_result?;
        let name = branch.name()?.unwrap_or("").to_string();
        let target_oid = branch
            .get()
            .target()
            .map(|oid| oid.to_string())
            .unwrap_or_default();
        let is_head = head_name.as_ref().map(|h| h == &name).unwrap_or(false);

        branches.push(BranchInfo {
            name,
            target_oid,
            is_head,
        });
    }

    Ok(branches)
}

/// Create a new branch
pub fn create_branch(
    base_dir: &Path,
    repo: &str,
    name: &str,
    from_oid: &str,
) -> Result<BranchInfo, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    if git_repo.find_branch(name, git2::BranchType::Local).is_ok() {
        return Err(GitError::BranchExists {
            name: name.to_string(),
        });
    }

    let oid = git2::Oid::from_str(from_oid).map_err(|_| GitError::ObjectNotFound {
        hash: from_oid.to_string(),
    })?;

    let commit = git_repo.find_commit(oid)?;
    let branch = git_repo.branch(name, &commit, false)?;

    let target_oid = branch
        .get()
        .target()
        .map(|oid| oid.to_string())
        .unwrap_or_default();

    Ok(BranchInfo {
        name: name.to_string(),
        target_oid,
        is_head: false,
    })
}

/// Delete a branch
pub fn delete_branch(base_dir: &Path, repo: &str, name: &str) -> Result<(), GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let mut branch = git_repo
        .find_branch(name, git2::BranchType::Local)
        .map_err(|_| GitError::ObjectNotFound {
            hash: name.to_string(),
        })?;

    branch.delete()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_repo_with_commits(n: usize) -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let name = "test-repo";

        let repo = git2::Repository::init(temp_dir.path().join(name)).unwrap();
        let sig = git2::Signature::now("Test User", "test@example.com").unwrap();

        for i in 0..n {
            let mut index = repo.index().unwrap();
            let path = format!("file_{}.txt", i);
            std::fs::write(
                temp_dir.path().join(name).join(&path),
                format!("content {}\n", i),
            )
            .unwrap();
            index.add_path(std::path::Path::new(&path)).unwrap();
            let oid = index.write_tree().unwrap();
            let tree = repo.find_tree(oid).unwrap();

            let parent = repo
                .find_branch("master", git2::BranchType::Local)
                .ok()
                .and_then(|b| b.get().target().and_then(|t| repo.find_commit(t).ok()));
            let parents: Vec<&git2::Commit> = parent.iter().collect();

            repo.commit(
                Some("refs/heads/master"),
                &sig,
                &sig,
                &format!("Commit {}", i + 1),
                &tree,
                &parents,
            )
            .unwrap();
        }

        (temp_dir, name.to_string())
    }

    #[test]
    fn test_list_branches() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let branches = list_branches(temp_dir.path(), &name).unwrap();
        assert!(branches.iter().any(|b| b.name == "master"));
    }

    #[test]
    fn test_create_branch() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();
        let head = repo.find_branch("master", git2::BranchType::Local).unwrap();
        let head_oid = head.get().target().unwrap().to_string();

        let branch = create_branch(temp_dir.path(), &name, "feature", &head_oid).unwrap();
        assert_eq!(branch.name, "feature");
    }

    #[test]
    fn test_delete_branch() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();
        let head = repo.find_branch("master", git2::BranchType::Local).unwrap();
        let head_oid = head.get().target().unwrap().to_string();

        create_branch(temp_dir.path(), &name, "feature", &head_oid).unwrap();
        delete_branch(temp_dir.path(), &name, "feature").unwrap();

        let result = delete_branch(temp_dir.path(), &name, "feature");
        assert!(matches!(result, Err(GitError::ObjectNotFound { .. })));
    }
}
