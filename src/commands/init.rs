use console::style;
use dialoguer::Select;
use std::fs;
use std::path::PathBuf;

use crate::agent::{detect_agents, Agent};
use crate::error::RalphResult;

/// Run the interactive project initialization
pub fn run_init() -> RalphResult<()> {
    println!("{}", style("Ralph Project Initialization").bold().cyan());
    println!("{}", style("============================").cyan());
    println!();

    // Step 1: Detect installed agents and select default AI tool
    let detected_agents = detect_agents();
    let default_tool = if detected_agents.is_empty() {
        println!();
        println!("{}", style("Warning: No AI Agent CLIs detected!").yellow());
        println!(
            "You can configure the default tool later using: ralph config --set default_tool <tool>"
        );
        None
    } else {
        println!();
        println!("{}", style("Select default AI tool:").bold());

        let agent_names: Vec<String> =
            detected_agents.iter().map(|a| a.name().to_string()).collect();
        let selection = Select::new()
            .with_prompt("Choose your default AI tool")
            .items(&agent_names)
            .default(0)
            .interact()?;

        Some(detected_agents[selection])
    };

    println!();

    // Step 4: Create directory structure
    println!("{}", style("Creating directory structure...").bold());

    let ralph_dir = PathBuf::from("ralph");
    let tasks_dir = ralph_dir.join("tasks");

    // Create ralph/ directory (main workspace for Ralph files)
    fs::create_dir_all(&ralph_dir)?;
    println!("  {} Created {}", style("✓").green(), ralph_dir.display());

    fs::create_dir_all(&tasks_dir)?;
    println!("  {} Created {}", style("✓").green(), tasks_dir.display());

    println!();

    // Step 6: Display next steps guide
    display_init_next_steps(default_tool);

    Ok(())
}

/// Display next steps guide after initialization
fn display_init_next_steps(default_tool: Option<Agent>) {
    println!("{}", style("============================").green());
    println!("{}", style("Initialization Complete!").bold().green());
    println!("{}", style("============================").green());
    println!();

    println!("{}", style("Next steps:").bold());
    println!();

    if let Some(agent) = default_tool {
        println!("{}", style("1. Create your PRD:").bold());
        match agent {
            Agent::CodeBuddy => {
                println!(
                    "   - Use the {} skill in CodeBuddy to generate it",
                    style("/prd").cyan()
                );
            }
            Agent::Claude => {
                println!("   - Use Claude Code to help create your PRD");
            }
            Agent::Amp => {
                println!("   - Use Amp to help create your PRD");
            }
        }
        println!("   - Place the generated PRD file in the {} directory", style("ralph/").cyan());
        println!();
    }

    println!("{}", style("2. Start working:").bold());
    println!(
        "   - Run {} to start the Ralph agent",
        style("ralph run").cyan()
    );
    println!(
        "   - Run {} to check project status",
        style("ralph status").cyan()
    );
    println!();

    println!("{}", style("Happy coding!").italic());
}
