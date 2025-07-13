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
    // /// 详细输出模式
    // #[arg(long, short)]
    // pub verbose: bool,
}

/// 处理 run 命令
pub async fn handle_run(args: &RunArgs) -> Result<()> {
    let runner = Runner::new(args.space.as_str(), args.project.as_str());
    runner.start().await?;
    Ok(())
}
