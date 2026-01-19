pub mod message;

use crate::command::message::{
    CommandType, GitHubRuntimeDetail, ProjectRuntimeDetail, ProjectRuntimeSummary,
    ProjectRuntimeUpdate, RunningCommandStatus,
};
use crate::common::push_current_process_status_to_frontend;
use crate::config::{save_tool_config, ToolConfig, TOOL_CONFIG};
use crate::error::Error;
use crate::project::{
    load_github_runtime_detail, load_project_runtime_detail, load_project_runtime_summaries,
};
use crate::repo::fetch_branch_list;
use crate::{process, repo};
use tauri::ipc::Channel;
use tauri::AppHandle;
use tokio::sync::mpsc::channel;
use tracing::{debug, error};

async fn save_project_runtime_update(
    tool_config: &mut ToolConfig,
    project_runtime_update: ProjectRuntimeUpdate,
    command_type: CommandType,
) -> Result<(), RunningCommandStatus> {
    let project_id = &project_runtime_update.project_id;
    let available_branches = {
        let github = &tool_config.github;
        let project = tool_config
            .projects
            .get(project_id)
            .ok_or(Error::ProjectNotFound(project_id.clone()))
            .map_err(|e| {
                error!("Fail to get project config: {e}");
                RunningCommandStatus::TerminatedFailure {
                    command_type,
                    project_id: project_id.clone(),
                }
            })?;
        fetch_branch_list(github, project).await.map_err(|e| {
            error!("Fail to fetch project branch: {e}");
            RunningCommandStatus::TerminatedFailure {
                command_type,
                project_id: project_id.clone(),
            }
        })?
    };

    let project = tool_config
        .projects
        .get_mut(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))
        .map_err(|e| {
            error!("Fail to get project config: {e}");
            RunningCommandStatus::TerminatedFailure {
                command_type,
                project_id: project_id.clone(),
            }
        })?;
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
    project.available_branches = available_branches;
    save_tool_config(tool_config).map_err(|e| {
        error!("Fail to save project config: {e}");
        RunningCommandStatus::TerminatedFailure {
            command_type,
            project_id: project_id.clone(),
        }
    })?;
    Ok(())
}

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
pub async fn save_project(
    project_runtime_update: ProjectRuntimeUpdate,
    response_channel: Channel<RunningCommandStatus>,
) {
    let mut tool_config = TOOL_CONFIG.write().await;
    if let Err(e) =
        save_project_runtime_update(&mut tool_config, project_runtime_update, CommandType::Save)
            .await
    {
        push_current_process_status_to_frontend(&response_channel, e);
    };
}

#[tauri::command]
pub async fn get_project_code(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    command_status_channel: Channel<RunningCommandStatus>,
) {
    debug!("Receive project runtime update: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    let mut tool_config = TOOL_CONFIG.write().await;
    if let Err(e) = save_project_runtime_update(
        &mut tool_config,
        project_runtime_update,
        CommandType::FetchCode,
    )
    .await
    {
        push_current_process_status_to_frontend(&command_status_channel, e);
        return;
    };
    if let Err(e) = repo::fetch_code(&tool_config, &app_handle, &project_id).await {
        error!("Failed to fetch project code: {e}");
        push_current_process_status_to_frontend(
            &command_status_channel,
            RunningCommandStatus::TerminatedFailure {
                command_type: CommandType::FetchCode,
                project_id: project_id.clone(),
            },
        );
        return;
    }
    push_current_process_status_to_frontend(
        &command_status_channel,
        RunningCommandStatus::TerminatedSuccess {
            command_type: CommandType::FetchCode,
            project_id: project_id.clone(),
        },
    );
}

#[tauri::command]
pub async fn exec_build_process(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    command_status_channel: Channel<RunningCommandStatus>,
) {
    debug!("Receive project runtime update for build project: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    let mut tool_config = TOOL_CONFIG.write().await;
    if let Err(e) =
        save_project_runtime_update(&mut tool_config, project_runtime_update, CommandType::Build)
            .await
    {
        push_current_process_status_to_frontend(&command_status_channel, e);
    };
    debug!("Complete to save project runtime update for build project.");
    let (child_process_status_tx, mut child_process_status_rs) = channel(1024);

    let project_config = match tool_config.projects.get(&project_id) {
        Some(project_config) => project_config,
        None => {
            push_current_process_status_to_frontend(
                &command_status_channel,
                RunningCommandStatus::TerminatedFailure {
                    command_type: CommandType::Build,
                    project_id: project_id.clone(),
                },
            );
            return;
        }
    };

    if let Err(e) = process::spawn_build_process(
        &app_handle,
        &project_id,
        project_config,
        child_process_status_tx,
    )
    .await
    {
        error!("Failed to spawn build process: {e}");
        push_current_process_status_to_frontend(
            &command_status_channel,
            RunningCommandStatus::TerminatedFailure {
                command_type: CommandType::Build,
                project_id: project_id.clone(),
            },
        );
        return;
    }
    debug!("Success to spawn build process.");
    tokio::spawn(async move {
        while let Some(current_process) = child_process_status_rs.recv().await {
            push_current_process_status_to_frontend(&command_status_channel, current_process);
        }
    });
}

#[tauri::command]
pub async fn exec_run_process(
    app_handle: AppHandle,
    project_runtime_update: ProjectRuntimeUpdate,
    command_status_channel: Channel<RunningCommandStatus>,
) {
    debug!("Receive project runtime update for run project: {project_runtime_update:#?}");
    let project_id = project_runtime_update.project_id.clone();
    let mut tool_config = TOOL_CONFIG.write().await;
    if let Err(e) =
        save_project_runtime_update(&mut tool_config, project_runtime_update, CommandType::Run)
            .await
    {
        push_current_process_status_to_frontend(&command_status_channel, e);
    };
    let (child_process_status_tx, mut child_process_status_rs) = channel(1024);
    let project_config = match tool_config.projects.get(&project_id) {
        Some(project_config) => project_config,
        None => {
            push_current_process_status_to_frontend(
                &command_status_channel,
                RunningCommandStatus::TerminatedFailure {
                    command_type: CommandType::Run,
                    project_id: project_id.clone(),
                },
            );
            return;
        }
    };

    if let Err(e) = process::spawn_run_process(
        &app_handle,
        &project_id,
        project_config,
        child_process_status_tx,
    )
    .await
    {
        error!("Failed to execute run process: {e}");
        push_current_process_status_to_frontend(
            &command_status_channel,
            RunningCommandStatus::TerminatedFailure {
                command_type: CommandType::Run,
                project_id: project_id.clone(),
            },
        );
        return;
    }
    tokio::spawn(async move {
        while let Some(current_process) = child_process_status_rs.recv().await {
            push_current_process_status_to_frontend(&command_status_channel, current_process);
        }
    });
}
