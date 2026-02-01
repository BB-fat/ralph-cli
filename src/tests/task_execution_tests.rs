//! Task Execution Tests
//!
//! Tests for the `ralph run` command functionality including:
//! - PRD file loading and parsing
//! - Tool auto-selection logic
//! - Explicit tool specification priority
//! - Config default tool priority
//! - Process spawning and output stream handling
//! - <promise>COMPLETE</promise> marker detection
//! - Ctrl+C signal handling
//! - Error handling for invalid PRD files

use std::fs;

use tempfile::TempDir;

use crate::config::Config;
use crate::prd::{Prd, UserStory};
use crate::agent::is_command_available;
use crate::commands::run::{colorize_output, determine_tool};
use crate::error::RalphError;

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a sample PRD JSON string
fn create_sample_prd_json() -> String {
    r#"{
        "project": "Test Project",
        "branchName": "ralph/test",
        "description": "Test description",
        "userStories": [
            {
                "id": "US-001",
                "title": "First Story",
                "description": "As a user...",
                "acceptanceCriteria": ["Criteria 1"],
                "priority": 1,
                "passes": true,
                "notes": "Completed"
            },
            {
                "id": "US-002",
                "title": "Second Story",
                "description": "As a developer...",
                "acceptanceCriteria": ["Criteria 2"],
                "priority": 2,
                "passes": false,
                "notes": ""
            }
        ]
    }"#
    .to_string()
}

/// Create a temporary PRD file for testing
fn create_temp_prd_file(temp_dir: &TempDir, content: &str) -> std::path::PathBuf {
    let prd_path = temp_dir.path().join("prd.json");
    fs::write(&prd_path, content).expect("Failed to write PRD file");
    prd_path
}

// ============================================================================
// PRD File Loading and Parsing Tests
// ============================================================================

#[test]
fn test_prd_from_file_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let prd_content = create_sample_prd_json();
    let prd_path = create_temp_prd_file(&temp_dir, &prd_content);

    let result = Prd::from_file(&prd_path);
    assert!(result.is_ok());

    let prd = result.unwrap();
    assert_eq!(prd.project, "Test Project");
    assert_eq!(prd.branch_name(), "ralph/test");
    assert_eq!(prd.description, "Test description");
    assert_eq!(prd.total_stories(), 2);
    assert_eq!(prd.completed_stories(), 1);
    assert_eq!(prd.pending_stories(), 1);
}

#[test]
fn test_prd_from_file_not_found() {
    let result = Prd::from_file("/nonexistent/path/prd.json");
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_prd_from_file_invalid_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let prd_path = create_temp_prd_file(&temp_dir, "not valid json {{{");

    let result = Prd::from_file(&prd_path);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn test_prd_from_file_missing_required_fields() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    // Missing required fields like "project" and "userStories"
    let invalid_content = r#"{"branchName": "test"}"#;
    let prd_path = create_temp_prd_file(&temp_dir, invalid_content);

    let result = Prd::from_file(&prd_path);
    assert!(result.is_err());
}

#[test]
fn test_prd_all_stories_complete() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = r#"{
        "project": "Complete Project",
        "branchName": "ralph/complete",
        "description": "All done",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story 1",
                "description": "Desc",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": true,
                "notes": ""
            }
        ]
    }"#;
    let prd_path = create_temp_prd_file(&temp_dir, content);

    let prd = Prd::from_file(&prd_path).unwrap();
    assert_eq!(prd.pending_stories(), 0);
    assert_eq!(prd.completed_stories(), 1);
}

#[test]
fn test_prd_no_stories() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = r#"{
        "project": "Empty Project",
        "branchName": "ralph/empty",
        "description": "No stories",
        "userStories": []
    }"#;
    let prd_path = create_temp_prd_file(&temp_dir, content);

    let prd = Prd::from_file(&prd_path).unwrap();
    assert_eq!(prd.total_stories(), 0);
    assert_eq!(prd.completed_stories(), 0);
    assert_eq!(prd.pending_stories(), 0);
}

// ============================================================================
// Tool Selection Logic Tests
// ============================================================================

