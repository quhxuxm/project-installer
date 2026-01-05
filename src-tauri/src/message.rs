use crate::common::{ProcessId, ProjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub reason: String,
}

impl From<String> for ErrorMessage {
    fn from(value: String) -> Self {
        Self { reason: value }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationStateMessage {
    pub github: GitHubStateMessage,
    pub projects: HashMap<ProjectId, ProjectStateMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubStateMessage {
    pub username: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommandStateMessage {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStateMessage {
    pub name: Option<String>,
    pub description: Option<String>,
    pub github_repo_url: Option<String>,
    pub github_branches: Vec<String>,
    pub configured_github_branch: Option<String>,
    pub local_repo_path: Option<PathBuf>,
    pub build_command: Option<CommandStateMessage>,
    pub run_command: Option<CommandStateMessage>,
    pub debug_command: Option<CommandStateMessage>,
    pub startup_dependencies: Vec<ProjectId>,
    pub backend_process_id: Option<ProcessId>,
}
