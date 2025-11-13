use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
        println!("\n{}", format!("▶ Analyzing job: {}", job_name).cyan().bold());
        println!(
            "  Runs on: {}",
            job.runs_on.as_deref().unwrap_or("N/A").blue()
        );

        if let Some(steps) = &job.steps {
            println!("\n{}", "Steps:".cyan());
            for (i, step) in steps.iter().enumerate() {
                if let Some(name) = &step.name {
                    println!("  {}. {}", i + 1, name);
                } else if let Some(uses) = &step.uses {
                    println!("  {}. Uses: {}", i + 1, uses.dimmed());
                } else if let Some(run) = &step.run {
                    println!(
                        "  {}. Run: {}",
                        i + 1,
                        run.lines().next().unwrap_or("").dimmed()
                    );
                }
            }
        }

        println!("\n{}", "─".repeat(60).dimmed());
        println!("\n{}", "ℹ GitHub Actions Execution".yellow().bold());
        println!("GitHub Actions workflows use marketplace actions (e.g., actions/checkout@v4)");
        println!("and require a GitHub Actions-compatible runtime.");
        println!("\nFor local execution, consider using:");
        println!("  • {} - Run GitHub Actions locally using Docker", "act".cyan());
        println!("    Install: brew install act");
        println!("    Usage: act -j {}", job_name);
        println!("\n{}", "─".repeat(60).dimmed());

        return Ok(());
    }

    anyhow::bail!("Job '{}' not found", job_name)
}
