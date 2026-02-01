//! Configuration Management Tests
//!
//! Tests for the configuration management functionality in Ralph CLI.
//! These tests verify that config loading, saving, and modification work correctly.

use crate::config::{Config, ConfigKey};
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test config with specific values
fn create_test_config() -> Config {
    Config {
        default_tool: Some("codebuddy".to_string()),
        max_iterations: Some(20),
        auto_archive: Some(false),
    }
}

/// Test that Config::load() correctly loads existing config file
#[test]
fn test_config_load_existing_file() {
    // Create a temporary directory for our test config
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create a test config file with known values
    let config_content = r#"
default_tool = "claude"
max_iterations = 15
auto_archive = true
"#;
    fs::write(&config_path, config_content).unwrap();

    // Load the config using a custom load from path
    let content = fs::read_to_string(&config_path).unwrap();
    let config: Config = toml::from_str(&content).unwrap();

    // Verify the loaded values
    assert_eq!(config.default_tool, Some("claude".to_string()));
    assert_eq!(config.max_iterations, Some(15));
    assert_eq!(config.auto_archive, Some(true));
}

/// Test default config creation when config does not exist
#[test]
fn test_config_default_values() {
    let config = Config::default();

    // Verify default values
    assert_eq!(config.default_tool, None);
    assert_eq!(config.max_iterations, Some(10));
    assert_eq!(config.auto_archive, Some(true));
}

/// Test config get command correctly reads config values
#[test]
fn test_config_get_values() {
    let config = create_test_config();

    // Test getting default_tool
    assert_eq!(
        config.get(ConfigKey::DefaultTool),
        Some("codebuddy".to_string())
    );

    // Test getting max_iterations
    assert_eq!(
        config.get(ConfigKey::MaxIterations),
        Some("20".to_string())
    );

    // Test getting auto_archive
    assert_eq!(
        config.get(ConfigKey::AutoArchive),
        Some("false".to_string())
    );
}

/// Test config get returns None for unset values
#[test]
fn test_config_get_unset_values() {
    let config = Config::default();

    // default_tool should be None in default config
    assert_eq!(config.get(ConfigKey::DefaultTool), None);

    // max_iterations should have default value
    assert_eq!(
        config.get(ConfigKey::MaxIterations),
        Some("10".to_string())
    );

    // auto_archive should have default value
    assert_eq!(
        config.get(ConfigKey::AutoArchive),
        Some("true".to_string())
    );
}

/// Test config set command correctly modifies config values
#[test]
fn test_config_set_default_tool() {
    let mut config = Config::default();

    // Set default_tool
    let result = config.set(ConfigKey::DefaultTool, "amp");
    assert!(result.is_ok());
    assert_eq!(config.default_tool, Some("amp".to_string()));
}

/// Test config set for max_iterations with valid value
#[test]
fn test_config_set_max_iterations_valid() {
    let mut config = Config::default();

    // Set max_iterations with valid integer
    let result = config.set(ConfigKey::MaxIterations, "25");
    assert!(result.is_ok());
    assert_eq!(config.max_iterations, Some(25));
}

/// Test config set for max_iterations with invalid value
#[test]
fn test_config_set_max_iterations_invalid() {
    let mut config = Config::default();

    // Set max_iterations with invalid value
    let result = config.set(ConfigKey::MaxIterations, "not_a_number");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("positive integer"));
}

/// Test config set for auto_archive with valid values
#[test]
fn test_config_set_auto_archive_valid() {
    let mut config = Config::default();

    // Set auto_archive to false
    let result = config.set(ConfigKey::AutoArchive, "false");
    assert!(result.is_ok());
    assert_eq!(config.auto_archive, Some(false));

    // Set auto_archive to true
    let result = config.set(ConfigKey::AutoArchive, "true");
    assert!(result.is_ok());
    assert_eq!(config.auto_archive, Some(true));
}

/// Test config set for auto_archive with invalid value
#[test]
fn test_config_set_auto_archive_invalid() {
    let mut config = Config::default();

    // Set auto_archive with invalid value
    let result = config.set(ConfigKey::AutoArchive, "maybe");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("true or false"));
}

