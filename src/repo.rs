use crate::config::ProjectConfig;
use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use std::path::Path;
use tracing::info;

pub fn get_project_repo(github_username: &str, github_token: &str, proxy_url: Option<&str>, project_name: &str, project_config: &ProjectConfig) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(github_username, github_token)
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        info!("工程[{project_name}]接受到：[{received_objects:?}]对象，总共：[{total_objects}]对象。" );
        true
    });

    let mut fetch_options = FetchOptions::new();
    if let Some(proxy_url) = proxy_url {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(proxy_url);
        fetch_options.proxy_options(proxy_options);
    }

    fetch_options.remote_callbacks(callbacks);

    let github_repo_url = project_config.github_repo_url();
    let project_local_path = project_config.project_local_path();
    let github_repo_branch = project_config.github_repo_branch();

    info!("正在获取仓库：[{github_repo_url}] 的 [{github_repo_branch}] 分支到 [{project_local_path:?}]");
    if project_local_path.exists() {
        let repository = Repository::open(project_local_path)?;
        repository.find_remote("origin")?.fetch(&[github_repo_branch], Some(&mut fetch_options), None)?;
    } else {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);
        builder.branch(github_repo_branch);
        builder.clone(github_repo_url, Path::new(project_local_path))?;
    }
    // 执行克隆操作

    Ok(())
}