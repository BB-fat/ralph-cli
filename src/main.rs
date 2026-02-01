use clap::Parser;
use console::style;

mod agent;
mod cli;
mod commands;
mod config;
mod error;
mod prd;
mod templates;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            if let Err(e) = commands::init::run_init() {
                eprintln!("{} {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Install) => {
            if let Err(e) = commands::install::run_install() {
                eprintln!("{} {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Run {
            tool,
            max_iterations,
            prd,
        }) => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            if let Err(e) = rt.block_on(commands::run::run_run(tool, max_iterations, prd)) {
                eprintln!("{} {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Config { get, set }) => {
            if let Err(e) = commands::config::run_config(get, set) {
                eprintln!("{} {}", style("Error:").red().bold(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Status) => {
            println!("Viewing project status...");
        }
        Some(Commands::Archive) => {
            println!("Managing archives...");
        }
        Some(Commands::Detect) => {
            commands::detect::run_detect();
        }
        None => {
            // When no subcommand is provided, clap will show help due to the derive macro
        }
    }
}

#[cfg(test)]
mod tests {
    mod agent_detection_tests;
    mod cli_parsing_tests;
    mod config_management_tests;
    mod error_handling_tests;
    mod integration_tests;
    mod prd_parsing_tests;
    mod project_init_tests;
    mod task_execution_tests;
}
