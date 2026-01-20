use crate::command::message::{
    CommandType, GitHubRuntimeDetail, ProjectRuntimeDetail, ProjectRuntimeSummary,
    RunningCommandStatus,
};
use crate::process::{ChildProcess, PROJECT_CHILD_PROCESS_REPO};
use crate::{common::ProjectId, config::TOOL_CONFIG, error::Error, repo};
use tracing::info;

pub async fn load_github_runtime_detail() -> Result<GitHubRuntimeDetail, Error> {
    let tool_config = TOOL_CONFIG.read().await;
    Ok(GitHubRuntimeDetail {
        username: tool_config.github.username.clone(),
        token: tool_config.github.token.clone(),
        proxy: tool_config.github.proxy.clone(),
    })
}

pub async fn load_project_runtime_summaries() -> Result<Vec<ProjectRuntimeSummary>, Error> {
    let tool_config = TOOL_CONFIG.read().await;
    let mut summaries = tool_config
        .projects
        .iter()
        .map(|(project_id, project_config)| ProjectRuntimeSummary {
            project_id: project_id.clone(),
            name: project_config.name.clone(),
            description: project_config.description.clone(),
        })
        .collect::<Vec<_>>();
    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

pub async fn load_project_runtime_detail(
    project_id: &ProjectId,
) -> Result<ProjectRuntimeDetail, Error> {
    let tool_config = TOOL_CONFIG.read().await;
    let project_config = tool_config
        .projects
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let available_branches = repo::fetch_branch_list(&tool_config.github, project_config).await?;


    let current_process = PROJECT_CHILD_PROCESS_REPO
        .lock()
        .await
        .get(project_id)
        .map(|process| {
            let (command_type, pid) = match process {
                ChildProcess::Build(child) => (CommandType::Build, child.pid()),
                ChildProcess::Run(child) => (CommandType::Run, child.pid()),
                ChildProcess::Debug(child) => (CommandType::Debug, child.pid()),
            };
            info!("Child process spawned with pid: {}", pid);
            RunningCommandStatus::Running {
                command_type,
                project_id: project_id.clone(),
            }
        });
    let project_runtime_detail = ProjectRuntimeDetail {
        name: project_config.name.clone(),
        description: project_config.description.clone(),
        available_branches,
        working_branch: project_config.working_branch.clone(),
        remote_repo_url: project_config.remote_repo_url.clone(),
        local_repo_path: project_config.local_repo_path.clone(),
        current_running_command_status: current_process,
        build_command: project_config.build_command.clone(),
        run_command: project_config.run_command.clone(),
        stop_command: project_config.stop_command.clone(),
        debug_command: project_config.debug_command.clone(),
        customized_build_command: project_config.customized_build_command.clone(),
        customized_run_command: project_config.customized_run_command.clone(),
        customized_stop_command: project_config.customized_stop_command.clone(),
        customized_debug_command: project_config.customized_debug_command.clone(),
        customized_properties:project_config.customized_properties.clone(),
    };
    Ok(project_runtime_detail)
}
