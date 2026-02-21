# Claude Code Prompt — `git-core` + `sanity` Crates
## ibzie.dev · Rust Workspace

---

## CONTEXT & ROLE

You are building two crates inside a Cargo workspace for a self-hosted Git platform called ibzie.dev. The overall platform is a Git hosting service written in Rust with a built-in code sanity scoring system. You are responsible ONLY for these two crates:

- `crates/git-core` — wraps libgit2 (via the `git2` crate) and exposes clean async-friendly Rust types for all Git operations the HTTP server needs
- `crates/sanity` — a pure analysis engine that takes diff/commit data from `git-core` and produces a weighted sanity score for a PR

You are NOT responsible for:
- The Axum HTTP server (`crates/git-server`) — it will call into your public API
- The async worker crate (`crates/workers`)
- The frontend (Bun + TypeScript)
- The Python AI container

Everything you build must be correct, idiomatic Rust. Prefer explicit error handling with `thiserror` over `unwrap()`. Minimize `clone()` where possible. All public types must derive `Debug`.

---

## WORKSPACE STRUCTURE

```
ibzie/
├── Cargo.toml                  ← workspace root, defines members
├── crates/
│   ├── git-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs
│   │       ├── repo.rs         ← repository open/init/clone
│   │       ├── commit.rs       ← commit reading, log traversal
│   │       ├── diff.rs         ← diff between refs/commits
│   │       ├── branch.rs       ← branch CRUD
│   │       ├── reference.rs    ← ref resolution helpers
│   │       └── types.rs        ← shared domain types
│   └── sanity/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── error.rs
│           ├── score.rs        ← score calculation, weighted sum
│           ├── metrics.rs      ← metric definitions and trait
│           ├── analyzers/
│           │   ├── mod.rs
│           │   ├── python.rs
│           │   ├── rust.rs
│           │   └── javascript.rs
│           └── config.rs       ← TOML config deserialization
```

---

## CRATE 1: `git-core`

### Purpose

A synchronous-at-heart wrapper around `git2` (libgit2 Rust bindings) that exposes the subset of Git operations the HTTP server needs. Because `git2::Repository` is not `Send`, all blocking operations must be wrapped in `tokio::task::spawn_blocking` at the call site (the server crate handles this — you expose sync functions here).

### Dependencies (Cargo.toml)

```toml
[dependencies]
git2 = "0.19"
thiserror = "1"
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tempfile = "3"
```

### Architecture Decisions

**Decision 1 — Sync API, not async.**
`git2::Repository` is not `Send + Sync`. Wrapping it in `Arc<Mutex<>>` and pretending it is async leads to subtle deadlocks. Instead, expose a clean sync API. The server crate will call `spawn_blocking`. This is the officially recommended pattern for blocking I/O in Tokio.

**Decision 2 — No re-exporting of `git2` types.**
All public types must be owned types defined in `crates/git-core/src/types.rs`. The `git2` crate is an implementation detail. This means the server and sanity crates have zero dependency on `git2` directly.

**Decision 3 — Repository is opened per-operation, not held open.**
Do not store `git2::Repository` in a long-lived struct. Open it, perform the operation, drop it. This avoids lock contention when multiple requests touch the same repo and is safe with libgit2's threading model.

**Decision 4 — All paths are validated before passing to libgit2.**
Repo paths come from user input (repo names in URLs). Always validate that the resolved path stays within the configured base directory to prevent path traversal attacks.

### Public Types (types.rs)

```rust
// All types must be owned, Serialize/Deserialize, Debug

pub struct RepoInfo {
    pub name: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub is_empty: bool,
}

pub struct CommitInfo {
    pub oid: String,           // hex SHA
    pub message: String,
    pub author: Signature,
    pub committer: Signature,
    pub parent_oids: Vec<String>,
}

pub struct Signature {
    pub name: String,
    pub email: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct DiffSummary {
    pub files_changed: Vec<FileDiff>,
    pub insertions: usize,
    pub deletions: usize,
}

pub struct FileDiff {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub status: FileStatus,      // Added, Deleted, Modified, Renamed
    pub insertions: usize,
    pub deletions: usize>,
    pub patch: Option<String>,   // unified diff text, Some only when requested
}

pub enum FileStatus { Added, Deleted, Modified, Renamed, Copied }

pub struct BranchInfo {
    pub name: String,
    pub target_oid: String,
    pub is_head: bool,
}
```

