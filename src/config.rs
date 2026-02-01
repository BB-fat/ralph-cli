use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Ralph CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default AI tool to use (amp, claude, codebuddy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tool: Option<String>,

    /// Default maximum iterations for task execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<u32>,

    /// Whether to auto archive history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_tool: None,
            max_iterations: Some(10),
            auto_archive: Some(true),
        }
    }
}

impl Config {
    /// Get the path to the config directory
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("ralph"))
    }

    /// Get the path to the config file
    pub fn config_file() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("config.toml"))
    }

    /// Load config from file, or return default if file doesn't exist
    pub fn load() -> io::Result<Self> {
        match Self::config_file() {
            Some(path) if path.exists() => {
                let content = fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&content)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Ok(config)
            }
            _ => Ok(Self::default()),
        }
    }

    /// Save config to file
    pub fn save(&self) -> io::Result<()> {
        let config_dir = Self::config_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not determine config directory"))?;
        let config_file = Self::config_file()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not determine config file path"))?;

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;

        // Serialize and write config
        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&config_file, content)?;

        Ok(())
    }

    /// Get a config value by key
    pub fn get(&self, key: ConfigKey) -> Option<String> {
        match key {
            ConfigKey::DefaultTool => self.default_tool.clone(),
            ConfigKey::MaxIterations => self.max_iterations.map(|v| v.to_string()),
            ConfigKey::AutoArchive => self.auto_archive.map(|v| v.to_string()),
        }
    }

    /// Set a config value by key
    pub fn set(&mut self, key: ConfigKey, value: &str) -> Result<(), String> {
        match key {
            ConfigKey::DefaultTool => {
                self.default_tool = Some(value.to_string());
            }
            ConfigKey::MaxIterations => {
                let val: u32 = value
                    .parse()
                    .map_err(|_| "max_iterations must be a positive integer".to_string())?;
                self.max_iterations = Some(val);
            }
            ConfigKey::AutoArchive => {
                let val: bool = value
                    .parse()
                    .map_err(|_| "auto_archive must be true or false".to_string())?;
                self.auto_archive = Some(val);
            }
        }
        Ok(())
    }
}

/// Configuration keys that can be get/set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigKey {
    DefaultTool,
    MaxIterations,
    AutoArchive,
}

impl ConfigKey {
    /// Get all available config keys
    pub fn all() -> &'static [ConfigKey] {
        &[ConfigKey::DefaultTool, ConfigKey::MaxIterations, ConfigKey::AutoArchive]
    }

    /// Get the string name of the key
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKey::DefaultTool => "default_tool",
            ConfigKey::MaxIterations => "max_iterations",
            ConfigKey::AutoArchive => "auto_archive",
        }
    }

    /// Get description of the key
    pub fn description(&self) -> &'static str {
        match self {
            ConfigKey::DefaultTool => "Default AI tool (amp, claude, codebuddy)",
            ConfigKey::MaxIterations => "Default maximum iterations for task execution",
            ConfigKey::AutoArchive => "Auto archive history on branch switch",
        }
    }

    /// Parse a config key from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "default_tool" => Some(ConfigKey::DefaultTool),
            "max_iterations" => Some(ConfigKey::MaxIterations),
            "auto_archive" => Some(ConfigKey::AutoArchive),
            _ => None,
        }
    }
}
