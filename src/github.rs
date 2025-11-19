use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::container;

/// Represents a single matrix combination
#[derive(Debug, Clone)]
struct MatrixCombination {
    values: HashMap<String, String>,
}

impl MatrixCombination {
    /// Interpolate matrix values into a string (e.g., "${{ matrix.os }}" -> "ubuntu-latest")
    fn interpolate(&self, input: &str) -> String {
        let mut result = input.to_string();
        for (key, value) in &self.values {
            let pattern = format!("${{{{ matrix.{} }}}}", key);
            result = result.replace(&pattern, value);
        }
        result
    }
}

/// Expand matrix configuration into all possible combinations
fn expand_matrix(matrix: &serde_yaml::Value) -> Result<Vec<MatrixCombination>> {
    let matrix_obj = matrix
        .as_mapping()
        .context("Matrix must be an object/mapping")?;

    let mut keys = Vec::new();
    let mut values_per_key: Vec<Vec<String>> = Vec::new();

    // Extract matrix dimensions
    for (key, value) in matrix_obj.iter() {
        let key_str = key
            .as_str()
            .context("Matrix key must be a string")?
            .to_string();

        // Handle both array and single value
        let vals = if let Some(arr) = value.as_sequence() {
            arr.iter()
                .map(|v| {
                    if let Some(s) = v.as_str() {
                        Ok(s.to_string())
                    } else if let Some(n) = v.as_i64() {
                        Ok(n.to_string())
                    } else if let Some(f) = v.as_f64() {
                        Ok(f.to_string())
                    } else if let Some(b) = v.as_bool() {
                        Ok(if b { "true" } else { "false" }.to_string())
                    } else {
                        anyhow::bail!("Matrix values must be strings, numbers, or booleans")
                    }
                })
                .collect::<Result<Vec<String>>>()?
        } else if let Some(s) = value.as_str() {
            vec![s.to_string()]
        } else if let Some(n) = value.as_i64() {
            vec![n.to_string()]
        } else if let Some(f) = value.as_f64() {
            vec![f.to_string()]
        } else if let Some(b) = value.as_bool() {
            vec![if b { "true" } else { "false" }.to_string()]
        } else {
            anyhow::bail!("Matrix values must be arrays, strings, numbers, or booleans");
        };

        keys.push(key_str);
        values_per_key.push(vals);
    }

    // Generate all combinations using cartesian product
    let mut combinations = vec![MatrixCombination {
        values: HashMap::new(),
    }];

    for (key, values) in keys.iter().zip(values_per_key.iter()) {
        let mut new_combinations = Vec::new();
        for combo in &combinations {
            for value in values {
                let mut new_combo = combo.clone();
                new_combo.values.insert(key.clone(), value.clone());
                new_combinations.push(new_combo);
            }
        }
        combinations = new_combinations;
    }

    Ok(combinations)
}

