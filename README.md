# 🌸 Magnolia
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

> [!IMPORTANT]
> This project is in the ideation phase. We may open PRs and address issues, but we're not actively monitoring repository activity.

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
mise use -g ubi:tuist/magnolia
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

# Run non-interactively (useful for scripts)
magnolia .github/workflows/test.yml --job test --non-interactive
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

# Automated migration (non-interactive)
magnolia migrate bitrise --non-interactive
```

**Example migration:**
```bash
$ magnolia migrate bitrise --dry-run --to github
Detecting CI configurations...
Source: Bitrise (bitrise.yml)
Target: GitHub Actions

Initializing AI agent for migration...

📋 Step 1/3: Analyzing source configuration...
  → Reading Bitrise pipeline from bitrise.yml

🔍 Step 2/3: Researching CI system documentation and generating configuration...
  → Consulting Bitrise and GitHub Actions documentation
  → This may take 30-60 seconds...

✅ Step 3/3: Migration complete!

Dry run - migration preview:

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

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/jsj"><img src="https://avatars.githubusercontent.com/u/13633271?v=4?s=100" width="100px;" alt="James Jackson"/><br /><sub><b>James Jackson</b></sub></a><br /><a href="https://github.com/tuist/magnolia/commits?author=jsj" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!