use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
                println!("{}", format!("Executing job: {}", job_name).green().bold());

                // Get the script
                if let Some(script) = job_map.get(&serde_yaml::Value::String("script".to_string()))
                {
                    if let Some(script_array) = script.as_sequence() {
                        println!("\n{}", "Script commands:".cyan());
                        for cmd in script_array {
                            if let Some(cmd_str) = cmd.as_str() {
                                println!("  {}", cmd_str.dimmed());
                            }
                        }

                        println!(
                            "\n{}",
                            "Note: Actual execution is not yet implemented.".yellow()
                        );
                        println!(
                            "This would execute the above commands in the specified environment."
                        );
                    }
                }

                return Ok(());
            }
        }
    }

    anyhow::bail!("Job '{}' not found in .gitlab-ci.yml", job_name)
}
