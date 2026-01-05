use crate::common::ProjectId;
use crate::messages::github_state::GitHubState;
use crate::messages::project_state::ProjectState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalState {
    pub github: GitHubState,
    pub projects: HashMap<ProjectId, ProjectState>,
}
