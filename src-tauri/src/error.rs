use config::ConfigError;
use java_properties::PropertiesError;
use tauri::ipc::InvokeError;
use thiserror::Error;

use crate::common::ProjectId;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Local configuration fail: {0}")]
    Config(#[from] ConfigError),
    #[error("GitHub operations fail: {0}")]
    GitHub(#[from] git2::Error),
    #[error("Project not found: {0}")]
    ProjectNotFound(ProjectId),
    #[error("Lock fail")]
    LockFail,
    #[error(transparent)]
    SerializeTomlFail(#[from] toml::ser::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Build command not found")]
    BuildCommandNotFound(ProjectId),
    #[error("Run command not found")]
    RunCommandNotFound(ProjectId),
    #[error("Program part not found: {0}")]
    ProgramPartNotFound(ProjectId),
    #[error(transparent)]
    PropertiesError(#[from] PropertiesError),
    #[error(transparent)]
    TauriShellPluginError(#[from] tauri_plugin_shell::Error),
}

impl From<Error> for InvokeError {
    fn from(value: Error) -> Self {
        InvokeError::from_error(value)
    }
}
