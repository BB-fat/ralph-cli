use console::style;

use crate::agent::{detect_agents, Agent};

/// Run the detect command to show installed agents
pub fn run_detect() {
    println!("Detecting installed AI Agent CLIs...\n");

    let detected = detect_agents();

    println!("Installed Agents:");
    println!("-----------------");

    let all_agents = vec![Agent::Amp, Agent::Claude, Agent::CodeBuddy];
    let mut found_count = 0;

    for agent in &all_agents {
        let is_installed = detected.contains(agent);
        let status = if is_installed {
            found_count += 1;
            style("✓ Installed").green()
        } else {
            style("✗ Not found").red()
        };
        println!("  {}: {}", agent.name(), status);
    }

    println!("-----------------");
    println!(
        "Total: {}/{} agents installed",
        found_count,
        all_agents.len()
    );
}
