use std::path::PathBuf;
use std::process::Command;

/// Represents an AI Agent CLI that can be detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    Amp,
    Claude,
    CodeBuddy,
}

impl Agent {
    pub fn name(&self) -> &'static str {
        match self {
            Agent::Amp => "Amp",
            Agent::Claude => "Claude Code",
            Agent::CodeBuddy => "CodeBuddy",
        }
    }

    pub fn command(&self) -> &'static str {
        match self {
            Agent::Amp => "amp",
            Agent::Claude => "claude",
            Agent::CodeBuddy => "codebuddy",
        }
    }

    /// Returns the global skills directory for this agent
    pub fn global_skills_dir(&self) -> Option<PathBuf> {
        match self {
            Agent::Amp => dirs::config_dir().map(|d| d.join("amp/skills")),
            Agent::Claude => dirs::home_dir().map(|d| d.join(".claude/skills")),
            Agent::CodeBuddy => dirs::home_dir().map(|d| d.join(".codebuddy/skills")),
        }
    }

}

/// Installation target location
#[derive(Debug, Clone)]
pub enum InstallTarget {
    AgentGlobal(Agent),
}

impl InstallTarget {
    pub fn display_name(&self) -> String {
        match self {
            InstallTarget::AgentGlobal(agent) => format!("{} Global", agent.name()),
        }
    }

    pub fn path(&self) -> PathBuf {
        match self {
            InstallTarget::AgentGlobal(agent) => {
                agent.global_skills_dir().expect("Could not determine global config directory")
            }
        }
    }
}

/// Detects which agent CLIs are available in PATH
pub fn detect_agents() -> Vec<Agent> {
    let agents = vec![Agent::Amp, Agent::Claude, Agent::CodeBuddy];
    agents
        .into_iter()
        .filter(|agent| is_command_available(agent.command()))
        .collect()
}

/// Check if a command is available in PATH
pub fn is_command_available(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}
