use crate::command::message::{
    CurrentProcess, GlobalLogLevel, GlobalNotificationLevel, ProcessStatus, ProcessType,
};
use crate::common::{
    generate_customized_properties_dir, parse_global_log_level,
    push_current_process_status_to_frontend, push_global_log_to_frontend,
    push_global_notification_to_frontend, ProjectId,
};
use crate::config::{ProjectConfig, TOOL_CONFIG};
use crate::error::Error;
use std::collections::HashMap;

use java_properties::write;
use std::io::BufWriter;
use std::ops::Deref;
use std::sync::{Arc, LazyLock};
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tokio::sync::Mutex;
use tracing::{error, info};

pub static PROJECT_CHILD_PROCESS_REPO: LazyLock<Arc<Mutex<HashMap<ProjectId, Arc<ChildProcess>>>>> =
    LazyLock::new(|| Default::default());

pub enum ChildProcess {
    Build(Arc<CommandChild>),
    Run(Arc<CommandChild>),
    Debug(Arc<CommandChild>),
    Stop(Arc<CommandChild>),
}

impl Deref for ChildProcess {
    type Target = CommandChild;

    fn deref(&self) -> &Self::Target {
        match self {
            ChildProcess::Build(child) => child,
            ChildProcess::Run(child) => child,
            ChildProcess::Debug(child) => child,
            ChildProcess::Stop(child) => child,
        }
    }
}

async fn execute_program(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    command: &str,
    project_config: &ProjectConfig,
    frontend_response_channel: Channel<CurrentProcess>,
    process_type: ProcessType,
) -> Result<Arc<CommandChild>, Error> {
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
    let child = Arc::new(child);
    let child_process_id = child.pid();
    push_global_log_to_frontend(
        &app_handle,
        &project_id,
        format!("Child process for [{command}] spawned, process id: {child_process_id}",),
        GlobalLogLevel::Info,
    );
    {
        let project_id = project_id.clone();
        let app_handle = app_handle.clone();
        let frontend_response_channel_clone = frontend_response_channel.clone();
        tokio::spawn(async move {
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
                        if let Some(0) = terminate.code {
                            push_global_notification_to_frontend(&app_handle, &project_id,
                                                                 format!(
                                                                     "Project {project_id} sub process {child_process_id} terminated with code: {:?}, signal: {:?}",
                                                                     terminate.code, terminate.signal
                                                                 ),
                                                                 format!("Project {project_id} build process success."),
                                                                 GlobalNotificationLevel::Success);

                            push_current_process_status_to_frontend(
                                &frontend_response_channel_clone,
                                CurrentProcess {
                                    process_type,
                                    pid: Some(child_process_id),
                                    project_id: project_id.clone(),
                                    status: ProcessStatus::TerminatedSuccess,
                                },
                            );
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
                            push_current_process_status_to_frontend(
                                &frontend_response_channel_clone,
                                CurrentProcess {
                                    process_type,
                                    pid: Some(child_process_id),
                                    project_id: project_id.clone(),
                                    status: ProcessStatus::TerminatedFailure,
                                },
                            );
                        }
                        PROJECT_CHILD_PROCESS_REPO.lock().await.remove(&project_id);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        });
    }

    push_current_process_status_to_frontend(
        &frontend_response_channel,
        CurrentProcess {
            process_type,
            pid: Some(child_process_id),
            project_id: project_id.clone(),
            status: ProcessStatus::Running,
        },
    );

    Ok(child)
}

pub async fn execute_build_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    response_channel: Channel<CurrentProcess>,
) -> Result<(), Error> {
    let tool_config = TOOL_CONFIG.read().await;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let build_command = project_config
        .customized_build_command
        .as_deref()
        .ok_or(Error::BuildCommandNotFound(project_id.clone()))?;
    let child = execute_program(
        app_handle,
        project_id,
        build_command,
        &project_config,
        response_channel,
        ProcessType::Build,
    )
    .await?;
    PROJECT_CHILD_PROCESS_REPO
        .lock()
        .await
        .insert(project_id.clone(), Arc::new(ChildProcess::Build(child)));
    Ok(())
}

pub async fn execute_run_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    response_channel: Channel<CurrentProcess>,
) -> Result<(), Error> {
    let tool_config = TOOL_CONFIG.read().await;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let customized_cfg_dir = generate_customized_properties_dir(project_config);
    if !customized_cfg_dir.exists() {
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
    let child = execute_program(
        app_handle,
        project_id,
        run_command,
        &project_config,
        response_channel,
        ProcessType::Run,
    )
    .await?;
    PROJECT_CHILD_PROCESS_REPO
        .lock()
        .await
        .insert(project_id.clone(), Arc::new(ChildProcess::Run(child)));
    Ok(())
}
