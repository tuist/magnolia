# Test Migration Directory

This directory contains a simple Bitrise pipeline for testing the `magnolia migrate` command.

## Testing the Migration Feature

### Prerequisites

Install either the Claude CLI or Codex CLI:
- **Claude CLI**: https://github.com/anthropics/claude-code
- **Codex CLI**: https://zed.dev/docs/assistant/commands

### Running the Migration

From this directory, you can test the migration feature:

```bash
# Preview the migration without writing files
magnolia migrate --dry-run

# Perform the migration (writes to .github/workflows/ or .gitlab-ci.yml based on git origin)
magnolia migrate

# Override the target CI system
magnolia migrate --to github
magnolia migrate --to gitlab
magnolia migrate --to forgejo

# Skip verification (don't run the migrated pipeline locally)
magnolia migrate --no-verify
```

## What to Expect

1. **Auto-Detection**: Magnolia will detect `bitrise.yml` as the source configuration
2. **Target Detection**: It will check your git remote origin to determine the target CI system
3. **AI Migration**: The agent will analyze the Bitrise config and generate an equivalent config for your target CI
4. **Verification**: By default, it will run the migrated pipeline locally to verify it works
5. **Output**: The migrated config will be written to the appropriate location

## Example Bitrise Pipeline

The included `bitrise.yml` is a simple workflow that:
- Clones the git repository
- Installs npm dependencies
- Runs tests
- Builds the application

This should map to most Git forge CI systems as:
- A single job with multiple steps
- Environment variable: `NODE_ENV=production`
- Sequential execution: install → test → build

## Troubleshooting

If you get an error about missing agent CLI:
```
Error: No agent CLI found. The migration feature requires either 'claude' or 'codex' CLI to be installed.
```

Install one of the CLIs mentioned in the Prerequisites section.

## Manual Migration

If you prefer to migrate manually without the AI agent:
1. Consult the documentation for both Bitrise and your target CI system
2. Map the workflow steps to equivalent steps in the target system
3. Pay attention to environment variables, caching, and artifacts
4. Test the migrated pipeline using `magnolia` to run it locally
