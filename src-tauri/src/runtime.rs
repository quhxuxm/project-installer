use crate::{common::ProjectId, config::load_tool_config, error::Error};
use git2::RemoteCallbacks;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tracing::error;

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
    let projects = &tool_config.projects;
    let project_states = projects
        .iter()
        .map_while(|(project_id, project_config)| {
            let github_branch = &project_config.github_branch;
            let github_repository_url = &project_config.github_repo_url;
            let local_repository_path = &project_config.local_repo_path;
            Some((
                project_id.clone(),
                ProjectRuntimeState {
                    available_github_branches: match get_github_branches(
                        &github_repository_url,
                        &tool_config.github.username,
                        &tool_config.github.token,
                    ) {
                        Ok(branches) => branches,
                        Err(e) => {
                            error!(
                                github_repository_url,
                                "Failed to retrieve GitHub branches: {}", e
                            );
                            vec![]
                        }
                    },
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
