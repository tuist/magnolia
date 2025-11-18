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

```bash
# Interactive mode - discover and select pipeline
magnolia

# Direct pipeline file
magnolia .gitlab-ci.yml
magnolia .github/workflows/test.yml
magnolia .forgejo/workflows/deploy.yml
```

### Migrating from External CI Providers

Magnolia can intelligently migrate CI pipelines from external providers (Bitrise, Codemagic, CircleCI) to your Git forge's native CI system using AI agents.

```bash
# Auto-detect source and target CI systems
magnolia migrate

# Override target CI system
magnolia migrate --to github
magnolia migrate --to gitlab
magnolia migrate --to forgejo

# Skip verification
magnolia migrate --no-verify

# Preview migration without writing files
magnolia migrate --dry-run

# Migrate specific source when multiple configs found
magnolia migrate bitrise
magnolia migrate circleci
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
