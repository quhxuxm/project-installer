use config::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Local configuration fail: {0}")]
    Config(#[from] ConfigError),
    #[error("GitHub operations fail: {0}")]
    GitHub(#[from] git2::Error),
}
