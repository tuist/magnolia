# Magnolia

A CLI tool to run GitLab CI, GitHub Actions, and Forgejo pipelines locally.

## Installation

You can run Magnolia using `mise` with the `ubi` backend:

```bash
mise x ubi:tuist/magnolia
```

Or download pre-built binaries from the [releases page](https://github.com/tuist/magnolia/releases).

## Features

- **Multi-Platform Support**: Run pipelines from GitLab CI, GitHub Actions, and Forgejo/Gitea Actions
- **Auto-Detection**: Automatically detects which CI systems are configured in your repository
- **Job Listing**: View all available jobs and workflows
- **Local Execution**: Test your CI/CD configurations without pushing to remote (simulation mode)

## Usage

### Detect CI Systems

Automatically detect which CI systems are configured in your repository:

```bash
magnolia detect
```

### List Jobs

List all available jobs in your repository:

```bash
magnolia list
```

You can also specify which CI system to use:

```bash
magnolia list --ci gitlab
magnolia list --ci github
magnolia list --ci forgejo
```

### Run a Job

Run a specific job (currently shows what would be executed):

```bash
magnolia run build
magnolia run test --ci github
```

## Supported CI Systems

### GitLab CI
- Configuration file: `.gitlab-ci.yml`
- Supports stages, jobs, scripts, and images

### GitHub Actions
- Configuration files: `.github/workflows/*.yml`
- Supports workflows, jobs, and steps

### Forgejo/Gitea Actions
- Configuration files: `.forgejo/workflows/*.yml` or `.gitea/workflows/*.yml`
- Supports workflows, jobs, steps, and containers

## Development

### Prerequisites

- Rust (latest stable)
- mise (for tool management)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/tuist/magnolia.git
cd magnolia

# Trust and install tools via mise
mise trust
mise install

# Build the project
mise exec -- cargo build --release

# The binary will be available at target/release/magnolia
```

### Running Tests

```bash
mise exec -- cargo test
```

## Roadmap

- [ ] Actual job execution (beyond simulation)
- [ ] Docker container support
- [ ] Environment variable handling
- [ ] Secret management
- [ ] Parallel job execution
- [ ] Job dependency resolution
- [ ] Cache support
- [ ] Interactive job selection
- [ ] Watch mode for continuous testing

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License - see [LICENSE.md](LICENSE.md) for details.

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) for CLI parsing
- [serde](https://github.com/serde-rs/serde) for configuration parsing
- [tokio](https://github.com/tokio-rs/tokio) for async runtime
- [colored](https://github.com/colored-rs/colored) for terminal colors
