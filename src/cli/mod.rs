use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod builder;
pub mod clean;
pub mod interactive;
pub mod run;

use builder::BuilderCommand;
use clean::CleanArgs;
use interactive::InteractiveCommand;
use run::RunArgs;

#[derive(Parser)]
#[command(name = "confkit")]
#[command(about = "confkit CLI - Configuration-driven build and deployment tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builder management.
    Builder(BuilderCommand),
    /// Run build task
    Run(RunArgs),
    /// Clean logs
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
            Some(Commands::Builder(cmd)) => cmd.execute().await,
            Some(Commands::Run(args)) => run::handle_run(&args).await,
            Some(Commands::Clean(args)) => clean::handle_clean(&args).await,
            None => InteractiveCommand::execute().await,
        }
    }
}
