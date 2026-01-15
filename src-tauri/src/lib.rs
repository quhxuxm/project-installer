mod command;
mod common;
mod config;
pub mod error;
pub mod process;
pub mod repo;
pub mod runtime;

use std::alloc::System;

use command::{
    exec_build_process, get_github_runtime_detail, get_project_code, get_project_runtime_detail,
    get_project_runtime_summaries, save_project,
};
use tauri::async_runtime;
use tokio::runtime::{Builder, Runtime};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let parallelism = std::thread::available_parallelism().expect("Fail to get system parallelism");
    let runtime = Builder::new_multi_thread()
        .worker_threads(parallelism)
        .enable_all()
        .build()
        .expect("Fail to create async runtime");
    tracing_subscriber::fmt().init();
    async_runtime::set(runtime);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_github_runtime_detail,
            get_project_runtime_detail,
            get_project_runtime_summaries,
            get_project_code,
            save_project,
            exec_build_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
