//! PRD Parsing Function Tests
//!
//! Tests for PRD JSON parsing functionality including:
//! - Prd::from_file() - loading and parsing valid prd.json
//! - total_stories() - counting total user stories
//! - completed_stories() - counting completed stories
//! - pending_stories() - counting pending stories
//! - highest_priority_pending() - finding next story to work on
//! - mark_story_passed() - updating story status
//! - save_to_file() - persisting PRD changes
//! - Error handling for invalid JSON
//! - Default value handling for missing fields

use std::io::Write;
use tempfile::TempDir;

use crate::prd::{Prd, UserStory};

/// Helper function to create a temporary PRD JSON file
fn create_temp_prd_file(temp_dir: &TempDir, content: &str) -> std::path::PathBuf {
    let file_path = temp_dir.path().join("prd.json");
    let mut file = std::fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

/// Helper function to create a sample valid PRD JSON string
fn sample_valid_prd_json() -> &'static str {
    r#"{
        "project": "Test Project",
        "branchName": "feature/test-branch",
        "description": "A test project description",
        "userStories": [
            {
                "id": "US-001",
                "title": "First Story",
                "description": "As a user, I want to do something",
                "acceptanceCriteria": ["Criteria 1", "Criteria 2"],
                "priority": 1,
                "passes": true,
                "notes": "Completed"
            },
            {
                "id": "US-002",
                "title": "Second Story",
                "description": "As a user, I want to do another thing",
                "acceptanceCriteria": ["Criteria 3"],
                "priority": 2,
                "passes": false,
                "notes": ""
            },
            {
                "id": "US-003",
                "title": "Third Story",
                "description": "As a user, I want a third feature",
                "acceptanceCriteria": [],
                "priority": 3,
                "passes": false,
                "notes": "Not started"
            }
        ]
    }"#
}

#[test]
fn test_prd_from_file_loads_valid_prd() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());

    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.project, "Test Project");
    assert_eq!(prd.branch_name, "feature/test-branch");
    assert_eq!(prd.description, "A test project description");
    assert_eq!(prd.user_stories.len(), 3);
}

#[test]
fn test_prd_from_file_returns_error_for_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.json");

    let result = Prd::from_file(&nonexistent_path);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_prd_from_file_returns_error_for_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_json = "this is not valid json {[";
    let file_path = create_temp_prd_file(&temp_dir, invalid_json);

    let result = Prd::from_file(&file_path);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn test_prd_from_file_returns_error_for_malformed_json() {
    let temp_dir = TempDir::new().unwrap();
    // Valid JSON but missing required fields
    let malformed_json = r#"{"project": "Test"}"#;
    let file_path = create_temp_prd_file(&temp_dir, malformed_json);

    let result = Prd::from_file(&file_path);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn test_total_stories_returns_correct_count() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.total_stories(), 3);
}

#[test]
fn test_total_stories_returns_zero_for_empty_stories() {
    let json = r#"{
        "project": "Empty Project",
        "branchName": "feature/empty",
        "description": "No stories",
        "userStories": []
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.total_stories(), 0);
}

#[test]
fn test_completed_stories_returns_correct_count() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let prd = Prd::from_file(&file_path).unwrap();

    // US-001 has passes: true, US-002 and US-003 have passes: false
    assert_eq!(prd.completed_stories(), 1);
}

#[test]
fn test_completed_stories_returns_zero_when_none_complete() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": false,
                "notes": ""
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.completed_stories(), 0);
}

#[test]
fn test_completed_stories_returns_all_when_all_complete() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story 1",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": true,
                "notes": ""
            },
            {
                "id": "US-002",
                "title": "Story 2",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 2,
                "passes": true,
                "notes": ""
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.completed_stories(), 2);
}

#[test]
fn test_pending_stories_returns_correct_count() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let prd = Prd::from_file(&file_path).unwrap();

    // US-001 has passes: true (completed), US-002 and US-003 have passes: false (pending)
    assert_eq!(prd.pending_stories(), 2);
}

#[test]
fn test_pending_stories_returns_zero_when_all_complete() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": true,
                "notes": ""
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.pending_stories(), 0);
}

#[test]
fn test_branch_name_method_returns_branch_name() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.branch_name(), "feature/test-branch");
}

#[test]
fn test_highest_priority_pending_returns_lowest_priority_number() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let prd = Prd::from_file(&file_path).unwrap();

    // US-001 is completed (priority 1), US-002 is pending (priority 2), US-003 is pending (priority 3)
    // Highest priority pending should be US-002 (lowest priority number among pending)
    let highest = prd.highest_priority_pending().unwrap();
    assert_eq!(highest.id, "US-002");
    assert_eq!(highest.priority, 2);
}

#[test]
fn test_highest_priority_pending_returns_none_when_all_complete() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": true,
                "notes": ""
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert!(prd.highest_priority_pending().is_none());
}

#[test]
fn test_highest_priority_pending_returns_none_for_empty_stories() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": []
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert!(prd.highest_priority_pending().is_none());
}

#[test]
fn test_mark_story_passed_updates_story_status() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let mut prd = Prd::from_file(&file_path).unwrap();

    // Initially US-002 has passes: false
    let story = prd.user_stories.iter().find(|s| s.id == "US-002").unwrap();
    assert!(!story.passes);

    // Mark it as passed
    prd.mark_story_passed("US-002", &file_path).unwrap();

    // Reload and verify
    let updated_prd = Prd::from_file(&file_path).unwrap();
    let updated_story = updated_prd.user_stories.iter().find(|s| s.id == "US-002").unwrap();
    assert!(updated_story.passes);
}

