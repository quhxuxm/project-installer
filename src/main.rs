mod config;
use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use std::path::Path;

static RGS_REPO_URL: &str = "https://github.com/igt-playcasino/rgs.git";
static RGS_REPO_PATH: &str = "D:\\tmp\\rgs";

static IGT_PLAY_CASINO_TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 定义一个回调函数来处理身份验证
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _, _allowed_types| {
        Cred::userpass_plaintext("quhxuxm", IGT_PLAY_CASINO_TOKEN)
    });
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    println!("正在克隆仓库：{} 到 {:?}", RGS_REPO_URL, RGS_REPO_PATH);
    // 执行克隆操作
    builder.clone(RGS_REPO_URL, Path::new(RGS_REPO_PATH))?;
    Ok(())
}
