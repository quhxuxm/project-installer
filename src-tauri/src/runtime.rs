use crate::{
    common::ProjectId,
    config::ToolConfig,
    error::Error,
    repo::{self, RetrieveGitHubBranchesRequest},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRuntimeState {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRuntimeState {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrentProcess {
    GitHubPull,
    RunningApplication(CommandRuntimeState),
    DebugingApplication(CommandRuntimeState),
    StopingApplication(CommandRuntimeState),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeState {
    pub name: String,
    pub description: String,
    pub github_branch: String,
    pub github_repo_url: String,
    pub local_repo_path: PathBuf,
    pub current_process: Option<CurrentProcess>,
    pub available_github_branches: Vec<String>,
    pub build_command: Option<CommandRuntimeState>,
    pub run_command: Option<CommandRuntimeState>,
    pub stop_command: Option<CommandRuntimeState>,
    pub debug_command: Option<CommandRuntimeState>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRuntimeState {
    pub github_state: GitHubRuntimeState,
    pub project_states: HashMap<ProjectId, ProjectRuntimeState>,
}

pub fn load_app_runtime_state(tool_config: &ToolConfig) -> Result<AppRuntimeState, Error> {
    let projects = &tool_config.projects;
    let project_states = projects
        .iter()
        .map_while(|(project_id, project_config)| {
            let github_branch = &project_config.github_branch;
            let github_repo_url = &project_config.github_repo_url;
            let local_repo_path = &project_config.local_repo_path;
            let retrive_branches_info = RetrieveGitHubBranchesRequest {
                github_username: tool_config.github.username.to_string(),
                github_token: tool_config.github.token.to_string(),
                github_repo_url: github_repo_url.to_string(),
                local_repo_path,
                proxy_url: tool_config.proxy_url.clone(),
            };
            let available_github_branches =
                match repo::retrieve_branches_from_github(retrive_branches_info) {
                    Ok(branches) => branches,
                    Err(err) => {
                        error!("Failed to retrieve branches for repo: {err}");
                        vec![]
                    }
                };
            Some((
                project_id.clone(),
                ProjectRuntimeState {
                    name: project_config.name.clone(),
                    description: project_config.description.clone(),
                    available_github_branches,
                    github_branch: github_branch.to_string(),
                    github_repo_url: github_repo_url.to_string(),
                    local_repo_path: local_repo_path.to_owned(),
                    current_process: None,
                    build_command: project_config.build_command.clone().map(|c| {
                        CommandRuntimeState {
                            cmd: c.command.clone(),
                            args: c.args.clone(),
                        }
                    }),
                    run_command: project_config
                        .run_command
                        .clone()
                        .map(|c| CommandRuntimeState {
                            cmd: c.command.clone(),
                            args: c.args.clone(),
                        }),
                    stop_command: project_config.stop_command.clone().map(|c| {
                        CommandRuntimeState {
                            cmd: c.command.clone(),
                            args: c.args.clone(),
                        }
                    }),
                    debug_command: project_config.debug_command.clone().map(|c| {
                        CommandRuntimeState {
                            cmd: c.command.clone(),
                            args: c.args.clone(),
                        }
                    }),
                },
            ))
        })
        .collect::<HashMap<ProjectId, ProjectRuntimeState>>();
    let app_runtime_state = AppRuntimeState {
        github_state: GitHubRuntimeState {
            username: tool_config.github.username.clone(),
            token: tool_config.github.token.clone(),
        },
        project_states,
    };
    //info!("Load application runtime status:{app_runtime_state:#?}");
    Ok(app_runtime_state)
}
