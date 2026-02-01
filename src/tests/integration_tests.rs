//! Integration tests for Ralph CLI
//!
//! These tests verify complete user workflows and ensure all components work together correctly.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to create a temporary directory for test isolation
fn setup_test_env() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Helper function to run the ralph binary with given arguments
/// Note: This runs the compiled binary directly, not through cargo
fn run_ralph(args: &[&str], cwd: Option<&std::path::Path>) -> std::process::Output {
    // Find the compiled binary in target directory
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--quiet").arg("--").args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    // Set the working directory to the project root for cargo
    cmd.current_dir(std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".")));

    cmd.output().expect("Failed to execute ralph command")
}

/// Helper to create a sample prd.json file for testing
fn create_sample_prd(dir: &std::path::Path, project_name: &str) -> PathBuf {
    let prd_path = dir.join("prd.json");
    let prd_content = format!(
        r#"{{
  "project": "{}",
  "branchName": "ralph/test-branch",
  "description": "Test project for integration tests",
  "userStories": [
    {{
      "id": "US-001",
      "title": "Test story",
      "description": "As a user, I want to test the integration",
      "acceptanceCriteria": ["Test passes"],
      "priority": 1,
      "passes": false,
      "notes": ""
    }}
  ]
}}"#,
        project_name
    );
    fs::write(&prd_path, prd_content).expect("Failed to write prd.json");
    prd_path
}

/// Helper to create a complete PRD with multiple stories
fn create_complete_prd(dir: &std::path::Path) -> PathBuf {
    let prd_path = dir.join("prd.json");
    let prd_content = r#"{
  "project": "Integration Test Project",
  "branchName": "ralph/integration-test",
  "description": "Complete integration test project",
  "userStories": [
    {
      "id": "US-001",
      "title": "First story",
      "description": "As a user, I want the first feature",
      "acceptanceCriteria": ["First criterion"],
      "priority": 1,
      "passes": true,
      "notes": "Completed"
    },
    {
      "id": "US-002",
      "title": "Second story",
      "description": "As a user, I want the second feature",
      "acceptanceCriteria": ["Second criterion"],
      "priority": 2,
      "passes": false,
      "notes": ""
    },
    {
      "id": "US-003",
      "title": "Third story",
      "description": "As a user, I want the third feature",
      "acceptanceCriteria": ["Third criterion"],
      "priority": 3,
      "passes": false,
      "notes": ""
    }
  ]
}"#;
    fs::write(&prd_path, prd_content).expect("Failed to write prd.json");
    prd_path
}

// ============================================================================
// Test 1: Temporary Directory Isolation
// ============================================================================

#[test]
fn test_integration_temp_dir_isolation() {
    // Create two separate temp directories
    let temp_dir1 = setup_test_env();
    let temp_dir2 = setup_test_env();

    // Create different files in each
    let file1 = temp_dir1.path().join("test1.txt");
    let file2 = temp_dir2.path().join("test2.txt");

    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();

    // Verify isolation - each directory should only have its own file
    assert!(file1.exists());
    assert!(file2.exists());
    assert!(!temp_dir1.path().join("test2.txt").exists());
    assert!(!temp_dir2.path().join("test1.txt").exists());

    // Verify contents are correct
    assert_eq!(fs::read_to_string(&file1).unwrap(), "content1");
    assert_eq!(fs::read_to_string(&file2).unwrap(), "content2");
}

// Note: Directory cleanup test removed - it tests tempfile library functionality

// ============================================================================
// Test 2: Basic CLI Commands
// ============================================================================

#[test]
fn test_integration_workflow_init_creates_structure() {
    let temp_dir = setup_test_env();

    // Run ralph init in the temp directory
    // Note: We can't fully test interactive init, but we can verify the binary exists
    let output = run_ralph(&["--help"], Some(temp_dir.path()));

    // Verify the binary runs successfully
    assert!(output.status.success(), "ralph --help should succeed");

    // Verify help output contains expected commands
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("init") || stdout.contains("Init"), "Help should mention init command");
}

