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

pub fn list_jobs(path: &PathBuf) -> Result<()> {
    let workflows_dir = path.join(".github").join("workflows");

    if !workflows_dir.exists() || !workflows_dir.is_dir() {
        anyhow::bail!("No .github/workflows directory found in {}", path.display());
    }

    let entries = std::fs::read_dir(&workflows_dir)
        .context("Failed to read workflows directory")?;

    println!("\n{}", "Available workflows and jobs:".green().bold());

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yml")
            || path.extension().and_then(|s| s.to_str()) == Some("yaml") {

            let filename = path.file_name().unwrap().to_string_lossy();
            let content = std::fs::read_to_string(&path)
                .context(format!("Failed to read {}", filename))?;

            let workflow: GitHubWorkflow = serde_yaml::from_str(&content)
                .context(format!("Failed to parse {}", filename))?;

            let workflow_name = workflow.name.as_deref().unwrap_or(filename.as_ref());
            println!("\n  {} {}", "Workflow:".cyan().bold(), workflow_name.yellow());

            for (job_name, job) in workflow.jobs {
                let runs_on = job.runs_on.as_deref().unwrap_or("N/A");
                let steps_count = job.steps.as_ref().map(|s| s.len()).unwrap_or(0);

                println!("    {} {} [runs-on: {}, steps: {}]",
                    "•".cyan(),
                    job_name.yellow(),
                    runs_on.blue(),
                    steps_count.to_string().dimmed());
            }
        }
    }

    println!();
    Ok(())
}

pub async fn run_job(path: &PathBuf, job_name: &str) -> Result<()> {
    let workflows_dir = path.join(".github").join("workflows");

    if !workflows_dir.exists() || !workflows_dir.is_dir() {
        anyhow::bail!("No .github/workflows directory found in {}", path.display());
    }

    let entries = std::fs::read_dir(&workflows_dir)
        .context("Failed to read workflows directory")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yml")
            || path.extension().and_then(|s| s.to_str()) == Some("yaml") {

            let content = std::fs::read_to_string(&path)?;
            let workflow: GitHubWorkflow = serde_yaml::from_str(&content)?;

            if let Some(job) = workflow.jobs.get(job_name) {
                println!("{}", format!("Executing job: {}", job_name).green().bold());
                println!("Runs on: {}", job.runs_on.as_deref().unwrap_or("N/A").blue());

                if let Some(steps) = &job.steps {
                    println!("\n{}", "Steps:".cyan());
                    for (i, step) in steps.iter().enumerate() {
                        if let Some(name) = &step.name {
                            println!("  {}. {}", i + 1, name);
                        } else if let Some(uses) = &step.uses {
                            println!("  {}. Uses: {}", i + 1, uses.dimmed());
                        } else if let Some(run) = &step.run {
                            println!("  {}. Run: {}", i + 1, run.lines().next().unwrap_or("").dimmed());
                        }
                    }
                }

                println!("\n{}", "Note: Actual execution is not yet implemented.".yellow());
                println!("This would execute the above steps in the specified environment.");
                return Ok(());
            }
        }
    }

    anyhow::bail!("Job '{}' not found in any GitHub workflow", job_name)
}
