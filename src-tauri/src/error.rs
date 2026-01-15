use config::ConfigError;
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
    #[error("Serialize toml fail: {0}")]
    SerializeTomlFail(#[from] toml::ser::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Build command not found")]
    BuildCommandNotFound(ProjectId),
    #[error("Program part not found: {0}")]
    ProgramPartNotFound(ProjectId),
    #[error("Tauri shell plugin has error: {0}")]
    TauriShellPluginError(#[from] tauri_plugin_shell::Error),
}

impl From<Error> for InvokeError {
    fn from(value: Error) -> Self {
        InvokeError::from_error(value)
    }
}
