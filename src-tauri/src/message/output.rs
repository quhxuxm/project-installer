use crate::common::{ProcessId, ProjectId};
use git2::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub reason: String,
}

impl From<String> for ErrorOutput {
    fn from(value: String) -> Self {
        Self { reason: value }
    }
}

impl From<git2::Error> for ErrorOutput {
    fn from(value: Error) -> Self {
        Self {
            reason: value.message().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationStateOutput {
    pub github: GitHubStateOutput,
    pub projects: HashMap<ProjectId, ProjectStateOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubStateOutput {
    pub username: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommandStateOutput {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStateOutput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub github_repo_url: Option<String>,
    pub github_branches: Vec<String>,
    pub configured_github_branch: Option<String>,
    pub local_repo_path: Option<PathBuf>,
    pub build_command: Option<CommandStateOutput>,
    pub run_command: Option<CommandStateOutput>,
    pub debug_command: Option<CommandStateOutput>,
    pub startup_dependencies: Vec<ProjectId>,
    pub backend_process_id: Option<ProcessId>,
}
