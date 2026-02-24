# ibzie.dev API Documentation

## Base URL

```
http://localhost:3000
```

## Authentication

All API endpoints (except `/`) require Bearer token authentication.

**Header:**
```
Authorization: Bearer <your-token>
```

**Environment Variable:** `IBZIE_API_TOKEN`

---

## Endpoints

### Root

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/` | No | Server health check |

**Response:**
```json
"ibzie.dev git server"
```

---

### Repositories

#### List Repositories

```
GET /api/repos
```

**Response:** `200 OK`
```json
[
  {
    "name": "my-repo",
    "description": "My awesome project",
    "default_branch": "main",
    "is_empty": false,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-20T14:22:00Z"
  }
]
```

#### Get Repository

```
GET /api/repos/:name
```

**Response:** `200 OK`
```json
{
  "name": "my-repo",
  "description": "My awesome project",
  "default_branch": "main",
  "is_empty": false,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-20T14:22:00Z"
}
```

#### Create Repository

```
POST /api/repos
```

**Request Body:**
```json
{
  "name": "new-repo",
  "description": "Description (optional)"
}
```

**Response:** `201 Created`
```json
{
  "name": "new-repo",
  "description": "Description (optional)",
  "default_branch": "main",
  "is_empty": true,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### Delete Repository

```
DELETE /api/repos/:name
```

**Response:** `204 No Content`

---

### Commits

#### List Commits

```
GET /api/repos/:repo/commits?branch=main&limit=50
```

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| branch | string | "main" | Branch name |
| limit | number | 50 | Max commits to return |

**Response:** `200 OK`
```json
[
  {
    "hash": "abc123def456...",
    "message": "feat: add new feature",
    "author": {
      "name": "John Doe",
      "email": "john@example.com",
      "timestamp": "2024-01-15T10:30:00Z"
    },
    "committer": {
      "name": "John Doe",
      "email": "john@example.com",
      "timestamp": "2024-01-15T10:30:00Z"
    },
    "parent_hashes": ["parent123..."],
    "tree_hash": "tree123..."
  }
]
```

#### Get Commit

```
GET /api/repos/:repo/commits/:hash
```

**Response:** `200 OK`
```json
{
  "hash": "abc123def456...",
  "message": "feat: add new feature",
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  "committer": {
    "name": "John Doe",
    "email": "john@example.com",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  "parent_hashes": ["parent123..."],
  "tree_hash": "tree123..."
}
```

---

### Branches

#### List Branches

```
GET /api/repos/:repo/branches
```

**Response:** `200 OK`
```json
[
  {
    "name": "main",
    "target_hash": "abc123...",
    "is_head": true
  },
  {
    "name": "feature/new-feature",
    "target_hash": "def456...",
    "is_head": false
  }
]
```

#### Create Branch

```
POST /api/repos/:repo/branches
```

**Request Body:**
```json
{
  "name": "feature/new-branch",
  "from_oid": "abc123def456..."
}
```

**Response:** `201 Created`
```json
{
  "name": "feature/new-branch",
  "target_hash": "abc123def456...",
  "is_head": false
}
```

#### Delete Branch

```
DELETE /api/repos/:repo/branches/:name
```

**Response:** `204 No Content`

---

### Diff

#### Get Diff

```
GET /api/repos/:repo/diff?base=main&head=feature&include_patch=true
```

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| base | string | Yes | Base ref (branch or commit) |
| head | string | Yes | Head ref (branch or commit) |
| include_patch | boolean | No | Include full patch text (default: false) |

**Response:** `200 OK`
```json
{
  "files_changed": [
    {
      "old_path": "src/main.rs",
      "new_path": "src/main.rs",
      "status": "modified",
      "insertions": 10,
      "deletions": 5,
      "patch": "@@ -1,5 +1,10 @@\n..."
    },
    {
      "old_path": null,
      "new_path": "src/new.rs",
      "status": "added",
      "insertions": 50,
      "deletions": 0,
      "patch": null
    },
    {
      "old_path": "src/old.rs",
      "new_path": null,
      "status": "deleted",
      "insertions": 0,
      "deletions": 30,
      "patch": null
    }
  ],
  "insertions": 60,
  "deletions": 35
}
```

**File Status Values:** `added`, `deleted`, `modified`, `renamed`, `copied`

---

### Files

#### Get File Tree

```
GET /api/repos/:repo/files/:tree_hash
```

**Response:** `200 OK`
```json
{
  "hash": "abc123...",
  "entries": [
    {
      "hash": "def456...",
      "name": "src",
      "mode": "tree"
    },
    {
      "hash": "ghi789...",
      "name": "main.rs",
      "mode": "blob"
    },
    {
      "hash": "jkl012...",
      "name": "Cargo.toml",
      "mode": "blob"
    }
  ]
}
```

**Mode Values:** `blob` (file), `blob_executable` (executable), `link` (symlink), `tree` (directory)

#### Get File Content

```
GET /api/repos/:repo/blob/:hash
```

**Response:** `200 OK`
```json
{
  "hash": "abc123...",
  "data": "base64-encoded-content",
  "is_binary": false
}
```

---

### Sanity Analysis

#### Analyze PR

```
POST /api/repos/:repo/analyze
```

**Request Body:**
```json
{
  "base": "main",
  "head": "feature-branch",
  "include_patch": true,
  "config": null  // Optional TOML config, uses defaults if null
}
```

**Response:** `200 OK`
```json
{
  "score": 85,
  "raw_score": 85,
  "passed": true,
  "metrics": [
    {
      "name": "LargeFile",
      "delta": -10,
      "detail": "File src/big.rs exceeds 500 lines (actual: 750)"
    },
    {
      "name": "ReadmeUpdated",
      "delta": 5,
      "detail": "README file was modified"
    },
    {
      "name": "TestCoverage",
      "delta": 0,
      "detail": "Test coverage ratio: 15%"
    }
  ]
}
```

**Score Ranges:**
- `score`: 0-100 (clamped for display)
- `raw_score`: Can be negative (catastrophically bad PRs)
- `passed`: true if score >= block_merge_below threshold (default: 50)

---

## Error Responses

### 400 Bad Request
```json
"Invalid parameters: base is required"
```

### 401 Unauthorized
```json
""
```
(Empty body, check `Authorization` header)

### 404 Not Found
```json
"repository not found: my-repo"
```

### 500 Internal Server Error
```json
"Internal server error message"
```

---

## Configuration

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `IBZIE_API_TOKEN` | Yes | - | Authentication token |
| `IBZIE_HOST` | No | 127.0.0.1 | Server bind address |
| `IBZIE_PORT` | No | 3000 | Server port |
| `IBZIE_REPO_DIR` | No | ./repos | Git repository storage |
| `IBZIE_DB_PATH` | No | ./ibzie.db | SQLite database path |

---

## Default Sanity Config

If no custom config is provided, these defaults are used:

```toml
enabled = true
block_merge_below = 50

[thresholds]
max_file_lines = 500
max_lines_per_commit = 150
min_test_ratio = 0.0

[weights]
large_file = -10
missing_docstring = -5
large_commit = -3
missing_tests = -15
readme_updated = 5
unwrap_usage = -4      # Rust only
console_log = -3      # JS/TS only
```
