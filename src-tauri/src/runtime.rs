use crate::{
    common::ProjectId,
    config::load_tool_config,
    error::Error,
    repo::{self, RetrieveGitHubBranchesRequest},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const DEFAULT_GITHUB_BRANCH: &str = "main";

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
    pub local_repository_path: PathBuf,
    pub current_process: CurrentProcess,
    pub available_github_branches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppRuntimeState {
    pub github_state: GitHubRuntimeState,
    pub project_states: HashMap<ProjectId, ProjectRuntimeState>,
}

pub fn load_app_runtime_state() -> Result<AppRuntimeState, Error> {
    let tool_config = load_tool_config()?;
    let projects = &tool_config.projects;
    let project_states = projects
        .iter()
        .map_while(|(project_id, project_config)| {
            let github_branch = &project_config.github_branch;
            let github_repository_url = &project_config.github_repo_url;
            let local_repository_path = &project_config.local_repo_path;
            let retrive_branches_info = RetrieveGitHubBranchesRequest {
                github_username: tool_config.github.username.to_string(),
                github_token: tool_config.github.token.to_string(),
                github_repo_url: github_repository_url.to_string(),
                local_repo_path: &local_repository_path,
                proxy_url: None,
            };
            let available_github_branches =
                repo::retrieve_branches_from_github(retrive_branches_info).ok()?;
            Some((
                project_id.clone(),
                ProjectRuntimeState {
                    available_github_branches,
                    github_branch: github_branch.to_string(),
                    github_repository_url: github_repository_url.to_string(),
                    local_repository_path: local_repository_path.to_owned(),
                    current_process: CurrentProcess::GitHubPull,
                },
            ))
        })
        .collect::<HashMap<ProjectId, ProjectRuntimeState>>();
    Ok(AppRuntimeState {
        github_state: GitHubRuntimeState {
            username: tool_config.github.username,
            token: tool_config.github.token,
        },
        project_states,
    })
}
