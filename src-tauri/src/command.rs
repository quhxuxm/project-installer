use crate::config::ToolConfig;

use crate::error::Error;
use crate::runtime::{load_app_runtime_state, AppRuntimeState};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn load_application_state(state: State<Arc<ToolConfig>>) -> Result<AppRuntimeState, Error> {
    load_app_runtime_state(&state)
}
