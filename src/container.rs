use anyhow::{Context, Result};
use colored::*;
use std::process::Stdio;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerRuntime {
    Podman,
    Docker,
}

impl ContainerRuntime {
    pub fn command(&self) -> &str {
        match self {
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Docker => "docker",
        }
    }
}

impl std::fmt::Display for ContainerRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerRuntime::Podman => write!(f, "Podman"),
            ContainerRuntime::Docker => write!(f, "Docker"),
        }
    }
}

/// Detect available container runtime, preferring Podman over Docker
pub async fn detect_runtime() -> Option<ContainerRuntime> {
    // Try Podman first
    if check_runtime_available("podman").await {
        return Some(ContainerRuntime::Podman);
    }

    // Fallback to Docker
    if check_runtime_available("docker").await {
        return Some(ContainerRuntime::Docker);
    }

    None
}

async fn check_runtime_available(command: &str) -> bool {
    tokio::process::Command::new(command)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Execute a command in a container
pub async fn execute_in_container(
    runtime: &ContainerRuntime,
    image: &str,
    command: &str,
    workdir: &str,
) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let current_dir_str = current_dir
        .to_str()
        .context("Invalid current directory path")?;

    println!(
        "  {} Using {} with image {}",
        "→".cyan(),
        runtime.to_string().blue(),
        image.yellow()
    );

    let status = tokio::process::Command::new(runtime.command())
        .arg("run")
        .arg("--rm") // Remove container after execution
        .arg("-v")
        .arg(format!("{}:{}", current_dir_str, workdir)) // Mount current dir
        .arg("-w")
        .arg(workdir) // Set working directory
        .arg(image)
        .arg("sh")
        .arg("-c")
        .arg(command)
        .status()
        .await
        .context(format!("Failed to execute {} command", runtime))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        anyhow::bail!("Container command failed with exit code {}", code);
    }

    Ok(())
}

/// Execute a command on the host (fallback when no container runtime available)
pub async fn execute_on_host(command: &str) -> Result<()> {
    let status = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .await
        .context("Failed to execute command on host")?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        anyhow::bail!("Host command failed with exit code {}", code);
    }

    Ok(())
}
