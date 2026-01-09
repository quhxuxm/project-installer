use crate::{common::ProjectId, error::Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ToolConfig {
    pub github: GitHubConfig,
    pub projects: HashMap<ProjectId, ProjectConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub github_repo_url: String,
    pub github_branch: String,
    pub local_repo_path: PathBuf,
    pub build_command: Option<CommandConfig>,
    pub run_command: Option<CommandConfig>,
    pub debug_command: Option<CommandConfig>,
    pub startup_dependencies: Vec<ProjectId>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    pub username: String,
    pub token: String,
}

pub fn load_tool_config() -> Result<ToolConfig, Error> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    let tool_config = config.try_deserialize::<ToolConfig>()?;
    Ok(tool_config)
}
