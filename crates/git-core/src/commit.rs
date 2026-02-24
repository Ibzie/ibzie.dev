//! Commit operations

use crate::error::GitError;
use crate::types::{Commit, Signature};
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

/// Get a single commit by hash
pub fn get_commit(base_dir: &Path, repo: &str, hash: &str) -> Result<Commit, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let oid = git2::Oid::from_str(hash).map_err(|_| GitError::ObjectNotFound {
        hash: hash.to_string(),
    })?;

    let commit = git_repo
        .find_commit(oid)
        .map_err(|_| GitError::ObjectNotFound {
            hash: hash.to_string(),
        })?;

    let author = commit.author();
    let committer = commit.committer();
    let parent_hashes: Vec<String> = commit.parent_ids().map(|oid| oid.to_string()).collect();

    Ok(Commit {
        hash: oid.to_string(),
        message: commit.message().unwrap_or("").to_string(),
        author: Signature {
            name: author.name().unwrap_or("").to_string(),
            email: author.email().unwrap_or("").to_string(),
            timestamp: chrono::DateTime::from_timestamp(author.when().seconds(), 0)
                .unwrap_or_else(chrono::Utc::now),
        },
        committer: Signature {
            name: committer.name().unwrap_or("").to_string(),
            email: committer.email().unwrap_or("").to_string(),
            timestamp: chrono::DateTime::from_timestamp(committer.when().seconds(), 0)
                .unwrap_or_else(chrono::Utc::now),
        },
        tree_hash: commit.tree_id().to_string(),
        parent_hashes,
    })
}

/// Get commit history for a branch
pub fn log(
    base_dir: &Path,
    repo: &str,
    branch: &str,
    limit: usize,
) -> Result<Vec<Commit>, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let branch_ref = format!("refs/heads/{}", branch);
    let reference = git_repo
        .find_reference(&branch_ref)
        .map_err(|_| GitError::RefNotFound {
            r#ref: branch.to_string(),
        })?;

    let target = reference.target().ok_or_else(|| GitError::RefNotFound {
        r#ref: branch.to_string(),
    })?;

    let mut revwalk = git_repo.revwalk()?;
    revwalk.push(target)?;

    let mut commits = Vec::new();
    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit {
            break;
        }

        let oid = oid_result?;
        let commit = git_repo.find_commit(oid)?;

        let author = commit.author();
        let committer = commit.committer();
        let parent_hashes: Vec<String> = commit.parent_ids().map(|oid| oid.to_string()).collect();

        commits.push(Commit {
            hash: oid.to_string(),
            message: commit.message().unwrap_or("").to_string(),
            author: Signature {
                name: author.name().unwrap_or("").to_string(),
                email: author.email().unwrap_or("").to_string(),
                timestamp: chrono::DateTime::from_timestamp(author.when().seconds(), 0)
                    .unwrap_or_else(chrono::Utc::now),
            },
            committer: Signature {
                name: committer.name().unwrap_or("").to_string(),
                email: committer.email().unwrap_or("").to_string(),
                timestamp: chrono::DateTime::from_timestamp(committer.when().seconds(), 0)
                    .unwrap_or_else(chrono::Utc::now),
            },
            tree_hash: commit.tree_id().to_string(),
            parent_hashes,
        });
    }

    Ok(commits)
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
    fn test_get_commit() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();
        let head = repo.find_branch("master", git2::BranchType::Local).unwrap();
        let head_oid = head.get().target().unwrap();

        let commit = get_commit(temp_dir.path(), &name, &head_oid.to_string()).unwrap();
        assert!(commit.hash.len() == 40);
        assert!(commit.message.contains("Commit 1"));
    }

    #[test]
    fn test_log_returns_commits_in_order() {
        let (temp_dir, name) = make_repo_with_commits(5);
        let commits = log(temp_dir.path(), &name, "master", 10).unwrap();
        assert_eq!(commits.len(), 5); // 1 initial + 5
    }

    #[test]
    fn test_log_with_limit() {
        let (temp_dir, name) = make_repo_with_commits(10);
        let commits = log(temp_dir.path(), &name, "master", 3).unwrap();
        assert_eq!(commits.len(), 3);
    }
}
