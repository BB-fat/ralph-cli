use clap::{Parser, Subcommand};

/// Ralph CLI - AI Agent aggregation tool
///
/// Provides interactive skill installation, guided project initialization,
/// and task launch experience for AI agents like Amp, Claude, and CodeBuddy.
#[derive(Parser)]
#[command(name = "ralph")]
#[command(about = "Ralph CLI - AI Agent aggregation tool")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Ralph project
    Init,
    /// Install skills to agents
    Install,
    /// Run Ralph tasks
    Run {
        /// AI tool to use (amp/claude/codebuddy/auto)
        #[arg(long, default_value = "auto")]
        tool: String,
        /// Maximum iterations (default: 10)
        #[arg(long)]
        max_iterations: Option<u32>,
        /// Path to prd.json file
        #[arg(long, default_value = "./ralph/prd.json")]
        prd: String,
    },
    /// View or set configuration
    Config {
        /// Get a specific config value
        #[arg(long)]
        get: Option<String>,
        /// Set a config value (requires key and value)
        #[arg(long, num_args = 2, value_names = ["KEY", "VALUE"])]
        set: Vec<String>,
    },
    /// View project status
    Status,
    /// Manage archives
    Archive,
    /// Detect installed agent CLIs
    Detect,
}
