//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: CLI implementation

use anyhow::Result;
use clap::{Parser, Subcommand};

mod builder;
mod clean;
mod image;
mod interactive;
mod log;
mod run;

use builder::BuilderCommand;
use clean::CleanArgs;
use image::ImageCommand;
use interactive::InteractiveCommand;
use log::LogArgs;
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
    /// Image management.
    Image(ImageCommand),
    /// Run build task
    Run(RunArgs),
    /// Clean logs
    Clean(CleanArgs),
    /// Log management.
    Log(LogArgs),
}
/// Space management.
// Space(SpaceCommand),
/// Project management.
// Project(ProjectCommand),
/// Task management.
// Task(TaskCommand),
/// Log management.
// Log(LogCommand),

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Some(Commands::Builder(cmd)) => cmd.execute().await,
            Some(Commands::Image(cmd)) => cmd.execute().await,
            Some(Commands::Run(args)) => run::handle_run(&args).await,
            Some(Commands::Clean(args)) => clean::handle_clean(&args).await,
            Some(Commands::Log(args)) => log::handle_log(&args).await,
            None => InteractiveCommand::execute().await,
        }
    }
}
