mod command;
mod common;
mod config;
pub mod error;
pub mod repo;
pub mod runtime;
use crate::config::load_tool_config;
use command::load_application_state;
use std::sync::Arc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();
    let tool_config = load_tool_config().expect("Failed to load tool configuration");

    tauri::Builder::default()
        .manage(Arc::new(tool_config))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_application_state,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
