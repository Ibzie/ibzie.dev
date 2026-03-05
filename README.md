# ibzie.dev — Rust Git Platform

A self-hosted Git hosting service written in Rust with a built-in code sanity scoring system for pull requests.

## Quick Start

### Prerequisites

- Rust 1.70+
- libgit2 (system package: `libgit2-dev` on Debian/Ubuntu, `libgit2` on Arch/Homebrew)

### Build

```bash
cargo build --workspace
```

### Run the server

```bash
IBZIE_API_TOKEN="your-secret-token" cargo run -p git-server
# Server starts on http://127.0.0.1:3000
```

### Environment variables

| Variable         | Required | Default      | Description                         |
|------------------|----------|--------------|-------------------------------------|
| `IBZIE_API_TOKEN`| **Yes**  | —            | Bearer token for API authentication |
| `IBZIE_HOST`     | No       | `127.0.0.1`  | Bind address                        |
| `IBZIE_PORT`     | No       | `3000`       | Server port                         |
| `IBZIE_REPO_DIR` | No       | `./repos`    | Directory where git repos are stored|
| `IBZIE_DB_PATH`  | No       | `./ibzie.db` | SQLite database file path           |

All authenticated requests require:
```
Authorization: Bearer <IBZIE_API_TOKEN>
```

---

## API Endpoints

Base URL: `http://localhost:3000`

### Root

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/` | No | Health ping — returns `"ibzie.dev git server"` |

### Repositories

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/repos` | Yes | List all repositories |
| POST | `/api/repos` | Yes | Create a new repository |
| GET | `/api/repos/:name` | Yes | Get repository metadata |
| DELETE | `/api/repos/:name` | Yes | Delete repository and all its files |

**POST /api/repos** body:
```json
{ "name": "my-repo", "description": "optional" }
```

**Response shape** (all repo endpoints):
```json
{
  "name": "my-repo",
  "description": "optional",
  "default_branch": "main",
  "is_empty": false,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Commits

| Method | Path | Auth | Query params | Description |
|--------|------|------|--------------|-------------|
| GET | `/api/repos/:repo/commits` | Yes | `branch` (default: `main`), `limit` (default: `50`) | List commits |
| GET | `/api/repos/:repo/commits/:hash` | Yes | — | Get a single commit by hash |

**Response shape**:
```json
{
  "hash": "abc123",
  "message": "feat: add thing",
  "author": { "name": "ibzie", "email": "...", "timestamp": "2024-01-01T00:00:00Z" },
  "committer": { "name": "ibzie", "email": "...", "timestamp": "2024-01-01T00:00:00Z" },
  "parent_hashes": ["def456"],
  "tree_hash": "789abc"
}
```

### Branches

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/repos/:repo/branches` | Yes | List all branches |
| POST | `/api/repos/:repo/branches` | Yes | Create a branch |
| DELETE | `/api/repos/:repo/branches/:name` | Yes | Delete a branch |

**POST body**: `{ "name": "feature-branch", "from_oid": "abc123def456" }`

### Diff

| Method | Path | Auth | Query params | Description |
|--------|------|------|--------------|-------------|
| GET | `/api/repos/:repo/diff` | Yes | `base` (required), `head` (required), `include_patch` (default: `false`) | Diff between two refs |