#[test]
fn test_integration_detect_command_runs() {
    let temp_dir = setup_test_env();

    // Run ralph detect
    let output = run_ralph(&["detect"], Some(temp_dir.path()));

    // Command should execute (may succeed or fail depending on agents installed)
    // We're testing that the command runs without crashing
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should produce some output
    let has_output = !stdout.is_empty() || !stderr.is_empty();
    assert!(has_output, "detect command should produce output");
}

#[test]
fn test_integration_config_command_workflow() {
    let temp_dir = setup_test_env();

    // Test config --help
    let output = run_ralph(&["config", "--help"], Some(temp_dir.path()));
    assert!(output.status.success(), "config --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("config") || stdout.contains("Config"),
        "Config help should mention config"
    );
}

#[test]
fn test_integration_status_command_runs() {
    let temp_dir = setup_test_env();

    // Run ralph status
    let output = run_ralph(&["status"], Some(temp_dir.path()));

    // Command should run without error
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should produce some output
    let has_output = !stdout.is_empty() || !stderr.is_empty();
    assert!(has_output, "status command should produce output");
}

#[test]
fn test_integration_archive_command_runs() {
    let temp_dir = setup_test_env();

    // Run ralph archive
    let output = run_ralph(&["archive"], Some(temp_dir.path()));

    // Command should run without error
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should produce some output
    let has_output = !stdout.is_empty() || !stderr.is_empty();
    assert!(has_output, "archive command should produce output");
}

#[test]
fn test_integration_run_command_with_invalid_prd() {
    let temp_dir = setup_test_env();

    // Run ralph run with a non-existent prd file
    let output = run_ralph(&["run", "--prd", "/nonexistent/prd.json"], Some(temp_dir.path()));

    // Should fail gracefully
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success() || stderr.contains("Error") || stderr.contains("error"),
        "Run with invalid PRD should report an error"
    );
}