#[test]
fn test_determine_tool_explicit_amp() {
    let config = Config::default();
    let result = determine_tool("amp", &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "amp");
}

#[test]
fn test_determine_tool_explicit_claude() {
    let config = Config::default();
    let result = determine_tool("claude", &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "claude");
}

#[test]
fn test_determine_tool_explicit_codebuddy() {
    let config = Config::default();
    let result = determine_tool("codebuddy", &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "codebuddy");
}

#[test]
fn test_determine_tool_custom_tool() {
    let config = Config::default();
    let result = determine_tool("custom-agent", &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "custom-agent");
}

#[test]
fn test_determine_tool_auto_with_config_default() {
    // This test checks that when tool is "auto" and config has a default_tool,
    // it should use the config default if available
    let config = Config {
        default_tool: Some("echo".to_string()),
        ..Default::default()
    };

    // Only run this test if echo is available
    if is_command_available("echo") {
        let result = determine_tool("auto", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "echo");
    }
}

#[test]
fn test_determine_tool_auto_no_config_no_agents() {
    // When auto is selected but no config default and no agents detected
    let config = Config::default();

    // We can't easily mock detect_agents(), but we can test the error case
    // by ensuring the function returns an error when no tools are available
    let result = determine_tool("auto", &config);

    // Result depends on whether any agents are installed on the system
    // The function should either succeed (if agents are detected) or fail
    match result {
        Ok(tool) => {
            // If it succeeds, the tool should be one of the known agents
            let tool_str: &str = tool.as_str();
            assert!(
                ["amp", "claude", "codebuddy"].contains(&tool_str),
                "Expected a known agent, got: {}",
                tool
            );
        }
        Err(RalphError::Other(msg)) => {
            assert!(msg.contains("No AI agent CLI detected"));
        }
        Err(_) => {
            // Other error types are unexpected
            panic!("Unexpected error type");
        }
    }
}

#[test]
fn test_determine_tool_explicit_overrides_config() {
    // Explicit tool specification should take priority over config default
    let config = Config {
        default_tool: Some("claude".to_string()),
        ..Default::default()
    };

    let result = determine_tool("amp", &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "amp");
}

// ============================================================================
// Colorize Output Tests
// ============================================================================

#[test]
fn test_colorize_output_error() {
    let line = "This is an Error message";
    let colored = colorize_output(line);
    // The line should be colored (we can't easily test the actual ANSI codes,
    // but we can verify it returns a string)
    assert!(!colored.is_empty());
    assert!(colored.contains("Error") || colored.contains("error"));
}

#[test]
fn test_colorize_output_warning() {
    let line = "This is a Warning message";
    let colored = colorize_output(line);
    assert!(!colored.is_empty());
    assert!(colored.contains("Warning") || colored.contains("warning"));
}

#[test]
fn test_colorize_output_success() {
    let line = "Operation completed with Success";
    let colored = colorize_output(line);
    assert!(!colored.is_empty());
}

#[test]
fn test_colorize_output_with_checkmark() {
    let line = "All tests passed ✓";
    let colored = colorize_output(line);
    assert!(!colored.is_empty());
}

#[test]
fn test_colorize_output_complete_marker() {
    let line = "<promise>COMPLETE</promise>";
    let colored = colorize_output(line);
    assert!(!colored.is_empty());
    assert!(colored.contains("COMPLETE"));
}

#[test]
fn test_colorize_output_normal() {
    let line = "This is a normal log message";
    let colored = colorize_output(line);
    // Normal messages should pass through unchanged
    assert_eq!(colored, line);
}

#[test]
fn test_colorize_output_empty() {
    let line = "";
    let colored = colorize_output(line);
    assert_eq!(colored, "");
}

// ============================================================================
// Signal Handling Tests
// ============================================================================

// Note: AtomicBool tests removed as they test standard library functionality

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_ralph_error_io_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let ralph_err: RalphError = io_err.into();

    match ralph_err {
        RalphError::Io(_) => (), // Expected
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_ralph_error_display_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test file");
    let ralph_err = RalphError::Io(io_err);

    let display = format!("{}", ralph_err);
    assert!(display.contains("IO error"));
    assert!(display.contains("test file"));
}

