use crate::command::generate_application_state_message;
use crate::common::ProjectId;
use crate::config::{CommandConfig, GitHubConfig, ProjectConfig, ToolConfig};
use crate::message::input::{GithubConfigInput, ProjectConfigInput};
use crate::message::output::{AppRuntimeStateOutput, ErrorOutput};
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub fn save_github_config(
    github_config_input: GithubConfigInput,
    tool_config: State<RwLock<ToolConfig>>,
) -> Result<AppRuntimeStateOutput, ErrorOutput> {
    let mut tool_config = tool_config.write().map_err(|_| ErrorOutput {
        reason: "Failed to read tool configuration".to_string(),
    })?;
    let mut github_config = GitHubConfig::default();
    github_config.set_token(github_config_input.token);
    github_config.set_username(github_config_input.username);
    tool_config.set_github(github_config);
    let application_state = generate_application_state_message(&tool_config)?;
    println!("The updated application state[save_github_config]: {application_state:#?}");
    Ok(application_state)
}

#[tauri::command]
pub fn save_project_config(
    project_config_input: ProjectConfigInput,
    tool_config: State<RwLock<ToolConfig>>,
) -> Result<AppRuntimeStateOutput, ErrorOutput> {
    let mut tool_config = tool_config.write().map_err(|_| ErrorOutput {
        reason: "Failed to read tool configuration".to_string(),
    })?;
    let mut project_config = ProjectConfig::default();
    project_config.set_github_branch(project_config_input.selected_github_branch);
    project_config.set_github_repo_url(project_config_input.github_repo_url);
    project_config.set_local_repo_path(project_config_input.local_repo_path);
    let build_command = CommandConfig::new(project_config_input.build_command);
    let run_command = CommandConfig::new(project_config_input.run_command);
    let debug_command = CommandConfig::new(project_config_input.debug_command);
    project_config.set_build_command(build_command);
    project_config.set_run_command(run_command);
    project_config.set_debug_command(debug_command);
    tool_config.add_project(ProjectId(project_config_input.project_id), project_config);
    let application_state = generate_application_state_message(&tool_config)?;
    println!("The updated application state[save_project_config]: {application_state:#?}");
    Ok(application_state)
}
