use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    reason: String,
}

impl From<String> for ErrorMessage {
    fn from(value: String) -> Self {
        Self { reason: value }
    }
}
