use crate::config::ToolConfig;
use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use std::path::Path;
use tracing::debug;

pub fn get_project_repo(tool_config: &ToolConfig, project_name: &str) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(tool_config.github_username(), tool_config.github_token())
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
        debug!("接收到：[{received_objects:?}]对象，总共：[{total_objects}]对象。" );
        true
    });

    let mut fetch_options = FetchOptions::new();
    if let Some(proxy_config) = tool_config.proxy() {
        let mut proxy_options = ProxyOptions::new();
        proxy_options.url(proxy_config.url());
        fetch_options.proxy_options(proxy_options);
    }
    
    fetch_options.remote_callbacks(callbacks);

    let github_repo = tool_config.projects().get(project_name).ok_or(anyhow::anyhow!("项目配置未找到"))?;
    let github_repo_url = github_repo.github_repo_url();
    let project_local_path = github_repo.project_local_path();
    let github_repo_branch = github_repo.github_repo_branch();

    debug!("正在获取仓库：[{github_repo_url}] 的 [{github_repo_branch}] 分支到 [{project_local_path:?}]");
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