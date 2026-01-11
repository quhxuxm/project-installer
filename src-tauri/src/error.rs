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
}

impl From<Error> for InvokeError {
    fn from(value: Error) -> Self {
        InvokeError::from_error(value)
    }
}
