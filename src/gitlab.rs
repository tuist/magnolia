use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::container;

#[derive(Debug, Deserialize, Serialize)]
struct GitLabCI {
    #[serde(flatten)]
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Job {
    #[serde(default)]
    script: Vec<String>,
    #[serde(default)]
    stage: Option<String>,
    #[serde(default)]
    image: Option<String>,
}

pub fn get_jobs_from_file(pipeline_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let ci: serde_yaml::Value = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    let mut jobs = Vec::new();

    if let Some(obj) = ci.as_mapping() {
        let reserved_keys = vec![
            "stages",
            "variables",
            "default",
            "include",
            "workflow",
            "image",
            "before_script",
            "after_script",
        ];

        for (key, value) in obj {
            if let Some(key_str) = key.as_str() {
                if reserved_keys.contains(&key_str) || key_str.starts_with('.') {
                    continue;
                }

                if value.as_mapping().is_some() {
                    jobs.push(key_str.to_string());
                }
            }
        }
    }

    Ok(jobs)
}

pub fn list_jobs(path: &PathBuf) -> Result<()> {
    let gitlab_ci_path = path.join(".gitlab-ci.yml");

    if !gitlab_ci_path.exists() {
        anyhow::bail!("No .gitlab-ci.yml found in {}", path.display());
    }

    let content =
        std::fs::read_to_string(&gitlab_ci_path).context("Failed to read .gitlab-ci.yml")?;

    let ci: serde_yaml::Value =
        serde_yaml::from_str(&content).context("Failed to parse .gitlab-ci.yml")?;

    println!("\n{}", "Available jobs:".green().bold());

    if let Some(obj) = ci.as_mapping() {
        let reserved_keys = vec![
            "stages",
            "variables",
            "default",
            "include",
            "workflow",
            "image",
            "before_script",
            "after_script",
        ];

        for (key, value) in obj {
            if let Some(key_str) = key.as_str() {
                if reserved_keys.contains(&key_str) || key_str.starts_with('.') {
                    continue;
                }

                if let Some(job) = value.as_mapping() {
                    let stage = job
                        .get(&serde_yaml::Value::String("stage".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let image = job
                        .get(&serde_yaml::Value::String("image".to_string()))
                        .and_then(|v| v.as_str())
                        .unwrap_or("N/A");

                    println!(
                        "  {} {} [stage: {}, image: {}]",
                        "•".cyan(),
                        key_str.yellow(),
                        stage.blue(),
                        image.dimmed()
                    );
                }
            }
        }
    }

    println!();
    Ok(())
}

pub async fn run_job_from_file(pipeline_path: &PathBuf, job_name: &str) -> Result<()> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let ci: serde_yaml::Value = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    if let Some(obj) = ci.as_mapping() {
        if let Some(job) = obj.get(&serde_yaml::Value::String(job_name.to_string())) {
            if let Some(job_map) = job.as_mapping() {
                println!(
                    "\n{}",
                    format!("▶ Executing job: {}", job_name).green().bold()
                );

                // Get job metadata
                let stage = job_map
                    .get(&serde_yaml::Value::String("stage".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");

                let image = job_map
                    .get(&serde_yaml::Value::String("image".to_string()))
                    .and_then(|v| v.as_str());

                println!("  Stage: {}", stage.blue());
                if let Some(img) = image {
                    println!("  Image: {}", img.dimmed());
                }

                // Get the script
                if let Some(script) = job_map.get(&serde_yaml::Value::String("script".to_string()))
                {
                    if let Some(script_array) = script.as_sequence() {
                        let commands: Vec<String> = script_array
                            .iter()
                            .filter_map(|cmd| cmd.as_str().map(String::from))
                            .collect();

                        if commands.is_empty() {
                            println!("\n{}", "No script commands found".yellow());
                            return Ok(());
                        }

                        println!("\n{}", "Commands to execute:".cyan());
                        for cmd in &commands {
                            println!("  $ {}", cmd.dimmed());
                        }

                        // Ask for confirmation
                        let confirm = inquire::Confirm::new("Execute these commands?")
                            .with_default(false)
                            .prompt()
                            .unwrap_or(false);

                        if !confirm {
                            println!("{}", "Execution cancelled".yellow());
                            return Ok(());
                        }

                        println!("\n{}", "─".repeat(60).dimmed());

                        // Detect container runtime
                        let runtime = container::detect_runtime().await;

                        // Determine execution strategy
                        let use_container = image.is_some() && runtime.is_some();

                        if use_container {
                            let img = image.unwrap();
                            let rt = runtime.unwrap();

                            // Combine all commands into a single script for container execution
                            let combined_script = commands.join(" && ");

                            println!("\n{} Executing in container", "→".cyan());

                            container::execute_in_container(
                                &rt,
                                img,
                                &combined_script,
                                "/workspace",
                            )
                            .await
                            .context("Container execution failed")?;

                            println!(
                                "\n{}",
                                format!("✓ Job '{}' completed successfully", job_name)
                                    .green()
                                    .bold()
                            );
                        } else {
                            // Host execution (original behavior)
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

                                let status = tokio::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(cmd)
                                    .status()
                                    .await
                                    .context(format!("Failed to execute command: {}", cmd))?;

                                if !status.success() {
                                    let code = status.code().unwrap_or(-1);
                                    println!(
                                        "\n{}",
                                        format!("✗ Command failed with exit code {}", code)
                                            .red()
                                            .bold()
                                    );
                                    anyhow::bail!("Command failed: {}", cmd);
                                } else {
                                    println!("{}", format!("✓ Command succeeded").green());
                                }
                            }

                            println!("\n{}", "─".repeat(60).dimmed());
                            println!(
                                "\n{}",
                                format!("✓ Job '{}' completed successfully", job_name)
                                    .green()
                                    .bold()
                            );
                        }
                    }
                } else {
                    println!("\n{}", "No script defined for this job".yellow());
                }

                return Ok(());
            }
        }
    }

    anyhow::bail!("Job '{}' not found in .gitlab-ci.yml", job_name)
}
