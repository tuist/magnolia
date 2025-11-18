# Magnolia - Claude Context

## Project Overview

Magnolia is a Rust CLI tool that enables developers to run GitLab CI, GitHub Actions, and Forgejo/Gitea Actions pipelines locally. This helps developers test their CI/CD configurations before pushing to remote repositories.

## Architecture

### Core Components

1. **Main CLI (`src/main.rs`)**: Entry point that handles command parsing using `clap` and dispatches to appropriate CI system handlers.

2. **CI System Modules**:
   - `src/gitlab.rs`: Parses and executes GitLab CI pipelines (.gitlab-ci.yml)
   - `src/github.rs`: Parses and executes GitHub Actions workflows (.github/workflows/*.yml)
     - Supports matrix strategy expansion (v0.3.0+)
     - Matrix interpolation in runs-on, step names, and commands
     - Support for fail-fast and max-parallel options
   - `src/forgejo.rs`: Parses and executes Forgejo/Gitea Actions workflows (.forgejo/workflows/*.yml or .gitea/workflows/*.yml)
   - `src/container.rs`: Container runtime detection and execution (Docker/Podman)
   - `src/actions.rs`: GitHub Actions execution support

### Commands

- `detect`: Auto-detects which CI systems are configured in the current repository
- `list`: Lists all available jobs/workflows in the repository
- `run <job>`: Executes a specific job locally (simulation mode for now)

### Dependencies

- **clap**: Command-line argument parsing with derive macros
- **serde/serde_yaml/serde_json**: Configuration file parsing
- **tokio**: Async runtime for future execution features
- **colored**: Terminal output styling
- **anyhow**: Error handling

## Release Automation

The project uses `git-cliff` for changelog generation and semantic versioning. The release workflow:

1. Checks for releasable commits using conventional commit messages
2. Calculates the next version automatically
3. Builds binaries for multiple platforms (Linux, macOS, Windows on x86_64 and ARM64)
4. Creates GitHub releases with binaries and checksums
5. Commits version bumps back to the repository

### Supported Platforms

- Linux: x86_64, aarch64, armv7 (GNU and musl)
- macOS: x86_64, aarch64
- Windows: x86_64, aarch64

## Development Workflow

1. Use `mise` for tool management (Rust, git-cliff)
2. Build: `mise exec -- cargo build`
3. Test: `mise exec -- cargo test`
4. Run locally: `mise exec -- cargo run -- <command>`

## Feature Roadmap

See [PLAN.md](PLAN.md) for the complete feature roadmap and implementation priorities.

### Recently Implemented

- **Matrix Strategies** (v0.3.0+): Full support for `strategy.matrix` with multi-dimensional expansion, interpolation, and fail-fast/max-parallel options

### High Priority (Next)

1. **Environment Variables**: Support for `env` at workflow/job/step levels with interpolation
2. **Caching**: Support for `actions/cache` and related actions
3. **Job Dependencies**: `needs` keyword with dependency graph resolution
4. **Conditional Execution**: `if` conditions on jobs and steps
5. **Context Expressions**: Full `${{ }}` expression evaluation

### In Progress

- Container-based execution (Docker/Podman support implemented)
- GitHub Actions parsing and execution (basic support implemented)

## Testing the CLI

The repository includes example pipelines in the `fixtures/` directory for all three CI systems:
- `fixtures/.gitlab-ci.yml`: GitLab CI example with build, test, and lint stages
- `fixtures/.github/workflows/test.yml`: GitHub Actions example
- `fixtures/.forgejo/workflows/test.yml`: Forgejo Actions example

Test the CLI with: `magnolia detect --path fixtures`

## CI/CD Workflows

The project uses GitHub Actions for continuous integration:
- `.github/workflows/magnolia.yml`: Main CI workflow that runs format checks, clippy, tests, and builds across all platforms (Linux, macOS, Windows)
- `.github/workflows/release.yml`: Automated release workflow triggered on pushes to main with releasable commits
