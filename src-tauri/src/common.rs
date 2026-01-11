use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

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