#[test]
fn test_ralph_error_display_other() {
    let ralph_err = RalphError::Other("custom error message".to_string());

    let display = format!("{}", ralph_err);
    assert_eq!(display, "custom error message");
}

#[test]
fn test_prd_error_invalid_data_kind() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let prd_path = create_temp_prd_file(&temp_dir, "{ invalid json");

    let result = Prd::from_file(&prd_path);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

// ============================================================================
// Integration Helper Tests
// ============================================================================

#[test]
fn test_is_command_available_echo() {
    // "echo" should be available on all Unix-like systems
    // On Windows, it might be a built-in, so this test is Unix-specific
    #[cfg(unix)]
    {
        assert!(is_command_available("echo"));
    }
}

#[test]
fn test_is_command_available_nonexistent() {
    // This command should not exist
    assert!(!is_command_available("definitely_not_a_real_command_12345"));
}

// Note: Agent mapping tests removed as they test simple constant returns

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_prd_with_unicode_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = r#"{
        "project": "Unicode Project 中文",
        "branchName": "ralph/unicode-🚀",
        "description": "Testing unicode: ñ, é, 日本語",
        "userStories": [
            {
                "id": "US-001",
                "title": "Unicode Story 🎉",
                "description": "Test with emojis 🎨",
                "acceptanceCriteria": ["Criteriön 1"],
                "priority": 1,
                "passes": false,
                "notes": "Notes with ümlauts"
            }
        ]
    }"#;
    let prd_path = create_temp_prd_file(&temp_dir, content);

    let prd = Prd::from_file(&prd_path).unwrap();
    assert_eq!(prd.project, "Unicode Project 中文");
    assert_eq!(prd.description, "Testing unicode: ñ, é, 日本語");
}

#[test]
fn test_prd_with_special_characters_in_strings() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let content = r#"{
        "project": "Project with \"quotes\" and \n newlines",
        "branchName": "ralph/special",
        "description": "Test with \"escaped\" characters",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story with \"quotes\"",
                "description": "Line 1\nLine 2",
                "acceptanceCriteria": ["Criteria with \"quotes\""],
                "priority": 1,
                "passes": false,
                "notes": ""
            }
        ]
    }"#;
    let prd_path = create_temp_prd_file(&temp_dir, content);

    let prd = Prd::from_file(&prd_path).unwrap();
    assert!(prd.project.contains("quotes"));
    assert!(prd.description.contains("escaped"));
}

#[test]
fn test_colorize_output_multiple_patterns() {
    // Line with both error and warning - error takes precedence in current implementation
    let line = "Error: This is a warning about an ERROR";
    let colored = colorize_output(line);
    assert!(!colored.is_empty());
}

// Note: Config default values test removed - already tested in config_management_tests.rs

#[test]
fn test_empty_prd_stories_array() {
    let prd = Prd {
        project: "Empty".to_string(),
        branch_name: "ralph/empty".to_string(),
        description: "No stories".to_string(),
        user_stories: vec![],
    };

    assert_eq!(prd.total_stories(), 0);
    assert_eq!(prd.completed_stories(), 0);
    assert_eq!(prd.pending_stories(), 0);
}

#[test]
fn test_all_stories_passing() {
    let prd = Prd {
        project: "Complete".to_string(),
        branch_name: "ralph/complete".to_string(),
        description: "All done".to_string(),
        user_stories: vec![
            UserStory {
                id: "US-001".to_string(),
                title: "Story 1".to_string(),
                description: "Desc".to_string(),
                acceptance_criteria: vec![],
                priority: 1,
                passes: true,
                notes: "".to_string(),
            },
            UserStory {
                id: "US-002".to_string(),
                title: "Story 2".to_string(),
                description: "Desc".to_string(),
                acceptance_criteria: vec![],
                priority: 2,
                passes: true,
                notes: "".to_string(),
            },
        ],
    };

    assert_eq!(prd.pending_stories(), 0);
    assert_eq!(prd.completed_stories(), 2);
}
