use crate::{common::ProjectId, error::Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ToolConfig {
    github: GitHubConfig,
    projects: HashMap<ProjectId, ProjectConfig>,
}

impl ToolConfig {
    pub fn github(&self) -> &GitHubConfig {
        &self.github
    }

    pub fn projects(&self) -> &HashMap<ProjectId, ProjectConfig> {
        &self.projects
    }

    pub fn set_github(&mut self, github: GitHubConfig) {
        self.github = github;
    }

    pub fn add_project(&mut self, project_id: ProjectId, project: ProjectConfig) {
        self.projects.insert(project_id, project);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    command: String,
    args: Vec<String>,
}

impl CommandConfig {
    pub fn new(command: String) -> Self {
        Self {
            command,
            args: vec![],
        }
    }
    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }

    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    name: Option<String>,
    description: Option<String>,
    github_repo_url: Option<String>,
    github_branch: Option<String>,
    local_repo_path: Option<PathBuf>,
    build_command: Option<CommandConfig>,
    run_command: Option<CommandConfig>,
    debug_command: Option<CommandConfig>,
    startup_dependencies: Vec<ProjectId>,
}

impl ProjectConfig {
    pub fn new(id: ProjectId) -> Self {
        Self {
            name: None,
            description: None,
            github_repo_url: None,
            github_branch: None,
            local_repo_path: None,
            build_command: None,
            run_command: None,
            debug_command: None,
            startup_dependencies: vec![],
        }
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn github_repo_url(&self) -> Option<&str> {
        self.github_repo_url.as_deref()
    }

    pub fn github_branch(&self) -> Option<&str> {
        self.github_branch.as_deref()
    }

    pub fn local_repo_path(&self) -> Option<&Path> {
        self.local_repo_path.as_deref()
    }

    pub fn build_command(&self) -> Option<&CommandConfig> {
        self.build_command.as_ref()
    }

    pub fn run_command(&self) -> Option<&CommandConfig> {
        self.run_command.as_ref()
    }

    pub fn debug_command(&self) -> Option<&CommandConfig> {
        self.debug_command.as_ref()
    }

    pub fn startup_dependencies(&self) -> &[ProjectId] {
        &self.startup_dependencies
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_github_repo_url(&mut self, github_repo_url: String) {
        self.github_repo_url = Some(github_repo_url);
    }

    pub fn set_github_branch(&mut self, github_branch: String) {
        self.github_branch = Some(github_branch);
    }

    pub fn set_local_repo_path(&mut self, local_repo_path: PathBuf) {
        self.local_repo_path = Some(local_repo_path);
    }

    pub fn set_build_command(&mut self, build_command: CommandConfig) {
        self.build_command = Some(build_command);
    }

    pub fn set_run_command(&mut self, run_command: CommandConfig) {
        self.run_command = Some(run_command);
    }

    pub fn set_debug_command(&mut self, debug_command: CommandConfig) {
        self.debug_command = Some(debug_command);
    }

    pub fn add_startup_dependency(&mut self, dependency: ProjectId) {
        self.startup_dependencies.push(dependency);
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitHubConfig {
    username: Option<String>,
    token: Option<String>,
}

impl GitHubConfig {
    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

pub fn load_tool_config() -> Result<ToolConfig, Error> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    let tool_config = config.try_deserialize::<ToolConfig>()?;
    Ok(tool_config)
}
