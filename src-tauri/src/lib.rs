use crate::config::{load_tool_config, ToolConfig};
use crate::values::error::ErrorMessage;
use crate::values::status::{CommandStatus, GitHubStatus, GlobalStatus, ProjectStatus};

use tauri::State;

mod common;
mod config;
mod values;

fn initialize_global_status(tool_config: &ToolConfig) -> Result<GlobalStatus, ErrorMessage> {
    let github_config = tool_config.github();
    let projects_config = tool_config.projects();
    let global_status = GlobalStatus {
        github: GitHubStatus {
            username: github_config.username().map(|v| v.to_owned()),
            token: github_config.token().map(|v| v.to_owned()),
        },
        projects: projects_config
            .iter()
            .map(|(k, v)| {
                let project_status = ProjectStatus {
                    name: v.name().map(|i| i.to_owned()),
                    description: v.description().map(|i| i.to_owned()),
                    github_repo_url: v.github_repo_url().map(|i| i.to_owned()),
                    github_branches: vec![],
                    configured_github_branch: v.github_branch().map(|i| i.to_owned()),
                    local_repo_path: v.local_repo_path().map(|i| i.to_owned()),
                    build_command: v.build_command().map(|i| {
                        let command_status = CommandStatus {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    run_command: v.run_command().map(|i| {
                        let command_status = CommandStatus {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    debug_command: v.debug_command().map(|i| {
                        let command_status = CommandStatus {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    startup_dependencies: v.startup_dependencies().to_vec(),
                    backend_process_id: None,
                };
                (k.clone(), project_status)
            })
            .collect(),
    };
    Ok(global_status)
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn load_global_status(
    application_state: State<GlobalStatus>,
) -> Result<GlobalStatus, ErrorMessage> {
    Ok(application_state.inner().clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tool_config = load_tool_config().expect("Failed to load tool configuration");
    let global_status =
        initialize_global_status(&tool_config).expect("Fail to initialize global status.");
    println!("Global status:\n{:#?}", global_status);
    tauri::Builder::default()
        .manage(global_status)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_global_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