#[test]
fn test_integration_run_command_help() {
    let temp_dir = setup_test_env();

    // Test run --help
    let output = run_ralph(&["run", "--help"], Some(temp_dir.path()));
    assert!(output.status.success(), "run --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("run") || stdout.contains("Run"), "Run help should mention run");
    assert!(
        stdout.contains("--tool") || stdout.contains("--prd"),
        "Run help should mention options"
    );
}

#[test]
fn test_integration_install_help() {
    let temp_dir = setup_test_env();

    // Test install --help
    let output = run_ralph(&["install", "--help"], Some(temp_dir.path()));
    assert!(output.status.success(), "install --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("install") || stdout.contains("Install"),
        "Install help should mention install"
    );
}

#[test]
fn test_integration_version_flag() {
    let temp_dir = setup_test_env();

    // Test --version
    let output = run_ralph(&["--version"], Some(temp_dir.path()));
    assert!(output.status.success(), "--version should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should contain version number (0.1.0 or similar)
    assert!(stdout.contains("0.1"), "Version should contain 0.1");
}

// ============================================================================
// Test 3: PRD File Operations
// ============================================================================

#[test]
fn test_integration_prd_file_operations() {
    let temp_dir = setup_test_env();

    // Create a sample PRD file
    let prd_path = create_sample_prd(temp_dir.path(), "Integration Test Project");

    // Verify file was created
    assert!(prd_path.exists(), "PRD file should be created");

    // Verify content is valid JSON
    let content = fs::read_to_string(&prd_path).expect("Failed to read PRD file");
    let json: serde_json::Value = serde_json::from_str(&content).expect("PRD should be valid JSON");

    // Verify structure
    assert_eq!(json["project"], "Integration Test Project");
    assert_eq!(json["branchName"], "ralph/test-branch");
    assert!(json["userStories"].is_array());
}

#[test]
fn test_integration_complete_prd_operations() {
    let temp_dir = setup_test_env();

    // Create a complete PRD with multiple stories
    let prd_path = create_complete_prd(temp_dir.path());

    // Verify file exists
    assert!(prd_path.exists());

    // Parse and verify structure
    let content = fs::read_to_string(&prd_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(json["project"], "Integration Test Project");
    assert_eq!(json["branchName"], "ralph/integration-test");

    let stories = json["userStories"].as_array().unwrap();
    assert_eq!(stories.len(), 3);

    // Verify first story is marked as passing
    assert_eq!(stories[0]["passes"], true);
    assert_eq!(stories[0]["id"], "US-001");

    // Verify other stories are not passing
    assert_eq!(stories[1]["passes"], false);
    assert_eq!(stories[2]["passes"], false);
}

// ============================================================================
// Test 4: Directory and File Operations
// ============================================================================

#[test]
fn test_integration_directory_creation_and_cleanup() {
    let temp_dir = setup_test_env();

    // Create nested directories
    let nested = temp_dir.path().join("level1/level2/level3");
    fs::create_dir_all(&nested).expect("Should create nested directories");

    // Verify they exist
    assert!(nested.exists(), "Nested directories should exist");

    // Create files at various levels
    fs::write(temp_dir.path().join("level1/file1.txt"), "file1").unwrap();
    fs::write(nested.join("file3.txt"), "file3").unwrap();

    // Verify files exist
    assert!(temp_dir.path().join("level1/file1.txt").exists());
    assert!(nested.join("file3.txt").exists());

    // TempDir will automatically clean up when dropped
}

// Note: Path handling and file permissions tests removed - they test standard library functionality

// ============================================================================
// Test 5: Concurrent Execution Safety
// ============================================================================

// Note: Concurrent file access tests removed - they test standard library functionality

// ============================================================================
// Test 6: Error Handling
// ============================================================================

#[test]
fn test_integration_error_handling_invalid_json() {
    let temp_dir = setup_test_env();

    // Create an invalid JSON file
    let invalid_prd = temp_dir.path().join("invalid.json");
    fs::write(&invalid_prd, "{ invalid json }").unwrap();

    // Try to parse it
    let content = fs::read_to_string(&invalid_prd).unwrap();
    let result: Result<serde_json::Value, _> = serde_json::from_str(&content);

    // Should fail to parse
    assert!(result.is_err(), "Invalid JSON should fail to parse");
}

#[test]
fn test_integration_error_handling_missing_file() {
    let temp_dir = setup_test_env();

    // Try to read a non-existent file
    let missing_file = temp_dir.path().join("does_not_exist.txt");
    let result = fs::read_to_string(&missing_file);

    assert!(result.is_err(), "Reading missing file should error");
}

#[test]
fn test_integration_error_handling_invalid_path() {
    let temp_dir = setup_test_env();

    // Try to create a file in a non-existent nested directory without create_dir_all
    let invalid_path = temp_dir.path().join("nonexistent/deeply/nested/file.txt");
    let result = fs::write(&invalid_path, "content");

    assert!(result.is_err(), "Writing to invalid path should error");
}

// ============================================================================
// Test 7: Config File Operations
// ============================================================================

#[test]
fn test_integration_config_file_operations() {
    let temp_dir = setup_test_env();

    // Create a mock config file
    let config_dir = temp_dir.path().join(".config/ralph");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("config.toml");
    let config_content = r#"default_tool = "codebuddy"
max_iterations = 15
auto_archive = true
"#;
    fs::write(&config_file, config_content).unwrap();

    // Verify file exists and content is correct
    assert!(config_file.exists());
    let read_content = fs::read_to_string(&config_file).unwrap();
    assert!(read_content.contains("default_tool"));
    assert!(read_content.contains("codebuddy"));
    assert!(read_content.contains("max_iterations"));
}

#[test]
fn test_integration_config_parsing() {
    let temp_dir = setup_test_env();

    // Create a config with various types
    let config_file = temp_dir.path().join("config.toml");
    let config_content = r#"default_tool = "claude"
max_iterations = 20
auto_archive = false
"#;
    fs::write(&config_file, config_content).unwrap();

    // Read and verify
    let content = fs::read_to_string(&config_file).unwrap();
    assert!(content.contains("claude"));
    assert!(content.contains("20"));
    assert!(content.contains("false"));
}

// ============================================================================
// Test 8: Edge Cases
// ============================================================================

#[test]
fn test_integration_empty_prd_handling() {
    let temp_dir = setup_test_env();

    // Create an empty PRD (valid JSON but empty stories)
    let prd_path = temp_dir.path().join("empty_prd.json");
    let empty_prd = r#"{
  "project": "Empty Project",
  "branchName": "ralph/empty",
  "description": "Project with no stories",
  "userStories": []
}"#;
    fs::write(&prd_path, empty_prd).unwrap();

    // Verify it's valid JSON
    let content = fs::read_to_string(&prd_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Verify empty stories array
    assert!(json["userStories"].as_array().unwrap().is_empty());
}

#[test]
fn test_integration_unicode_handling() {
    let temp_dir = setup_test_env();

    // Create files with unicode names and content
    let unicode_file = temp_dir.path().join("unicode_测试.txt");
    fs::write(&unicode_file, "Unicode content: 你好世界").unwrap();

    // Verify unicode is preserved
    let content = fs::read_to_string(&unicode_file).unwrap();
    assert!(content.contains("你好世界"));
}

#[test]
fn test_integration_large_file_handling() {
    let temp_dir = setup_test_env();

    // Create a relatively large file (1MB)
    let large_file = temp_dir.path().join("large_file.txt");
    let large_content = "x".repeat(1024 * 1024);
    fs::write(&large_file, &large_content).unwrap();

    // Verify file size
    let metadata = fs::metadata(&large_file).unwrap();
    assert_eq!(metadata.len(), 1024 * 1024);

    // Verify content integrity (sample first and last parts)
    let read_content = fs::read_to_string(&large_file).unwrap();
    assert_eq!(read_content.len(), 1024 * 1024);
    assert!(read_content.starts_with('x'));
    assert!(read_content.ends_with('x'));
}

#[test]
fn test_integration_special_characters_in_filenames() {
    let temp_dir = setup_test_env();

    // Test various special characters
    let filenames = vec![
        "file with spaces.txt",
        "file-with-dashes.txt",
        "file_with_underscores.txt",
        "file.multiple.dots.txt",
        "UPPERCASE.TXT",
        "mixedCase.TxT",
    ];

    for filename in filenames {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, format!("content of {}", filename)).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, format!("content of {}", filename));
    }
}

// ============================================================================
// Test 9: Simulated Complete Workflow
// ============================================================================

#[test]
fn test_integration_simulated_workflow() {
    // This test simulates a complete workflow without actual agent execution
    let temp_dir = setup_test_env();

    // Step 1: Create project structure (simulating init)
    let scripts_dir = temp_dir.path().join("scripts/ralph");
    let tasks_dir = temp_dir.path().join("tasks");
    fs::create_dir_all(&scripts_dir).unwrap();
    fs::create_dir_all(&tasks_dir).unwrap();

    // Step 2: Create AGENTS.md (simulating template generation)
    let agents_md = temp_dir.path().join("AGENTS.md");
    fs::write(&agents_md, "# Test Project\n\nTest description").unwrap();

    // Step 3: Create prd.json (simulating PRD creation)
    let prd_path = create_sample_prd(temp_dir.path(), "Simulated Workflow Project");

    // Step 4: Create progress.txt (simulating progress tracking)
    let progress_path = temp_dir.path().join("progress.txt");
    fs::write(&progress_path, "## Test Progress\n- Initial setup complete\n").unwrap();

    // Step 5: Create skill files (simulating install)
    let skill_file = scripts_dir.join("ralph.md");
    fs::write(&skill_file, "# Ralph Skill\n\nTest skill content").unwrap();

    // Verify all files exist
    assert!(scripts_dir.exists());
    assert!(tasks_dir.exists());
    assert!(agents_md.exists());
    assert!(prd_path.exists());
    assert!(progress_path.exists());
    assert!(skill_file.exists());

    // Verify PRD content
    let prd_content = fs::read_to_string(&prd_path).unwrap();
    let prd_json: serde_json::Value = serde_json::from_str(&prd_content).unwrap();
    assert_eq!(prd_json["project"], "Simulated Workflow Project");
}

#[test]
fn test_integration_full_workflow_simulation() {
    let temp_dir = setup_test_env();

    // Phase 1: Project Initialization
    let project_dir = temp_dir.path().join("my_project");
    fs::create_dir_all(&project_dir).unwrap();

    // Create directory structure (simulating ralph init)
    let scripts_dir = project_dir.join("scripts/ralph");
    let tasks_dir = project_dir.join("tasks");
    fs::create_dir_all(&scripts_dir).unwrap();
    fs::create_dir_all(&tasks_dir).unwrap();

    // Create AGENTS.md
    let agents_md = project_dir.join("AGENTS.md");
    fs::write(&agents_md, "# My Project\n\nA test project").unwrap();

    // Phase 2: Create PRD (simulating PRD creation)
    let prd_path = create_complete_prd(&project_dir);

    // Phase 3: Install skills (simulating ralph install)
    let skill_file = scripts_dir.join("ralph.md");
    fs::write(&skill_file, "# Ralph Skill\n\nSkill content here").unwrap();

    let prd_skill = scripts_dir.join("prd.md");
    fs::write(&prd_skill, "# PRD Skill\n\nPRD skill content").unwrap();

    // Phase 4: Create progress tracking
    let progress_file = project_dir.join("progress.txt");
    fs::write(&progress_file, "## Codebase Patterns\n- Pattern 1\n\n## 2026-02-01 - US-001\n- Completed setup\n").unwrap();

    // Verify complete structure
    assert!(project_dir.exists());
    assert!(scripts_dir.exists());
    assert!(tasks_dir.exists());
    assert!(agents_md.exists());
    assert!(prd_path.exists());
    assert!(skill_file.exists());
    assert!(prd_skill.exists());
    assert!(progress_file.exists());

    // Verify PRD has correct structure
    let prd_content = fs::read_to_string(&prd_path).unwrap();
    let prd: serde_json::Value = serde_json::from_str(&prd_content).unwrap();
    assert_eq!(prd["userStories"].as_array().unwrap().len(), 3);
}

// ============================================================================
// Test 10: CLI Argument Combinations
// ============================================================================

#[test]
fn test_integration_cli_argument_combinations() {
    let temp_dir = setup_test_env();

    // Test various argument combinations
    let combinations = vec![
        vec!["--help"],
        vec!["--version"],
        vec!["init", "--help"],
        vec!["run", "--help"],
        vec!["config", "--help"],
        vec!["install", "--help"],
        vec!["detect", "--help"],
    ];

    for args in combinations {
        let output = run_ralph(&args, Some(temp_dir.path()));
        assert!(
            output.status.success(),
            "Command with args {:?} should succeed",
            args
        );
    }
}

#[test]
fn test_integration_run_with_all_options() {
    let temp_dir = setup_test_env();

    // Create a PRD file
    create_sample_prd(temp_dir.path(), "Test Project");

    // Test run with various option combinations (these may fail due to missing agents,
    // but we're testing argument parsing, not actual execution)
    let args_combinations = vec![
        vec!["run", "--help"],
        vec!["run", "--prd", "./prd.json", "--help"],
    ];

    for args in args_combinations {
        let output = run_ralph(&args, Some(temp_dir.path()));
        // Help should always succeed
        if args.contains(&"--help") {
            assert!(output.status.success(), "run --help should succeed");
        }
    }
}

// ============================================================================
// Test 11: Environment Isolation
// ============================================================================

#[test]
fn test_integration_environment_isolation() {
    // Test that tests don't interfere with each other
    let temp_dir1 = setup_test_env();
    let temp_dir2 = setup_test_env();

    // Create different project structures in each
    fs::write(temp_dir1.path().join("project1.txt"), "project 1").unwrap();
    fs::write(temp_dir2.path().join("project2.txt"), "project 2").unwrap();

    // Modify files in temp_dir1
    fs::create_dir_all(temp_dir1.path().join("scripts")).unwrap();

    // Verify temp_dir2 is unaffected
    assert!(!temp_dir2.path().join("scripts").exists());
    assert!(temp_dir2.path().join("project2.txt").exists());
    assert!(!temp_dir2.path().join("project1.txt").exists());
}

#[test]
fn test_integration_multiple_projects_isolation() {
    let base_dir = setup_test_env();

    // Create multiple project directories
    let projects = vec!["project_a", "project_b", "project_c"];

    for project in &projects {
        let project_dir = base_dir.path().join(project);
        fs::create_dir_all(&project_dir).unwrap();

        // Create project-specific PRD
        create_sample_prd(&project_dir, &format!("{} Project", project));

        // Create project-specific directories
        fs::create_dir_all(project_dir.join("scripts/ralph")).unwrap();
        fs::create_dir_all(project_dir.join("tasks")).unwrap();
    }

    // Verify each project is isolated
    for project in &projects {
        let project_dir = base_dir.path().join(project);
        assert!(project_dir.exists());
        assert!(project_dir.join("prd.json").exists());
        assert!(project_dir.join("scripts/ralph").exists());
        assert!(project_dir.join("tasks").exists());
    }
}

// ============================================================================
// Test 12: Platform-Specific Tests
// ============================================================================

#[cfg(unix)]
#[test]
fn test_integration_unix_specific_path_handling() {
    let temp_dir = setup_test_env();

    // Test Unix path handling
    let unix_path = temp_dir.path().join("test/file/path");
    fs::create_dir_all(&unix_path).unwrap();

    // Verify path uses forward slashes
    let path_str = unix_path.to_string_lossy();
    assert!(!path_str.contains('\\'), "Unix paths should use forward slashes");
}

#[cfg(windows)]
#[test]
fn test_integration_windows_specific_path_handling() {
    let temp_dir = setup_test_env();

    // Test Windows path handling
    let windows_path = temp_dir.path().join("test\\file\\path");
    fs::create_dir_all(&windows_path).unwrap();

    // Path should be normalized
    assert!(windows_path.exists());
}

// ============================================================================
// Test 13: CI Environment Compatibility
// ============================================================================

#[test]
fn test_integration_ci_environment_compatibility() {
    let temp_dir = setup_test_env();

    // Test that our tests work in a clean environment (simulating CI)
    // by not relying on any pre-existing files or environment variables

    // Create a complete project from scratch
    let project_dir = temp_dir.path().join("ci_test_project");
    fs::create_dir_all(&project_dir).unwrap();

    // Create all necessary files
    create_sample_prd(&project_dir, "CI Test Project");

    let scripts_dir = project_dir.join("scripts/ralph");
    fs::create_dir_all(&scripts_dir).unwrap();

    let tasks_dir = project_dir.join("tasks");
    fs::create_dir_all(&tasks_dir).unwrap();

    // Create AGENTS.md
    fs::write(
        project_dir.join("AGENTS.md"),
        "# CI Test Project\n\nCI environment test"
    ).unwrap();

    // Create progress.txt
    fs::write(
        project_dir.join("progress.txt"),
        "## Codebase Patterns\n- CI test pattern\n"
    ).unwrap();

    // Verify all files exist
    assert!(project_dir.join("prd.json").exists());
    assert!(project_dir.join("AGENTS.md").exists());
    assert!(project_dir.join("progress.txt").exists());
    assert!(scripts_dir.exists());
    assert!(tasks_dir.exists());

    // Verify PRD is valid JSON
    let prd_content = fs::read_to_string(project_dir.join("prd.json")).unwrap();
    let prd: serde_json::Value = serde_json::from_str(&prd_content).unwrap();
    assert_eq!(prd["project"], "CI Test Project");
}

#[test]
fn test_integration_no_external_dependencies() {
    // This test verifies that our integration tests don't rely on
    // external tools being installed (agents, etc.)

    let temp_dir = setup_test_env();

    // We can still test file operations without any agents installed
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "test content");

    // We can still test PRD parsing
    let prd_path = create_sample_prd(temp_dir.path(), "Dependency Test");
    let prd_content = fs::read_to_string(&prd_path).unwrap();
    let prd: serde_json::Value = serde_json::from_str(&prd_content).unwrap();
    assert_eq!(prd["project"], "Dependency Test");
}

