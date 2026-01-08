use crate::{common::ProjectId, config::load_tool_config, error::Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRuntimeState {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRuntimeState {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CurrentProcess {
    GitHubPull,
    RunningApplication(CommandRuntimeState),
    DebugingApplication(CommandRuntimeState),
    StopingApplication(CommandRuntimeState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectRuntimeState {
    pub github_branch: String,
    pub github_repository_url: String,
    pub local_repository_path: Option<PathBuf>,
    pub current_process: CurrentProcess,
    pub available_github_branches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppRuntimeState {
    pub github_state: GitHubRuntimeState,
    pub project_states: HashMap<ProjectId, ProjectRuntimeState>,
}

pub fn save_app_runtime_state(state: AppRuntimeState) -> Result<(), Error> {
    Ok(())
}

fn get_github_branches(
    github_repo_url: &str,
    username: &str,
    token: &str,
) -> Result<Vec<String>, Error> {
    todo!()
}
pub fn load_app_runtime_state() -> Result<AppRuntimeState, Error> {
    let tool_config = load_tool_config()?;
    let projects = tool_config.projects();
    let project_states = projects
        .iter()
        .map(|(project_id, project_config)| {
            let github_branch = project_config.github_branch().unwrap_or("main").to_string();
            let github_repository_url = project_config
                .github_repo_url()
                .unwrap_or_default()
                .to_string();
            let local_repository_path = project_config.local_repo_path().map(PathBuf::from);
            (
                project_id.clone(),
                ProjectRuntimeState {
                    available_github_branches: get_github_branches(
                        &github_repository_url,
                        tool_config.github().username().unwrap_or_default(),
                        tool_config.github().token().unwrap_or_default(),
                    )?,
                    github_branch,
                    github_repository_url,
                    local_repository_path,
                    current_process: CurrentProcess::GitHubPull,
                },
            )
        })
        .collect::<HashMap<ProjectId, ProjectRuntimeState>>();
    Ok(AppRuntimeState {
        github_state: GitHubRuntimeState {
            username: tool_config
                .github()
                .username()
                .unwrap_or_default()
                .to_string(),
            token: tool_config.github().token().unwrap_or_default().to_string(),
        },
        project_states,
    })
}