### Public API (lib.rs re-exports)

```rust
// Repository operations
pub fn init_repo(base_dir: &Path, name: &str) -> Result<RepoInfo, GitError>;
pub fn open_repo(base_dir: &Path, name: &str) -> Result<RepoInfo, GitError>;
pub fn list_repos(base_dir: &Path) -> Result<Vec<RepoInfo>, GitError>;

// Commit operations
pub fn get_commit(base_dir: &Path, repo: &str, oid: &str) -> Result<CommitInfo, GitError>;
pub fn log(base_dir: &Path, repo: &str, branch: &str, limit: usize) -> Result<Vec<CommitInfo>, GitError>;

// Diff operations  
pub fn diff_commits(base_dir: &Path, repo: &str, base_oid: &str, head_oid: &str, include_patch: bool) -> Result<DiffSummary, GitError>;
pub fn diff_branch_from_base(base_dir: &Path, repo: &str, base_branch: &str, head_branch: &str, include_patch: bool) -> Result<DiffSummary, GitError>;

// Branch operations
pub fn list_branches(base_dir: &Path, repo: &str) -> Result<Vec<BranchInfo>, GitError>;
pub fn create_branch(base_dir: &Path, repo: &str, name: &str, from_oid: &str) -> Result<BranchInfo, GitError>;
pub fn delete_branch(base_dir: &Path, repo: &str, name: &str) -> Result<(), GitError>;
```

### Error Type (error.rs)

```rust
#[derive(thiserror::Error, Debug)]
pub enum GitError {
    #[error("repository not found: {name}")]
    RepoNotFound { name: String },

    #[error("path traversal attempt detected")]
    PathTraversal,

    #[error("ref not found: {r#ref}")]
    RefNotFound { r#ref: String },

    #[error("commit not found: {oid}")]
    CommitNotFound { oid: String },

    #[error("branch already exists: {name}")]
    BranchExists { name: String },

    #[error("libgit2 error: {0}")]
    Git2(#[from] git2::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Testing Strategy

**Unit tests** live in the same file as the function using `#[cfg(test)]`. Use `tempfile::TempDir` for all filesystem operations — never hardcode paths.

**What to test:**
- `init_repo` creates a bare repository at the expected path
- `open_repo` returns `RepoNotFound` for a missing repo
- Path traversal: `open_repo(base, "../../../etc")` returns `PathTraversal`
- `log` returns commits in reverse chronological order
- `diff_commits` correctly counts insertions/deletions
- `diff_branch_from_base` against itself returns empty diff
- `create_branch` from a valid OID succeeds; from an invalid OID returns `RefNotFound`
- `delete_branch` removes the branch; calling again returns an error

**Test helper to scaffold a repo with commits:**

```rust
#[cfg(test)]
mod helpers {
    use super::*;
    use tempfile::TempDir;

    pub fn make_repo_with_commits(n: usize) -> (TempDir, String) {
        // init a non-bare repo in tempdir
        // create n commits with distinct file changes
        // return (tempdir, repo_name)
    }
}
```

**Run tests:**
```bash
cargo test -p git-core
cargo test -p git-core -- --nocapture   # to see println! output
```

**Expected binary output:** This crate produces no binary. It is a `lib` crate only. `[lib]` in Cargo.toml, no `[[bin]]` section.

---

## CRATE 2: `sanity`

### Purpose

