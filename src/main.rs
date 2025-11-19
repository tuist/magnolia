use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};

mod actions;
mod agent;
mod container;
mod forgejo;
mod github;
mod gitlab;
mod migrate;

#[derive(Parser)]
#[command(name = "magnolia")]
#[command(about = "Run GitLab, GitHub, and Forgejo pipelines locally", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to a specific pipeline file (e.g., .gitlab-ci.yml, .github/workflows/test.yml)
    /// If not provided, will detect and prompt for available pipelines
    pipeline: Option<PathBuf>,

    /// Job to run (useful for non-interactive mode)
    #[arg(long, short)]
    job: Option<String>,

    /// Disable interactive prompts
    #[arg(long, short = 'n', global = true)]
    non_interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Migrate CI pipelines from external providers to Git forge CI systems
    Migrate {
        /// Source CI system to migrate from (e.g., bitrise, codemagic, circleci)
        /// If not specified, will auto-detect from repository
        source: Option<String>,

        /// Target CI system to migrate to (github, gitlab, forgejo)
        /// If not specified, will auto-detect from git remote origin
        #[arg(long)]
        to: Option<String>,

        /// Skip verification by running the migrated pipeline locally
        #[arg(long)]
        no_verify: bool,

        /// Preview the migration without writing files
        #[arg(long)]
        dry_run: bool,

        /// Path to repository (defaults to current directory)
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Copy)]
enum CISystem {
    GitLab,
    GitHub,
    Forgejo,
}

impl std::fmt::Display for CISystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CISystem::GitLab => write!(f, "GitLab CI"),
            CISystem::GitHub => write!(f, "GitHub Actions"),
            CISystem::Forgejo => write!(f, "Forgejo Actions"),
        }
    }
}

#[allow(dead_code)]
fn detect_ci_systems(path: &Path) -> Vec<CISystem> {
    let mut systems = Vec::new();

    // Check for GitLab CI
    let gitlab_ci = path.join(".gitlab-ci.yml");
    if gitlab_ci.exists() {
        systems.push(CISystem::GitLab);
    }

    // Check for GitHub Actions
    let github_workflows = path.join(".github").join("workflows");
    if github_workflows.exists() && github_workflows.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&github_workflows) {
            if entries.count() > 0 {
                systems.push(CISystem::GitHub);
            }
        }
    }

    // Check for Forgejo Actions
    let forgejo_workflows = path.join(".forgejo").join("workflows");
    let gitea_workflows = path.join(".gitea").join("workflows");
    if (forgejo_workflows.exists() && forgejo_workflows.is_dir())
        || (gitea_workflows.exists() && gitea_workflows.is_dir())
    {
        systems.push(CISystem::Forgejo);
    }

    systems
}

fn discover_pipelines(path: &Path) -> Result<Vec<(String, PathBuf, CISystem)>> {
    let mut pipelines = Vec::new();

    // Check for GitLab CI
    let gitlab_ci = path.join(".gitlab-ci.yml");
    if gitlab_ci.exists() {
        pipelines.push(("GitLab CI".to_string(), gitlab_ci, CISystem::GitLab));
    }

    // Check for GitHub Actions
    let github_workflows = path.join(".github").join("workflows");
    if github_workflows.exists() && github_workflows.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&github_workflows) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yml")
                    || path.extension().and_then(|s| s.to_str()) == Some("yaml")
                {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    pipelines.push((
                        format!("GitHub Actions: {}", filename),
                        path,
                        CISystem::GitHub,
                    ));
                }
            }
        }
    }

    // Check for Forgejo Actions
    let workflows_dirs = vec![
        path.join(".forgejo").join("workflows"),
        path.join(".gitea").join("workflows"),
    ];

    for workflows_dir in workflows_dirs {
        if workflows_dir.exists() && workflows_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&workflows_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("yml")
                        || path.extension().and_then(|s| s.to_str()) == Some("yaml")
                    {
                        let filename = path.file_name().unwrap().to_string_lossy().to_string();
                        pipelines.push((
                            format!("Forgejo Actions: {}", filename),
                            path,
                            CISystem::Forgejo,
                        ));
                    }
                }
            }
        }
    }

    Ok(pipelines)
}

fn detect_ci_type(pipeline_path: &Path) -> Result<CISystem> {
    let filename = pipeline_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if filename == ".gitlab-ci.yml" {
        return Ok(CISystem::GitLab);
    }

    // Check if it's in .github/workflows or .forgejo/workflows or .gitea/workflows
    if let Some(parent) = pipeline_path.parent() {
        if let Some(grandparent) = parent.parent() {
            let parent_name = parent.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let grandparent_name = grandparent
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if parent_name == "workflows" {
                if grandparent_name == ".github" {
                    return Ok(CISystem::GitHub);
                } else if grandparent_name == ".forgejo" || grandparent_name == ".gitea" {
                    return Ok(CISystem::Forgejo);
                }
            }
        }
    }

    anyhow::bail!(
        "Could not determine CI system from path: {}",
        pipeline_path.display()
    )
}

