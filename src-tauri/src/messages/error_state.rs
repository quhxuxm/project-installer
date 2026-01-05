use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorState {
    reason: String,
}

impl From<String> for ErrorState {
    fn from(value: String) -> Self {
        Self { reason: value }
    }
}
