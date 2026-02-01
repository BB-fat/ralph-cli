//! Agent Detection Tests
//!
//! Tests for the agent detection functionality in Ralph CLI.
//! These tests verify that the system correctly detects installed AI agents.

use crate::agent::{Agent, detect_agents, is_command_available};

/// Test that detect_agents returns a list of available agents
#[test]
fn test_detect_agents_returns_correct_list() {
    let detected = detect_agents();

    // The result should be a vector
    // We can't predict which agents are installed, but we can verify structure
    for agent in &detected {
        // Each detected agent should have a valid command
        let cmd: &str = agent.command();
        assert!(!cmd.is_empty(), "Agent command should not be empty");

        // The command should be available
        assert!(
            is_command_available(cmd),
            "Detected agent {} should have available command",
            agent.name()
        );
    }
}

/// Test that is_command_available correctly checks if a command exists in PATH
#[test]
fn test_is_command_available_with_existing_command() {
    // 'cargo' should be available in a Rust development environment
    assert!(
        is_command_available("cargo"),
        "cargo should be available in PATH"
    );
}

/// Test that is_command_available returns false for non-existent commands
#[test]
fn test_is_command_available_with_nonexistent_command() {
    // Use a command name that definitely doesn't exist
    let fake_cmd = "this_command_definitely_does_not_exist_12345";
    assert!(
        !is_command_available(fake_cmd),
        "Non-existent command should return false"
    );
}

/// Test that is_command_available handles commands that fail with --version
#[test]
fn test_is_command_available_with_failing_command() {
    // Test with an invalid command that will fail
    // Using a command that exists but might fail with --version
    // We just need to ensure it doesn't panic
    let _result = is_command_available("false");
    // The function should not panic, result depends on whether 'false' is available
}

/// Test empty result handling when no agents are installed
#[test]
fn test_detect_agents_empty_result() {
    // We can't control the environment to ensure no agents are installed,
    // but we can verify the function handles empty results gracefully
    let detected = detect_agents();

    // The function should always return a valid vector (even if empty)
    // This test documents the expected behavior
    assert!(detected.len() <= 3, "Should detect at most 3 agents");
}

/// Test detection when multiple agents might be present
#[test]
fn test_detect_agents_multiple_agents() {
    let detected = detect_agents();

    // Verify no duplicates
    let mut unique_agents: Vec<_> = detected.clone();
    unique_agents.sort_by(|a: &Agent, b: &Agent| {
        a.command().cmp(b.command())
    });
    unique_agents.dedup_by(|a: &mut Agent, b: &mut Agent| a.command() == b.command());

    assert_eq!(
        detected.len(),
        unique_agents.len(),
        "detect_agents should not return duplicate agents"
    );
}

// Note: Agent constant getter tests removed - they test simple constant returns

/// Test Agent global skills directory path structure
#[test]
fn test_agent_global_skills_dir_structure() {
    // We can't test the exact path (depends on system),
    // but we can verify the structure is correct

    // Amp should use config_dir
    let amp_dir = Agent::Amp.global_skills_dir();
    if let Some(path) = amp_dir {
        let path_str: std::borrow::Cow<'_, str> = path.to_string_lossy();
        assert!(path_str.contains("amp"));
        assert!(path.to_string_lossy().contains("skills"));
    }

    // Claude should use home_dir
    let claude_dir = Agent::Claude.global_skills_dir();
    if let Some(path) = claude_dir {
        let path_str: std::borrow::Cow<'_, str> = path.to_string_lossy();
        assert!(path_str.contains(".claude"));
        assert!(path.to_string_lossy().contains("skills"));
    }

    // CodeBuddy should use home_dir
    let codebuddy_dir = Agent::CodeBuddy.global_skills_dir();
    if let Some(path) = codebuddy_dir {
        let path_str: std::borrow::Cow<'_, str> = path.to_string_lossy();
        assert!(path_str.contains(".codebuddy"));
        assert!(path.to_string_lossy().contains("skills"));
    }
}

/// Test that all agents have unique commands
#[test]
fn test_agent_commands_unique() {
    let commands = vec![
        Agent::Amp.command(),
        Agent::Claude.command(),
        Agent::CodeBuddy.command(),
    ];

    let mut unique = commands.clone();
    unique.sort();
    unique.dedup();

    assert_eq!(
        commands.len(),
        unique.len(),
        "All agent commands should be unique"
    );
}

/// Test that all agents have unique names
#[test]
fn test_agent_names_unique() {
    let names = vec![
        Agent::Amp.name(),
        Agent::Claude.name(),
        Agent::CodeBuddy.name(),
    ];

    let mut unique = names.clone();
    unique.sort();
    unique.dedup();

    assert_eq!(
        names.len(),
        unique.len(),
        "All agent names should be unique"
    );
}


/// Integration test: Verify detected agents match manual detection
#[test]
fn test_detect_agents_manual_verification() {
    // Manually check each agent
    let agents = vec![Agent::Amp, Agent::Claude, Agent::CodeBuddy];
    let mut manually_detected = Vec::new();

    for agent in &agents {
        if is_command_available(agent.command()) {
            let _: &Agent = agent;
            manually_detected.push(*agent);
        }
    }

    let auto_detected = detect_agents();

    assert_eq!(
        manually_detected.len(),
        auto_detected.len(),
        "Manual and automatic detection should find the same number of agents"
    );

    // Verify each manually detected agent is in the auto-detected list
    for agent in &manually_detected {
        assert!(
            auto_detected.contains(agent),
            "Agent {} should be in detected list",
            agent.name()
        );
    }
}
