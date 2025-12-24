use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    github_token: String,
    github_username: String,
    proxy: Option<ProxyConfig>,
    projects: HashMap<String, ProjectConfig>,
}

impl ToolConfig {
    pub fn github_token(&self) -> &str {
        &self.github_token
    }

    pub fn github_username(&self) -> &str {
        &self.github_username
    }

    pub fn proxy(&self) -> Option<&ProxyConfig> {
        self.proxy.as_ref()
    }

    pub fn projects(&self) -> &HashMap<String, ProjectConfig> {
        &self.projects
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    github_repo_owner: String,
    github_repo_branch: String,
    project_local_path: PathBuf,
    github_repo_url: String,
    configuration_properties: HashMap<String, String>,
    build_cmd: String,
    startup_cmd: String,
    debug_cmd: String,
}


impl ProjectConfig {
    pub fn github_repo_owner(&self) -> &str {
        &self.github_repo_owner
    }

    pub fn github_repo_branch(&self) -> &str {
        &self.github_repo_branch
    }

    pub fn project_local_path(&self) -> &Path {
        &self.project_local_path
    }

    pub fn github_repo_url(&self) -> &str {
        &self.github_repo_url
    }

    pub fn configuration_properties(&self) -> &HashMap<String, String> {
        &self.configuration_properties
    }

    pub fn build_cmd(&self) -> &str {
        &self.build_cmd
    }

    pub fn startup_cmd(&self) -> &str {
        &self.startup_cmd
    }

    pub fn debug_cmd(&self) -> &str {
        &self.debug_cmd
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyConfig {
    url: String,
}

impl ProxyConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
}
