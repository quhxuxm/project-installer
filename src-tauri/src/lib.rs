use crate::config::{load_tool_config, ToolConfig};

use crate::message::{
    ApplicationStateMessage, CommandStateMessage, ErrorMessage, GitHubStateMessage,
    ProjectStateMessage,
};
use tauri::State;

mod common;
mod config;
pub mod message;

fn generate_application_state_message(
    tool_config: &ToolConfig,
) -> Result<ApplicationStateMessage, ErrorMessage> {
    let github_config = tool_config.github();
    let projects_config = tool_config.projects();
    let application_state_message = ApplicationStateMessage {
        github: GitHubStateMessage {
            username: github_config.username().map(|v| v.to_owned()),
            token: github_config.token().map(|v| v.to_owned()),
        },
        projects: projects_config
            .iter()
            .map(|(k, v)| {
                let project_state = ProjectStateMessage {
                    name: v.name().map(|i| i.to_owned()),
                    description: v.description().map(|i| i.to_owned()),
                    github_repo_url: v.github_repo_url().map(|i| i.to_owned()),
                    github_branches: vec![],
                    configured_github_branch: v.github_branch().map(|i| i.to_owned()),
                    local_repo_path: v.local_repo_path().map(|i| i.to_owned()),
                    build_command: v.build_command().map(|i| {
                        let command_status = CommandStateMessage {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    run_command: v.run_command().map(|i| {
                        let command_status = CommandStateMessage {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    debug_command: v.debug_command().map(|i| {
                        let command_status = CommandStateMessage {
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
    Ok(application_state_message)
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn load_application_state(
    tool_config: State<ToolConfig>,
) -> Result<ApplicationStateMessage, ErrorMessage> {
    let application_state_message = generate_application_state_message(&tool_config)?;
    Ok(application_state_message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tool_config = load_tool_config().expect("Failed to load tool configuration");
    tauri::Builder::default()
        .manage(tool_config)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_application_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
