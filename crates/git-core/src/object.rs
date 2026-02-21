//! Object operations - read Blob, Tree, and Tag objects

use crate::error::GitError;
use crate::types::{Blob, FileMode, Signature, Tag, Tree, TreeEntry};
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

/// Read a blob object by hash
pub fn get_blob(
    base_dir: &Path,
    repo: &str,
    hash: &str,
) -> Result<Blob, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let oid = git2::Oid::from_str(hash).map_err(|_| GitError::ObjectNotFound {
        hash: hash.to_string(),
    })?;

    let blob = git_repo
        .find_blob(oid)
        .map_err(|_| GitError::ObjectNotFound {
            hash: hash.to_string(),
        })?;

    Ok(Blob {
        hash: oid.to_string(),
        data: blob.content().to_vec(),
        is_binary: blob.is_binary(),
    })
}

/// Read a tree object by hash
pub fn get_tree(base_dir: &Path, repo: &str, hash: &str) -> Result<Tree, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let oid = git2::Oid::from_str(hash).map_err(|_| GitError::ObjectNotFound {
        hash: hash.to_string(),
    })?;

    let tree = git_repo.find_tree(oid).map_err(|_| GitError::ObjectNotFound {
        hash: hash.to_string(),
    })?;

    let entries: Vec<TreeEntry> = tree
        .iter()
        .map(|entry| TreeEntry {
            hash: entry.id().to_string(),
            name: entry.name().unwrap_or("").to_string(),
            mode: match entry.filemode() {
                0x100644 => FileMode::Blob,
                0x100755 => FileMode::BlobExecutable,
                0x120000 => FileMode::Link,
                0x40000 => FileMode::Tree,
                _ => FileMode::Blob,
            },
        })
        .collect();

    Ok(Tree {
        hash: oid.to_string(),
        entries,
    })
}

/// Read a tag object by name
pub fn get_tag(base_dir: &Path, repo: &str, name: &str) -> Result<Tag, GitError> {
    let repo_path = validate_repo_path(base_dir, repo)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let tag_ref = format!("refs/tags/{}", name);
    let reference = git_repo
        .find_reference(&tag_ref)
        .map_err(|_| GitError::ObjectNotFound {
            hash: name.to_string(),
        })?;

    let oid = reference
        .target()
        .ok_or_else(|| GitError::ObjectNotFound {
            hash: name.to_string(),
        })?;

    let tag = git_repo.find_tag(oid).map_err(|_| GitError::ObjectNotFound {
        hash: name.to_string(),
    })?;

    let tagger = tag.tagger().map(|sig| Signature {
        name: sig.name().unwrap_or("").to_string(),
        email: sig.email().unwrap_or("").to_string(),
        timestamp: chrono::DateTime::from_timestamp(sig.when().seconds(), 0)
            .unwrap_or_else(chrono::Utc::now),
    });

    Ok(Tag {
        hash: oid.to_string(),
        name: name.to_string(),
        message: tag.message().map(String::from),
        tagger,
        target_hash: tag.target_id().to_string(),
        target_type: tag
            .target_type()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| "commit".to_string()),
    })
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
    fn test_get_blob() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();

        let head = repo
            .find_branch("master", git2::BranchType::Local)
            .unwrap();
        let head_oid = head.get().target().unwrap();
        let commit = repo.find_commit(head_oid).unwrap();
        let tree = commit.tree().unwrap();

        let entry = tree.get(0).expect("tree should have at least one entry");
        let blob_oid = entry.id();

        let blob = get_blob(temp_dir.path(), &name, &blob_oid.to_string()).unwrap();
        assert!(!blob.data.is_empty());
    }

    #[test]
    fn test_get_tree() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();

        let head = repo
            .find_branch("master", git2::BranchType::Local)
            .unwrap();
        let head_oid = head.get().target().unwrap();
        let commit = repo.find_commit(head_oid).unwrap();
        let tree_oid = commit.tree_id();

        let tree = get_tree(temp_dir.path(), &name, &tree_oid.to_string()).unwrap();
        assert!(!tree.entries.is_empty());
    }
}
