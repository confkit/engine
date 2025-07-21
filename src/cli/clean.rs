//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Clean logs subcommand implementation

use crate::core::clean::log::LogCleaner;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// clean all
    #[arg(short, long, default_value = "false")]
    pub all: bool,

    /// clean space
    #[arg(short, long)]
    pub space: Option<String>,

    /// clean project
    #[arg(short, long)]
    pub project: Option<String>,

    /// clean task
    #[arg(short, long)]
    pub task: Option<String>,
}

/// 处理 run 命令
pub async fn handle_clean(args: &CleanArgs) -> Result<()> {
    if args.task.is_some() {
        LogCleaner::clean_task(
            args.space.as_ref().unwrap(),
            args.project.as_ref().unwrap(),
            args.task.as_ref().unwrap(),
        )?;
        return Ok(());
    } else if args.project.is_some() {
        LogCleaner::clean_project(args.space.as_ref().unwrap(), args.project.as_ref().unwrap())?;
        return Ok(());
    } else if args.space.is_some() {
        LogCleaner::clean_space(args.space.as_ref().unwrap())?;
        return Ok(());
    } else if args.all {
        LogCleaner::clean_all()?;
        return Ok(());
    }

    tracing::warn!("Please provide parameters to clean");

    Ok(())
}
