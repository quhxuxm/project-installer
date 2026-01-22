use crate::common::ProjectId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeUpdate {
    pub project_id: ProjectId,
    pub working_branch: String,
    pub remote_repo_url: String,
    pub local_repo_path: String,
    pub build_command: Option<String>,
    pub debug_command: Option<String>,
    pub run_command: Option<String>,
    pub customized_properties: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalLogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLogEvent {
    pub project_id: ProjectId,
    pub message: String,
    pub level: GlobalLogLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalNotificationLevel {
    Success,
    Info,
    Error,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalNotificationEvent {
    pub project_id: ProjectId,
    pub message: String,
    pub summary: String,
    pub level: GlobalNotificationLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRuntimeDetail {
    pub username: String,
    pub token: String,
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CommandType {
    Save,
    FetchCode,
    Build,
    Run,
    Debug,
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum RunningCommandStatus {
    #[serde(rename_all = "camelCase")]
    Running {
        command_type: CommandType,
        project_id: ProjectId,
    },
    #[serde(rename_all = "camelCase")]
    TerminatedFailure {
        command_type: CommandType,
        project_id: ProjectId,
    },
    #[serde(rename_all = "camelCase")]
    TerminatedSuccess {
        command_type: CommandType,
        project_id: ProjectId,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeDetail {
    pub name: String,
    pub description: String,
    pub working_branch: String,
    pub remote_repo_url: String,
    pub local_repo_path: PathBuf,
    pub current_running_command_status: Option<RunningCommandStatus>,
    pub available_branches: Vec<String>,
    pub build_command: Option<String>,
    pub run_command: Option<String>,
    pub stop_command: Option<String>,
    pub debug_command: Option<String>,
    pub default_customized_properties: String,
    pub customized_build_command: Option<String>,
    pub customized_run_command: Option<String>,
    pub customized_stop_command: Option<String>,
    pub customized_debug_command: Option<String>,
    pub branch_customized_properties: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeSummary {
    pub project_id: ProjectId,
    pub name: String,
    pub description: String,
}
