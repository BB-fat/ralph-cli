use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

/// PRD (Product Requirements Document) structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prd {
    pub project: String,
    #[serde(rename = "branchName")]
    pub branch_name: String,
    pub description: String,
    #[serde(rename = "userStories")]
    pub user_stories: Vec<UserStory>,
}

impl Prd {
    /// Load PRD from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let prd: Prd = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(prd)
    }

    /// Get the branch name
    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    /// Count total user stories
    pub fn total_stories(&self) -> usize {
        self.user_stories.len()
    }

    /// Count completed (passing) user stories
    pub fn completed_stories(&self) -> usize {
        self.user_stories.iter().filter(|s| s.passes).count()
    }

    /// Count pending user stories
    pub fn pending_stories(&self) -> usize {
        self.user_stories.iter().filter(|s| !s.passes).count()
    }

    /// Get the highest priority pending story
    #[allow(dead_code)]
    pub fn highest_priority_pending(&self) -> Option<&UserStory> {
        self.user_stories
            .iter()
            .filter(|s| !s.passes)
            .min_by_key(|s| s.priority)
    }

    /// Update a story's passes field and save back to file
    #[allow(dead_code)]
    pub fn mark_story_passed<P: AsRef<Path>>(&mut self, story_id: &str, path: P) -> io::Result<()> {
        if let Some(story) = self.user_stories.iter_mut().find(|s| s.id == story_id) {
            story.passes = true;
            self.save_to_file(path)?;
        }
        Ok(())
    }

    /// Save PRD to a JSON file
    #[allow(dead_code)]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)?;
        Ok(())
    }
}

/// User Story structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "acceptanceCriteria")]
    pub acceptance_criteria: Vec<String>,
    pub priority: u32,
    pub passes: bool,
    pub notes: String,
}

impl UserStory {
    /// Get formatted display string for the story
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!("{} - {}", self.id, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prd_parsing() {
        let json = r#"{
            "project": "Test Project",
            "branchName": "feature/test",
            "description": "Test description",
            "userStories": [
                {
                    "id": "US-001",
                    "title": "Test Story",
                    "description": "As a user...",
                    "acceptanceCriteria": ["Criteria 1"],
                    "priority": 1,
                    "passes": false,
                    "notes": ""
                }
            ]
        }"#;

        let prd: Prd = serde_json::from_str(json).unwrap();
        assert_eq!(prd.project, "Test Project");
        assert_eq!(prd.branch_name(), "feature/test");
        assert_eq!(prd.total_stories(), 1);
        assert_eq!(prd.completed_stories(), 0);
    }
}
