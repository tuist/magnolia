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

/// Map GitHub Actions / Forgejo Actions runner names to Docker images
pub fn map_runner_to_image(runs_on: &str) -> Option<&'static str> {
    match runs_on {
        // Ubuntu runners - use catthehacker images which include standard CI tools (git, curl, etc.)
        "ubuntu-latest" | "ubuntu-24.04" => Some("catthehacker/ubuntu:runner-latest"),
        "ubuntu-22.04" => Some("catthehacker/ubuntu:runner-22.04"),
        "ubuntu-20.04" => Some("catthehacker/ubuntu:runner-20.04"),

        // Debian runners
        "debian-latest" | "debian-12" => Some("debian:12"),
        "debian-11" => Some("debian:11"),

        // Alpine runners
        "alpine-latest" | "alpine-3" => Some("alpine:latest"),

        // macOS and Windows runners cannot run in containers on Linux
        s if s.starts_with("macos") => None,
        s if s.starts_with("windows") => None,

        // Unknown runner
        _ => None,
    }
}

/// Execute multiple commands (steps) in a container or on host
pub async fn execute_steps(
    runtime: Option<&ContainerRuntime>,
    image: Option<&str>,
    commands: &[String],
) -> Result<()> {
    if let (Some(rt), Some(img)) = (runtime, image) {
        // Container execution - combine commands
        let combined_script = commands.join(" && ");

        println!("\n{} Executing in container", "→".cyan());

        execute_in_container(rt, img, &combined_script, "/workspace").await?;
    } else {
        // Host execution
        if image.is_some() && runtime.is_none() {
            println!(
                "\n{} No container runtime found (Podman/Docker), falling back to host execution",
                "⚠".yellow()
            );
        }

        println!("\n{} Executing on host", "→".cyan());

        for (i, cmd) in commands.iter().enumerate() {
            println!(
                "\n{} {}",
                format!("[{}/{}]", i + 1, commands.len()).cyan(),
                cmd.yellow()
            );

            execute_on_host(cmd)
                .await
                .context(format!("Command failed: {}", cmd))?;

            println!("{}", "✓ Command succeeded".green());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_runner_to_image() {
        assert_eq!(
            map_runner_to_image("ubuntu-latest"),
            Some("catthehacker/ubuntu:runner-latest")
        );
        assert_eq!(
            map_runner_to_image("ubuntu-24.04"),
            Some("catthehacker/ubuntu:runner-latest")
        );
        assert_eq!(
            map_runner_to_image("ubuntu-22.04"),
            Some("catthehacker/ubuntu:runner-22.04")
        );
        assert_eq!(map_runner_to_image("debian-latest"), Some("debian:12"));
        assert_eq!(map_runner_to_image("macos-latest"), None);
    }
}
