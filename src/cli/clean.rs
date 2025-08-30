//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Clean logs subcommand implementation

use crate::core::clean::{log::LogCleaner, volumes::VolumesCleaner};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// clean log
    #[arg(short, long)]
    pub log: bool,

    /// clean workspace
    #[arg(long)]
    pub workspace: bool,

    /// clean artifacts
    #[arg(long)]
    pub artifacts: bool,

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
    if args.workspace {
        tracing::debug!("Cleaning workspace");

        VolumesCleaner::clean_workspace().await?;
        return Ok(());
    } else if args.artifacts {
        tracing::debug!("Cleaning artifacts");

        VolumesCleaner::clean_artifacts().await?;
        return Ok(());
    }

    if args.log {
        tracing::debug!("Cleaning log");

        return match (&args.task, &args.project, &args.space, args.all) {
            (Some(task), Some(project), Some(space), _) => {
                LogCleaner::clean_task(space, project, task).await?;
                Ok(())
            }
            (None, Some(project), Some(space), _) => {
                LogCleaner::clean_project(space, project).await?;
                Ok(())
            }
            (None, None, Some(space), _) => {
                LogCleaner::clean_space(space).await?;
                Ok(())
            }
            (None, None, None, true) => {
                LogCleaner::clean_all().await?;
                Ok(())
            }
            _ => {
                tracing::warn!("Please provide valid parameters for log cleaning");
                Ok(())
            }
        };
    }

    if args.all {
        tracing::debug!("Cleaning all");

        VolumesCleaner::clean_workspace().await?;
        VolumesCleaner::clean_artifacts().await?;
        LogCleaner::clean_all().await?;
        return Ok(());
    }

    tracing::warn!("Please provide parameters to clean");

    Ok(())
}
