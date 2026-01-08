use crate::config::load_tool_config;
use std::sync::RwLock;

use command::load_application_state;
use command::retrieve_code;
use command::save_github_config;
use command::save_project_config;
mod command;
mod common;
mod config;
pub mod error;
pub mod message;
pub mod runtime;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tool_config = load_tool_config().expect("Failed to load tool configuration");

    tauri::Builder::default()
        .manage(RwLock::new(tool_config))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_application_state,
            save_github_config,
            save_project_config,
            retrieve_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
