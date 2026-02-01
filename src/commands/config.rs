use console::style;

use crate::config::{Config, ConfigKey};
use crate::error::{RalphError, RalphResult};

/// Run the config command to view or set configuration
pub fn run_config(get: Option<String>, set: Vec<String>) -> RalphResult<()> {
    // Handle --get <key>
    if let Some(key_str) = get {
        let key = ConfigKey::from_str(&key_str)
            .ok_or_else(|| RalphError::Other(format!("Unknown config key: {}", key_str)))?;

        let config = Config::load()?;
        match config.get(key) {
            Some(value) => println!("{} = {}", key_str, value),
            None => println!("{} is not set", key_str),
        }
        return Ok(());
    }

    // Handle --set <key> <value>
    if !set.is_empty() {
        if set.len() != 2 {
            return Err(RalphError::Other(
                "Usage: ralph config --set <key> <value>".to_string(),
            ));
        }

        let key_str = &set[0];
        let value = &set[1];

        let key = ConfigKey::from_str(key_str)
            .ok_or_else(|| RalphError::Other(format!("Unknown config key: {}", key_str)))?;

        let mut config = Config::load()?;
        config.set(key, value).map_err(RalphError::Other)?;
        config.save()?;

        println!("{} Set {} = {}", style("✓").green(), key_str, value);
        return Ok(());
    }

    // No flags provided - display all config
    println!("{}", style("Ralph Configuration").bold().cyan());
    println!("{}", style("===================").cyan());
    println!();

    let config = Config::load()?;
    let config_file = Config::config_file();

    println!("{}", style("Config file location:").bold());
    match &config_file {
        Some(path) => println!("  {}", path.display()),
        None => println!(
            "  {}",
            style("Unknown (could not determine config directory)").yellow()
        ),
    }
    println!();

    println!("{}", style("Current settings:").bold());
    println!();

    for key in ConfigKey::all() {
        let value = config.get(*key);
        let display_value = match value {
            Some(v) => style(v).green(),
            None => style("not set".to_string()).dim(),
        };
        println!("  {} = {}", style(key.as_str()).bold(), display_value);
        println!("    {}", style(key.description()).dim());
        println!();
    }

    println!("{}", style("Usage:").bold());
    println!("  ralph config              # Show all config");
    println!("  ralph config --get <key>  # Get specific value");
    println!("  ralph config --set <key> <value>  # Set value");

    Ok(())
}
