use crate::{common::ProjectId, error::Error};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::RwLock;
use std::{collections::HashMap, fs, sync::LazyLock};

const CONFIG_FILE_NAME: &str = "config.toml";
pub static TOOL_CONFIG: LazyLock<RwLock<ToolConfig>> =
    LazyLock::new(|| RwLock::new(load_tool_config().expect("Failed to load tool configuration")));

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ToolConfig {
    pub github: GitHubConfig,
    pub projects: HashMap<ProjectId, ProjectConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub github_repo_url: String,
    pub github_branch: String,
    pub local_repo_path: PathBuf,
    pub build_command: Option<String>,
    pub customized_build_command: Option<String>,
    pub run_command: Option<String>,
    pub customized_run_command: Option<String>,
    pub debug_command: Option<String>,
    pub customized_debug_command: Option<String>,
    pub stop_command: Option<String>,
    pub customized_stop_command: Option<String>,
    pub startup_dependencies: Vec<ProjectId>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    pub username: String,
    pub token: String,
    pub proxy: Option<String>,
}

fn load_tool_config() -> Result<ToolConfig, Error> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    let tool_config = config.try_deserialize::<ToolConfig>()?;
    Ok(tool_config)
}

pub fn save_tool_config(tool_config: &ToolConfig) -> Result<(), Error> {
    let content = toml::to_string_pretty(tool_config)?;
    fs::write(CONFIG_FILE_NAME, content)?;
    Ok(())
}
