# ibzie.dev To-Do List

## git-core

- [ ] Add more comprehensive unit tests
- [ ] Add integration tests with real git operations
- [ ] Consider async variants using tokio-uring or git2-rs async bindings
- [ ] Add repository path configuration validation at startup
- [ ] Performance optimization for large repositories
- [ ] Add support for git hooks (server-side)

---

## sanity

- [ ] Add more language analyzers (Go, Python, Java)
- [ ] Add complexity metrics (cyclomatic complexity, cognitive complexity)
- [ ] Add security scanning metrics (hardcoded secrets, SQL injection patterns)
- [ ] Add language-specific linter integration
- [ ] Add caching for repeated analysis
- [ ] Add metric for detecting duplicate code

---

## git-server

### Security
- [ ] Fix timing attack vulnerability (use constant-time comparison)
- [ ] Add HTTPS/TLS support
- [ ] Add rate limiting middleware
- [ ] Add per-user API tokens with rotation
- [ ] Add token revocation capability
- [ ] Add request logging/audit trail

### Features
- [ ] Add repository visibility (public/private)
- [ ] Add user authentication (not just API token)
- [ ] Add webhooks for repo events
- [ ] Add SSH key management
- [ ] Add pull request workflow
- [ ] Add CI/CD integration hooks

### Testing
- [ ] Add integration tests for all endpoints
- [ ] Add auth middleware tests
- [ ] Add load tests

### Monitoring
- [ ] Add health check endpoint (`/health`)
- [ ] Add metrics endpoint for monitoring
- [ ] Add structured logging

---

## Frontend (Bun + TypeScript)

- [ ] Set up Bun project
- [ ] Implement API client
- [ ] Create repo list view
- [ ] Create repo detail view
- [ ] Create commit history view
- [ ] Create diff viewer
- [ ] Create branch management UI
- [ ] Create sanity score display
- [ ] Add user authentication flow
- [ ] Add dark/light theme

---

## Infrastructure

- [ ] Add Dockerfile for server
- [ ] Add docker-compose.yml
- [ ] Add CI/CD pipeline
- [ ] Add deployment documentation
- [ ] Set up monitoring/alerting
