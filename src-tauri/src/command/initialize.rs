use crate::config::ToolConfig;

use crate::command::generate_application_state_message;
use crate::message::output::{ApplicationStateOutput, ErrorOutput};
use std::sync::RwLock;
use tauri::State;

#[tauri::command]
pub fn load_application_state(
    tool_config: State<RwLock<ToolConfig>>,
) -> Result<ApplicationStateOutput, ErrorOutput> {
    let tool_config = tool_config.read().map_err(|_| ErrorOutput {
        reason: "Failed to read tool configuration".to_string(),
    })?;
    let application_state_message = generate_application_state_message(&tool_config)?;
    Ok(application_state_message)
}
