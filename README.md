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

```bash
# Interactive mode - discover and select pipeline
magnolia

# Direct pipeline file
magnolia .gitlab-ci.yml
magnolia .github/workflows/test.yml
magnolia .forgejo/workflows/deploy.yml
```

### ⚡ Execution

- **GitLab CI**: Executes jobs in containers (Podman/Docker) when `image:` is specified, or on host otherwise.
- **GitHub Actions / Forgejo Actions**: Executes `run:` steps in containers based on `runs-on:` runner. Marketplace actions (`uses:`) support coming soon.

## 🔧 Supported Systems

- 🦊 GitLab CI (`.gitlab-ci.yml`)
- 🐙 GitHub Actions (`.github/workflows/*.yml`)
- 🍵 Forgejo/Gitea Actions (`.forgejo/workflows/*.yml` or `.gitea/workflows/*.yml`)

## 📄 License

MIT
