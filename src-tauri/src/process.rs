use crate::command::message::LogLevel;
use crate::common::{ack_frontend_action, parse_log_level, push_log_to_frontend, ProjectId};
use crate::config::{ProjectConfig, TOOL_CONFIG};
use crate::error::Error;
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use tracing::{error, info};

fn execute_program(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    command: &str,
    project_config: &ProjectConfig,
    response_channel: Channel<bool>,
) -> Result<(), Error> {
    let shell = app_handle.shell();
    let (mut receiver, child) = shell
        .command("cmd")
        .args(["/C", command])
        .current_dir(
            project_config
                .local_repo_path
                .join(&project_config.working_branch),
        )
        .spawn()?;
    push_log_to_frontend(
        &app_handle,
        &project_id,
        format!(
            "Child process for [{command}] spawned, process id: {}",
            child.pid()
        ),
        LogLevel::Info,
    );
    let project_id = project_id.clone();
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(command_event) = receiver.recv().await {
            match command_event {
                CommandEvent::Stderr(std_error) => {
                    let line = match String::from_utf8(std_error) {
                        Ok(line) => line,
                        Err(e) => {
                            error!("Fail to parse utf8 line: {e:?}");
                            continue;
                        }
                    };

                    push_log_to_frontend(&app_handle, &project_id, line.clone(), LogLevel::Error);
                }
                CommandEvent::Stdout(std_out) => {
                    let line = match String::from_utf8(std_out) {
                        Ok(line) => line,
                        Err(e) => {
                            error!("Fail to parse utf8 line: {e:?}");
                            continue;
                        }
                    };
                    let level = parse_log_level(&line);
                    push_log_to_frontend(&app_handle, &project_id, line.clone(), level);
                    info!("[CHILD OUTPUT] {}", line);
                }
                CommandEvent::Error(error) => {
                    push_log_to_frontend(&app_handle, &project_id, error, LogLevel::Error);
                }
                CommandEvent::Terminated(terminate) => {
                    ack_frontend_action(&response_channel);
                    let mut level = LogLevel::Error;
                    if let Some(0) = terminate.code {
                        level = LogLevel::Info
                    }
                    push_log_to_frontend(
                        &app_handle,
                        &project_id,
                        format!(
                            "Terminated by signal: {:?}, code: {:?}",
                            terminate.signal, terminate.code
                        ),
                        level,
                    );
                }
                _ => {
                    continue;
                }
            }
        }
    });

    Ok(())
}

pub fn execute_build_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    response_channel: Channel<bool>,
) -> Result<(), Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let build_command = project_config
        .customized_build_command
        .as_deref()
        .ok_or(Error::BuildCommandNotFound(project_id.clone()))?;
    execute_program(
        app_handle,
        project_id,
        build_command,
        &project_config,
        response_channel,
    )?;
    Ok(())
}
