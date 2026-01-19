use crate::command::message::{
    CommandType, GlobalLogLevel, GlobalNotificationLevel, RunningCommandStatus,
};
use crate::common::{
    generate_customized_properties_dir, parse_global_log_level, push_global_log_to_frontend,
    push_global_notification_to_frontend, ProjectId,
};
use crate::config::ProjectConfig;
use crate::error::Error;
use std::collections::HashMap;

use java_properties::write;
use std::io::BufWriter;
use std::ops::Deref;
use std::sync::{Arc, LazyLock};
use tauri::AppHandle;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tracing::{error, info, trace};

pub type ProjectChildProcessRepo = LazyLock<Arc<Mutex<HashMap<ProjectId, ChildProcess>>>>;

pub static PROJECT_CHILD_PROCESS_REPO: ProjectChildProcessRepo = LazyLock::new(Default::default);

pub enum ChildProcess {
    Build(CommandChild),
    Run(CommandChild),
    Debug(CommandChild),
}

impl Deref for ChildProcess {
    type Target = CommandChild;

    fn deref(&self) -> &Self::Target {
        match self {
            ChildProcess::Build(child) => child,
            ChildProcess::Run(child) => child,
            ChildProcess::Debug(child) => child,
        }
    }
}

async fn spawn_child_program(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    command: &str,
    project_config: &ProjectConfig,
    command_type: CommandType,
    child_process_status_tx: Sender<RunningCommandStatus>,
) -> Result<CommandChild, Error> {
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
        app_handle,
        project_id,
        format!("Child process for [{command}] spawned, process id: {child_process_id}",),
        GlobalLogLevel::Info,
    );
    {
        let project_id = project_id.clone();
        let app_handle = app_handle.clone();
        let child_process_status_tx = child_process_status_tx.clone();

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
                        trace!("[CHILD OUTPUT] {}", line);
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
                            push_global_notification_to_frontend(&app_handle, &project_id, format!(
                                "Project {project_id} sub process {child_process_id} terminated with code: {:?}, signal: {:?}",
                                terminate.code, terminate.signal
                            ),format!("Project {project_id} build process success."),GlobalNotificationLevel::Success);
                            if let Err(e) = child_process_status_tx
                                .send(RunningCommandStatus::TerminatedSuccess {
                                    command_type,
                                    project_id: project_id.clone(),
                                })
                                .await
                            {
                                error!("Fail to send child process terminated successfully: {e:?}");
                            }
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
                            if let Err(e) = child_process_status_tx
                                .send(RunningCommandStatus::TerminatedFailure {
                                    command_type,
                                    project_id: project_id.clone(),
                                })
                                .await
                            {
                                error!("Fail to send child process terminated failure: {e:?}");
                            }
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
    if let Err(e) = child_process_status_tx
        .send(RunningCommandStatus::Running {
            command_type,
            project_id: project_id.clone(),
        })
        .await
    {
        error!("Fail to send child process running failure: {e:?}");
    };
    Ok(child)
}

pub async fn spawn_build_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    project_config: &ProjectConfig,
    child_process_status_tx: Sender<RunningCommandStatus>,
) -> Result<(), Error> {
    let build_command = project_config
        .customized_build_command
        .as_deref()
        .ok_or(Error::BuildCommandNotFound(project_id.clone()))?;
    let child = spawn_child_program(
        app_handle,
        project_id,
        build_command,
        project_config,
        CommandType::Build,
        child_process_status_tx,
    )
    .await?;
    PROJECT_CHILD_PROCESS_REPO
        .lock()
        .await
        .insert(project_id.clone(), ChildProcess::Build(child));
    Ok(())
}

pub async fn spawn_stop_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    child_process_status_tx: Sender<RunningCommandStatus>,
) -> Result<(), Error> {
    let mut project_child_process_repo = PROJECT_CHILD_PROCESS_REPO.lock().await;
    let mut project_child_process = project_child_process_repo.remove(project_id);
    if let Some(project_child_process) = project_child_process.take() {
        match project_child_process {
            ChildProcess::Build(command_child) => {
                info!("Begin to stop child process (building) for project: {project_id}");
                command_child.kill()?;
                info!("Stop child process (building) for project: {project_id}");
                push_global_log_to_frontend(
                    app_handle,
                    project_id,
                    format!("Child process (building) for project [{project_id:?}] is killed.",),
                    GlobalLogLevel::Info,
                );
            }
            ChildProcess::Run(command_child) => {
                info!("Begin to stop child process (running) for project: {project_id}");
                command_child.kill()?;
                info!("Stop child process (running) for project: {project_id}");
                push_global_log_to_frontend(
                    app_handle,
                    project_id,
                    format!("Child process (running) for project [{project_id:?}] is killed.",),
                    GlobalLogLevel::Info,
                );
            }
            ChildProcess::Debug(command_child) => {
                info!("Begin to stop child process (debugging) for project: {project_id}");
                command_child.kill()?;
                info!("Stop child process (debugging) for project: {project_id}");
                push_global_log_to_frontend(
                    app_handle,
                    project_id,
                    format!("Child process (debugging) for project [{project_id:?}] is killed.",),
                    GlobalLogLevel::Info,
                );
            }
        }
    }
    if let Err(e) = child_process_status_tx
        .send(RunningCommandStatus::TerminatedSuccess {
            command_type: CommandType::Stop,
            project_id: project_id.clone(),
        })
        .await
    {
        error!("Fail to send child process terminated successfully: {e:?}");
    }

    Ok(())
}

pub async fn spawn_run_process(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    project_config: &ProjectConfig,
    child_process_status_tx: Sender<RunningCommandStatus>,
) -> Result<(), Error> {
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
    let child = spawn_child_program(
        app_handle,
        project_id,
        run_command,
        project_config,
        CommandType::Run,
        child_process_status_tx,
    )
    .await?;
    PROJECT_CHILD_PROCESS_REPO
        .lock()
        .await
        .insert(project_id.clone(), ChildProcess::Run(child));
    Ok(())
}
