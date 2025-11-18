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
        if Command::new("claude")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some(AgentCli::Claude);
        }

        // Check for 'codex' CLI
        if Command::new("codex")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some(AgentCli::Codex);
        }

        None
    }

    /// Get the command name for this CLI
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
            .context("No agent CLI found. Please install 'claude' or 'codex' CLI.")?;
        Ok(Self { cli })
    }

    /// Create a client with a specific CLI
    pub fn with_cli(cli: AgentCli) -> Self {
        Self { cli }
    }

    /// Execute a task using the agent
    pub async fn execute(&self, task: AgentTask) -> Result<AgentResponse> {
        let task_json = serde_json::to_string(&task)
            .context("Failed to serialize task")?;

        let output = tokio::process::Command::new(self.cli.command_name())
            .arg("task")
            .arg("--json")
            .arg(&task_json)
            .output()
            .await
            .context(format!("Failed to execute {} CLI", self.cli.command_name()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "Agent task failed: {}",
                stderr
            );
        }

        let response: AgentResponse = serde_json::from_slice(&output.stdout)
            .context("Failed to parse agent response")?;

        Ok(response)
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
            "Migrate the following {} CI configuration to {}. \
            Analyze the source configuration, research documentation for both CI systems, \
            and generate an equivalent configuration for the target system.\n\n\
            Source Configuration:\n{}\n\n\
            Git Context:\n{}",
            source_ci, target_ci, source_config, git_context
        );

        let task = AgentTask {
            prompt,
            context: Some(format!(
                "You are migrating from {} to {}. \
                Focus on finding equivalent features and preserving the intent of the original pipeline. \
                If a feature doesn't have a direct equivalent, suggest the closest alternative.",
                source_ci, target_ci
            )),
        };

        let response = self.execute(task).await?;

        if !response.success {
            anyhow::bail!(
                "Migration failed: {}",
                response.error.unwrap_or_else(|| "Unknown error".to_string())
            );
        }

        Ok(response.output)
    }

    /// Research CI system documentation
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
                response.error.unwrap_or_else(|| "Unknown error".to_string())
            );
        }

        Ok(response.output)
    }

    /// Validate and fix a generated configuration
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
                response.error.unwrap_or_else(|| "Unknown error".to_string())
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
