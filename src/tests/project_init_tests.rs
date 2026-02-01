//! Project Initialization Tests
//!
//! Tests for the `ralph init` command functionality including:
//! - Directory structure creation
//! - AGENTS.md template generation
//! - prd.json.example template generation
//! - Default value handling
//! - Existing directory handling

use std::fs;
use tempfile::TempDir;

// Import the functions from templates module
use crate::templates::get_prd_json_template;

/// Helper function to create a temporary directory for testing
fn setup_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Test that get_prd_json_template generates valid JSON structure
#[test]
fn test_prd_json_template_structure() {
    let project_name = "My Project";
    let project_description = "My project description";

    let content = get_prd_json_template(project_name, project_description, None);

    // Check that the content is valid JSON by parsing it
    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    // Verify structure
    assert_eq!(parsed["project"], project_name);
    assert_eq!(parsed["description"], project_description);
    assert!(parsed["branchName"].as_str().unwrap().starts_with("ralph/"));
    assert!(parsed["userStories"].is_array());
}

/// Test that get_prd_json_template generates correct branch name
#[test]
fn test_prd_json_template_branch_name_generation() {
    let project_name = "My Awesome Project";
    let content = get_prd_json_template(project_name, "Description", None);

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    // Branch name should be lowercase with hyphens
    let branch_name = parsed["branchName"].as_str().unwrap();
    assert_eq!(branch_name, "ralph/my-awesome-project");
}

/// Test that get_prd_json_template handles special characters in project name
#[test]
fn test_prd_json_template_special_characters() {
    let project_name = "Project 123_Test";
    let content = get_prd_json_template(project_name, "Description", None);

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    // Project name should be preserved as-is
    assert_eq!(parsed["project"], project_name);
    // Branch name should handle spaces
    let branch_name = parsed["branchName"].as_str().unwrap();
    assert!(branch_name.contains("project-123_test"));
}

/// Test that get_prd_json_template includes default user story
#[test]
fn test_prd_json_template_default_user_story() {
    let content = get_prd_json_template("Test", "Description", None);

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    let stories = parsed["userStories"].as_array().unwrap();
    assert!(!stories.is_empty(), "Should have at least one default user story");

    let first_story = &stories[0];
    assert_eq!(first_story["id"], "US-001");
    assert_eq!(first_story["title"], "Initial setup");
    assert_eq!(first_story["passes"], false);
    assert!(first_story["acceptanceCriteria"].is_array());
}

/// Test that generated directories can be created successfully
#[test]
fn test_directory_creation() {
    let temp_dir = setup_temp_dir();
    let base_path = temp_dir.path();

    let scripts_dir = base_path.join("scripts/ralph");
    let tasks_dir = base_path.join("tasks");

    // Create directories
    fs::create_dir_all(&scripts_dir).expect("Failed to create scripts directory");
    fs::create_dir_all(&tasks_dir).expect("Failed to create tasks directory");

    // Verify directories exist
    assert!(scripts_dir.exists(), "scripts/ralph directory should exist");
    assert!(tasks_dir.exists(), "tasks directory should exist");

    // Verify they are directories
    assert!(scripts_dir.is_dir());
    assert!(tasks_dir.is_dir());
}

/// Test that existing directories don't cause errors
#[test]
fn test_existing_directory_handling() {
    let temp_dir = setup_temp_dir();
    let base_path = temp_dir.path();

    let scripts_dir = base_path.join("scripts/ralph");

    // Create directory first
    fs::create_dir_all(&scripts_dir).expect("Failed to create initial directory");

    // Creating it again should not fail
    let result = fs::create_dir_all(&scripts_dir);
    assert!(result.is_ok(), "Creating existing directory should not fail");
}

/// Test that the complete init structure can be created
#[test]
fn test_complete_init_structure() {
    let temp_dir = setup_temp_dir();
    let base_path = temp_dir.path();

    // Create directory structure (matches actual init.rs behavior)
    let ralph_dir = base_path.join("ralph");
    let tasks_dir = ralph_dir.join("tasks");
    fs::create_dir_all(&ralph_dir).expect("Failed to create ralph directory");
    fs::create_dir_all(&tasks_dir).expect("Failed to create tasks directory");

    // Create template files (matches actual init.rs behavior)
    let prd_path = ralph_dir.join("prd.json");

    fs::write(&prd_path, get_prd_json_template("My Project", "My Description", None))
        .expect("Failed to write prd.json");

    // Verify all expected items exist
    assert!(ralph_dir.exists(), "ralph/ directory should exist");
    assert!(tasks_dir.exists(), "ralph/tasks/ directory should exist");
    assert!(prd_path.exists(), "ralph/prd.json should exist");

    // Verify the structure matches expected layout
    assert!(ralph_dir.is_dir(), "ralph should be a directory");
    assert!(tasks_dir.is_dir(), "tasks should be a directory");
}

/// Test that prd.json.example contains valid user story structure
#[test]
fn test_prd_template_user_story_validation() {
    let content = get_prd_json_template("Test", "Description", None);

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    let stories = parsed["userStories"].as_array().unwrap();
    for story in stories {
        // Each story should have required fields
        assert!(story.get("id").is_some(), "Story should have id");
        assert!(story.get("title").is_some(), "Story should have title");
        assert!(story.get("description").is_some(), "Story should have description");
        assert!(story.get("acceptanceCriteria").is_some(), "Story should have acceptanceCriteria");
        assert!(story.get("priority").is_some(), "Story should have priority");
        assert!(story.get("passes").is_some(), "Story should have passes");
    }
}

/// Test that prd.json.example has correct JSON types for all fields
#[test]
fn test_prd_template_field_types() {
    let content = get_prd_json_template("Test", "Description", None);

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("Generated PRD template should be valid JSON");

    // Check field types
    assert!(parsed["project"].is_string());
    assert!(parsed["branchName"].is_string());
    assert!(parsed["description"].is_string());
    assert!(parsed["userStories"].is_array());

    let first_story = &parsed["userStories"].as_array().unwrap()[0];
    assert!(first_story["id"].is_string());
    assert!(first_story["title"].is_string());
    assert!(first_story["description"].is_string());
    assert!(first_story["acceptanceCriteria"].is_array());
    assert!(first_story["priority"].is_number());
    assert!(first_story["passes"].is_boolean());
}
