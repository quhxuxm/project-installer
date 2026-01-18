pub mod message;

use crate::command::message::{
    CurrentProcess, GitHubRuntimeDetail, ProjectRuntimeDetail, ProjectRuntimeSummary,
    ProjectRuntimeUpdate,
};
use crate::config::{save_tool_config, TOOL_CONFIG};
use crate::error::Error;
use crate::project::{
    load_github_runtime_detail, load_project_runtime_detail, load_project_runtime_summaries,
};
use crate::{process, repo};
use tauri::ipc::Channel;
use tauri::AppHandle;
use tracing::info;

#[tauri::command]
pub async fn get_project_runtime_summaries() -> Result<Vec<ProjectRuntimeSummary>, Error> {
    load_project_runtime_summaries().await
}
#[tauri::command]
pub async fn get_project_runtime_detail(project_id: &str) -> Result<ProjectRuntimeDetail, Error> {
    load_project_runtime_detail(&project_id.into()).await
}

#[tauri::command]
pub async fn get_github_runtime_detail() -> Result<GitHubRuntimeDetail, Error> {
    load_github_runtime_detail().await
}

#[tauri::command]
pub async fn save_project(project_runtime_update: ProjectRuntimeUpdate) -> Result<(), Error> {
    let mut tool_config = TOOL_CONFIG.write().await;
    let project = tool_config
        .projects
        .get_mut(&project_runtime_update.project_id)
        .ok_or(Error::ProjectNotFound(
            project_runtime_update.project_id.clone(),
        ))?;
    project.local_repo_path = project_runtime_update.local_repo_path.into();
    project.working_branch = project_runtime_update.working_branch;
    project.remote_repo_url = project_runtime_update.remote_repo_url;
    project.customized_build_command = project_runtime_update.build_command;
    project.customized_run_command = project_runtime_update.run_command;
    project.customized_debug_command = project_runtime_update.debug_command;
    project.customized_properties = project_runtime_update
        .customized_properties
        .iter()
        .map(|item| (item.key.clone(), item.value.clone()))
        .collect();
    save_tool_config(&tool_config)?;
    Ok(())
}

#[tauri::command]
pub async fn get_project_code(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    response_channel: Channel<CurrentProcess>,
) -> Result<(), Error> {
    info!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    save_project(project_runtime_update).await?;
    repo::fetch_code(&app_handle, &project_id.into(), response_channel).await
}

#[tauri::command]
pub async fn exec_build_process(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    response_channel: Channel<CurrentProcess>,
) -> Result<(), Error> {
    info!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    save_project(project_runtime_update).await?;
    process::execute_build_process(&app_handle, &project_id.into(), response_channel).await
}

#[tauri::command]
pub async fn exec_run_process(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    response_channel: Channel<CurrentProcess>,
) -> Result<(), Error> {
    info!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    save_project(project_runtime_update).await?;
    process::execute_run_process(&app_handle, &project_id.into(), response_channel).await
}
