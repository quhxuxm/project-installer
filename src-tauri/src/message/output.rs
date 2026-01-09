use crate::common::{ProcessId, ProjectId};
use crate::error::Error;
use crate::runtime::AppRuntimeState;

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

impl From<Error> for ErrorOutput {
    fn from(value: Error) -> Self {
        Self {
            reason: value.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppRuntimeStateOutput {
    pub github: GitHubRuntimeStateOutput,
    pub projects: HashMap<ProjectId, ProjectRuntimeStateOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRuntimeStateOutput {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommandRuntimeStateOutput {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeStateOutput {
    pub name: String,
    pub description: String,
    pub github_repo_url: String,
    pub github_branches: Vec<String>,
    pub configured_github_branch: String,
    pub local_repo_path: PathBuf,
    pub build_command: Option<CommandRuntimeStateOutput>,
    pub run_command: Option<CommandRuntimeStateOutput>,
    pub debug_command: Option<CommandRuntimeStateOutput>,
    pub startup_dependencies: Vec<ProjectId>,
    pub backend_process_id: ProcessId,
}

fn generate_app_runtime_state_message(
    runtime_state: &AppRuntimeState,
) -> Result<AppRuntimeStateOutput, ErrorOutput> {
    let github_state = &runtime_state.github_state;
    let project_states = &runtime_state.project_states;
    let application_state = AppRuntimeStateOutput {
        github: GitHubRuntimeStateOutput {
            username: github_config.username().map(|v| v.to_owned()),
            token: github_config.token().map(|v| v.to_owned()),
        },
        projects: projects_config
            .iter()
            .map(|(k, v)| {
                let project_state = ProjectRuntimeStateOutput {
                    name: v.name().map(|i| i.to_owned()),
                    description: v.description().map(|i| i.to_owned()),
                    github_repo_url: v.github_repo_url().map(|i| i.to_owned()),
                    github_branches: vec![
                        "6.9.0-release".to_string(),
                        "6.10.0-release".to_string(),
                        "6.9-develop".to_string(),
                        "6.10.0-develop".to_string(),
                    ],
                    configured_github_branch: v.github_branch().map(|i| i.to_owned()),
                    local_repo_path: v.local_repo_path().map(|i| i.to_owned()),
                    build_command: v.build_command().map(|i| {
                        let command_status = CommandRuntimeStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    run_command: v.run_command().map(|i| {
                        let command_status = CommandRuntimeStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    debug_command: v.debug_command().map(|i| {
                        let command_status = CommandRuntimeStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    startup_dependencies: v.startup_dependencies().to_vec(),
                    backend_process_id: None,
                };
                (k.clone(), project_state)
            })
            .collect(),
    };
    Ok(application_state)
}
