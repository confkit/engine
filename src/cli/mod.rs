//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: CLI implementation

use anyhow::Result;
use clap::{Parser, Subcommand};

mod builder;
mod clean;
mod config;
mod image;
mod interactive;
mod log;
mod run;

use builder::BuilderCommand;
use clean::CleanCommand;
use config::ConfigCommand;
use image::ImageCommand;
use interactive::InteractiveCommand;
use log::LogCommand;
use run::RunArgs;

#[derive(Parser)]
#[command(name = "confkit")]
#[command(about = "confkit CLI - Configuration-driven build and deployment tool")]
#[command(version, disable_version_flag = true)]
pub struct Cli {
    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    pub version: (),

    /// Hide level information in logs
    #[arg(long, global = true)]
    pub hide_level: bool,
    /// Command to execute
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builder management.
    Builder(BuilderCommand),
    /// Image management.
    Image(ImageCommand),
    /// Run build task
    Run(RunArgs),
    /// Clean resources.
    Clean(CleanCommand),
    /// Log management.
    Log(LogCommand),
    /// Configuration management.
    Config(ConfigCommand),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        let result = match self.command {
            Some(Commands::Builder(cmd)) => cmd.execute().await,
            Some(Commands::Image(cmd)) => cmd.execute().await,
            Some(Commands::Run(args)) => run::handle_run(&args).await,
            Some(Commands::Clean(cmd)) => cmd.execute().await,
            Some(Commands::Log(cmd)) => cmd.execute().await,
            Some(Commands::Config(cmd)) => cmd.execute().await,
            None => InteractiveCommand::execute().await,
        };

        // 根据命令执行结果决定退出状态
        match &result {
            Ok(_) => {
                tracing::debug!("Command executed successfully");
            }
            Err(e) => {
                tracing::error!("Command failed: {}", e);
                std::process::exit(1);
            }
        }

        result
    }

    pub fn parse_args() -> Self {
        Self::parse()
    }
}
