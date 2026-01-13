mod message;

use crate::command::message::ProjectRuntimeUpdate;

use crate::config::{save_tool_config, TOOL_CONFIG};
use crate::error::Error;
use crate::runtime::{
    load_github_runtime_detail, load_project_runtime_detail, load_project_runtime_summaries,
    GitHubRuntimeDetail, ProjectRuntimeDetail, ProjectRuntimeSummary,
};
use crate::{process, repo};
use tracing::info;

#[tauri::command]
pub async fn get_project_runtime_summaries() -> Result<Vec<ProjectRuntimeSummary>, Error> {
    load_project_runtime_summaries()
}
#[tauri::command]
pub async fn get_project_runtime_detail(project_id: &str) -> Result<ProjectRuntimeDetail, Error> {
    load_project_runtime_detail(&project_id.into())
}

#[tauri::command]
pub async fn get_github_runtime_detail() -> Result<GitHubRuntimeDetail, Error> {
    load_github_runtime_detail()
}

#[tauri::command]
pub async fn save_project(project_runtime_update: ProjectRuntimeUpdate) -> Result<(), Error> {
    let mut tool_config = TOOL_CONFIG.write().map_err(|_| Error::LockFail)?;
    let project = tool_config
        .projects
        .get_mut(&project_runtime_update.project_id)
        .ok_or(Error::ProjectNotFound(
            project_runtime_update.project_id.clone(),
        ))?;
    project.local_repo_path = project_runtime_update.local_repo_path.into();
    project.github_branch = project_runtime_update.github_branch;
    project.github_repo_url = project_runtime_update.github_repo_url;
    project.customized_build_command = project_runtime_update.build_command;
    project.customized_run_command = project_runtime_update.run_command;
    project.customized_debug_command = project_runtime_update.debug_command;
    save_tool_config(&tool_config)?;
    Ok(())
}

#[tauri::command]
pub async fn get_project_code(project_runtime_update: ProjectRuntimeUpdate) -> Result<(), Error> {
    info!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    save_project(project_runtime_update).await?;
    repo::get_project_github_code(&project_id.into())
}

#[tauri::command]
pub async fn exec_build_process(project_runtime_update: ProjectRuntimeUpdate) -> Result<(), Error> {
    info!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    save_project(project_runtime_update).await?;
    process::execute_build_process(&project_id.into())
}
