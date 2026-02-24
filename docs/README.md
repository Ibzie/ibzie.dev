# ibzie.dev — Rust Git Platform

A self-hosted Git hosting service written in Rust with a built-in code sanity scoring system for pull requests.

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
