use crate::command::message::{GlobalLogLevel, GlobalNotificationLevel};
use crate::common::{
    push_global_log_to_frontend, push_global_notification_to_frontend, GIT_DIR, RGS_PMT_DIR,
};

use crate::config::{GitHubConfig, ProjectConfig, ToolConfig};
use crate::{common::ProjectId, error::Error};
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tauri::AppHandle;

#[derive(Debug)]
pub struct GetBranchesRequest {
    pub project_id: ProjectId,
    pub github_repo_url: String,
}

pub async fn fetch_code(
    tool_config: &ToolConfig,
    app_handle: &AppHandle,
    project_id: &ProjectId,
) -> Result<(), Error> {
    std::env::set_var("GIT_CONFIG_PARAMETERS", "'core.longpaths=true'");
    let project_id = project_id.clone();
    let app_handle = app_handle.clone();

    let github_config = &tool_config.github;
    let projects_config = &tool_config.projects;
    let project_config = projects_config
        .get(&project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&github_config.username, &github_config.token)
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        push_global_log_to_frontend(
            &app_handle,
            &project_id,
            format!("Receiving data from GitHub: {received_objects}/{total_objects}"),
            GlobalLogLevel::Info,
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
        .join(&project_config.working_branch);
    push_global_notification_to_frontend(
        &app_handle,
        &project_id,
        format!(
            "Begin to cloned data from GitHub: {} to {:?}",
            project_config.remote_repo_url, project_local_path
        ),
        "GitHub repository clone begin.".to_string(),
        GlobalNotificationLevel::Info,
    );
    push_global_log_to_frontend(
        &app_handle,
        &project_id,
        format!(
            "Cloning data from GitHub: {} to {:?}",
            project_config.remote_repo_url, project_local_path
        ),
        GlobalLogLevel::Info,
    );
    if project_local_path.exists() {
        let repository = Repository::open(&project_local_path)?;

        let mut repository = repository.find_remote("origin")?;

        repository.fetch(
            &[&project_config.working_branch],
            Some(&mut fetch_options),
            None,
        )?;
    } else {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);
        builder.branch(&project_config.working_branch);

        builder.clone(&project_config.remote_repo_url, &project_local_path)?;
        push_global_notification_to_frontend(
            &app_handle,
            &project_id,
            format!(
                "Successfully cloned data from GitHub: {} to {:?}",
                project_config.remote_repo_url, project_local_path
            ),
            "GitHub repository clone success.".to_string(),
            GlobalNotificationLevel::Success,
        );
        push_global_log_to_frontend(
            &app_handle,
            &project_id,
            format!(
                "Successfully cloned data from GitHub: {} to {:?}",
                project_config.remote_repo_url, project_local_path
            ),
            GlobalLogLevel::Info,
        );
    }

    Ok(())
}

pub async fn fetch_branch_list(
    github_config: &GitHubConfig,
    project_config: &ProjectConfig,
) -> Result<Vec<String>, Error> {
    if !project_config.available_branches.is_empty() {
        return Ok(project_config.available_branches.clone());
    }
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&github_config.username, &github_config.token)
    });
    let proxy_options = github_config.proxy.as_ref().map(|url| {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(url);
        proxy_options
    });
    let temp_repo = Repository::init_bare(
        project_config
            .local_repo_path
            .join(RGS_PMT_DIR)
            .join(GIT_DIR),
    )?;
    let mut remote = temp_repo.remote_anonymous(&project_config.remote_repo_url)?;
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
