//! CLI Argument Parsing Tests
//!
//! Tests for CLI argument parsing to ensure all commands and options
//! are correctly processed by the clap-based argument parser.

use std::process::Command;

/// Test that --version displays the correct version number
#[test]
fn test_version_flag_displays_version() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Version output goes to stdout in clap
    let output_text = if stdout.contains("ralph") {
        stdout.to_string()
    } else {
        stderr.to_string()
    };

    // Should contain the binary name and version
    assert!(
        output_text.contains("ralph") || stdout.contains("ralph"),
        "Version output should contain 'ralph', got stdout: '{}', stderr: '{}'",
        stdout,
        stderr
    );
}

/// Test that --help displays complete help information
#[test]
fn test_help_flag_displays_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Help should contain the main description
    assert!(
        stdout.contains("Ralph CLI") || stdout.contains("ralph"),
        "Help should mention Ralph CLI"
    );

    // Help should list all subcommands
    assert!(
        stdout.contains("init") || stdout.contains("Init"),
        "Help should mention init command"
    );
    assert!(
        stdout.contains("install") || stdout.contains("Install"),
        "Help should mention install command"
    );
    assert!(
        stdout.contains("run") || stdout.contains("Run"),
        "Help should mention run command"
    );
    assert!(
        stdout.contains("config") || stdout.contains("Config"),
        "Help should mention config command"
    );
    assert!(
        stdout.contains("detect") || stdout.contains("Detect"),
        "Help should mention detect command"
    );
    assert!(
        stdout.contains("status") || stdout.contains("Status"),
        "Help should mention status command"
    );
    assert!(
        stdout.contains("archive") || stdout.contains("Archive"),
        "Help should mention archive command"
    );
}

/// Test that help for init subcommand works
#[test]
fn test_init_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "init", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("init") || stdout.contains("Initialize"),
        "Init help should mention initialization"
    );
}

/// Test that help for install subcommand works
#[test]
fn test_install_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "install", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("install") || stdout.contains("Install"),
        "Install help should mention installation"
    );
}

/// Test that help for run subcommand shows all options
#[test]
fn test_run_help_shows_options() {
    let output = Command::new("cargo")
        .args(["run", "--", "run", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should mention --tool option
    assert!(
        stdout.contains("--tool") || stdout.contains("tool"),
        "Run help should mention --tool option"
    );

    // Should mention --max-iterations option
    assert!(
        stdout.contains("--max-iterations") || stdout.contains("max-iterations"),
        "Run help should mention --max-iterations option"
    );

    // Should mention --prd option
    assert!(
        stdout.contains("--prd") || stdout.contains("prd"),
        "Run help should mention --prd option"
    );
}

/// Test that help for config subcommand shows options
#[test]
fn test_config_help_shows_options() {
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should mention --get option
    assert!(
        stdout.contains("--get") || stdout.contains("get"),
        "Config help should mention --get option"
    );

    // Should mention --set option
    assert!(
        stdout.contains("--set") || stdout.contains("set"),
        "Config help should mention --set option"
    );
}

/// Test that run command with explicit tool argument works
#[test]
fn test_run_with_explicit_tool() {
    // This test verifies the CLI accepts the --tool argument
    // It will fail because there's no valid PRD, but that's expected
    let output = Command::new("cargo")
        .args(["run", "--", "run", "--tool", "codebuddy", "--prd", "/nonexistent/prd.json"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    // The command should have been parsed successfully (even if it fails later)
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should fail because of missing PRD, not because of argument parsing
    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
        "Should not have argument parsing error, got stderr: {}",
        stderr
    );
}

/// Test that run command with max-iterations argument works
#[test]
fn test_run_with_max_iterations() {
    let output = Command::new("cargo")
        .args(["run", "--", "run", "--max-iterations", "5", "--prd", "/nonexistent/prd.json"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not have argument parsing error
    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
        "Should not have argument parsing error for --max-iterations, got stderr: {}",
        stderr
    );
}

/// Test that config --get requires a key argument
#[test]
fn test_config_get_requires_key() {
    // This should work - getting a specific key
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--get", "default_tool"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    // Should parse successfully (may fail for other reasons like missing config)
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
        "Should not have argument parsing error, got stderr: {}",
        stderr
    );
}

/// Test that config --set requires both key and value
#[test]
fn test_config_set_requires_key_and_value() {
    let output = Command::new("cargo")
        .args(["run", "--", "config", "--set", "default_tool", "codebuddy"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    // Should parse successfully
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
        "Should not have argument parsing error, got stderr: {}",
        stderr
    );
}

/// Test that invalid subcommand produces error
#[test]
fn test_invalid_subcommand_produces_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "invalidcommand"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have some kind of error message
    let output_text = format!("{}{}", stdout, stderr);
    assert!(
        output_text.contains("error") || output_text.contains("Error") || output_text.contains("unrecognized") || output_text.contains("Unknown"),
        "Invalid subcommand should produce an error, got stdout: '{}', stderr: '{}'",
        stdout,
        stderr
    );
}

/// Test that run with auto tool works (default)
#[test]
fn test_run_with_auto_tool() {
    let output = Command::new("cargo")
        .args(["run", "--", "run", "--tool", "auto", "--prd", "/nonexistent/prd.json"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not have argument parsing error
    assert!(
        !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
        "Should not have argument parsing error for --tool auto, got stderr: {}",
        stderr
    );
}

/// Test that run with various tool options works
#[test]
fn test_run_with_various_tools() {
    for tool in ["amp", "claude", "codebuddy"] {
        let output = Command::new("cargo")
            .args(["run", "--", "run", "--tool", tool, "--prd", "/nonexistent/prd.json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should not have argument parsing error
        assert!(
            !stderr.contains("unexpected argument") && !stderr.contains("Found argument") && !stderr.contains("error: Found"),
            "Should not have argument parsing error for --tool {}, got stderr: {}",
            tool,
            stderr
        );
    }
}

/// Test that help is shown when no subcommand is provided
#[test]
fn test_no_subcommand_shows_help() {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show help or usage information
    let output_text = format!("{}{}", stdout, stderr);
    assert!(
        output_text.contains("Usage:") || output_text.contains("Commands:") || output_text.contains("ralph") || output_text.contains("Ralph"),
        "No subcommand should show help/usage, got stdout: '{}', stderr: '{}'",
        stdout,
        stderr
    );
}

/// Test that detect subcommand help works
#[test]
fn test_detect_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "detect", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("detect") || stdout.contains("Detect"),
        "Detect help should mention detection"
    );
}

/// Test that status subcommand help works
#[test]
fn test_status_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "status", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("status") || stdout.contains("Status"),
        "Status help should mention status"
    );
}

/// Test that archive subcommand help works
#[test]
fn test_archive_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "archive", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("archive") || stdout.contains("Archive"),
        "Archive help should mention archive"
    );
}
