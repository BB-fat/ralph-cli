#[cfg(test)]
use crate::agent::Agent;

/// Get the PRD skill content
pub fn get_prd_skill_content() -> String {
    include_str!("templates/prd_skill.md").to_string()
}

/// Get the Ralph skill content
pub fn get_ralph_skill_content() -> String {
    include_str!("templates/ralph_skill.md").to_string()
}

/// Get the agent prompt content (shared by all agents)
pub fn get_agent_prompt() -> &'static str {
    include_str!("templates/prompt.md")
}

/// Get the prd.json.example template content
#[cfg(test)]
pub fn get_prd_json_template(
    project_name: &str,
    project_description: &str,
    _default_tool: Option<Agent>,
) -> String {
    let branch_name = format!("ralph/{}", project_name.to_lowercase().replace(" ", "-"));

    include_str!("templates/prd_json_template.json")
        .replace("{project_name}", project_name)
        .replace("{branch_name}", &branch_name)
        .replace("{project_description}", project_description)
}
