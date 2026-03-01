//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::logger::log;

#[derive(Args)]
pub struct LogCommand {
    #[command(subcommand)]
    command: LogSubcommand,
}

#[derive(Subcommand)]
pub enum LogSubcommand {
    /// List log files for a project.
    List {
        /// Space name.
        #[arg(short, long)]
        space: String,
        /// Project name.
        #[arg(short, long)]
        project: String,
    },
    /// Show a specific task log.
    Show {
        /// Space name.
        #[arg(short, long)]
        space: String,
        /// Project name.
        #[arg(short, long)]
        project: String,
        /// Task ID or log filename fragment.
        #[arg(short, long)]
        task: String,
    },
}

impl LogCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            LogSubcommand::List { space, project } => {
                log::list_task_logs(&space, &project)?;
            }
            LogSubcommand::Show { space, project, task } => {
                log::print_task_log(&space, &project, &task)?;
            }
        }
        Ok(())
    }
}
