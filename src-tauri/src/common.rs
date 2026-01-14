use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use derive_more::Display;
use crate::command::message::LogLevel;

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

pub fn parse_log_level_for_frontend(line: &str)->LogLevel{
    if line.to_uppercase().contains("[ERROR]") {
        LogLevel::Error
    }else{
        if line.to_uppercase().contains("[DEBUG]") {
            LogLevel::Debug
        }else{
            if line.to_uppercase().contains("[WARN]") {
                LogLevel::Warn
            }else{
                LogLevel::Info
            }
        }
    }
}
