use console::style;
use dialoguer::{Confirm, MultiSelect, Select};
use std::fs;

use crate::agent::{detect_agents, Agent, InstallTarget};
use crate::error::RalphResult;
use crate::templates::{get_prd_skill_content, get_ralph_skill_content};

/// Run the interactive skill installation
pub fn run_install() -> RalphResult<()> {
    println!("{}", style("Ralph Skill Installation").bold().cyan());
    println!("{}", style("========================").cyan());
    println!();

    // Step 1: Detect available agents
    let detected_agents = detect_agents();
    if detected_agents.is_empty() {
        println!("{}", style("No AI Agent CLIs detected!").yellow());
        println!("Please install Amp, Claude Code, or CodeBuddy first.");
        return Ok(());
    }

    // Step 2: Interactive selection of target agents
    let selected_agents = select_agents(&detected_agents)?;
    if selected_agents.is_empty() {
        println!("No agents selected. Exiting.");
        return Ok(());
    }

    // Step 3: Select installation location
    let install_target = select_install_location(&selected_agents)?;

    // Step 4: Install skills
    install_skills(&selected_agents, &install_target)?;

    // Step 5: Display success message
    display_success_message(&selected_agents, &install_target);

    Ok(())
}

/// Interactive multi-select for agents
fn select_agents(detected_agents: &[Agent]) -> RalphResult<Vec<Agent>> {
    println!("{}", style("Select target agents:").bold());

    let agent_names: Vec<String> = detected_agents.iter().map(|a| a.name().to_string()).collect();

    let defaults = vec![true; detected_agents.len()];
    let selections = MultiSelect::new()
        .with_prompt("Select agents (space to toggle, enter to confirm)")
        .items(&agent_names)
        .defaults(&defaults)
        .interact()?;

    let selected: Vec<Agent> = selections
        .into_iter()
        .map(|idx| detected_agents[idx])
        .collect();

    println!();
    Ok(selected)
}

/// Select installation location (global only)
fn select_install_location(selected_agents: &[Agent]) -> RalphResult<InstallTarget> {
    println!("{}", style("Select installation location:").bold());

    // Build list of global options for selected agents
    let mut options = vec![];
    for agent in selected_agents {
        if agent.global_skills_dir().is_some() {
            options.push(InstallTarget::AgentGlobal(*agent));
        }
    }

    let display_names: Vec<String> = options.iter().map(|o| o.display_name()).collect();

    let selection = Select::new()
        .with_prompt("Choose installation location")
        .items(&display_names)
        .default(0)
        .interact()?;

    println!();
    Ok(options[selection].clone())
}

/// Install skills to the selected location
pub fn install_skills(_agents: &[Agent], target: &InstallTarget) -> RalphResult<()> {
    // Get embedded skill content
    let prd_skill = get_prd_skill_content();
    let ralph_skill = get_ralph_skill_content();

    let InstallTarget::AgentGlobal(agent) = target;

    // Global install: create ralph/ subdirectory and install SKILL.md files
    let skills_dir = target.path();
    let ralph_dir = skills_dir.join("ralph");
    let prd_dir = skills_dir.join("prd");

    println!("{}", style("Installing skills...").bold());
    println!("Target directory: {}", ralph_dir.display());
    println!();

    // Install ralph.md (main skill file)
    fs::create_dir_all(&ralph_dir)?;
    let ralph_file = ralph_dir.join("SKILL.md");
    install_skill_file(&ralph_file, &ralph_skill, "ralph/SKILL.md")?;

    // Install prd.md (PRD creation skill)
    fs::create_dir_all(&prd_dir)?;
    let prd_file = prd_dir.join("SKILL.md");
    install_skill_file(&prd_file, &prd_skill, "prd/SKILL.md")?;

    println!(
        "  {} Installed skills globally for {}",
        style("✓").green(),
        agent.name()
    );

    println!();
    Ok(())
}

/// Helper function to install a single skill file with overwrite confirmation
fn install_skill_file(file_path: &std::path::Path, content: &str, display_name: &str) -> RalphResult<()> {
    if file_path.exists() {
        let should_overwrite = Confirm::new()
            .with_prompt(format!(
                "Skill file {} already exists. Overwrite?",
                file_path.display()
            ))
            .default(false)
            .interact()?;

        if should_overwrite {
            fs::write(file_path, content)?;
            println!("  {} Installed {}", style("✓").green(), display_name);
        } else {
            println!("  Skipping {}", display_name);
        }
    } else {
        fs::write(file_path, content)?;
        println!("  {} Installed {}", style("✓").green(), display_name);
    }
    Ok(())
}

/// Display success message and next steps
fn display_success_message(agents: &[Agent], target: &InstallTarget) {
    println!("{}", style("========================").green());
    println!("{}", style("Installation Complete!").bold().green());
    println!("{}", style("========================").green());
    println!();

    println!("{}", style("Installed agents:").bold());
    for agent in agents {
        println!("  {} {}", style("✓").green(), agent.name());
    }
    println!();

    println!("{}", style("Installation location:").bold());
    let skills_dir = target.path();
    let ralph_dir = skills_dir.join("ralph");
    println!("  {}", ralph_dir.display());
    println!();

    println!("{}", style("Next steps:").bold());
    println!("  1. Your skills are installed globally for the selected agents.");
    println!("  2. The skills will be available in all your projects.");
    println!(
        "  3. Run {} to start a new Ralph project.",
        style("ralph init").cyan()
    );
    println!();
}
