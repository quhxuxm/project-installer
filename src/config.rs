use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInstallerConfig{
    github_token: String,
    github_username: String,
}

pub struct RepoConfig{
    repo_name: String,
    repo_owner: String,
    repo_branch: String,
    repo_path: String,
}