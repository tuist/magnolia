# Magnolia

Run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

## Installation

```bash
mise x ubi:tuist/magnolia
```

Or download from [releases](https://github.com/tuist/magnolia/releases).

## Usage

```bash
# Interactive mode - discover and select pipeline
magnolia

# Direct pipeline file
magnolia .gitlab-ci.yml
magnolia .github/workflows/test.yml
magnolia .forgejo/workflows/deploy.yml
```

## Supported Systems

- GitLab CI (`.gitlab-ci.yml`)
- GitHub Actions (`.github/workflows/*.yml`)
- Forgejo/Gitea Actions (`.forgejo/workflows/*.yml` or `.gitea/workflows/*.yml`)

## License

MIT
