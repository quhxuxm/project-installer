use crate::command::message::{
    GlobalLogEvent, GlobalLogLevel, GlobalNotificationEvent, GlobalNotificationLevel,
};
use crate::config::ProjectConfig;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter};
use tracing::error;

pub static RGS_PMT_DIR: &str = ".rgspmt";
pub static GIT_DIR: &str = ".git";
#[derive(Debug, Copy, Clone, Display)]
pub enum BackendEvent {
    #[display("__backend_event_global_log__")]
    GlobalLogEvent,
    #[display("__backend_event_global_notification__")]
    GlobalNotificationEvent,
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

pub fn parse_global_log_level(line: &str) -> GlobalLogLevel {
    if line.to_uppercase().contains("[ERROR]") {
        GlobalLogLevel::Error
    } else {
        if line.to_uppercase().contains("[DEBUG]") {
            GlobalLogLevel::Debug
        } else {
            if line.to_uppercase().contains("[WARN]") {
                GlobalLogLevel::Warn
            } else {
                GlobalLogLevel::Info
            }
        }
    }
}

pub fn push_global_log_to_frontend(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    message: String,
    level: GlobalLogLevel,
) {
    if let Err(e) = app_handle.emit(
        &BackendEvent::GlobalLogEvent.to_string(),
        GlobalLogEvent {
            project_id: project_id.clone(),
            message,
            level,
        },
    ) {
        error!("Fail to emit backend event to frontend: {e:?}")
    };
}

pub fn push_global_notification_to_frontend(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    message: String,
    summary: String,
    level: GlobalNotificationLevel,
) {
    if let Err(e) = app_handle.emit(
        &BackendEvent::GlobalNotificationEvent.to_string(),
        GlobalNotificationEvent {
            project_id: project_id.clone(),
            message,
            summary,
            level,
        },
    ) {
        error!("Fail to emit backend notification to frontend: {e:?}")
    };
}

pub fn ack_frontend_action(response_channel: &Channel<bool>) {
    if let Err(e) = response_channel.send(true) {
        error!("Fail ack to frontend action: {e:?}");
    };
}

pub fn generate_customized_properties_dir(project: &ProjectConfig) -> PathBuf {
    project
        .local_repo_path
        .join(format!("{}-configuration", project.working_branch))
}
