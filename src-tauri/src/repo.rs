use crate::command::message::LogLevel;
use crate::common::{ack_frontend_action, push_log_to_frontend, RGS_PMT_DIR};
use crate::{common::ProjectId, config::TOOL_CONFIG, error::Error};
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tauri::ipc::Channel;
use tauri::AppHandle;
use tracing::error;

#[derive(Debug)]
pub struct GetBranchesRequest {
    pub project_id: ProjectId,
    pub github_repo_url: String,
}

pub fn clone_code(
    app_handle: &AppHandle,
    project_id: &ProjectId,
    response_channel: Channel<bool>,
) -> Result<(), Error> {
    std::env::set_var("GIT_CONFIG_PARAMETERS", "'core.longpaths=true'");
    let project_id = project_id.clone();
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let tool_config = match TOOL_CONFIG.read().map_err(|_| Error::LockFail) {
            Ok(config) => config,
            Err(e) => {
                ack_frontend_action(&response_channel);
                push_log_to_frontend(
                    &app_handle,
                    &project_id,
                    format!("Fail to get tool config: {e}"),
                    LogLevel::Error,
                );
                return;
            }
        };
        let github_config = &tool_config.github;
        let projects_config = &tool_config.projects;
        let project_config = match projects_config.get(&project_id) {
            Some(project_config) => project_config,
            None => {
                ack_frontend_action(&response_channel);
                push_log_to_frontend(
                    &app_handle,
                    &project_id,
                    format!("Fail to get project config: {project_id}"),
                    LogLevel::Error,
                );
                return;
            }
        };
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _, _allowed_types| {
            Cred::userpass_plaintext(&github_config.username, &github_config.token)
        });
        callbacks.transfer_progress(|progress| {
            let received_objects = progress.received_objects();
            let total_objects = progress.total_objects();
            push_log_to_frontend(
                &app_handle,
                &project_id,
                format!("Receiving data from GitHub: {received_objects}/{total_objects}"),
                LogLevel::Info,
            );
            true
        });
        let mut fetch_options = FetchOptions::new();
        if let Some(proxy) = github_config.proxy.as_ref() {
            let mut proxy_options = ProxyOptions::new();
            proxy_options.url(proxy);
            fetch_options.proxy_options(proxy_options);
        }
        fetch_options.remote_callbacks(callbacks);
        let project_local_path = project_config
            .local_repo_path
            .join(&project_config.github_branch);
        push_log_to_frontend(
            &app_handle,
            &project_id,
            format!(
                "Cloning data from GitHub: {} to {:?}",
                project_config.github_repo_url, project_local_path
            ),
            LogLevel::Info,
        );
        if project_local_path.exists() {
            let repository = match Repository::open(&project_local_path) {
                Ok(repository) => repository,
                Err(e) => {
                    error!(
                        "Fail to clone data from GitHib: {} to {:?} because of error: {e:?}",
                        project_config.github_repo_url, project_local_path
                    );
                    ack_frontend_action(&response_channel);
                    push_log_to_frontend(
                        &app_handle,
                        &project_id,
                        format!(
                            "Fail to clone data from GitHub: {} to {:?}",
                            project_config.github_repo_url, project_local_path
                        ),
                        LogLevel::Error,
                    );
                    return;
                }
            };

            let mut repository = match repository.find_remote("origin") {
                Ok(repository) => repository,
                Err(e) => {
                    error!(
                        "Fail to clone data from GitHib: {} to {:?} because of error: {e:?}",
                        project_config.github_repo_url, project_local_path
                    );
                    ack_frontend_action(&response_channel);
                    push_log_to_frontend(
                        &app_handle,
                        &project_id,
                        format!(
                            "Fail to clone data from GitHub: {} to {:?}",
                            project_config.github_repo_url, project_local_path
                        ),
                        LogLevel::Error,
                    );
                    return;
                }
            };

            if let Err(e) = repository.fetch(
                &[&project_config.github_branch],
                Some(&mut fetch_options),
                None,
            ) {
                error!(
                    "Fail to clone data from GitHib: {} to {:?} because of error: {e:?}",
                    project_config.github_repo_url, project_local_path
                );
                ack_frontend_action(&response_channel);
                push_log_to_frontend(
                    &app_handle,
                    &project_id,
                    format!(
                        "Fail to clone data from GitHub: {} to {:?}",
                        project_config.github_repo_url, project_local_path
                    ),
                    LogLevel::Error,
                );
                return;
            };
        } else {
            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder.branch(&project_config.github_branch);
            if let Err(e) = builder.clone(&project_config.github_repo_url, &project_local_path) {
                error!("Fail to clone GitHub repository: {e:?}");
                ack_frontend_action(&response_channel);
                push_log_to_frontend(
                    &app_handle,
                    &project_id,
                    format!(
                        "Fail to clone data from GitHub: {} to {:?}",
                        project_config.github_repo_url, project_local_path
                    ),
                    LogLevel::Error,
                );
            };
        }
        ack_frontend_action(&response_channel);
        push_log_to_frontend(
            &app_handle,
            &project_id,
            format!(
                "Successfully cloned data from GitHub: {} to {:?}",
                project_config.github_repo_url, project_local_path
            ),
            LogLevel::Info,
        );
    });

    Ok(())
}

pub fn get_branches(project_id: &ProjectId) -> Result<Vec<String>, Error> {
    let tool_config = TOOL_CONFIG.read().map_err(|_| Error::LockFail)?;
    let github_config = &tool_config.github;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&github_config.username, &github_config.token)
    });
    let proxy_options = github_config.proxy.as_ref().map(|url| {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(url);
        proxy_options
    });
    let temp_repo = Repository::init_bare(project_config.local_repo_path.join(RGS_PMT_DIR))?;
    let mut remote = temp_repo.remote_anonymous(&project_config.github_repo_url)?;
    remote.connect_auth(git2::Direction::Fetch, Some(callbacks), proxy_options)?;
    let remote_refs = remote.list()?;
    let mut branches = Vec::new();
    for remote_ref in remote_refs {
        let remote_ref_name = remote_ref.name();
        if remote_ref_name.starts_with("refs/heads/") {
            let branch_name = remote_ref_name.trim_start_matches("refs/heads/");
            branches.push(branch_name.to_string());
        }
    }
    Ok(branches)
}
