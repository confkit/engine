//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log subcommand implementation

use anyhow::Result;
use clap::Args;

use crate::core::logger::log::print_task_log;

#[derive(Debug, Args)]
pub struct LogArgs {
    /// space name
    #[arg(short, long)]
    pub space: String,

    /// project name
    #[arg(short, long)]
    pub project: String,

    /// task id
    #[arg(short, long)]
    pub task: String,
}

pub async fn handle_log(args: &LogArgs) -> Result<()> {
    print_task_log(args.space.as_str(), args.project.as_str(), args.task.as_str())?;

    Ok(())
}
