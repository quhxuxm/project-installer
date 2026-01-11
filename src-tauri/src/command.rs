use crate::runtime::{
    load_github_runtime_detail, load_project_runtime_detail, load_project_runtime_summaries,
    GitHubRuntimeDetail, ProjectRuntimeDetail, ProjectRuntimeSummary,
};

use crate::error::Error;

#[tauri::command]
pub fn get_project_runtime_summaries() -> Result<Vec<ProjectRuntimeSummary>, Error> {
    load_project_runtime_summaries()
}
#[tauri::command]
pub fn get_project_runtime_detail(project_id: &str) -> Result<ProjectRuntimeDetail, Error> {
    load_project_runtime_detail(&project_id.into())
}

#[tauri::command]
pub fn get_github_runtime_detail() -> Result<GitHubRuntimeDetail, Error> {
    load_github_runtime_detail()
}
