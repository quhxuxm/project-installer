use std::{env::temp_dir, path::Path};

use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tracing::info;

use crate::{common::ProjectId, config::TOOL_CONFIG, error::Error};

#[derive(Debug)]
pub struct GetCodeRequest<'a> {
    pub github_branch: String,
    pub github_repo_url: String,
    pub local_repo_path: &'a Path,
}

#[derive(Debug)]
pub struct GetBranchesRequest {
    pub github_repo_url: String,
}

pub fn get_project_github_code(project_id: &ProjectId) -> Result<(), Error> {
    let github_config = &TOOL_CONFIG.github;
    let projects_config = &TOOL_CONFIG.projects;
    let project_config = projects_config
        .get(project_id)
        .ok_or(Error::ProjectNotFound(project_id.clone()))?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&github_config.username, &github_config.token)
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        info!("Receiving data from GitHub: {received_objects}/{total_objects}");
        true
    });
    let mut fetch_options = FetchOptions::new();
    if let Some(proxy) = github_config.proxy.as_ref() {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(proxy);
        fetch_options.proxy_options(proxy_options);
    }
    fetch_options.remote_callbacks(callbacks);
    if project_config.local_repo_path.exists() {
        let repository = Repository::open(&project_config.local_repo_path)?;
        repository.find_remote("origin")?.fetch(
            &[&project_config.github_branch],
            Some(&mut fetch_options),
            None,
        )?;
    } else {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);
        builder.branch(&project_config.github_branch);
        builder.clone(
            &project_config.github_repo_url,
            &project_config.local_repo_path,
        )?;
    }
    Ok(())
}

pub fn get_project_github_branches(project_id: &ProjectId) -> Result<Vec<String>, Error> {
    let github_config = &TOOL_CONFIG.github;
    let projects_config = &TOOL_CONFIG.projects;
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
    let temp_dir = temp_dir();
    let temp_repo = Repository::init_bare(&temp_dir)?;
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
