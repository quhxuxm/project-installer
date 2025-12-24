mod config;
mod repo;

use crate::config::{ProjectConfig, ToolConfig};
use crate::repo::get_project_repo;
use anyhow::Result;
use ::config as lib_config;
use lib_config::Config;
use std::io;
use std::io::Write;
use std::process::Command;
use tracing::{error, info};


/// 处理工程
fn handle_project(tool_config: &ToolConfig, project_name: &str, project_config: &ProjectConfig) -> Result<()> {
    info!("开始取得工程[{project_name}]的代码...");
    get_source_code_from_github(tool_config, project_name, project_config)?;
    info!("开始构建工程[{project_name}]...");
    build_source_code(project_config)?;
    Ok(())
}

/// 从 GitHub 上取得工程代码
fn get_source_code_from_github(tool_config: &ToolConfig, project_name: &str, project_config: &ProjectConfig) -> Result<()> {
    get_project_repo(tool_config.github_username(), tool_config.github_token(), tool_config.proxy().map(|config| config.url()), project_name, project_config)?;
    info!("成功从GitHub上取得 [{project_name}] 的代码");
    Ok(())
}

///使用 Maven 构建工程
fn build_source_code(project_config: &ProjectConfig) -> Result<()> {
    let mut command = Command::new("mvn");
    command.arg("clean").arg("package").current_dir(project_config.project_local_path());
    let command_output = command.output()?;
    io::stdout().write_all(&command_output.stdout)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let tool_config = Config::builder().add_source(lib_config::File::with_name("config")).build()?;
    let tool_config = tool_config.try_deserialize::<ToolConfig>()?;
    tool_config.projects().iter().for_each(|(project_name, project_config)| {
        if let Err(e) = handle_project(&tool_config, project_name, project_config) {
            error!("处理工程[{project_name}]失败，错误信息：{e}");
        }
    });
    Ok(())
}
