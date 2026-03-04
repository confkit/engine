//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Clean subcommand implementation

use crate::core::clean::{log::LogCleaner, volumes::VolumesCleaner};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct CleanCommand {
    #[command(subcommand)]
    command: CleanSubcommand,
}

#[derive(Subcommand)]
pub enum CleanSubcommand {
    /// Clean workspace directory.
    Workspace,
    /// Clean artifacts directory.
    Artifacts,
    /// Clean cache directory.
    Cache,
    /// Clean temp directory.
    Temp,
    /// Clean log files.
    Log {
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
    /// Clean all (workspace, artifacts, cache, temp, logs).
    All,
}

impl CleanCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            CleanSubcommand::Workspace => {
                tracing::debug!("Cleaning workspace");
                VolumesCleaner::clean_workspace().await?;
            }
            CleanSubcommand::Artifacts => {
                tracing::debug!("Cleaning artifacts");
                VolumesCleaner::clean_artifacts().await?;
            }
            CleanSubcommand::Cache => {
                tracing::debug!("Cleaning cache");
                VolumesCleaner::clean_cache().await?;
            }
            CleanSubcommand::Temp => {
                tracing::debug!("Cleaning temp");
                VolumesCleaner::clean_temp().await?;
            }
            CleanSubcommand::Log { space, project, task, all } => {
                tracing::debug!("Cleaning log");
                handle_clean_log(space, project, task, all).await?;
            }
            CleanSubcommand::All => {
                tracing::debug!("Cleaning all");
                VolumesCleaner::clean_workspace().await?;
                VolumesCleaner::clean_artifacts().await?;
                VolumesCleaner::clean_cache().await?;
                VolumesCleaner::clean_temp().await?;
                LogCleaner::clean_all().await?;
            }
        }
        Ok(())
    }
}

async fn handle_clean_log(
    space: Option<String>,
    project: Option<String>,
    task: Option<String>,
    all: bool,
) -> Result<()> {
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
    Ok(())
}
