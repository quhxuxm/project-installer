use crate::common::{ProcessId, ProjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStatus {
    pub github: GitHubStatus,
    pub projects: HashMap<ProjectId, ProjectStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubStatus {
    pub username: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommandStatus {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatus {
    pub name: Option<String>,
    pub description: Option<String>,
    pub github_repo_url: Option<String>,
    pub github_branches: Vec<String>,
    pub configured_github_branch: Option<String>,
    pub local_repo_path: Option<PathBuf>,
    pub build_command: Option<CommandStatus>,
    pub run_command: Option<CommandStatus>,
    pub debug_command: Option<CommandStatus>,
    pub startup_dependencies: Vec<ProjectId>,
    pub backend_process_id: Option<ProcessId>,
}
