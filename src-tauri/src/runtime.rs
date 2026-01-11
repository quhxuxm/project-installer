use crate::{common::ProjectId, config::TOOL_CONFIG, error::Error, repo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRuntimeDetail {
    pub username: String,
    pub token: String,
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRuntimeState {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrentProcess {
    GitHubPull,
    BuildingApplication(CommandRuntimeState),
    RunningApplication(CommandRuntimeState),
    DebugingApplication(CommandRuntimeState),
    StopingApplication(CommandRuntimeState),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeDetail {
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
pub struct ProjectRuntimeSummary {
    pub project_id: ProjectId,
    pub name: String,
    pub description: String,
}

pub fn load_github_runtime_detail() -> Result<GitHubRuntimeDetail, Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    Ok(GitHubRuntimeDetail {
        username: tool_config.github.username.clone(),
        token: tool_config.github.token.clone(),
        proxy: tool_config.github.proxy.clone(),
    })
}

pub fn load_project_runtime_summaries() -> Result<Vec<ProjectRuntimeSummary>, Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let mut summaries = tool_config
        .projects
        .iter()
        .map(|(project_id, project_config)| ProjectRuntimeSummary {
            project_id: project_id.clone(),
            name: project_config.name.clone(),
            description: project_config.description.clone(),
        })
        .collect::<Vec<_>>();
    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

pub fn load_project_runtime_detail(project_id: &ProjectId) -> Result<ProjectRuntimeDetail, Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let project_config = tool_config
        .projects
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let available_github_branches = repo::get_project_github_branches(project_id)?;
    let project_runtime_detail = ProjectRuntimeDetail {
        name: project_config.name.clone(),
        description: project_config.description.clone(),
        available_github_branches,
        github_branch: project_config.github_branch.clone(),
        github_repo_url: project_config.github_repo_url.clone(),
        local_repo_path: project_config.local_repo_path.clone(),
        current_process: None,
        build_command: project_config
            .build_command
            .clone()
            .map(|c| CommandRuntimeState {
                command: c.command.clone(),
                args: c.args.clone(),
            }),
        run_command: project_config
            .run_command
            .clone()
            .map(|c| CommandRuntimeState {
                command: c.command.clone(),
                args: c.args.clone(),
            }),
        stop_command: project_config
            .stop_command
            .clone()
            .map(|c| CommandRuntimeState {
                command: c.command.clone(),
                args: c.args.clone(),
            }),
        debug_command: project_config
            .debug_command
            .clone()
            .map(|c| CommandRuntimeState {
                command: c.command.clone(),
                args: c.args.clone(),
            }),
    };
    Ok(project_runtime_detail)
}
