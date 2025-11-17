use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a parsed action reference (e.g., "actions/checkout@v4")
#[derive(Debug, Clone)]
pub struct ActionRef {
    pub owner: String,
    pub repo: String,
    pub ref_name: String, // tag, branch, or commit SHA
}

impl ActionRef {
    /// Parse an action reference string like "actions/checkout@v4"
    pub fn parse(action_ref: &str) -> Result<Self> {
        let parts: Vec<&str> = action_ref.split('@').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid action reference: {}. Expected format: owner/repo@ref", action_ref);
        }

        let repo_parts: Vec<&str> = parts[0].split('/').collect();
        if repo_parts.len() != 2 {
            anyhow::bail!("Invalid action repository: {}. Expected format: owner/repo", parts[0]);
        }

        Ok(ActionRef {
            owner: repo_parts[0].to_string(),
            repo: repo_parts[1].to_string(),
            ref_name: parts[1].to_string(),
        })
    }

    /// Get the cache directory for this action
    pub fn cache_dir(&self) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".magnolia")
            .join("actions")
            .join(&self.owner)
            .join(&self.repo)
            .join(&self.ref_name)
    }

    /// Get the GitHub URL for this action
    pub fn github_url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.repo)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "using")]
pub enum ActionType {
    #[serde(rename = "composite")]
    Composite,
    #[serde(rename = "docker")]
    Docker { image: String },
    #[serde(rename = "node20")]
    Node20 { main: String },
    #[serde(rename = "node16")]
    Node16 { main: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMetadata {
    pub name: String,
    pub description: Option<String>,
    pub runs: ActionRuns,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionRuns {
    Composite {
        using: String,
        steps: Vec<CompositeStep>,
    },
    Docker {
        using: String,
        image: String,
    },
    Node {
        using: String,
        main: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeStep {
    pub name: Option<String>,
    pub run: Option<String>,
    pub uses: Option<String>,
    pub shell: Option<String>,
}

/// Download an action from GitHub if not already cached
pub async fn download_action(action_ref: &ActionRef) -> Result<PathBuf> {
    let cache_dir = action_ref.cache_dir();

    // Check if already cached
    if cache_dir.exists() {
        println!(
            "  {} Using cached action: {}/{}@{}",
            "→".cyan(),
            action_ref.owner,
            action_ref.repo,
            action_ref.ref_name
        );
        return Ok(cache_dir);
    }

    println!(
        "  {} Downloading action: {}/{}@{}",
        "→".cyan(),
        action_ref.owner.yellow(),
        action_ref.repo.yellow(),
        action_ref.ref_name.yellow()
    );

    // Create cache directory
    std::fs::create_dir_all(&cache_dir)
        .context(format!("Failed to create cache directory: {}", cache_dir.display()))?;

    // Clone the repository
    let status = tokio::process::Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg(&action_ref.ref_name)
        .arg(action_ref.github_url())
        .arg(&cache_dir)
        .status()
        .await
        .context("Failed to clone action repository")?;

    if !status.success() {
        anyhow::bail!("Failed to download action from GitHub");
    }

    Ok(cache_dir)
}

/// Load action metadata from action.yml or action.yaml
pub fn load_action_metadata(action_dir: &PathBuf) -> Result<ActionMetadata> {
    let yml_path = action_dir.join("action.yml");
    let yaml_path = action_dir.join("action.yaml");

    let metadata_path = if yml_path.exists() {
        yml_path
    } else if yaml_path.exists() {
        yaml_path
    } else {
        anyhow::bail!("No action.yml or action.yaml found in {}", action_dir.display());
    };

    let content = std::fs::read_to_string(&metadata_path)
        .context(format!("Failed to read {}", metadata_path.display()))?;

    let metadata: ActionMetadata = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", metadata_path.display()))?;

    Ok(metadata)
}

/// Determine the action type from metadata
pub fn get_action_type(metadata: &ActionMetadata) -> Result<ActionType> {
    match &metadata.runs {
        ActionRuns::Composite { using, .. } if using == "composite" => {
            Ok(ActionType::Composite)
        }
        ActionRuns::Docker { using, image } if using == "docker" => {
            Ok(ActionType::Docker { image: image.clone() })
        }
        ActionRuns::Node { using, main } if using == "node20" => {
            Ok(ActionType::Node20 { main: main.clone() })
        }
        ActionRuns::Node { using, main } if using == "node16" => {
            Ok(ActionType::Node16 { main: main.clone() })
        }
        _ => anyhow::bail!("Unknown or unsupported action type"),
    }
}

/// Execute a composite action
pub async fn execute_composite_action(
    steps: &[CompositeStep],
    action_dir: &PathBuf,
) -> Result<()> {
    println!("  {} Executing composite action steps", "→".cyan());

    for (i, step) in steps.iter().enumerate() {
        let default_name = format!("Step {}", i + 1);
        let step_name = step.name.as_deref().unwrap_or(&default_name);
        println!("\n  {} {}", format!("[{}/{}]", i + 1, steps.len()).cyan(), step_name.yellow());

        if let Some(run) = &step.run {
            // Execute the run command
            let shell = step.shell.as_deref().unwrap_or("sh");

            let status = tokio::process::Command::new(shell)
                .arg("-c")
                .arg(run)
                .current_dir(action_dir)
                .status()
                .await
                .context(format!("Failed to execute composite step: {}", step_name))?;

            if !status.success() {
                let code = status.code().unwrap_or(-1);
                anyhow::bail!("Composite step '{}' failed with exit code {}", step_name, code);
            }

            println!("    {}", "✓ Step succeeded".green());
        } else if let Some(uses) = &step.uses {
            // Nested action - recursively execute
            println!("    {} Nested action: {}", "→".cyan(), uses.dimmed());
            Box::pin(execute_action(uses)).await?;
        }
    }

    Ok(())
}

/// Execute a Docker action
pub async fn execute_docker_action(
    image: &str,
    action_dir: &PathBuf,
    runtime: &crate::container::ContainerRuntime,
) -> Result<()> {
    println!("  {} Executing Docker action with image: {}", "→".cyan(), image.yellow());

    // Resolve image path (could be Dockerfile or image name)
    let resolved_image = if image.starts_with("Dockerfile") || image.starts_with("./") {
        // Build the Docker image from Dockerfile
        let dockerfile_path = action_dir.join(image);
        println!("    {} Building Docker image from {}", "→".cyan(), dockerfile_path.display());

        // Generate a tag for the built image
        let tag = format!("magnolia-action-{}", uuid::Uuid::new_v4());

        let status = tokio::process::Command::new(runtime.command())
            .arg("build")
            .arg("-t")
            .arg(&tag)
            .arg("-f")
            .arg(&dockerfile_path)
            .arg(action_dir)
            .status()
            .await
            .context("Failed to build Docker image")?;

        if !status.success() {
            anyhow::bail!("Failed to build Docker image from {}", dockerfile_path.display());
        }

        tag
    } else {
        image.to_string()
    };

    // Run the Docker container
    crate::container::execute_in_container(
        runtime,
        &resolved_image,
        "echo 'Action executed'", // Actions typically have their entrypoint defined
        "/github/workspace",
    )
    .await?;

    Ok(())
}

/// Execute a Node.js action
pub async fn execute_node_action(
    main: &str,
    action_dir: &PathBuf,
    node_version: &str,
) -> Result<()> {
    println!("  {} Executing Node.js action: {}", "→".cyan(), main.yellow());

    // Check if node is available
    let node_check = tokio::process::Command::new("node")
        .arg("--version")
        .output()
        .await;

    if node_check.is_err() {
        anyhow::bail!(
            "Node.js is required to run this action but was not found. Please install Node.js {}",
            node_version
        );
    }

    let main_file = action_dir.join(main);
    if !main_file.exists() {
        anyhow::bail!("Action main file not found: {}", main_file.display());
    }

    let status = tokio::process::Command::new("node")
        .arg(&main_file)
        .current_dir(action_dir)
        .status()
        .await
        .context(format!("Failed to execute Node.js action: {}", main))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        anyhow::bail!("Node.js action failed with exit code {}", code);
    }

    println!("  {}", "✓ Node.js action succeeded".green());

    Ok(())
}

/// Main entry point: execute any action by reference
pub async fn execute_action(action_ref: &str) -> Result<()> {
    let action = ActionRef::parse(action_ref)?;

    // Download action (or use cached)
    let action_dir = download_action(&action).await?;

    // Load metadata
    let metadata = load_action_metadata(&action_dir)?;
    println!("  {} Action: {}", "→".cyan(), metadata.name.blue());

    // Determine action type and execute
    let action_type = get_action_type(&metadata)?;

    match action_type {
        ActionType::Composite => {
            if let ActionRuns::Composite { steps, .. } = metadata.runs {
                execute_composite_action(&steps, &action_dir).await?;
            }
        }
        ActionType::Docker { image } => {
            // Detect container runtime
            let runtime = crate::container::detect_runtime().await
                .context("Docker actions require Podman or Docker to be installed")?;
            execute_docker_action(&image, &action_dir, &runtime).await?;
        }
        ActionType::Node20 { main } => {
            execute_node_action(&main, &action_dir, "20").await?;
        }
        ActionType::Node16 { main } => {
            execute_node_action(&main, &action_dir, "16").await?;
        }
    }

    Ok(())
}
