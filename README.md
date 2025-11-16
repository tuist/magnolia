# 🌸 Magnolia

Run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

## 🌸 The Magnolia Manifesto

**[Verse 1]**
They say that if you were to see
Your CI running locally
In that terminal window, free
It brings you good luck
All of you have come
Even the vendors who dismissed us
Today, they watch

**[Chorus]**
Throw magnolias at me
Run your pipelines locally
Throw magnolias at me
Own your CI destiny

**[Verse 2]**
Over their platforms, open forges burning bright
Tears and FUD melt into the code
GitLab and GitHub, Forgejo's might
Dancing with freedom on top of vendor lock
Today it's all sovereignty mocking fate
And what you couldn't test locally, you test before you commit

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