### Files / Tree

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/repos/:repo/files/:ref` | Yes | Get file tree at a ref or commit hash |
| GET | `/api/repos/:repo/files/:ref/*path` | Yes | Get nested subtree at path |
| GET | `/api/repos/:repo/files/blob/:hash` | Yes | Get blob content by hash |

### Sanity Analysis (PR Scoring)

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/repos/:repo/analyze` | Yes | Score a PR between two refs |

**POST body**:
```json
{ "base": "main", "head": "feature-branch", "include_patch": true, "config": null }
```

**Response**:
```json
{
  "score": 85,
  "raw_score": 85,
  "passed": true,
  "metrics": [
    { "name": "LargeFile", "delta": -10, "detail": "src/big.rs exceeds 500 lines" },
    { "name": "ReadmeUpdated", "delta": 5, "detail": "README was modified" }
  ]
}
```

---

## Overview

This workspace contains the core libraries for ibzie.dev:

- **git-core** — A safe, async-friendly wrapper around libgit2 for all Git operations
- **foos** — A CLI tool for Git operations (alternative to using git directly)
- **sanity** — A pure computation engine that scores pull requests based on configurable metrics

## Crates

### git-core

A synchronous-at-heart wrapper around the `git2` crate (libgit2 Rust bindings). Exposes a clean, owned-type API for repository operations without leaking libgit2 types to consumers.

**Key features:**
- Repository initialization and opening
- Commit history traversal
- Diff generation between commits and branches
- Branch CRUD operations
- Path traversal validation for security

```rust
use git_core::{init_repo, open_repo, log, diff_commits};

// Initialize a new repository
let repo = init_repo(&base_dir, "my-project")?;

// Get commit history
let commits = log(&base_dir, "my-project", "main", 10)?;

// Generate diff between commits
let diff = diff_commits(&base_dir, "my-project", base_oid, head_oid, true)?;
```

### foos

A CLI tool that wraps `git-core` for command-line Git operations. Provides an alternative to direct `git` commands with structured output.

```bash
# Initialize a repository
foos init my-project

# View commit history
foos log my-project main --limit 10

# View diff between branches
foos diff my-project main..feature-branch

# List branches
foos branch list my-project
```

### sanity

A pure analysis engine that scores pull requests based on configurable metrics. Takes diff/commit data and produces a weighted sanity score with per-metric breakdown.

**Key features:**
- Zero I/O — pure function evaluation for easy testing
- Configurable weights and thresholds via TOML
- Extensible metric system via trait
- Built-in metrics: large files, large commits, test coverage, docstrings, unwrap usage, console.log detection

```rust
use sanity::{analyze, load_config, SanityConfig};

let config = load_config(r#"
    [weights]
    large_commit = -5
    large_file = -10
"#)?;

let report = analyze(&diff_summary, &commits, &config);

println!("Sanity Score: {}/100", report.score);
if report.passed {
    println!("PR can be merged!");
}
```

## Architecture

### Decision: Sync API, Not Async

`git2::Repository` is not `Send + Sync`. Rather than wrapping in `Arc<Mutex<>>` (which leads to subtle deadlocks), we expose a clean sync API. The server crate calls `spawn_blocking` for each operation. This is the officially recommended pattern for blocking I/O in Tokio.

### Decision: Repository Per-Operation

Repositories are opened, used, and dropped per-operation rather than held open long-term. This avoids lock contention and works safely with libgit2's threading model.

### Decision: Path Traversal Prevention

All paths are validated before passing to libgit2. Repo paths come from user input (repo names in URLs). The resolved path must stay within the configured base directory.

### Decision: Pure Sanity Computation

The `sanity` crate never reads files or touches the filesystem. The server reads `sanity.toml`, calls `SanityConfig::from_str()`, then passes everything to `analyze()`. This keeps the crate trivially testable.

## Configuration

### Sanity Config (sanity.toml)

```toml
[enabled]
enabled = true

# Block merges below this score
block_merge_below = 50

[thresholds]
# Maximum lines in a single file
max_file_lines = 500
# Maximum changes (insertions + deletions) per commit
max_changes_per_commit = 500
# Minimum ratio of test code to source code (0.0 = disabled)
min_test_ratio = 0.0

[weights]
# Penalties (negative values)
large_file = -10
large_commit = -5
missing_docstring = -5
missing_tests = -15
unwrap_usage = -4          # Rust only
console_log = -3            # JS/TS only

# Bonuses (positive values)
readme_updated = +5
```

## Scoring

The sanity score starts at 100 and each metric applies its weight as a delta:

- **Positive deltas** increase the score (bonuses)
- **Negative deltas** decrease the score (penalties)

The final score is clamped to 0–100 for display, but the raw score is preserved for debugging.

### Built-in Metrics

| Metric | Description | Languages |
|--------|-------------|-----------|
| LargeFileMetric | Penalizes files exceeding line threshold | All |
| LargeCommitMetric | Penalizes commits with >500 changes | All |
| ReadmeMetric | Bonus for README modifications | All |
| TestCoverageMetric | Penalizes low test coverage | All |
| RustUnwrapMetric | Penalizes `.unwrap()` calls | Rust |
| JsConsoleLogMetric | Penalizes `console.log` statements | JavaScript/TypeScript |
| PythonDocstringMetric | Penalizes missing docstrings | Python |

## Development

### Prerequisites

- Rust 1.70+
- cargo

### Building

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p git-core
cargo build -p sanity
cargo build -p foos
```

### Testing

```bash
# Test all crates
cargo test --workspace

# Test specific crate
cargo test -p git-core
cargo test -p sanity
```

### Code Quality

```bash
# Format code
cargo fmt --workspace

# Run clippy
cargo clippy --workspace -- -D warnings
```

## License

MIT