A pure computation crate. It receives a `DiffSummary` and `Vec<CommitInfo>` from `git-core` (via the server layer) and a `SanityConfig` loaded from the repo's `sanity.toml`. It returns a `SanityReport` containing an overall score and per-metric breakdown.

This crate has **zero dependency on git2 or any I/O**. It is a pure function: same inputs always produce same outputs. This makes it trivially testable.

### Dependencies (Cargo.toml)

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "1"
toml = "0.8"
regex = "1"

# git-core for shared types only
git-core = { path = "../git-core" }

[dev-dependencies]
# none needed beyond std
```

### Architecture Decisions

**Decision 1 — Pure functions, no I/O.**
The sanity crate never reads files, never opens repos, never touches the filesystem. The server crate reads `sanity.toml`, calls `SanityConfig::from_str()`, then passes everything into `analyze()`. This keeps the crate testable without any filesystem setup.

**Decision 2 — Metric trait for extensibility.**
Each language analyzer and each metric category implements a `Metric` trait. This makes adding new languages or new metric types additive — no existing code changes.

**Decision 3 — Score is i32, can go negative.**
Scores start at 100 and penalties are subtracted. The final score is clamped to 0..=100 for display but the raw value is kept for debugging. This means a catastrophically bad PR can score negative internally.

**Decision 4 — Language detection is by file extension only.**
No content sniffing. `.py` → Python, `.rs` → Rust, `.js`/`.ts` → JavaScript. Unknown extensions are skipped by analyzers but still counted in general metrics (file size, commit size).

**Decision 5 — Config is additive over defaults.**
`SanityConfig::default()` returns sensible weights. A repo's `sanity.toml` only needs to override what it wants to change. Missing keys fall back to defaults via `#[serde(default)]`.

### Config Schema (config.rs)

```rust
#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct SanityConfig {
    pub enabled: bool,                    // default: true
    pub block_merge_below: i32,           // default: 50
    pub thresholds: Thresholds,
    pub weights: Weights,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Thresholds {
    pub max_file_lines: usize,            // default: 500
    pub max_lines_per_commit: usize,      // default: 150
    pub min_test_ratio: f32,              // default: 0.0 (off)
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Weights {
    pub large_file: i32,                  // default: -10
    pub missing_docstring: i32,           // default: -5
    pub large_commit: i32,                // default: -3
    pub missing_tests: i32,               // default: -15
    pub readme_updated: i32,              // default: +5
    pub unwrap_usage: i32,                // default: -4  (Rust only)
    pub console_log: i32,                 // default: -3  (JS/TS only)
}
```

### Metric Trait (metrics.rs)

```rust
pub struct MetricInput<'a> {
    pub diff: &'a git_core::DiffSummary,
    pub commits: &'a [git_core::CommitInfo],
    pub config: &'a SanityConfig,
}

pub struct MetricResult {
    pub name: &'static str,
    pub delta: i32,           // positive = bonus, negative = penalty
    pub detail: String,       // human-readable explanation
}

pub trait Metric {
    fn evaluate(&self, input: &MetricInput) -> MetricResult;
}
```

### Built-in Metrics

Each is a zero-sized struct implementing `Metric`:

- `LargeFileMetric` — penalizes files over `thresholds.max_file_lines` (requires patch line count)
- `LargeCommitMetric` — penalizes commits over `thresholds.max_lines_per_commit`
- `ReadmeMetric` — bonus if any `README*` file is modified in the diff
- `TestCoverageMetric` — penalizes if lines of test files added < 10% of lines of source added
- `RustUnwrapMetric` — penalizes `.rs` files with `unwrap()` calls in the patch text
- `JsConsoleLogMetric` — penalizes `.js`/`.ts` files with `console.log` in the patch
- `PythonDocstringMetric` — penalizes new Python functions/classes without docstrings (regex-based)

### Public API (lib.rs)

