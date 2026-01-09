use std::{env::temp_dir, path::Path};

use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tracing::info;

use crate::error::Error;

pub struct RetrieveGitHubCodeRequest<'a> {
    github_username: String,
    github_token: String,
    github_branch: String,
    github_repo_url: String,
    local_repo_path: &'a Path,
    proxy_url: Option<String>,
}

pub fn retrieve_code_from_github(retrieve_info: RetrieveGitHubCodeRequest) -> Result<(), Error> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&retrieve_info.github_username, &retrieve_info.github_token)
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        info!("Receiving data from GitHub: {received_objects}/{total_objects}");
        true
    });

    let mut fetch_options = FetchOptions::new();
    if let Some(proxy_url) = &retrieve_info.proxy_url {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(proxy_url);
        fetch_options.proxy_options(proxy_options);
    }

    fetch_options.remote_callbacks(callbacks);

    if retrieve_info.local_repo_path.exists() {
        let repository = Repository::open(retrieve_info.local_repo_path)?;
        repository.find_remote("origin")?.fetch(
            &[retrieve_info.github_branch],
            Some(&mut fetch_options),
            None,
        )?;
    } else {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);
        builder.branch(&retrieve_info.github_branch);
        builder.clone(
            &retrieve_info.github_repo_url,
            retrieve_info.local_repo_path,
        )?;
    }

    Ok(())
}

pub struct RetrieveGitHubBranchesRequest<'a> {
    github_username: String,
    github_token: String,
    github_repo_url: String,
    local_repo_path: &'a Path,
    proxy_url: Option<String>,
}

pub fn retrieve_branches_from_github(
    retrieve_info: RetrieveGitHubBranchesRequest,
) -> Result<Vec<String>, Error> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&retrieve_info.github_username, &retrieve_info.github_token)
    });

    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        info!("Receiving data from GitHub: {received_objects}/{total_objects}");
        true
    });

    let mut fetch_options = FetchOptions::new();
    if let Some(proxy_url) = &retrieve_info.proxy_url {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(proxy_url);
        fetch_options.proxy_options(proxy_options);
    }

    fetch_options.remote_callbacks(callbacks);
    let temp_dir = temp_dir();
    let temp_repo = Repository::init_bare(temp_dir.as_path())?;
    let remote = temp_repo.remote("origin", &retrieve_info.github_repo_url)?;
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
