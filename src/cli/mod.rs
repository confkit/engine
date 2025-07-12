use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod builder;
pub mod interactive;
pub mod logs;
pub mod run;
pub mod task;

// 重新导出主要的命令结构

use builder::BuilderCommand;
use interactive::InteractiveCommand;
use logs::LogsCommand;
use run::RunArgs;
use task::TaskCommand;

#[derive(Parser)]
#[command(name = "confkit")]
#[command(about = "confkit CLI - 配置驱动的构建和部署工具")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 运行构建任务
    Run(RunArgs),
    /// 管理构建器
    Builder(BuilderCommand),
    /// 管理任务
    Task(TaskCommand),
    /// 查看日志
    Logs(LogsCommand),
    /// 交互式模式
    Interactive(InteractiveCommand),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Run(args) => run::handle_run(&args).await,
            Commands::Builder(cmd) => cmd.execute().await,
            Commands::Task(cmd) => cmd.execute().await,
            Commands::Logs(cmd) => cmd.execute().await,
            Commands::Interactive(cmd) => cmd.execute().await,
        }
    }
}
