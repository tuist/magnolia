# 🌸 Magnolia

Run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

## 🌸 The Magnolia Manifesto

**[Verse 1]**<br>
They say that if you were to see<br>
Your CI running locally<br>
In that terminal window, free<br>
It brings you good luck<br>
All of you have come<br>
Even the vendors who dismissed us<br>
Today, they watch

**[Chorus]**<br>
Throw magnolias at me<br>
Run your pipelines locally<br>
Throw magnolias at me<br>
Own your CI destiny

**[Verse 2]**<br>
Over their platforms, open forges burning bright<br>
Tears and FUD melt into the code<br>
GitLab and GitHub, Forgejo's might<br>
Dancing with freedom on top of vendor lock<br>
Today it's all sovereignty mocking fate<br>
And what you couldn't test locally, you test before you commit

## 📦 Installation

```bash
mise x ubi:tuist/magnolia
```

Or download from [releases](https://github.com/tuist/magnolia/releases).

## 🚀 Usage

### Running Pipelines Locally

Test your CI/CD pipelines before pushing to your Git forge:

```bash
# Interactive mode - discover and select pipeline
magnolia

# Run a specific workflow
magnolia .github/workflows/test.yml

# Run a specific job from a workflow
magnolia .github/workflows/test.yml
# Then select the job interactively
```

**Example workflow:**
```bash
$ magnolia .github/workflows/ci.yml
Discovering pipelines...
Select a pipeline: GitHub Actions: ci.yml

Select a job to run:
  > build
    test
    deploy

Running job build from .github/workflows/ci.yml
✓ Step: Checkout code
✓ Step: Install dependencies
✓ Step: Build application
```

### Migrating from External CI Providers

Seamlessly migrate from external CI providers to your Git forge's native CI using AI-powered translation:

```bash
# Auto-detect source and target CI systems
magnolia migrate

# Preview migration without writing files
magnolia migrate --dry-run

# Override target CI system
magnolia migrate --to github

# Migrate specific source when multiple configs found
magnolia migrate bitrise
```

**Example migration:**
```bash
$ magnolia migrate bitrise --dry-run --to github
Detecting CI configurations...
Source: Bitrise (bitrise.yml)
Target: GitHub Actions

Initializing AI agent for migration...
Analyzing source configuration and researching documentation...

Generated configuration:
================================================================================
name: CI/CD Pipeline

on:
  push:
    branches: ['**']

jobs:
  primary:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: npm install
      - name: Run tests
        run: npm test
      - name: Build
        run: npm run build
================================================================================

Would be written to: .github/workflows/migrated-workflow.yml
```

**Common migration scenarios:**
```bash
# CircleCI → GitHub Actions
magnolia migrate circleci --to github

# Buildkite → GitLab CI
magnolia migrate buildkite --to gitlab

# Mobile app (AppCircle) → GitHub Actions
magnolia migrate appcircle --to github
```

**Supported Migration Sources:**
- Bitrise (`bitrise.yml` or `.bitrise/bitrise.yml`)
- Codemagic (`codemagic.yaml` or `.codemagic/codemagic.yaml`)
- CircleCI (`.circleci/config.yml`)
- AppCircle (`appcircle.yaml`, `configuration.yaml`, or `.appcircle/config.yaml`)
- Buildkite (`.buildkite/pipeline.yml` or `.buildkite/pipeline.yaml`)

**Migration Targets (auto-detected from git remote):**
- GitHub Actions (`.github/workflows/*.yml`)
- GitLab CI (`.gitlab-ci.yml`)
- Forgejo Actions (`.forgejo/workflows/*.yml`)

**Requirements:**
- Install either `claude` or `codex` CLI for AI-powered migration
- The migration feature uses the agentic client protocol to delegate complex translation tasks

### ⚡ Execution

- **GitLab CI**: Executes jobs in containers (Podman/Docker) when `image:` is specified, or on host otherwise.
- **GitHub Actions / Forgejo Actions**:
  - Executes `run:` steps in containers based on `runs-on:` runner
  - Executes marketplace actions (`uses:`) - composite, Docker, and Node.js actions supported
  - Actions are downloaded once and cached locally in `~/.magnolia/actions`

## 🔧 Supported Systems

- 🦊 GitLab CI (`.gitlab-ci.yml`)
- 🐙 GitHub Actions (`.github/workflows/*.yml`)
- 🍵 Forgejo/Gitea Actions (`.forgejo/workflows/*.yml` or `.gitea/workflows/*.yml`)

## 📄 License

MIT
