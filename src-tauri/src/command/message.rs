use crate::common::ProjectId;
use crate::runtime::PropertyItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRuntimeUpdate {
    pub project_id: ProjectId,
    pub working_branch: String,
    pub remote_repo_url: String,
    pub local_repo_path: String,
    pub build_command: Option<String>,
    pub debug_command: Option<String>,
    pub run_command: Option<String>,
    pub customized_properties: Vec<PropertyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalLogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLogEvent {
    pub project_id: ProjectId,
    pub message: String,
    pub level: GlobalLogLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalNotificationLevel {
    Success,
    Info,
    Error,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalNotificationEvent {
    pub project_id: ProjectId,
    pub message: String,
    pub summary: String,
    pub level: GlobalNotificationLevel,
}
