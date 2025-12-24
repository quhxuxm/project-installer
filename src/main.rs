mod config;
mod repo;

use crate::config::ToolConfig;
use crate::repo::get_project_repo;
use anyhow::Result;
use ::config as lib_config;
use lib_config::Config;


#[tokio::main]
async fn main() -> Result<()> {
    let tool_config = Config::builder().add_source(lib_config::File::with_name("config")).build()?;
    let tool_config = tool_config.try_deserialize::<ToolConfig>()?;
    tool_config.projects().iter().try_for_each(|(k, _)| {
        get_project_repo(&tool_config, k)
    })?;
    Ok(())
}
