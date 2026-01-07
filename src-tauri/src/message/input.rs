use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubConfigInput {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfigInput {
    pub project_id: String,
    pub github_repo_url: String,
    pub selected_github_branch: String,
    pub local_repo_path: PathBuf,
    pub build_command: String,
    pub run_command: String,
    pub debug_command: String,
}
