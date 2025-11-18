# Testing CI Migration

This directory contains example CI configurations for testing the `magnolia migrate` command.

## Available Source CI Configs

The following source CI configurations are available for migration testing:

- **`bitrise.yml`**: Bitrise configuration with build, test, and deploy workflows
- **`codemagic.yaml`**: Codemagic configuration with multiple workflows
- **`.circleci/config.yml`**: CircleCI configuration with job dependencies

## How to Test Migration

### Prerequisites

Install either the Claude CLI or Codex CLI:
- **Claude CLI**: https://github.com/anthropics/claude-code
- **Codex CLI**: https://zed.dev/docs/assistant/commands

### Testing from the fixtures directory

From the `fixtures/` directory, you can test migration:

```bash
cd fixtures

# Preview migration of Bitrise config (dry-run)
magnolia migrate bitrise --dry-run

# Preview migration of Codemagic config
magnolia migrate codemagic --dry-run

# Preview migration of CircleCI config
magnolia migrate circleci --dry-run

# Perform actual migration to GitHub Actions
magnolia migrate bitrise --to github

# Perform actual migration to GitLab CI
magnolia migrate codemagic --to gitlab

# Perform actual migration to Forgejo Actions
magnolia migrate circleci --to forgejo
```

### Auto-detection

If you don't specify which source CI to migrate, Magnolia will:
1. Detect all available source configs (Bitrise, Codemagic, CircleCI)
2. Prompt you to select which one to migrate
3. Auto-detect the target CI from git remote origin

```bash
# This will show an interactive selection menu
magnolia migrate --dry-run
```

### Skip Verification

By default, Magnolia will run the migrated pipeline locally to verify it works. To skip this:

```bash
magnolia migrate bitrise --no-verify --to github
```

## What Gets Generated

Depending on the target CI system:

- **GitHub Actions**: `.github/workflows/migrated-workflow.yml`
- **GitLab CI**: `.gitlab-ci.yml`
- **Forgejo Actions**: `.forgejo/workflows/migrated-workflow.yml`

These are gitignored to avoid committing test migrations.

## Example: Testing Bitrise Migration

```bash
cd fixtures

# Preview the migration
magnolia migrate bitrise --dry-run --to github

# Perform the migration
magnolia migrate bitrise --to github

# Verify the generated config
cat .github/workflows/migrated-workflow.yml

# Test the migrated pipeline locally
magnolia .github/workflows/migrated-workflow.yml
```

## Troubleshooting

### Error: No agent CLI found

If you see this error:
```
Error: No agent CLI found. The migration feature requires either 'claude' or 'codex' CLI to be installed.
```

Install one of the agent CLIs mentioned in the Prerequisites section.

### Multiple sources detected

If you run `magnolia migrate` without specifying a source, you'll see:
```
Found multiple CI configurations:
  1. Bitrise (bitrise.yml)
  2. Codemagic (codemagic.yaml)
  3. CircleCI (.circleci/config.yml)

Which would you like to migrate? [1-3]:
```

You can either select interactively or specify the source:
```bash
magnolia migrate bitrise
```

## Source CI Config Details

### Bitrise (`bitrise.yml`)
- **Workflows**: primary, deploy
- **Features**: Git clone, npm install/test/build, caching, deploy
- **Environment**: NODE_ENV=production

### Codemagic (`codemagic.yaml`)
- **Workflows**: build-and-test, deploy-production
- **Features**: Node.js setup, caching, artifacts, email/Slack notifications
- **Environment**: Multiple environment variables and groups

### CircleCI (`.circleci/config.yml`)
- **Jobs**: install-dependencies, lint, test, build, deploy
- **Features**: Docker executor, caching, workspaces, job dependencies
- **Workflow**: Sequential execution with dependencies

These examples cover common CI/CD patterns and should translate well to any target CI system.
