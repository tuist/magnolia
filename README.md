# 🌸 Magnolia

Run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

## 📦 Installation

```bash
mise x ubi:tuist/magnolia
```

Or download from [releases](https://github.com/tuist/magnolia/releases).

## 🚀 Usage

```bash
# Interactive mode - discover and select pipeline
magnolia

# Direct pipeline file
magnolia .gitlab-ci.yml
magnolia .github/workflows/test.yml
magnolia .forgejo/workflows/deploy.yml
```

### ⚡ Execution

- **GitLab CI**: Scripts are executed directly on your machine after confirmation. You'll see each command before it runs.
- **GitHub Actions / Forgejo Actions**: Displays job details and recommends using [act](https://github.com/nektos/act) for local execution with Docker containers.

## 🔧 Supported Systems

- 🦊 GitLab CI (`.gitlab-ci.yml`)
- 🐙 GitHub Actions (`.github/workflows/*.yml`)
- 🍵 Forgejo/Gitea Actions (`.forgejo/workflows/*.yml` or `.gitea/workflows/*.yml`)

## 📄 License

MIT
