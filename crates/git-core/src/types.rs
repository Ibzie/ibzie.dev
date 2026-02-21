//! Git storage types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Git blob object - stores file content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    /// The SHA-1 hash of this blob
    pub hash: String,
    /// The raw content of the blob
    pub data: Vec<u8>,
    /// Whether the blob is binary (non-text)
    pub is_binary: bool,
}

/// A Git tree object - stores directory structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// The SHA-1 hash of this tree
    pub hash: String,
    /// Entries in this tree (files and subdirectories)
    pub entries: Vec<TreeEntry>,
}

/// A single entry in a tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    /// The SHA-1 hash of this entry
    pub hash: String,
    /// The name of this entry (filename or directory name)
    pub name: String,
    /// The mode (file, symlink, directory, or executable)
    pub mode: FileMode,
}

/// File mode for tree entries
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileMode {
    /// 100644 - regular file
    Blob,
    /// 100755 - executable file
    BlobExecutable,
    /// 120000 - symbolic link
    Link,
    /// 040000 - directory
    Tree,
}

/// A Git commit object - stores a snapshot with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// The SHA-1 hash of this commit
    pub hash: String,
    /// The commit message
    pub message: String,
    /// The author of the commit
    pub author: Signature,
    /// The committer of the commit
    pub committer: Signature,
    /// The SHA-1 hash of the tree this commit points to
    pub tree_hash: String,
    /// Parent commit hashes (can be multiple for merge commits)
    pub parent_hashes: Vec<String>,
}

/// A Git signature (author or committer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

/// A Git tag object - marks a specific commit with a name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// The SHA-1 hash of this tag
    pub hash: String,
    /// The tag name
    pub name: String,
    /// The tag message (for annotated tags)
    pub message: Option<String>,
    /// The tagger (author of the tag)
    pub tagger: Option<Signature>,
    /// The SHA-1 hash of the commit this tag points to
    pub target_hash: String,
    /// The type of object being tagged (usually "commit")
    pub target_type: String,
}

/// Repository metadata - stores information about a specific repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    /// The repository name
    pub name: String,
    /// The repository description
    pub description: Option<String>,
    /// Whether the repository is bare (no working directory)
    pub is_bare: bool,
    /// Whether the repository is empty (no commits)
    pub is_empty: bool,
    /// The default branch name (e.g., "main", "master")
    pub default_branch: String,
    /// When the repository was created
    pub created_at: Option<DateTime<Utc>>,
    /// When the repository was last updated
    pub updated_at: Option<DateTime<Utc>>,
}
