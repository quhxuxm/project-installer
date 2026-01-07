mod configuration;
mod initialize;
mod remote_repo;

use crate::config::ToolConfig;
use crate::message::output::{
    ApplicationStateOutput, CommandStateOutput, ErrorOutput, GitHubStateOutput, ProjectStateOutput,
};
pub use configuration::save_github_config;
pub use configuration::save_project_config;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
pub use initialize::load_application_state;
pub use remote_repo::retrieve_code;
use std::path::Path;

pub struct RetrieveCodeFromGithubInfo<'a> {
    github_username: String,
    github_token: String,
    github_branch: String,
    github_repo_url: String,
    local_repo_path: &'a Path,
    proxy_url: Option<String>,
}

fn retrieve_code_from_github(retrieve_info: RetrieveCodeFromGithubInfo) -> Result<(), ErrorOutput> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext(&retrieve_info.github_username, &retrieve_info.github_token)
    });
    callbacks.transfer_progress(|progress| {
        let received_objects = progress.received_objects();
        let total_objects = progress.total_objects();
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

fn generate_application_state_message(
    tool_config: &ToolConfig,
) -> Result<ApplicationStateOutput, ErrorOutput> {
    let github_config = tool_config.github();
    let projects_config = tool_config.projects();
    let application_state = ApplicationStateOutput {
        github: GitHubStateOutput {
            username: github_config.username().map(|v| v.to_owned()),
            token: github_config.token().map(|v| v.to_owned()),
        },
        projects: projects_config
            .iter()
            .map(|(k, v)| {
                let project_state = ProjectStateOutput {
                    name: v.name().map(|i| i.to_owned()),
                    description: v.description().map(|i| i.to_owned()),
                    github_repo_url: v.github_repo_url().map(|i| i.to_owned()),
                    github_branches: vec![
                        "6.9.0-release".to_string(),
                        "6.10.0-release".to_string(),
                        "6.9-develop".to_string(),
                        "6.10.0-develop".to_string(),
                    ],
                    configured_github_branch: v.github_branch().map(|i| i.to_owned()),
                    local_repo_path: v.local_repo_path().map(|i| i.to_owned()),
                    build_command: v.build_command().map(|i| {
                        let command_status = CommandStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    run_command: v.run_command().map(|i| {
                        let command_status = CommandStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    debug_command: v.debug_command().map(|i| {
                        let command_status = CommandStateOutput {
                            cmd: i.command().to_owned(),
                            args: i.args().to_vec(),
                        };
                        command_status
                    }),
                    startup_dependencies: v.startup_dependencies().to_vec(),
                    backend_process_id: None,
                };
                (k.clone(), project_state)
            })
            .collect(),
    };
    Ok(application_state)
}