#[derive(Debug, Deserialize, Serialize)]
struct GitHubWorkflow {
    name: Option<String>,
    on: Option<serde_yaml::Value>,
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Job {
    #[serde(rename = "runs-on")]
    runs_on: Option<serde_yaml::Value>,
    steps: Option<Vec<Step>>,
    strategy: Option<Strategy>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Strategy {
    matrix: Option<serde_yaml::Value>,
    #[serde(rename = "fail-fast")]
    fail_fast: Option<bool>,
    #[serde(rename = "max-parallel")]
    max_parallel: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Step {
    name: Option<String>,
    run: Option<String>,
    uses: Option<String>,
    with: Option<std::collections::HashMap<String, serde_yaml::Value>>,
}

pub fn get_jobs_from_file(pipeline_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let workflow: GitHubWorkflow = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    let jobs: Vec<String> = workflow.jobs.keys().cloned().collect();
    Ok(jobs)
}

#[allow(dead_code)]
pub fn list_jobs(path: &Path) -> Result<()> {
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
                let steps_count = job.steps.as_ref().map(|s| s.len()).unwrap_or(0);

                // Check if job has a matrix strategy
                if let Some(strategy) = &job.strategy {
                    if let Some(matrix) = &strategy.matrix {
                        match expand_matrix(matrix) {
                            Ok(combinations) => {
                                println!(
                                    "    {} {} {} matrix combinations]",
                                    "•".cyan(),
                                    job_name.yellow(),
                                    format!("[{}", combinations.len()).blue()
                                );
                                for combo in combinations.iter().take(3) {
                                    let combo_str = combo
                                        .values
                                        .iter()
                                        .map(|(k, v)| format!("{}={}", k, v))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    println!("      {} {}", "↳".dimmed(), combo_str.dimmed());
                                }
                                if combinations.len() > 3 {
                                    println!(
                                        "      {} and {} more...",
                                        "↳".dimmed(),
                                        (combinations.len() - 3).to_string().dimmed()
                                    );
                                }
                            }
                            Err(_) => {
                                println!(
                                    "    {} {} [matrix: invalid, steps: {}]",
                                    "•".cyan(),
                                    job_name.yellow(),
                                    steps_count.to_string().dimmed()
                                );
                            }
                        }
                        continue;
                    }
                }

                // Non-matrix job
                let runs_on = job
                    .runs_on
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .unwrap_or("N/A");

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

/// Execute a job with a specific matrix combination
async fn execute_job_with_matrix(
    job: &Job,
    matrix_combo: &MatrixCombination,
    runs_on: &str,
) -> Result<()> {
    if let Some(steps) = &job.steps {
        println!("\n{}", "Steps:".cyan());

        if steps.is_empty() {
            println!("\n{}", "No executable steps found in this job".yellow());
            return Ok(());
        }

        println!("\n{}", "─".repeat(60).dimmed());

        // Detect container runtime
        let runtime = container::detect_runtime().await;

        // Map runner to image
        let image = container::map_runner_to_image(runs_on);

        if let Some(img) = image {
            println!("  Mapped {} → {}", runs_on.blue(), img.yellow());
        } else if runs_on.starts_with("macos") || runs_on.starts_with("windows") {
            println!(
                "\n{} {} runners not supported in containers, using host execution",
                "⚠".yellow(),
                runs_on
            );
        }

        // Execute all steps in order
        for (i, step) in steps.iter().enumerate() {
            let default_name = format!("Step {}", i + 1);
            let step_name = step.name.as_deref().unwrap_or(&default_name);

            // Interpolate matrix values in step name
            let interpolated_name = matrix_combo.interpolate(step_name);

            println!(
                "\n{} {}",
                format!("[{}/{}]", i + 1, steps.len()).cyan(),
                interpolated_name.yellow()
            );

            if let Some(run) = &step.run {
                // Interpolate matrix values in run command
                let interpolated_run = matrix_combo.interpolate(run);

                // Execute run step
                container::execute_steps(
                    runtime.as_ref(),
                    image,
                    std::slice::from_ref(&interpolated_run),
                )
                .await?;
            } else if let Some(uses) = &step.uses {
                // Interpolate matrix values in action reference
                let interpolated_uses = matrix_combo.interpolate(uses);

                // Execute action step
                use crate::actions;
                actions::execute_action(&interpolated_uses, step.with.as_ref()).await?;
            }
        }

        println!("\n{}", "─".repeat(60).dimmed());
    } else {
        println!("\n{}", "No steps defined in this job".yellow());
    }

    Ok(())
}

pub async fn run_job_from_file(
    pipeline_path: &PathBuf,
    job_name: &str,
    non_interactive: bool,
) -> Result<()> {
    let content = std::fs::read_to_string(pipeline_path)
        .context(format!("Failed to read {}", pipeline_path.display()))?;

    let workflow: GitHubWorkflow = serde_yaml::from_str(&content)
        .context(format!("Failed to parse {}", pipeline_path.display()))?;

    if let Some(job) = workflow.jobs.get(job_name) {
        // Check if job has a matrix strategy
        if let Some(strategy) = &job.strategy {
            if let Some(matrix) = &strategy.matrix {
                let combinations = expand_matrix(matrix)?;

                println!(
                    "\n{}",
                    format!(
                        "▶ Executing job: {} ({} matrix combinations)",
                        job_name,
                        combinations.len()
                    )
                    .green()
                    .bold()
                );

                // Show all combinations
                for (idx, combo) in combinations.iter().enumerate() {
                    let combo_str = combo
                        .values
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("  {} [{}] {}", "•".cyan(), idx + 1, combo_str.yellow());
                }

                // Ask for confirmation
                let confirm = if non_interactive {
                    true
                } else {
                    inquire::Confirm::new(&format!(
                        "Execute all {} matrix combinations?",
                        combinations.len()
                    ))
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false)
                };

                if !confirm {
                    println!("{}", "Execution cancelled".yellow());
                    return Ok(());
                }

                let max_parallel = strategy.max_parallel.unwrap_or(combinations.len() as u32);
                let fail_fast = strategy.fail_fast.unwrap_or(true);

                println!(
                    "\n{} max-parallel: {}, fail-fast: {}",
                    "Strategy:".cyan(),
                    max_parallel.to_string().blue(),
                    fail_fast.to_string().blue()
                );

                // Execute each matrix combination
                for (idx, combo) in combinations.iter().enumerate() {
                    println!("\n{}", "═".repeat(60).dimmed());
                    println!(
                        "{} Matrix combination [{}/{}]",
                        "▶".green().bold(),
                        idx + 1,
                        combinations.len()
                    );

                    let combo_str = combo
                        .values
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("  {}", combo_str.yellow());

                    // Interpolate runs_on with matrix values
                    let runs_on_str = if let Some(runs_on_val) = &job.runs_on {
                        if let Some(s) = runs_on_val.as_str() {
                            combo.interpolate(s)
                        } else {
                            "ubuntu-latest".to_string()
                        }
                    } else {
                        "ubuntu-latest".to_string()
                    };

                    println!("  Runs on: {}", runs_on_str.blue());

                    // Execute the job with this matrix combination
                    let result = execute_job_with_matrix(job, combo, &runs_on_str).await;

                    if let Err(e) = result {
                        eprintln!(
                            "\n{} Matrix combination [{}/{}] failed: {}",
                            "✗".red().bold(),
                            idx + 1,
                            combinations.len(),
                            e
                        );

                        if fail_fast {
                            anyhow::bail!("Job failed with fail-fast enabled");
                        }
                    } else {
                        println!(
                            "\n{} Matrix combination [{}/{}] succeeded",
                            "✓".green().bold(),
                            idx + 1,
                            combinations.len()
                        );
                    }
                }

                println!("\n{}", "═".repeat(60).dimmed());
                println!(
                    "\n{}",
                    format!("✓ Job '{}' completed all matrix combinations", job_name)
                        .green()
                        .bold()
                );

                return Ok(());
            }
        }

        // Non-matrix job execution
        println!(
            "\n{}",
            format!("▶ Executing job: {}", job_name).green().bold()
        );

        let runs_on = job
            .runs_on
            .as_ref()
            .and_then(|v| v.as_str())
            .unwrap_or("ubuntu-latest");
        println!("  Runs on: {}", runs_on.blue());

        if let Some(steps) = &job.steps {
            println!("\n{}", "Steps:".cyan());
            for (i, step) in steps.iter().enumerate() {
                if let Some(_run) = &step.run {
                    let step_name = step.name.as_deref().unwrap_or("(unnamed)");
                    println!("  {}. {} {}", i + 1, "Run:".green(), step_name);
                } else if let Some(uses) = &step.uses {
                    let step_name = step.name.as_deref().unwrap_or("(unnamed)");
                    println!(
                        "  {}. {} {} - {}",
                        i + 1,
                        "Uses:".blue(),
                        uses.dimmed(),
                        step_name
                    );
                }
            }

            if steps.is_empty() {
                println!("\n{}", "No executable steps found in this job".yellow());
                return Ok(());
            }

            // Ask for confirmation
            let confirm = if non_interactive {
                true
            } else {
                inquire::Confirm::new("Execute these steps?")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false)
            };

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

            // Execute all steps in order
            for (i, step) in steps.iter().enumerate() {
                let default_name = format!("Step {}", i + 1);
                let step_name = step.name.as_deref().unwrap_or(&default_name);
                println!(
                    "\n{} {}",
                    format!("[{}/{}]", i + 1, steps.len()).cyan(),
                    step_name.yellow()
                );

                if let Some(run) = &step.run {
                    // Execute run step
                    container::execute_steps(runtime.as_ref(), image, std::slice::from_ref(run))
                        .await?;
                } else if let Some(uses) = &step.uses {
                    // Execute action step
                    use crate::actions;
                    actions::execute_action(uses, step.with.as_ref()).await?;
                }
            }

            println!("\n{}", "─".repeat(60).dimmed());
            println!(
                "\n{}",
                format!("✓ Job '{}' completed successfully", job_name)
                    .green()
                    .bold()
            );

            return Ok(());
        } else {
            println!("\n{}", "No steps defined in this job".yellow());
            return Ok(());
        }
    }

    anyhow::bail!("Job '{}' not found", job_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_expansion_simple() {
        let yaml = r#"
os: [ubuntu-latest, macos-latest, windows-latest]
"#;
        let matrix: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let combinations = expand_matrix(&matrix).unwrap();

        assert_eq!(combinations.len(), 3);
        assert_eq!(combinations[0].values.get("os").unwrap(), "ubuntu-latest");
        assert_eq!(combinations[1].values.get("os").unwrap(), "macos-latest");
        assert_eq!(combinations[2].values.get("os").unwrap(), "windows-latest");
    }

    #[test]
    fn test_matrix_expansion_multi_dimension() {
        let yaml = r#"
os: [ubuntu-latest, macos-latest]
rust: [stable, nightly]
"#;
        let matrix: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let combinations = expand_matrix(&matrix).unwrap();

        assert_eq!(combinations.len(), 4);

        // Verify all combinations exist
        let has_ubuntu_stable = combinations.iter().any(|c| {
            c.values.get("os") == Some(&"ubuntu-latest".to_string())
                && c.values.get("rust") == Some(&"stable".to_string())
        });
        let has_ubuntu_nightly = combinations.iter().any(|c| {
            c.values.get("os") == Some(&"ubuntu-latest".to_string())
                && c.values.get("rust") == Some(&"nightly".to_string())
        });
        let has_macos_stable = combinations.iter().any(|c| {
            c.values.get("os") == Some(&"macos-latest".to_string())
                && c.values.get("rust") == Some(&"stable".to_string())
        });
        let has_macos_nightly = combinations.iter().any(|c| {
            c.values.get("os") == Some(&"macos-latest".to_string())
                && c.values.get("rust") == Some(&"nightly".to_string())
        });

        assert!(has_ubuntu_stable);
        assert!(has_ubuntu_nightly);
        assert!(has_macos_stable);
        assert!(has_macos_nightly);
    }

    #[test]
    fn test_matrix_interpolation() {
        let mut combo = MatrixCombination {
            values: HashMap::new(),
        };
        combo
            .values
            .insert("os".to_string(), "ubuntu-latest".to_string());
        combo
            .values
            .insert("version".to_string(), "3.9".to_string());

        let result =
            combo.interpolate("Run tests on ${{ matrix.os }} with Python ${{ matrix.version }}");
        assert_eq!(result, "Run tests on ubuntu-latest with Python 3.9");
    }

    #[test]
    fn test_matrix_expansion_with_numbers() {
        let yaml = r#"
version: ["3.8", "3.9", "3.10"]
node: [16, 18, 20]
"#;
        let matrix: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let combinations = expand_matrix(&matrix).unwrap();

        assert_eq!(combinations.len(), 9); // 3 x 3 = 9 combinations

        // Test string versions
        assert!(combinations
            .iter()
            .any(|c| c.values.get("version") == Some(&"3.8".to_string())));
        assert!(combinations
            .iter()
            .any(|c| c.values.get("version") == Some(&"3.9".to_string())));
        assert!(combinations
            .iter()
            .any(|c| c.values.get("version") == Some(&"3.10".to_string())));

        // Test integer versions (should be converted to strings)
        assert!(combinations
            .iter()
            .any(|c| c.values.get("node") == Some(&"16".to_string())));
        assert!(combinations
            .iter()
            .any(|c| c.values.get("node") == Some(&"18".to_string())));
        assert!(combinations
            .iter()
            .any(|c| c.values.get("node") == Some(&"20".to_string())));
    }

    #[test]
    fn test_matrix_interpolation_runs_on() {
        let mut combo = MatrixCombination {
            values: HashMap::new(),
        };
        combo
            .values
            .insert("os".to_string(), "ubuntu-latest".to_string());

        let result = combo.interpolate("${{ matrix.os }}");
        assert_eq!(result, "ubuntu-latest");
    }
}
