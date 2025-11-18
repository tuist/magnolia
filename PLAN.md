# Magnolia Feature Roadmap

This document outlines the planned features and enhancements for Magnolia, prioritized based on usage frequency in real-world GitHub Actions workflows.

## Completed Features

### Matrix Strategies ✅
- **Status**: Implemented in v0.3.0+
- **Description**: Support for `strategy.matrix` to run jobs across multiple configurations
- **Features**:
  - Multi-dimensional matrix expansion (e.g., `os x rust-version`)
  - Matrix value interpolation in `runs-on`, step names, and commands
  - Support for `fail-fast` and `max-parallel` strategy options
  - Comprehensive unit tests for matrix expansion and interpolation
- **Location**: `src/github.rs:9-86`

## High Priority Features

### 1. Environment Variables
**Priority**: P0 - Critical
**Complexity**: Medium
**Impact**: Required by nearly all workflows

**Description**: Support for environment variables at workflow, job, and step levels.

**Implementation Tasks**:
- [ ] Add `env` field to `GitHubWorkflow`, `Job`, and `Step` structs
- [ ] Implement environment variable interpolation (`${{ env.* }}`)
- [ ] Support for `.env` file loading
- [ ] Merge env vars from workflow → job → step (most specific wins)
- [ ] Pass environment variables to container and host execution

**Example**:
```yaml
env:
  GLOBAL_VAR: "value"

jobs:
  build:
    env:
      JOB_VAR: "value"
    steps:
      - name: Test
        env:
          STEP_VAR: "value"
        run: echo $STEP_VAR
```

**Architecture Considerations**:
- Create `EnvContext` struct to manage variable resolution
- Implement variable precedence rules (step > job > workflow)
- Support both `${{ env.VAR }}` and `$VAR` syntax

---

### 2. Caching Support
**Priority**: P0 - Critical
**Complexity**: High
**Impact**: Dramatically improves workflow performance

**Description**: Support for `actions/cache` and similar caching actions.

**Implementation Tasks**:
- [ ] Implement cache key generation (support for hashFiles function)
- [ ] Create local cache storage (e.g., `~/.magnolia/cache/`)
- [ ] Add cache hit/miss detection
- [ ] Support for cache restore and save operations
- [ ] Implement cache size limits and LRU eviction
- [ ] Support for `actions/cache@v4` and `Swatinem/rust-cache@v2`

**Example**:
```yaml
- uses: actions/cache@v4
  with:
    path: target
    key: ${{ runner.os }}-cargo-${{ hashFiles('Cargo.lock') }}
```

**Architecture Considerations**:
- Design pluggable cache backend (filesystem, Redis, etc.)
- Implement cache key hashing with support for `hashFiles()`
- Consider cache compression to save disk space

---

### 3. Job Dependencies (`needs`)
**Priority**: P0 - Critical
**Complexity**: Medium
**Impact**: Required for complex pipelines

**Description**: Support for job dependencies and execution ordering.

**Implementation Tasks**:
- [ ] Add `needs` field to `Job` struct
- [ ] Implement dependency graph resolution
- [ ] Add cycle detection for circular dependencies
- [ ] Support for parallel job execution
- [ ] Handle job failure propagation with `fail-fast`
- [ ] Update job selection UI to show dependency tree

**Example**:
```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps: [...]

  test:
    needs: build
    runs-on: ubuntu-latest
    steps: [...]
```

**Architecture Considerations**:
- Use topological sort for dependency resolution
- Implement async job execution with tokio
- Create job status tracking for dependent jobs

---

### 4. Conditional Execution (`if`)
**Priority**: P1 - High
**Complexity**: High
**Impact**: Common pattern for workflow control

**Description**: Support for conditional job and step execution.

**Implementation Tasks**:
- [ ] Add `if` field to `Job` and `Step` structs
- [ ] Implement expression evaluator for GitHub Actions expressions
- [ ] Support for contexts: `github`, `env`, `matrix`, `needs`, etc.
- [ ] Support for common functions: `contains()`, `startsWith()`, `endsWith()`, etc.
- [ ] Handle boolean operators: `&&`, `||`, `!`
- [ ] Skip execution when condition evaluates to false

**Example**:
```yaml
jobs:
  test:
    if: github.event_name == 'pull_request'
    steps:
      - name: Deploy
        if: success() && github.ref == 'refs/heads/main'
        run: ./deploy.sh
```

**Architecture Considerations**:
- Create expression parser (consider using pest or nom)
- Implement context providers for different scopes
- Cache expression evaluation results

---

### 5. Context Expression Evaluation
**Priority**: P1 - High
**Complexity**: High
**Impact**: Enables dynamic workflows

**Description**: Full support for GitHub Actions context expressions.

**Implementation Tasks**:
- [ ] Implement `${{ github.* }}` context (event, ref, sha, actor, etc.)
- [ ] Implement `${{ runner.* }}` context (os, arch, temp, workspace)
- [ ] Implement `${{ job.* }}` context (status, container)
- [ ] Implement `${{ steps.* }}` context (outputs, outcome, conclusion)
- [ ] Support for `${{ needs.* }}` context (outputs from dependent jobs)
- [ ] Implement built-in functions (fromJSON, toJSON, format, join, etc.)

