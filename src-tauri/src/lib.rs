mod command;
mod common;
mod config;
pub mod error;
pub mod repo;
pub mod runtime;
use command::{
    get_github_runtime_detail, get_project_runtime_detail, get_project_runtime_summaries,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_github_runtime_detail,
            get_project_runtime_detail,
            get_project_runtime_summaries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
