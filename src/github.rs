use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::container;

#[derive(Debug, Deserialize, Serialize)]
struct GitHubWorkflow {
    name: Option<String>,
    on: Option<serde_yaml::Value>,
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Job {
    #[serde(rename = "runs-on")]
    runs_on: Option<String>,
    steps: Option<Vec<Step>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Step {
    name: Option<String>,
    run: Option<String>,
    uses: Option<String>,
}

pub fn get_jobs_from_file(pipeline_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let workflow: GitHubWorkflow = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    let jobs: Vec<String> = workflow.jobs.keys().cloned().collect();
    Ok(jobs)
}

pub fn list_jobs(path: &PathBuf) -> Result<()> {
    let workflows_dir = path.join(".github").join("workflows");

    if !workflows_dir.exists() || !workflows_dir.is_dir() {
        anyhow::bail!("No .github/workflows directory found in {}", path.display());
    }

    let entries =
        std::fs::read_dir(&workflows_dir).context("Failed to read workflows directory")?;

    println!("\n{}", "Available workflows and jobs:".green().bold());

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yml")
            || path.extension().and_then(|s| s.to_str()) == Some("yaml")
        {
            let filename = path.file_name().unwrap().to_string_lossy();
            let content =
                std::fs::read_to_string(&path).context(format!("Failed to read {}", filename))?;

            let workflow: GitHubWorkflow =
                serde_yaml::from_str(&content).context(format!("Failed to parse {}", filename))?;

            let workflow_name = workflow.name.as_deref().unwrap_or(filename.as_ref());
            println!(
                "\n  {} {}",
                "Workflow:".cyan().bold(),
                workflow_name.yellow()
            );

            for (job_name, job) in workflow.jobs {
                let runs_on = job.runs_on.as_deref().unwrap_or("N/A");
                let steps_count = job.steps.as_ref().map(|s| s.len()).unwrap_or(0);

                println!(
                    "    {} {} [runs-on: {}, steps: {}]",
                    "•".cyan(),
                    job_name.yellow(),
                    runs_on.blue(),
                    steps_count.to_string().dimmed()
                );
            }
        }
    }

    println!();
    Ok(())
}

pub async fn run_job_from_file(pipeline_path: &PathBuf, job_name: &str) -> Result<()> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let workflow: GitHubWorkflow = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    if let Some(job) = workflow.jobs.get(job_name) {
        println!("\n{}", format!("▶ Executing job: {}", job_name).green().bold());

        let runs_on = job.runs_on.as_deref().unwrap_or("ubuntu-latest");
        println!("  Runs on: {}", runs_on.blue());

        if let Some(steps) = &job.steps {
            // Extract run commands and check for uses steps
            let mut run_commands = Vec::new();
            let mut has_uses_steps = false;
            let mut skipped_steps = Vec::new();

            println!("\n{}", "Steps:".cyan());
            for (i, step) in steps.iter().enumerate() {
                if let Some(run) = &step.run {
                    let step_name = step.name.as_deref().unwrap_or("(unnamed)");
                    println!("  {}. {} {}", i + 1, "Run:".green(), step_name);
                    run_commands.push(run.clone());
                } else if let Some(uses) = &step.uses {
                    has_uses_steps = true;
                    skipped_steps.push(uses.clone());
                    println!("  {}. {} {} {}", i + 1, "Uses:".yellow(), uses.dimmed(), "(skipped)".dimmed());
                }
            }

            if run_commands.is_empty() {
                if has_uses_steps {
                    println!("\n{}", "─".repeat(60).dimmed());
                    println!("\n{}", "⚠ GitHub Actions marketplace actions not yet supported".yellow().bold());
                    println!("This workflow only contains 'uses' steps (marketplace actions like actions/checkout).");
                    println!("Support for marketplace actions is coming soon!");
                    println!("\n{}", "─".repeat(60).dimmed());
                } else {
                    println!("\n{}", "No executable steps found in this job".yellow());
                }
                return Ok(());
            }

            // Show commands to execute
            println!("\n{}", "Commands to execute:".cyan());
            for cmd in &run_commands {
                let first_line = cmd.lines().next().unwrap_or("");
                println!("  $ {}", first_line.dimmed());
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

            // Map runner to image
            let image = container::map_runner_to_image(runs_on);

            if image.is_some() {
                if let Some(img) = image {
                    println!("  Mapped {} → {}", runs_on.blue(), img.yellow());
                }
            } else if runs_on.starts_with("macos") || runs_on.starts_with("windows") {
                println!(
                    "\n{} {} runners not supported in containers, using host execution",
                    "⚠".yellow(),
                    runs_on
                );
            }

            // Execute steps
            container::execute_steps(runtime.as_ref(), image, &run_commands).await?;

            println!("\n{}", "─".repeat(60).dimmed());
            println!("\n{}", format!("✓ Job '{}' completed successfully", job_name).green().bold());

            // Show note about skipped uses steps
            if has_uses_steps {
                println!("\n{}", "Note:".yellow().bold());
                println!("The following 'uses' steps were skipped (not yet supported):");
                for uses in skipped_steps {
                    println!("  - {}", uses.dimmed());
                }
            }

            return Ok(());
        } else {
            println!("\n{}", "No steps defined in this job".yellow());
            return Ok(());
        }
    }

    anyhow::bail!("Job '{}' not found", job_name)
}
