//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Run build task subcommand implementation

use std::collections::HashMap;

use crate::core::executor::runner::Runner;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// space name
    #[arg(short, long)]
    pub space: String,

    /// project name
    #[arg(short, long)]
    pub project: String,

    /// environments - not required
    #[arg(short, long)]
    pub environments: Option<Vec<String>>,
    // /// 详细输出模式
    // #[arg(long, short)]
    // pub verbose: bool,
}

/// 处理 run 命令
pub async fn handle_run(args: &RunArgs) -> Result<()> {
    let environment_from_args = parse_environments(args.environments.clone()).await?;

    let mut runner =
        Runner::new(args.space.as_str(), args.project.as_str(), environment_from_args).await?;

    runner.start().await?;

    Ok(())
}

// 解析环境变量
async fn parse_environments(environments: Option<Vec<String>>) -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();

    if let Some(env_strings) = environments {
        for env_str in env_strings {
            if let Some((key, value)) = env_str.split_once('=') {
                env_map.insert(key.to_string(), value.to_string());
            } else {
                // 格式不正确时，记录警告并跳过
                tracing::warn!("跳过格式不正确的环境变量: {}", env_str);
            }
        }
    }

    Ok(env_map)
}
