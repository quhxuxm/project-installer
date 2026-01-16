use std::fs::OpenOptions;
use crate::command::message::{GlobalLogLevel, GlobalNotificationLevel};
use crate::common::{
    ack_frontend_action, generate_customized_properties_dir, parse_global_log_level,
    push_global_log_to_frontend, push_global_notification_to_frontend, ProjectId,
};
use crate::config::{ProjectConfig, TOOL_CONFIG};
use crate::error::Error;

use java_properties::write;
use std::io::BufWriter;
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
    let child_process_id = child.pid();
    push_global_log_to_frontend(
        &app_handle,
        &project_id,
        format!("Child process for [{command}] spawned, process id: {child_process_id}",),
        GlobalLogLevel::Info,
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

                    push_global_log_to_frontend(
                        &app_handle,
                        &project_id,
                        line.clone(),
                        GlobalLogLevel::Error,
                    );
                }
                CommandEvent::Stdout(std_out) => {
                    let line = match String::from_utf8(std_out) {
                        Ok(line) => line,
                        Err(e) => {
                            error!("Fail to parse utf8 line: {e:?}");
                            continue;
                        }
                    };
                    let level = parse_global_log_level(&line);
                    push_global_log_to_frontend(&app_handle, &project_id, line.clone(), level);
                    info!("[CHILD OUTPUT] {}", line);
                }
                CommandEvent::Error(error) => {
                    push_global_log_to_frontend(
                        &app_handle,
                        &project_id,
                        error,
                        GlobalLogLevel::Error,
                    );
                }
                CommandEvent::Terminated(terminate) => {
                    ack_frontend_action(&response_channel);
                    let mut level = GlobalLogLevel::Error;
                    if let Some(0) = terminate.code {
                        level = GlobalLogLevel::Info;
                        push_global_notification_to_frontend(&app_handle, &project_id,
                        format!(
                            "Project {project_id} sub process {child_process_id} terminated with code: {:?}, signal: {:?}",
                            terminate.code, terminate.signal
                        ),
                         format!("Project {project_id} build process success."),
                        GlobalNotificationLevel::Success);
                    } else {
                        push_global_notification_to_frontend(
                            &app_handle,
                            &project_id,
                            format!(
                                "Project {project_id} sub process {child_process_id} terminated with code: {:?}, signal: {:?}",
                                terminate.code, terminate.signal
                            ),
                            format!("Project {project_id} build process fail."),
                            GlobalNotificationLevel::Error);
                    }

                    push_global_log_to_frontend(
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

pub fn execute_run_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    response_channel: Channel<bool>,
) -> Result<(), Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let customized_cfg_dir = generate_customized_properties_dir(project_config);
    if !customized_cfg_dir.exists(){
        std::fs::create_dir_all(&customized_cfg_dir)?;
    }
    let customized_properties_file = customized_cfg_dir.join(format!("{project_id}.properties"));
    let customized_properties_file = std::fs::File::create(customized_properties_file)?;
    write(
        BufWriter::new(customized_properties_file),
        &project_config.customized_properties,
    )?;
    let run_command = project_config
        .customized_run_command
        .as_deref()
        .ok_or(Error::RunCommandNotFound(project_id.clone()))?;
    execute_program(
        app_handle,
        project_id,
        run_command,
        &project_config,
        response_channel,
    )?;
    Ok(())
}
