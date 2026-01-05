use crate::common::{ProcessId, ProjectId};
use crate::messages::command_state::CommandState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectState {
    pub name: Option<String>,
    pub description: Option<String>,
    pub github_repo_url: Option<String>,
    pub github_branches: Vec<String>,
    pub configured_github_branch: Option<String>,
    pub local_repo_path: Option<PathBuf>,
    pub build_command: Option<CommandState>,
    pub run_command: Option<CommandState>,
    pub debug_command: Option<CommandState>,
    pub startup_dependencies: Vec<ProjectId>,
    pub backend_process_id: Option<ProcessId>,
}
