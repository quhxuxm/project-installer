use crate::common::ProjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeUpdate {
    pub project_id: ProjectId,
    pub github_branch: String,
    pub github_repo_url: String,
    pub local_repo_path: String,
    pub build_command: Option<String>,
    pub debug_command: Option<String>,
    pub run_command: Option<String>,
}
