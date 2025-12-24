use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallerConfig {
    github_token: String,
    github_username: String,
    projects:HashMap<String,ProjectConfig>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig{
    github_repo_owner: String,
    github_repo_branch: String,
    project_local_path: PathBuf,
    github_repo_url: String,
}