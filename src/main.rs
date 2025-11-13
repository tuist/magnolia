use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod forgejo;
mod github;
mod gitlab;

#[derive(Parser)]
#[command(name = "magnolia")]
#[command(about = "Run GitLab, GitHub, and Forgejo pipelines locally", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect and list available CI pipelines
    Detect {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// List all jobs in the pipeline
    List {
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// CI system to use (gitlab, github, forgejo)
        #[arg(short, long)]
        ci: Option<String>,
    },
    /// Run a specific job from the pipeline
    Run {
        /// Name of the job to run
        job: String,
        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// CI system to use (gitlab, github, forgejo)
        #[arg(short, long)]
        ci: Option<String>,
    },
}

#[derive(Debug)]
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

fn detect_ci_systems(path: &PathBuf) -> Vec<CISystem> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect { path } => {
            println!("{}", "Detecting CI systems...".cyan().bold());
            let systems = detect_ci_systems(&path);

            if systems.is_empty() {
                println!("{}", "No CI systems detected in this repository.".yellow());
                println!("Magnolia supports:");
                println!("  - GitLab CI (.gitlab-ci.yml)");
                println!("  - GitHub Actions (.github/workflows/*.yml)");
                println!(
                    "  - Forgejo Actions (.forgejo/workflows/*.yml or .gitea/workflows/*.yml)"
                );
            } else {
                println!("{}", "Found the following CI systems:".green().bold());
                for system in systems {
                    println!("  {} {}", "✓".green(), system);
                }
            }
        }
        Commands::List { path, ci } => {
            let ci_system = if let Some(ci_name) = ci {
                match ci_name.to_lowercase().as_str() {
                    "gitlab" => CISystem::GitLab,
                    "github" => CISystem::GitHub,
                    "forgejo" | "gitea" => CISystem::Forgejo,
                    _ => anyhow::bail!(
                        "Unknown CI system: {}. Use 'gitlab', 'github', or 'forgejo'",
                        ci_name
                    ),
                }
            } else {
                // Auto-detect
                let systems = detect_ci_systems(&path);
                if systems.is_empty() {
                    anyhow::bail!("No CI systems detected. Use --ci to specify manually.");
                }
                if systems.len() > 1 {
                    anyhow::bail!(
                        "Multiple CI systems detected. Please specify which one to use with --ci"
                    );
                }
                systems.into_iter().next().unwrap()
            };

            println!(
                "{} {}",
                "Listing jobs for".cyan().bold(),
                ci_system.to_string().cyan().bold()
            );

            match ci_system {
                CISystem::GitLab => gitlab::list_jobs(&path)?,
                CISystem::GitHub => github::list_jobs(&path)?,
                CISystem::Forgejo => forgejo::list_jobs(&path)?,
            }
        }
        Commands::Run { job, path, ci } => {
            let ci_system = if let Some(ci_name) = ci {
                match ci_name.to_lowercase().as_str() {
                    "gitlab" => CISystem::GitLab,
                    "github" => CISystem::GitHub,
                    "forgejo" | "gitea" => CISystem::Forgejo,
                    _ => anyhow::bail!(
                        "Unknown CI system: {}. Use 'gitlab', 'github', or 'forgejo'",
                        ci_name
                    ),
                }
            } else {
                // Auto-detect
                let systems = detect_ci_systems(&path);
                if systems.is_empty() {
                    anyhow::bail!("No CI systems detected. Use --ci to specify manually.");
                }
                if systems.len() > 1 {
                    anyhow::bail!(
                        "Multiple CI systems detected. Please specify which one to use with --ci"
                    );
                }
                systems.into_iter().next().unwrap()
            };

            println!(
                "{} {} {} {}",
                "Running job".cyan().bold(),
                job.yellow(),
                "on".cyan().bold(),
                ci_system.to_string().cyan().bold()
            );

            match ci_system {
                CISystem::GitLab => gitlab::run_job(&path, &job).await?,
                CISystem::GitHub => github::run_job(&path, &job).await?,
                CISystem::Forgejo => forgejo::run_job(&path, &job).await?,
            }
        }
    }

    Ok(())
}
