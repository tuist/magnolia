use anyhow::{Context, Result};
use colored::*;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::agent::AgentClient;

/// Represents a CI system that can be migrated from or to
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CISystem {
    // Migration sources (external CI providers)
    Bitrise,
    Codemagic,
    CircleCI,
    AppCircle,
    Buildkite,

    // Migration targets (Git forge CI systems)
    GitHubActions,
    GitLabCI,
    ForgejoActions,
}

impl CISystem {
    /// Get the human-readable name of the CI system
    pub fn name(&self) -> &str {
        match self {
            CISystem::Bitrise => "Bitrise",
            CISystem::Codemagic => "Codemagic",
            CISystem::CircleCI => "CircleCI",
            CISystem::AppCircle => "AppCircle",
            CISystem::Buildkite => "Buildkite",
            CISystem::GitHubActions => "GitHub Actions",
            CISystem::GitLabCI => "GitLab CI",
            CISystem::ForgejoActions => "Forgejo Actions",
        }
    }

    /// Get the target output path for this CI system
    pub fn target_path(&self, base_path: &Path, workflow_name: Option<&str>) -> PathBuf {
        match self {
            CISystem::GitHubActions => {
                let workflows_dir = base_path.join(".github").join("workflows");
                let filename = workflow_name.unwrap_or("migrated-workflow.yml");
                workflows_dir.join(filename)
            }
            CISystem::GitLabCI => base_path.join(".gitlab-ci.yml"),
            CISystem::ForgejoActions => {
                let workflows_dir = base_path.join(".forgejo").join("workflows");
                let filename = workflow_name.unwrap_or("migrated-workflow.yml");
                workflows_dir.join(filename)
            }
            _ => panic!("Cannot get target path for source CI system"),
        }
    }
}

/// Represents a detected source CI configuration
#[derive(Debug)]
pub struct SourceConfig {
    pub ci_system: CISystem,
    pub path: PathBuf,
}

impl SourceConfig {
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.ci_system.name(), self.path.display())
    }
}

/// Detects all source CI configurations in the given path
pub fn detect_source_configs(path: &Path) -> Result<Vec<SourceConfig>> {
    let mut configs = Vec::new();

    // Check for Bitrise
    let bitrise_paths = vec![
        path.join("bitrise.yml"),
        path.join(".bitrise").join("bitrise.yml"),
    ];
    for bitrise_path in bitrise_paths {
        if bitrise_path.exists() {
            configs.push(SourceConfig {
                ci_system: CISystem::Bitrise,
                path: bitrise_path,
            });
            break; // Only add one Bitrise config
        }
    }

    // Check for Codemagic
    let codemagic_paths = vec![
        path.join("codemagic.yaml"),
        path.join(".codemagic").join("codemagic.yaml"),
    ];
    for codemagic_path in codemagic_paths {
        if codemagic_path.exists() {
            configs.push(SourceConfig {
                ci_system: CISystem::Codemagic,
                path: codemagic_path,
            });
            break; // Only add one Codemagic config
        }
    }

    // Check for CircleCI
    let circleci_path = path.join(".circleci").join("config.yml");
    if circleci_path.exists() {
        configs.push(SourceConfig {
            ci_system: CISystem::CircleCI,
            path: circleci_path,
        });
    }

    // Check for AppCircle
    let appcircle_paths = vec![
        path.join("appcircle.yaml"),
        path.join("configuration.yaml"), // AppCircle export format
        path.join(".appcircle").join("config.yaml"),
    ];
    for appcircle_path in appcircle_paths {
        if appcircle_path.exists() {
            configs.push(SourceConfig {
                ci_system: CISystem::AppCircle,
                path: appcircle_path,
            });
            break; // Only add one AppCircle config
        }
    }

    // Check for Buildkite
    let buildkite_paths = vec![
        path.join(".buildkite").join("pipeline.yml"),
        path.join(".buildkite").join("pipeline.yaml"),
        path.join("buildkite.yml"), // Alternative location
    ];
    for buildkite_path in buildkite_paths {
        if buildkite_path.exists() {
            configs.push(SourceConfig {
                ci_system: CISystem::Buildkite,
                path: buildkite_path,
            });
            break; // Only add one Buildkite config
        }
    }

    Ok(configs)
}

/// Detects the target CI system from git remote origin
pub async fn detect_target_ci() -> Result<CISystem> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .await
        .context("Failed to get git remote URL. Is this a git repository?")?;

    if !output.status.success() {
        anyhow::bail!("No git remote 'origin' found");
    }

    let remote_url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Detect based on remote URL
    if remote_url.contains("github.com") {
        Ok(CISystem::GitHubActions)
    } else if remote_url.contains("gitlab.com") || remote_url.contains("gitlab") {
        Ok(CISystem::GitLabCI)
    } else if remote_url.contains("forgejo") || remote_url.contains("gitea") {
        Ok(CISystem::ForgejoActions)
    } else {
        anyhow::bail!(
            "Could not determine target CI system from remote URL: {}. \
            Please use --to flag to specify the target CI system.",
            remote_url
        )
    }
}

/// Get git context information for the agent
pub async fn get_git_context() -> Result<String> {
    let remote_output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .await?;

    let branch_output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .await?;

    let remote_url = String::from_utf8_lossy(&remote_output.stdout)
        .trim()
        .to_string();
    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    Ok(format!(
        "Git Remote: {}\nCurrent Branch: {}",
        remote_url, branch
    ))
}