**Example**:
```yaml
- name: Print context
  run: echo "Branch is ${{ github.ref }}"
```

**Architecture Considerations**:
- Create unified context system with all available data
- Implement lazy evaluation for performance
- Add comprehensive function library

---

### 6. Artifact Upload/Download
**Priority**: P1 - High
**Complexity**: Medium
**Impact**: Common pattern for build artifacts

**Description**: Full support for artifact actions.

**Implementation Tasks**:
- [ ] Implement `actions/upload-artifact@v4` support
- [ ] Implement `actions/download-artifact@v4` support
- [ ] Create local artifact storage (e.g., `~/.magnolia/artifacts/`)
- [ ] Support for artifact retention and cleanup
- [ ] Handle artifact path patterns and compression
- [ ] Support artifact sharing between jobs

**Example**:
```yaml
- uses: actions/upload-artifact@v4
  with:
    name: build-output
    path: target/release/
```

**Architecture Considerations**:
- Design artifact metadata storage (JSON index)
- Implement efficient compression (tar.gz)
- Consider artifact size limits

---

## Medium Priority Features

### 7. Improved Action Execution
**Priority**: P2 - Medium
**Complexity**: High
**Impact**: Better third-party action support

**Implementation Tasks**:
- [ ] Parse action.yml from action repositories
- [ ] Support for Docker container actions
- [ ] Support for JavaScript actions (via Node.js)
- [ ] Support for composite actions
- [ ] Implement action input/output handling
- [ ] Clone actions from GitHub when needed

---

### 8. Secrets Management
**Priority**: P2 - Medium
**Complexity**: Medium
**Impact**: Required for production workflows

**Implementation Tasks**:
- [ ] Design secure secrets storage (keychain integration)
- [ ] Implement `${{ secrets.* }}` interpolation
- [ ] Add CLI commands to manage secrets
- [ ] Support for `.env` file integration
- [ ] Mask secret values in output logs

---

### 9. Service Containers
**Priority**: P2 - Medium
**Complexity**: High
**Impact**: Required for integration tests

**Implementation Tasks**:
- [ ] Add `services` field to `Job` struct
- [ ] Start service containers before job execution
- [ ] Configure service networking (ports, healthchecks)
- [ ] Clean up services after job completion
- [ ] Support for service credentials and env vars

---

### 10. Reusable Workflows
**Priority**: P2 - Medium
**Complexity**: High
**Impact**: Code reuse across workflows

**Implementation Tasks**:
- [ ] Support for `workflow_call` trigger
- [ ] Implement workflow input parameters
- [ ] Support for workflow outputs
- [ ] Handle nested workflow execution
- [ ] Implement workflow limits (max 10 nested)

---

## Lower Priority Features

### 11. Workflow Dispatch
**Priority**: P3 - Low
**Complexity**: Medium

**Implementation Tasks**:
- [ ] Support for `workflow_dispatch` trigger
- [ ] Implement input parameters via CLI
- [ ] Support for input types (string, choice, boolean)

---

### 12. Parallel Execution
**Priority**: P3 - Low
**Complexity**: High

**Implementation Tasks**:
- [ ] Implement concurrent job execution
- [ ] Support for `max-parallel` limits
- [ ] Resource-aware scheduling

---

### 13. Path/Branch Filters
**Priority**: P3 - Low
**Complexity**: Low

**Implementation Tasks**:
- [ ] Parse `on.push.paths` and `on.pull_request.paths`
- [ ] Implement glob pattern matching
- [ ] Support for branch patterns

---

## Testing Strategy

For each feature, implement:

1. **Unit Tests**: Test core logic in isolation
2. **Integration Tests**: Test with real workflow files
3. **Fixture Workflows**: Add example workflows to `fixtures/`
4. **Documentation**: Update README and CLAUDE.md

## Design Principles

1. **Modularity**: Each feature should be self-contained
2. **Testing**: Comprehensive tests before merging
3. **Performance**: Lazy evaluation and caching where possible
4. **Compatibility**: Match GitHub Actions behavior as closely as possible
5. **Error Handling**: Clear, actionable error messages
6. **Architecture**: Follow existing patterns in the codebase

## Contributing

When implementing features from this roadmap:

1. Create a feature branch: `feat/<feature-name>`
2. Update this document with implementation notes
3. Add comprehensive tests
4. Update CLAUDE.md with architecture notes
5. Create a PR with detailed description

## Version Planning

- **v0.4.0**: Environment Variables + Caching
- **v0.5.0**: Job Dependencies + Conditional Execution
- **v0.6.0**: Context Expressions + Artifacts
- **v0.7.0**: Improved Actions + Secrets
- **v0.8.0**: Service Containers + Reusable Workflows
- **v1.0.0**: Production-ready with all high-priority features
