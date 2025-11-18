use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Represents the type of agent CLI available
#[derive(Debug, Clone, Copy)]
pub enum AgentCli {
    Claude,
    Codex,
}

impl AgentCli {
    /// Auto-detect which agent CLI is available on the system
    pub fn detect() -> Option<Self> {
        // Check for 'claude' CLI first
        if Command::new("claude").arg("--version").output().is_ok() {
            return Some(AgentCli::Claude);
        }

        // Check for 'codex' CLI
        if Command::new("codex").arg("--version").output().is_ok() {
            return Some(AgentCli::Codex);
        }

        None
    }

    /// Get the command name for this CLI
    #[allow(dead_code)]
    fn command_name(&self) -> &str {
        match self {
            AgentCli::Claude => "claude",
            AgentCli::Codex => "codex",
        }
    }
}

/// Agent task request following MCP protocol
#[derive(Debug, Serialize)]
pub struct AgentTask {
    pub prompt: String,
    pub context: Option<String>,
}

/// Agent task response
#[derive(Debug, Deserialize)]
pub struct AgentResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Agent client for delegating tasks
pub struct AgentClient {
    cli: AgentCli,
}

impl AgentClient {
    /// Create a new agent client with auto-detection
    pub fn new() -> Result<Self> {
        let cli = AgentCli::detect()
            .context(
                "No agent CLI found. The migration feature requires either 'claude' or 'codex' CLI to be installed.\n\n\
                Install options:\n\
                  - Claude CLI: https://github.com/anthropics/claude-code\n\
                  - Codex CLI: https://zed.dev/docs/assistant/commands\n\n\
                Alternatively, you can manually migrate your CI configuration by consulting the documentation for both systems."
            )?;
        Ok(Self { cli })
    }

    /// Create a client with a specific CLI
    #[allow(dead_code)]
    pub fn with_cli(cli: AgentCli) -> Self {
        Self { cli }
    }

    /// Execute a task using the agent
    pub async fn execute(&self, task: AgentTask) -> Result<AgentResponse> {
        // Build the full prompt with context if provided
        let full_prompt = if let Some(context) = &task.context {
            format!("{}\n\n{}", context, task.prompt)
        } else {
            task.prompt.clone()
        };

        let output = match self.cli {
            AgentCli::Claude => {
                // Use Claude CLI with --print for non-interactive mode
                tokio::process::Command::new("claude")
                    .arg("--print")
                    .arg(&full_prompt)
                    .output()
                    .await
                    .context("Failed to execute claude CLI")?
            }
            AgentCli::Codex => {
                // Use Codex CLI (assuming similar interface)
                tokio::process::Command::new("codex")
                    .arg("--print")
                    .arg(&full_prompt)
                    .output()
                    .await
                    .context("Failed to execute codex CLI")?
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Agent task failed: {}", stderr);
        }

        let output_text = String::from_utf8_lossy(&output.stdout).to_string();

        // Return a simplified response
        Ok(AgentResponse {
            success: true,
            output: output_text,
            error: None,
        })
    }

    /// Execute a migration task with full context
    pub async fn migrate_pipeline(
        &self,
        source_config: &str,
        source_ci: &str,
        target_ci: &str,
        git_context: &str,
    ) -> Result<String> {
        let prompt = format!(
            "You are a CI/CD migration expert. Migrate the following {} CI configuration to {}.\n\n\
            IMPORTANT: Your response must contain ONLY the complete {} configuration file in YAML format. \
            Do not include any explanations, markdown code blocks, or additional text - just the raw YAML.\n\n\
            Requirements:\n\
            - Preserve all workflows, jobs, and steps from the source configuration\n\
            - Map environment variables correctly\n\
            - Convert caching mechanisms to {} equivalents\n\
            - Maintain the same execution order and dependencies\n\
            - Use appropriate {} syntax and best practices\n\n\
            Source {} Configuration:\n\
            ```yaml\n{}\n```\n\n\
            Git Context:\n{}\n\n\
            Output ONLY the {} YAML configuration:",
            source_ci, target_ci, target_ci, target_ci, target_ci, source_ci, source_config, git_context, target_ci
        );

        let task = AgentTask {
            prompt,
            context: None, // Context is now in the main prompt
        };

        let response = self.execute(task).await?;

        if !response.success {
            anyhow::bail!(
                "Migration failed: {}",
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            );
        }

        // Clean up the response - remove markdown code blocks if present
        let cleaned = response.output.trim();
        let cleaned = if cleaned.starts_with("```yaml") || cleaned.starts_with("```") {
            // Remove markdown code fences
            cleaned
                .lines()
                .skip(1) // Skip opening ```
                .take_while(|line| !line.starts_with("```")) // Stop at closing ```
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            cleaned.to_string()
        };

        Ok(cleaned)
    }

    /// Research CI system documentation
    #[allow(dead_code)]
    pub async fn research_docs(&self, ci_system: &str, feature: &str) -> Result<String> {
        let prompt = format!(
            "Research {} documentation for the following feature: {}. \
            Provide a concise summary of how this feature works, its syntax, and any important considerations.",
            ci_system, feature
        );

        let task = AgentTask {
            prompt,
            context: Some(format!(
                "You are researching {} documentation to help with CI pipeline migration.",
                ci_system
            )),
        };

        let response = self.execute(task).await?;

        if !response.success {
            anyhow::bail!(
                "Documentation research failed: {}",
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            );
        }

        Ok(response.output)
    }

    /// Validate and fix a generated configuration
    #[allow(dead_code)]
    pub async fn validate_and_fix(
        &self,
        config: &str,
        target_ci: &str,
        error_message: Option<&str>,
    ) -> Result<String> {
        let prompt = if let Some(error) = error_message {
            format!(
                "The following {} configuration failed validation with this error:\n{}\n\n\
                Configuration:\n{}\n\n\
                Please analyze the error and provide a corrected version of the configuration.",
                target_ci, error, config
            )
        } else {
            format!(
                "Validate the following {} configuration and fix any issues you find:\n\n{}",
                target_ci, config
            )
        };

        let task = AgentTask {
            prompt,
            context: Some(format!(
                "You are validating and fixing a {} CI configuration. \
                Ensure it follows best practices and is syntactically correct.",
                target_ci
            )),
        };

        let response = self.execute(task).await?;

        if !response.success {
            anyhow::bail!(
                "Validation failed: {}",
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            );
        }

        Ok(response.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_cli_detection() {
        // This test will pass if either CLI is installed, or skip if none are
        if let Some(cli) = AgentCli::detect() {
            match cli {
                AgentCli::Claude => assert_eq!(cli.command_name(), "claude"),
                AgentCli::Codex => assert_eq!(cli.command_name(), "codex"),
            }
        }
    }

    #[test]
    fn test_agent_task_serialization() {
        let task = AgentTask {
            prompt: "Test prompt".to_string(),
            context: Some("Test context".to_string()),
        };

        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("Test prompt"));
        assert!(json.contains("Test context"));
    }
}