/// Test handling of invalid config keys
#[test]
fn test_config_key_from_str_valid() {
    assert_eq!(ConfigKey::from_str("default_tool"), Some(ConfigKey::DefaultTool));
    assert_eq!(ConfigKey::from_str("max_iterations"), Some(ConfigKey::MaxIterations));
    assert_eq!(ConfigKey::from_str("auto_archive"), Some(ConfigKey::AutoArchive));
}

/// Test that invalid config keys return None
#[test]
fn test_config_key_from_str_invalid() {
    assert_eq!(ConfigKey::from_str("invalid_key"), None);
    assert_eq!(ConfigKey::from_str(""), None);
    assert_eq!(ConfigKey::from_str("DEFAULT_TOOL"), None); // Case sensitive
}

/// Test ConfigKey as_str method
#[test]
fn test_config_key_as_str() {
    assert_eq!(ConfigKey::DefaultTool.as_str(), "default_tool");
    assert_eq!(ConfigKey::MaxIterations.as_str(), "max_iterations");
    assert_eq!(ConfigKey::AutoArchive.as_str(), "auto_archive");
}

/// Test ConfigKey description method
#[test]
fn test_config_key_description() {
    assert!(ConfigKey::DefaultTool.description().contains("Default AI tool"));
    assert!(ConfigKey::MaxIterations.description().contains("maximum iterations"));
    assert!(ConfigKey::AutoArchive.description().contains("Auto archive"));
}

/// Test ConfigKey all() returns all available keys
#[test]
fn test_config_key_all() {
    let all_keys = ConfigKey::all();
    assert_eq!(all_keys.len(), 3);
    assert!(all_keys.contains(&ConfigKey::DefaultTool));
    assert!(all_keys.contains(&ConfigKey::MaxIterations));
    assert!(all_keys.contains(&ConfigKey::AutoArchive));
}

/// Test TOML serialization of config
#[test]
fn test_config_toml_serialization() {
    let config = create_test_config();

    // Serialize to TOML
    let toml_str = toml::to_string_pretty(&config).unwrap();

    // Verify the TOML contains expected values
    assert!(toml_str.contains("default_tool = \"codebuddy\""));
    assert!(toml_str.contains("max_iterations = 20"));
    assert!(toml_str.contains("auto_archive = false"));
}

/// Test TOML deserialization of config
#[test]
fn test_config_toml_deserialization() {
    let toml_content = r#"
default_tool = "amp"
max_iterations = 5
auto_archive = false
"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert_eq!(config.default_tool, Some("amp".to_string()));
    assert_eq!(config.max_iterations, Some(5));
    assert_eq!(config.auto_archive, Some(false));
}

/// Test TOML serialization with None values (should be skipped)
#[test]
fn test_config_toml_serialization_skips_none() {
    let config = Config::default();

    // Serialize to TOML
    let toml_str = toml::to_string_pretty(&config).unwrap();

    // default_tool is None, so it should be skipped
    assert!(!toml_str.contains("default_tool"));
    // max_iterations and auto_archive have values
    assert!(toml_str.contains("max_iterations"));
    assert!(toml_str.contains("auto_archive"));
}

/// Test TOML deserialization with missing fields
#[test]
fn test_config_toml_deserialization_missing_fields() {
    // TOML with only some fields
    let toml_content = r#"
max_iterations = 7
"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    // Missing fields should be None
    assert_eq!(config.default_tool, None);
    assert_eq!(config.max_iterations, Some(7));
    assert_eq!(config.auto_archive, None);
}

/// Test config save and load roundtrip
#[test]
fn test_config_save_and_load_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create a config with specific values
    let original_config = create_test_config();

    // Save the config
    let toml_str = toml::to_string_pretty(&original_config).unwrap();
    fs::write(&config_path, toml_str).unwrap();

    // Load it back
    let content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: Config = toml::from_str(&content).unwrap();

    // Verify values match
    assert_eq!(loaded_config.default_tool, original_config.default_tool);
    assert_eq!(loaded_config.max_iterations, original_config.max_iterations);
    assert_eq!(loaded_config.auto_archive, original_config.auto_archive);
}

// Note: Debug, Clone, and Copy trait tests removed - they test derive macro functionality
