//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::clean::log::LogCleaner;
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
    /// Show task metadata info.
    Info {
        /// Space name.
        #[arg(short, long)]
        space: String,
        /// Project name.
        #[arg(short, long)]
        project: String,
        /// Task ID.
        #[arg(short, long)]
        task: String,
    },
    /// Clean log files.
    Clean {
        /// Space name.
        #[arg(short, long)]
        space: Option<String>,
        /// Project name.
        #[arg(short, long)]
        project: Option<String>,
        /// Task ID.
        #[arg(short, long)]
        task: Option<String>,
        /// Clean all logs.
        #[arg(short, long, default_value = "false")]
        all: bool,
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
            LogSubcommand::Info { space, project, task } => {
                log::print_task_info(&space, &project, &task)?;
            }
            LogSubcommand::Clean { space, project, task, all } => {
                match (task, project, space, all) {
                    (Some(task), Some(project), Some(space), _) => {
                        LogCleaner::clean_task(&space, &project, &task).await?;
                    }
                    (None, Some(project), Some(space), _) => {
                        LogCleaner::clean_project(&space, &project).await?;
                    }
                    (None, None, Some(space), _) => {
                        LogCleaner::clean_space(&space).await?;
                    }
                    (None, None, None, true) => {
                        LogCleaner::clean_all().await?;
                    }
                    _ => {
                        tracing::warn!(
                            "Please provide valid parameters for log cleaning"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