#[test]
fn test_mark_story_passed_does_nothing_for_invalid_story_id() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let mut prd = Prd::from_file(&file_path).unwrap();

    // Try to mark a non-existent story
    let result = prd.mark_story_passed("US-999", &file_path);

    // Should not error, just do nothing
    assert!(result.is_ok());

    // Verify original file is unchanged
    let reloaded_prd = Prd::from_file(&file_path).unwrap();
    assert_eq!(reloaded_prd.completed_stories(), 1);
}

#[test]
fn test_save_to_file_persists_changes() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, sample_valid_prd_json());
    let mut prd = Prd::from_file(&file_path).unwrap();

    // Modify the PRD
    prd.project = "Modified Project".to_string();
    prd.user_stories[0].title = "Modified Title".to_string();

    // Save it
    prd.save_to_file(&file_path).unwrap();

    // Reload and verify
    let reloaded_prd = Prd::from_file(&file_path).unwrap();
    assert_eq!(reloaded_prd.project, "Modified Project");
    assert_eq!(reloaded_prd.user_stories[0].title, "Modified Title");
}

#[test]
fn test_user_story_structure_parsing() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Test Story",
                "description": "As a user, I want to test",
                "acceptanceCriteria": ["Criteria 1", "Criteria 2", "Criteria 3"],
                "priority": 5,
                "passes": false,
                "notes": "Some notes here"
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    let story = &prd.user_stories[0];
    assert_eq!(story.id, "US-001");
    assert_eq!(story.title, "Test Story");
    assert_eq!(story.description, "As a user, I want to test");
    assert_eq!(story.acceptance_criteria.len(), 3);
    assert_eq!(story.acceptance_criteria[0], "Criteria 1");
    assert_eq!(story.acceptance_criteria[1], "Criteria 2");
    assert_eq!(story.acceptance_criteria[2], "Criteria 3");
    assert_eq!(story.priority, 5);
    assert!(!story.passes);
    assert_eq!(story.notes, "Some notes here");
}

#[test]
fn test_user_story_display_format() {
    let story = UserStory {
        id: "US-042".to_string(),
        title: "Test Story Display".to_string(),
        description: "Description".to_string(),
        acceptance_criteria: vec![],
        priority: 1,
        passes: false,
        notes: "".to_string(),
    };

    assert_eq!(story.display(), "US-042 - Test Story Display");
}

#[test]
fn test_empty_acceptance_criteria_handling() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story",
                "description": "Description",
                "acceptanceCriteria": [],
                "priority": 1,
                "passes": false,
                "notes": ""
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert!(prd.user_stories[0].acceptance_criteria.is_empty());
}

#[test]
fn test_special_characters_in_fields() {
    let json = r#"{
        "project": "Test \"Project\"",
        "branchName": "feature/test-branch_with.special-chars",
        "description": "Description with \"quotes\" and \n newlines",
        "userStories": [
            {
                "id": "US-001",
                "title": "Story with \"quotes\"",
                "description": "Description with unicode: 你好世界 🎉",
                "acceptanceCriteria": ["Criteria with <special> & chars"],
                "priority": 1,
                "passes": false,
                "notes": "Notes with\ttabs and\nnewlines"
            }
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.project, "Test \"Project\"");
    assert_eq!(prd.user_stories[0].description, "Description with unicode: 你好世界 🎉");
}

#[test]
fn test_progress_percentage_calculation() {
    let json = r#"{
        "project": "Test",
        "branchName": "feature/test",
        "description": "Test",
        "userStories": [
            {"id": "US-001", "title": "Story 1", "description": "Desc", "acceptanceCriteria": [], "priority": 1, "passes": true, "notes": ""},
            {"id": "US-002", "title": "Story 2", "description": "Desc", "acceptanceCriteria": [], "priority": 2, "passes": true, "notes": ""},
            {"id": "US-003", "title": "Story 3", "description": "Desc", "acceptanceCriteria": [], "priority": 3, "passes": false, "notes": ""},
            {"id": "US-004", "title": "Story 4", "description": "Desc", "acceptanceCriteria": [], "priority": 4, "passes": false, "notes": ""}
        ]
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    // 2 out of 4 stories completed = 50%
    let completed = prd.completed_stories();
    let total = prd.total_stories();
    let percentage = (completed as f64 / total as f64) * 100.0;

    assert_eq!(completed, 2);
    assert_eq!(total, 4);
    assert_eq!(percentage, 50.0);
}

#[test]
fn test_large_number_of_stories() {
    let mut stories = Vec::new();
    for i in 1..=100 {
        stories.push(format!(
            r#"{{
                "id": "US-{}",
                "title": "Story {}",
                "description": "Description {}",
                "acceptanceCriteria": [],
                "priority": {},
                "passes": {},
                "notes": ""
            }}"#,
            i,
            i,
            i,
            i,
            i % 2 == 0 // Even numbers are completed
        ));
    }

    let json = format!(
        r#"{{
            "project": "Large Project",
            "branchName": "feature/large",
            "description": "Test with many stories",
            "userStories": [{}]
        }}"#,
        stories.join(",")
    );

    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, &json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.total_stories(), 100);
    assert_eq!(prd.completed_stories(), 50); // Even numbers from 1-100
    assert_eq!(prd.pending_stories(), 50);
}

#[test]
fn test_prd_with_minimal_fields() {
    // Test that PRD parses correctly with only required fields
    let json = r#"{
        "project": "Minimal",
        "branchName": "feature/minimal",
        "description": "Minimal PRD",
        "userStories": []
    }"#;
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_prd_file(&temp_dir, json);
    let prd = Prd::from_file(&file_path).unwrap();

    assert_eq!(prd.project, "Minimal");
    assert_eq!(prd.total_stories(), 0);
}