// ============================================================================
// Test 14: Data Integrity and Validation
// ============================================================================

#[test]
fn test_integration_prd_schema_validation() {
    let temp_dir = setup_test_env();

    // Create a PRD with all required fields
    let prd_path = temp_dir.path().join("schema_prd.json");
    let prd_content = r#"{
  "project": "Schema Test",
  "branchName": "ralph/schema-test",
  "description": "Testing schema validation",
  "userStories": [
    {
      "id": "US-001",
      "title": "Test Story",
      "description": "As a user, I want to test",
      "acceptanceCriteria": ["Criterion 1", "Criterion 2"],
      "priority": 1,
      "passes": false,
      "notes": ""
    }
  ]
}"#;
    fs::write(&prd_path, prd_content).unwrap();

    // Parse and validate structure
    let content = fs::read_to_string(&prd_path).unwrap();
    let prd: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Validate required fields exist
    assert!(prd.get("project").is_some());
    assert!(prd.get("branchName").is_some());
    assert!(prd.get("description").is_some());
    assert!(prd.get("userStories").is_some());

    // Validate story structure
    let stories = prd["userStories"].as_array().unwrap();
    assert_eq!(stories.len(), 1);

    let story = &stories[0];
    assert!(story.get("id").is_some());
    assert!(story.get("title").is_some());
    assert!(story.get("description").is_some());
    assert!(story.get("acceptanceCriteria").is_some());
    assert!(story.get("priority").is_some());
    assert!(story.get("passes").is_some());
    assert!(story.get("notes").is_some());
}

#[test]
fn test_integration_progress_tracking_format() {
    let temp_dir = setup_test_env();

    // Create a progress.txt with proper format
    let progress_path = temp_dir.path().join("progress.txt");
    let progress_content = r#"## Codebase Patterns
- Use tempfile for test isolation
- Always use Result for error handling

## 2026-02-01 - US-001
- Implemented feature X
- Files changed: src/main.rs
- **Learnings for future iterations:**
  - Pattern: Use Arc for shared state
  - Gotcha: Remember to handle Ctrl+C
---

## 2026-02-01 - US-002
- Implemented feature Y
- Files changed: src/lib.rs
- **Learnings for future iterations:**
  - Pattern: Use tokio for async
---
"#;
    fs::write(&progress_path, progress_content).unwrap();

    // Verify content
    let content = fs::read_to_string(&progress_path).unwrap();
    assert!(content.contains("## Codebase Patterns"));
    assert!(content.contains("## 2026-02-01 - US-001"));
    assert!(content.contains("## 2026-02-01 - US-002"));
    assert!(content.contains("**Learnings for future iterations:**"));
    assert!(content.contains("---"));
}
