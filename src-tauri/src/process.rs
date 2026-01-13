use crate::common::ProjectId;
use crate::config::{ProjectConfig, TOOL_CONFIG};
use crate::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use tracing::info;

fn execute_program(
    project_id: &ProjectId,
    command: &str,
    project_config: &ProjectConfig,
) -> Result<(), Error> {
    info!("Executing: {}", command);
    let shell = shell_words::split(command)?;
    let (program, args) = shell
        .split_first()
        .ok_or(Error::ProgramPartNotFound(project_id.clone()))?;
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .current_dir(
            project_config
                .local_repo_path
                .join(&project_config.github_branch),
        )
        .spawn()?;
    info!("Child process for [{command}] spawned");
    if let Some(stdout) = child.stdout.take() {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("[CHILD OUTPUT] {}", line);
                }
            }
        });
    }
    let exit_status = child.wait()?;
    info!("Child process for [{command}] exited with status: {exit_status}");
    Ok(())
}
pub fn execute_build_process(project_id: &ProjectId) -> Result<(), Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let build_command = project_config
        .customized_build_command
        .as_deref()
        .ok_or(Error::BuildCommandNotFound(project_id.clone()))?;
    execute_program(project_id, build_command, &project_config)?;
    Ok(())
}
