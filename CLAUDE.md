# Magnolia - Claude Context

## Project Overview

Magnolia is a Rust CLI tool that enables developers to run GitLab CI, GitHub Actions, and Forgejo/Gitea Actions pipelines locally. This helps developers test their CI/CD configurations before pushing to remote repositories.

## Architecture

### Core Components

1. **Main CLI (`src/main.rs`)**: Entry point that handles command parsing using `clap` with subcommands and dispatches to appropriate handlers.

2. **CI System Modules**:
   - `src/gitlab.rs`: Parses and executes GitLab CI pipelines (.gitlab-ci.yml)
   - `src/github.rs`: Parses and executes GitHub Actions workflows (.github/workflows/*.yml)
     - Supports matrix strategy expansion (v0.3.0+)
     - Matrix interpolation in runs-on, step names, and commands
     - Support for fail-fast and max-parallel options
   - `src/forgejo.rs`: Parses and executes Forgejo/Gitea Actions workflows (.forgejo/workflows/*.yml or .gitea/workflows/*.yml)
   - `src/container.rs`: Container runtime detection and execution (Docker/Podman)
   - `src/actions.rs`: GitHub Actions execution support

3. **Migration System** (v0.4.0+):
   - `src/migrate.rs`: CI pipeline migration orchestration
     - Auto-detects source CI configurations (Bitrise, Codemagic, CircleCI)
     - Auto-detects target CI from git remote origin
     - Supports interactive selection when multiple sources found
     - Implements verification via local pipeline execution
   - `src/agent.rs`: Agentic client protocol implementation
     - Auto-detects `claude` or `codex` CLI
     - MCP-based communication protocol
     - Hybrid agent delegation model (main, documentation, parser, validator agents)
     - Task delegation with context passing

### Commands

- `magnolia [pipeline]`: Run a pipeline locally (interactive mode if no path specified)
- `magnolia migrate [source]`: Migrate CI pipeline from external providers
  - `--to <target>`: Override target CI system (github, gitlab, forgejo)
  - `--no-verify`: Skip verification by running migrated pipeline locally
  - `--dry-run`: Preview migration without writing files
  - `--path <path>`: Path to repository (defaults to current directory)

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

## Feature Development

For the complete feature roadmap, implementation priorities, and detailed technical specifications, see [PLAN.md](PLAN.md).

## Testing the CLI

The repository includes example pipelines in the `fixtures/` directory for all supported CI systems:

**Target CI Systems (for local execution):**
- `fixtures/.gitlab-ci.yml`: GitLab CI example with build, test, and lint stages
- `fixtures/.github/workflows/test.yml`: GitHub Actions example
- `fixtures/.forgejo/workflows/test.yml`: Forgejo Actions example

**Source CI Systems (for migration testing):**
- `fixtures/bitrise.yml`: Bitrise configuration with build, test, and deploy workflows
- `fixtures/codemagic.yaml`: Codemagic configuration with multiple workflows
- `fixtures/.circleci/config.yml`: CircleCI configuration with job dependencies

Test the CLI with:
```bash
# Run a pipeline locally
magnolia

# Test migration (requires claude or codex CLI)
cd fixtures && magnolia migrate --dry-run
```

## CI/CD Workflows

The project uses GitHub Actions for continuous integration:
- `.github/workflows/magnolia.yml`: Main CI workflow that runs format checks, clippy, tests, and builds across all platforms (Linux, macOS, Windows)
- `.github/workflows/release.yml`: Automated release workflow triggered on pushes to main with releasable commits