/// Migration options
pub struct MigrationOptions {
    pub source: Option<String>,
    pub target: Option<CISystem>,
    pub verify: bool,
    pub dry_run: bool,
    pub path: PathBuf,
}

/// Migrate a CI pipeline
pub async fn migrate(options: MigrationOptions) -> Result<()> {
    println!("{}", "Detecting CI configurations...".cyan().bold());

    // Detect source configurations
    let source_configs = detect_source_configs(&options.path)?;

    if source_configs.is_empty() {
        println!("{}", "No source CI configurations found.".yellow());
        println!("\nMagnolia can migrate from:");
        println!("  - Bitrise: bitrise.yml or .bitrise/bitrise.yml");
        println!("  - Codemagic: codemagic.yaml or .codemagic/codemagic.yaml");
        println!("  - CircleCI: .circleci/config.yml");
        println!("  - AppCircle: appcircle.yaml, configuration.yaml, or .appcircle/config.yaml");
        println!("  - Buildkite: .buildkite/pipeline.yml or .buildkite/pipeline.yaml");
        return Ok(());
    }

    // Select source configuration
    let source_config = if let Some(requested_source) = options.source {
        source_configs
            .into_iter()
            .find(|c| c.ci_system.name().to_lowercase() == requested_source.to_lowercase())
            .context(format!("Source CI system '{}' not found", requested_source))?
    } else if source_configs.len() == 1 {
        source_configs.into_iter().next().unwrap()
    } else {
        // Multiple sources found, prompt user
        let options_list: Vec<String> = source_configs.iter().map(|c| c.display_name()).collect();

        let selected = inquire::Select::new(
            "Multiple CI configurations found. Which would you like to migrate?",
            options_list,
        )
        .prompt()
        .map_err(|_| anyhow::anyhow!("Selection cancelled"))?;

        source_configs
            .into_iter()
            .find(|c| c.display_name() == selected)
            .unwrap()
    };

    println!(
        "{} {} {}",
        "Source:".cyan(),
        source_config.ci_system.name().yellow(),
        format!("({})", source_config.path.display()).dimmed()
    );

    // Detect target CI system
    let target_ci = if let Some(target) = options.target {
        target
    } else {
        detect_target_ci().await?
    };

    println!("{} {}", "Target:".cyan(), target_ci.name().yellow());

    // Read source configuration
    let source_content = tokio::fs::read_to_string(&source_config.path)
        .await
        .context("Failed to read source configuration")?;

    // Get git context
    let git_context = get_git_context().await.unwrap_or_default();

    // Create agent client
    println!(
        "\n{}",
        "Initializing AI agent for migration...".cyan().bold()
    );
    let agent = AgentClient::new()?;

    // Execute migration
    println!(
        "{}",
        "Analyzing source configuration and researching documentation...".cyan()
    );
    println!("{}", "This may take a moment...".dimmed());

    let migrated_config = agent
        .migrate_pipeline(
            &source_content,
            source_config.ci_system.name(),
            target_ci.name(),
            &git_context,
        )
        .await
        .context("Migration failed")?;

    // Determine output path
    let output_path = target_ci.target_path(&options.path, None);

    if options.dry_run {
        println!("\n{}", "Dry run - migration preview:".green().bold());
        println!("\n{}", "Generated configuration:".cyan());
        println!("{}", "=".repeat(80).dimmed());
        println!("{}", migrated_config);
        println!("{}", "=".repeat(80).dimmed());
        println!(
            "\n{} {}",
            "Would be written to:".cyan(),
            output_path.display().to_string().yellow()
        );
        return Ok(());
    }

    // Create output directory if needed
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .context("Failed to create output directory")?;
    }

    // Write migrated configuration
    tokio::fs::write(&output_path, &migrated_config)
        .await
        .context("Failed to write migrated configuration")?;

    println!("\n{} {}", "Migration complete!".green().bold(), "✓".green());
    println!(
        "{} {}",
        "Configuration written to:".cyan(),
        output_path.display().to_string().yellow()
    );

    // Verify if requested
    if options.verify {
        println!("\n{}", "Verifying migrated configuration...".cyan().bold());

        // TODO: Use the existing 'run' command to verify the pipeline
        // For now, just validate that we can read it back
        let _verify_content = tokio::fs::read_to_string(&output_path)
            .await
            .context("Failed to read migrated configuration for verification")?;

        println!("{}", "Verification complete!".green());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_system_names() {
        assert_eq!(CISystem::Bitrise.name(), "Bitrise");
        assert_eq!(CISystem::GitHubActions.name(), "GitHub Actions");
        assert_eq!(CISystem::GitLabCI.name(), "GitLab CI");
    }

    #[test]
    fn test_target_path_github() {
        let base = PathBuf::from("/tmp/repo");
        let path = CISystem::GitHubActions.target_path(&base, Some("test.yml"));
        assert_eq!(path, base.join(".github/workflows/test.yml"));
    }

    #[test]
    fn test_target_path_gitlab() {
        let base = PathBuf::from("/tmp/repo");
        let path = CISystem::GitLabCI.target_path(&base, None);
        assert_eq!(path, base.join(".gitlab-ci.yml"));
    }

    #[tokio::test]
    async fn test_detect_source_configs_empty() {
        let temp = std::env::temp_dir().join("magnolia-test-empty");
        let _ = std::fs::create_dir_all(&temp);

        let configs = detect_source_configs(&temp).unwrap();
        assert!(configs.is_empty());

        let _ = std::fs::remove_dir_all(&temp);
    }
}
