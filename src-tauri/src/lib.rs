use crate::config::{load_tool_config, ToolConfig};
use crate::messages::command_state::CommandState;
use crate::messages::error_state::ErrorState;
use crate::messages::github_state::GitHubState;
use crate::messages::global_state::GlobalState;
use crate::messages::project_state::ProjectState;
use tauri::State;

mod common;
mod config;
mod messages;

fn initialize_global_status(tool_config: &ToolConfig) -> Result<GlobalState, ErrorState> {
    let github_config = tool_config.github();
    let projects_config = tool_config.projects();
    let global_status = GlobalState {
        github: GitHubState {
            username: github_config.username().map(|v| v.to_owned()),
            token: github_config.token().map(|v| v.to_owned()),
        },
        projects: projects_config
            .iter()
            .map(|(k, v)| {
                let project_status = ProjectState {
                    name: v.name().map(|i| i.to_owned()),
                    description: v.description().map(|i| i.to_owned()),
                    github_repo_url: v.github_repo_url().map(|i| i.to_owned()),
                    github_branches: vec![],
                    configured_github_branch: v.github_branch().map(|i| i.to_owned()),
                    local_repo_path: v.local_repo_path().map(|i| i.to_owned()),
                    build_command: v.build_command().map(|i| {
                        let command_status = CommandState {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    run_command: v.run_command().map(|i| {
                        let command_status = CommandState {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    debug_command: v.debug_command().map(|i| {
                        let command_status = CommandState {
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
fn load_global_state(application_state: State<GlobalState>) -> Result<GlobalState, ErrorState> {
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
        .invoke_handler(tauri::generate_handler![load_global_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