```rust
pub struct SanityReport {
    pub score: i32,                      // clamped 0..=100
    pub raw_score: i32,                  // unclamped, for debug
    pub passed: bool,                    // score >= config.block_merge_below
    pub metrics: Vec<MetricResult>,      // per-metric breakdown
}

/// Main entry point. Pure function.
pub fn analyze(
    diff: &git_core::DiffSummary,
    commits: &[git_core::CommitInfo],
    config: &SanityConfig,
) -> SanityReport;

/// Load config from TOML string. Falls back to defaults on parse error.
pub fn load_config(toml_str: &str) -> Result<SanityConfig, SanityError>;
```

### Error Type (error.rs)

```rust
#[derive(thiserror::Error, Debug)]
pub enum SanityError {
    #[error("config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),
}
```

### Testing Strategy

**All tests are pure — no filesystem, no git, no async.**
Construct `DiffSummary` and `CommitInfo` values directly in test code using struct literals or builder helpers.

**What to test:**

- `analyze()` with empty diff returns score of 100
- `LargeFileMetric` fires when a file exceeds threshold, silent when under
- `LargeCommitMetric` fires correctly per commit
- `ReadmeMetric` grants bonus when diff includes a README file modification
- `TestCoverageMetric` penalizes when no test files are in the diff
- `RustUnwrapMetric` detects `unwrap()` in `.rs` patch text
- `JsConsoleLogMetric` detects `console.log` in `.js` patch text
- Score is clamped at 0 even when raw score is deeply negative
- `passed` is false when score < `block_merge_below`
- `load_config("")` returns defaults without panicking
- `load_config` with partial TOML overrides only specified keys

**Example test structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn empty_diff() -> git_core::DiffSummary {
        git_core::DiffSummary {
            files_changed: vec![],
            insertions: 0,
            deletions: 0,
        }
    }

    #[test]
    fn empty_diff_scores_100() {
        let report = analyze(&empty_diff(), &[], &SanityConfig::default());
        assert_eq!(report.score, 100);
        assert!(report.passed);
    }

    #[test]
    fn score_clamped_at_zero() {
        // construct a maximally bad diff
        // assert report.score == 0
        // assert report.raw_score < 0
    }
}
```

**Run tests:**
```bash
cargo test -p sanity
cargo test -p sanity -- --nocapture
```

**Expected binary output:** This crate produces no binary. `lib` crate only.

---

## WORKSPACE Cargo.toml

```toml
[workspace]
members = [
    "crates/git-core",
    "crates/sanity",
    # "crates/git-server",   ← add later
    # "crates/workers",      ← add later
]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }
```

---

## IMPLEMENTATION ORDER

Follow this order strictly. Do not move to the next step until the current one has passing tests.

1. Scaffold the workspace `Cargo.toml` and both crate skeletons (`cargo new --lib`)
2. Define all types in `git-core/src/types.rs` — no logic yet
3. Implement `GitError` in `git-core/src/error.rs`
4. Implement `init_repo` and `open_repo` with path traversal validation — tests first
5. Implement `log` and `get_commit` — tests first
6. Implement `diff_commits` and `diff_branch_from_base` — tests first
7. Implement `list_branches`, `create_branch`, `delete_branch` — tests first
8. Define `SanityConfig` with full `Default` impl in `sanity/src/config.rs`
9. Define `Metric` trait and `MetricInput`/`MetricResult` in `sanity/src/metrics.rs`
10. Implement each `Metric` one at a time with a test per metric
11. Implement `analyze()` as a loop over all metrics starting from score 100
12. Wire `load_config` and verify partial TOML override tests pass
13. Run `cargo test --workspace` — all tests must pass before handoff to server crate

---

## CONSTRAINTS & RULES

- No `unwrap()` or `expect()` in non-test code. Use `?` and return `Result`.
- No `unsafe` blocks.
- No `println!` in library code. Use `tracing` if logging is needed (add as optional dependency).
- Every public function must have a doc comment (`///`).
- `cargo clippy --workspace -- -D warnings` must pass clean.
- `cargo fmt --workspace` must produce no diff.
- Minimum Rust edition: 2021.