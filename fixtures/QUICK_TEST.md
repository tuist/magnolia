# Quick Migration Test

## Prerequisites

You need the Claude CLI installed:
```bash
npm install -g @anthropic-ai/claude-cli
```

Or follow: https://github.com/anthropics/claude-code

## Test from Repository Root

```bash
# Bitrise → GitHub Actions (dry-run)
cargo run -- migrate bitrise --dry-run --to github --path fixtures

# Bitrise → GitLab CI (dry-run)
cargo run -- migrate bitrise --dry-run --to gitlab --path fixtures

# Codemagic → GitHub Actions (dry-run)
cargo run -- migrate codemagic --dry-run --to github --path fixtures

# CircleCI → GitHub Actions (actual migration, writes file)
cargo run -- migrate circleci --to github --path fixtures

# Then test the generated workflow
cargo run -- .github/workflows/migrated-workflow.yml
```

## Expected Output

The migration should:
1. Detect the source CI config
2. Auto-detect or use specified target CI
3. Show "Analyzing source configuration..." message
4. Generate a complete YAML configuration
5. In dry-run mode, display the config without writing
6. In normal mode, write to `.github/workflows/`, `.gitlab-ci.yml`, or `.forgejo/workflows/`

## Successful Migration Example

```
Detecting CI configurations...
Source: Bitrise (fixtures/bitrise.yml)
Target: GitHub Actions

Initializing AI agent for migration...
Analyzing source configuration and researching documentation...
This may take a moment...

Dry run - migration preview:

Generated configuration:
================================================================================
name: CI/CD Pipeline

on:
  push:
    branches:
      - '**'
...
```

The generated YAML should be syntactically valid and include all jobs/steps from the source config.

## Troubleshooting

**"No agent CLI found"**: Install the Claude CLI as shown in Prerequisites

**"No source CI configurations found"**: Make sure you're using `--path fixtures` from the repo root

**Output is explanatory text instead of YAML**: This has been fixed in the latest commit
