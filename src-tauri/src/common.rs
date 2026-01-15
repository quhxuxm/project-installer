use crate::command::message::{LogEvent, LogLevel};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter};
use tracing::error;

pub static RGS_PMT_DIR: &str = ".rgspmt";
#[derive(Debug, Copy, Clone, Display)]
pub enum BackendEvent {
    #[display("__backend_event_log_message__")]
    LogMessage,
}
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ProjectId(pub String);

impl Display for ProjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ProjectId {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ProjectId {
    fn from(value: String) -> Self {
        ProjectId(value)
    }
}

impl From<&str> for ProjectId {
    fn from(value: &str) -> Self {
        ProjectId(value.to_string())
    }
}

pub fn parse_log_level(line: &str) -> LogLevel {
    if line.to_uppercase().contains("[ERROR]") {
        LogLevel::Error
    } else {
        if line.to_uppercase().contains("[DEBUG]") {
            LogLevel::Debug
        } else {
            if line.to_uppercase().contains("[WARN]") {
                LogLevel::Warn
            } else {
                LogLevel::Info
            }
        }
    }
}

pub fn push_log_to_frontend(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    message: String,
    level: LogLevel,
) {
    if let Err(e) = app_handle.emit(
        &BackendEvent::LogMessage.to_string(),
        LogEvent {
            project_id: project_id.clone(),
            message,
            level,
        },
    ) {
        error!("Fail to emit backend event to frontend: {e:?}")
    };
}

pub fn ack_frontend_action(response_channel: &Channel<bool>) {
    if let Err(e) = response_channel.send(true) {
        error!("Fail ack to frontend action: {e:?}");
    };
}
