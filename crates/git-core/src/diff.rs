//! Diff operations

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

/// Diff output type
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiffSummary {
    pub files_changed: Vec<FileDiff>,
    pub insertions: usize,
    pub deletions: usize,
}

/// Individual file change
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDiff {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub status: FileStatus,
    pub insertions: usize,
    pub deletions: usize,
    pub patch: Option<String>,
}

/// Status of a file in a diff
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
}

/// Get diff between two commits
pub fn diff_commits(
    base_dir: &Path,
    repo_name: &str,
    base_oid: &str,
    head_oid: &str,
    include_patch: bool,
) -> Result<DiffSummary, GitError> {
    let repo_path = validate_repo_path(base_dir, repo_name)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let base_oid = git2::Oid::from_str(base_oid).map_err(|_| GitError::ObjectNotFound {
        hash: base_oid.to_string(),
    })?;
    let head_oid = git2::Oid::from_str(head_oid).map_err(|_| GitError::ObjectNotFound {
        hash: head_oid.to_string(),
    })?;

    let base_commit = git_repo.find_commit(base_oid)?;
    let head_commit = git_repo.find_commit(head_oid)?;

    let base_tree = base_commit.tree()?;
    let head_tree = head_commit.tree()?;

    let diff = git_repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)?;

    let mut files_changed = Vec::new();
    let mut insertions = 0;
    let mut deletions = 0;

    diff.foreach(
        &mut |delta, _| {
            let old_path = delta
                .old_file()
                .path()
                .map(|p| p.to_string_lossy().to_string());
            let new_path = delta
                .new_file()
                .path()
                .map(|p| p.to_string_lossy().to_string());
            let status = match delta.status() {
                git2::Delta::Added => FileStatus::Added,
                git2::Delta::Deleted => FileStatus::Deleted,
                git2::Delta::Modified => FileStatus::Modified,
                git2::Delta::Renamed => FileStatus::Renamed,
                git2::Delta::Copied => FileStatus::Copied,
                _ => FileStatus::Modified,
            };

            files_changed.push(FileDiff {
                old_path,
                new_path,
                status,
                insertions: 0,
                deletions: 0,
                patch: None,
            });
            true
        },
        None,
        None,
        None,
    )?;

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        match line.origin() {
            '+' => insertions += 1,
            '-' => deletions += 1,
            _ => {}
        }

        let path = delta
            .new_file()
            .path()
            .map(|p| p.to_string_lossy().to_string());
        if let Some(file) = files_changed.iter_mut().find(|f| f.new_path == path) {
            match line.origin() {
                '+' => file.insertions += 1,
                '-' => file.deletions += 1,
                _ => {}
            }
        }

        if include_patch {
            if let Some(file) = files_changed.iter_mut().find(|f| f.new_path == path) {
                let prefix = match line.origin() {
                    '+' => "+",
                    '-' => "-",
                    ' ' => " ",
                    _ => "",
                };
                if let Ok(content) = std::str::from_utf8(line.content()) {
                    let patch = file.patch.get_or_insert_with(String::new);
                    patch.push_str(prefix);
                    patch.push_str(content);
                }
            }
        }

        true
    })?;

    Ok(DiffSummary {
        files_changed,
        insertions,
        deletions,
    })
}

/// Get diff between two branches
pub fn diff_branch_from_base(
    base_dir: &Path,
    repo_name: &str,
    base_branch: &str,
    head_branch: &str,
    include_patch: bool,
) -> Result<DiffSummary, GitError> {
    let repo_path = validate_repo_path(base_dir, repo_name)?;
    let git_repo = git2::Repository::open(&repo_path)?;

    let base_ref = git_repo
        .find_branch(base_branch, git2::BranchType::Local)
        .map_err(|_| GitError::RefNotFound {
            r#ref: base_branch.to_string(),
        })?;
    let head_ref = git_repo
        .find_branch(head_branch, git2::BranchType::Local)
        .map_err(|_| GitError::RefNotFound {
            r#ref: head_branch.to_string(),
        })?;

    let base_oid = base_ref
        .get()
        .target()
        .ok_or_else(|| GitError::RefNotFound {
            r#ref: base_branch.to_string(),
        })?;
    let head_oid = head_ref
        .get()
        .target()
        .ok_or_else(|| GitError::RefNotFound {
            r#ref: head_branch.to_string(),
        })?;

    diff_commits(
        base_dir,
        repo_name,
        &base_oid.to_string(),
        &head_oid.to_string(),
        include_patch,
    )
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
    fn test_diff_empty_when_same_commit() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let repo_path = temp_dir.path().join(&name);
        let repo = git2::Repository::open(&repo_path).unwrap();
        let head = repo.find_branch("master", git2::BranchType::Local).unwrap();
        let head_oid = head.get().target().unwrap().to_string();

        let diff = diff_commits(temp_dir.path(), &name, &head_oid, &head_oid, false).unwrap();
        assert_eq!(diff.files_changed.len(), 0);
        assert_eq!(diff.insertions, 0);
        assert_eq!(diff.deletions, 0);
    }

    #[test]
    fn test_diff_branch_from_base_same_branch() {
        let (temp_dir, name) = make_repo_with_commits(1);
        let diff =
            diff_branch_from_base(temp_dir.path(), &name, "master", "master", false).unwrap();
        assert_eq!(diff.files_changed.len(), 0);
    }
}