fn parse_target_ci(target: &str) -> Result<migrate::CISystem> {
    match target.to_lowercase().as_str() {
        "github" => Ok(migrate::CISystem::GitHubActions),
        "gitlab" => Ok(migrate::CISystem::GitLabCI),
        "forgejo" | "gitea" => Ok(migrate::CISystem::ForgejoActions),
        _ => anyhow::bail!(
            "Unknown target CI system: {}. Valid options: github, gitlab, forgejo",
            target
        ),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::Migrate {
                source,
                to,
                no_verify,
                dry_run,
                path,
            } => {
                // Parse target CI system if provided
                let target = if let Some(target_str) = to {
                    Some(parse_target_ci(&target_str)?)
                } else {
                    None
                };

                let options = migrate::MigrationOptions {
                    source,
                    target,
                    verify: !no_verify,
                    dry_run,
                    path,
                    non_interactive: cli.non_interactive,
                };

                return migrate::migrate(options).await;
            }
        }
    }

    // Original behavior: run a pipeline
    let (pipeline_path, ci_system) = if let Some(path) = cli.pipeline {
        // Direct pipeline file specified
        if !path.exists() {
            anyhow::bail!("Pipeline file not found: {}", path.display());
        }
        let ci_type = detect_ci_type(&path)?;
        (path, ci_type)
    } else {
        // Discover and prompt for pipeline
        println!("{}", "Discovering pipelines...".cyan().bold());
        let pipelines = discover_pipelines(&PathBuf::from("."))?;

        if pipelines.is_empty() {
            println!("{}", "No pipelines found in current directory.".yellow());
            println!("\nMagnolia supports:");
            println!("  - GitLab CI: .gitlab-ci.yml");
            println!("  - GitHub Actions: .github/workflows/*.yml");
            println!("  - Forgejo Actions: .forgejo/workflows/*.yml or .gitea/workflows/*.yml");
            return Ok(());
        }

        if cli.non_interactive {
            if pipelines.len() == 1 {
                let p = pipelines.first().unwrap();
                (p.1.clone(), p.2)
            } else {
                anyhow::bail!("Multiple pipelines found. Please specify one or use interactive mode.");
            }
        } else {
            let pipeline_options: Vec<String> =
                pipelines.iter().map(|(name, _, _)| name.clone()).collect();

            let selected_idx = inquire::Select::new("Select a pipeline:", pipeline_options)
                .prompt()
                .map_err(|_| anyhow::anyhow!("Selection cancelled"))?;

            let selected = pipelines
                .iter()
                .find(|(name, _, _)| name == &selected_idx)
                .unwrap();

            (selected.1.clone(), selected.2)
        }
    };

    // Get available jobs from the selected pipeline
    let available_jobs: Vec<String> = match &ci_system {
        CISystem::GitLab => gitlab::get_jobs_from_file(&pipeline_path)?,
        CISystem::GitHub => github::get_jobs_from_file(&pipeline_path)?,
        CISystem::Forgejo => forgejo::get_jobs_from_file(&pipeline_path)?,
    };

    if available_jobs.is_empty() {
        anyhow::bail!("No jobs found in pipeline");
    }

    // Select job
    let selected_job = if let Some(job_name) = cli.job {
        if !available_jobs.contains(&job_name) {
            anyhow::bail!("Job '{}' not found in pipeline. Available jobs: {:?}", job_name, available_jobs);
        }
        job_name
    } else if cli.non_interactive {
        if available_jobs.len() == 1 {
            available_jobs.first().unwrap().clone()
        } else {
             anyhow::bail!("Multiple jobs found. Please specify one with --job or use interactive mode. Available jobs: {:?}", available_jobs);
        }
    } else {
        inquire::Select::new("Select a job to run:", available_jobs)
            .prompt()
            .map_err(|_| anyhow::anyhow!("Selection cancelled"))?
    };

    println!(
        "\n{} {} {} {}",
        "Running job".cyan().bold(),
        selected_job.yellow(),
        "from".cyan(),
        pipeline_path.display().to_string().dimmed()
    );

    // Run the selected job
    match ci_system {
        CISystem::GitLab => {
            gitlab::run_job_from_file(&pipeline_path, &selected_job, cli.non_interactive).await?
        }
        CISystem::GitHub => {
            github::run_job_from_file(&pipeline_path, &selected_job, cli.non_interactive).await?
        }
        CISystem::Forgejo => {
            forgejo::run_job_from_file(&pipeline_path, &selected_job, cli.non_interactive).await?
        }
    }

    Ok(())
}
