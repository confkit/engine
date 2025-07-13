use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod builder;
pub mod clean;
pub mod run;
// pub mod interactive;
// pub mod log;
// pub mod task;

// 重新导出主要的命令结构

use builder::BuilderCommand;
// use interactive::InteractiveCommand;
// use log::LogCommand;
use clean::CleanArgs;
use run::RunArgs;
// use task::TaskCommand;

// 重新导出log模块的公开函数
// pub use log::{handle_log_list, handle_log_show};

#[derive(Parser)]
#[command(name = "confkit")]
#[command(about = "confkit CLI - Configuration-driven build and deployment tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builder management.
    Builder(BuilderCommand),
    /// Run build task
    Run(RunArgs),
    /// Clean volumes
    Clean(CleanArgs),
}

// /// 查看空间
// Space(SpaceCommand),
// /// 查看项目
// Project(ProjectCommand),
// /// 管理任务
// Task(TaskCommand),
// /// 查看日志
// Log(LogCommand),
// /// 交互 式模式
// Interactive(InteractiveCommand),

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Builder(cmd) => cmd.execute().await,
            Commands::Run(args) => run::handle_run(&args).await,
            Commands::Clean(args) => clean::handle_clean(&args).await,
            // Commands::Task(cmd) => cmd.execute().await,
            // Commands::Log(cmd) => cmd.execute().await,
            // Commands::Interactive(cmd) => cmd.execute().await,
        }
    }
}
