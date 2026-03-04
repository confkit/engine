//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::clean::log::LogCleaner;
use crate::core::logger::log;
use crate::infra::db::task_db::{PageParams, TaskFilter};

#[derive(Args)]
pub struct LogCommand {
    #[command(subcommand)]
    command: LogSubcommand,
}

#[derive(Subcommand)]
pub enum LogSubcommand {
    /// List log files (supports filtering and pagination).
    List {
        /// Space name (optional filter).
        #[arg(short, long)]
        space: Option<String>,
        /// Project name (optional filter).
        #[arg(short, long)]
        project: Option<String>,
        /// Page number (default: 1).
        #[arg(long, default_value = "1")]
        page: usize,
        /// Page size (default: 20).
        #[arg(long, default_value = "20")]
        size: usize,
    },
    /// Show a specific task log.
    Show {
        /// Task ID.
        #[arg(short, long)]
        task: String,
    },
    /// Show task metadata info.
    Info {
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
            LogSubcommand::List { space, project, page, size } => {
                let filter = TaskFilter { space_name: space, project_name: project };
                let page_params = PageParams { page, size };
                log::list_task_logs(&filter, &page_params)?;
            }
            LogSubcommand::Show { task } => {
                log::print_task_log(&task)?;
            }
            LogSubcommand::Info { task } => {
                log::print_task_info(&task)?;
            }
            LogSubcommand::Clean { space, project, task, all } => {
                match (task, project, space, all) {
                    (Some(task), _, _, _) => {
                        LogCleaner::clean_task(&task).await?;
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
                        tracing::warn!("Please provide valid parameters for log cleaning");
                    }
                }
            }
        }
        Ok(())
    }
}
