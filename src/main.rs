use anyhow::Result;
use clap::Parser;

mod cli;
mod core;
mod infrastructure;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 执行命令
    cli.execute().await?;

    Ok(())
}
