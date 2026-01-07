use crate::command::{retrieve_code_from_github, RetrieveCodeFromGithubInfo};
use crate::config::ToolConfig;
use crate::message::input::{GithubConfigInput, ProjectConfigInput};
use crate::message::output::ErrorOutput;
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub fn retrieve_code(
    github_config_input: GithubConfigInput,
    project_config_input: ProjectConfigInput,
    _tool_config: State<RwLock<ToolConfig>>,
) -> Result<(), ErrorOutput> {
    let retrieve_info = RetrieveCodeFromGithubInfo {
        github_username: github_config_input.username.to_string(),
        github_token: github_config_input.token.to_string(),
        github_branch: project_config_input.selected_github_branch,
        github_repo_url: project_config_input.github_repo_url,
        local_repo_path: &project_config_input.local_repo_path,
        proxy_url: None,
    };
    retrieve_code_from_github(retrieve_info)?;
    Ok(())
}
