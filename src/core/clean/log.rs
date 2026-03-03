//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Log cleaner implementation

use anyhow::Result;
use std::fs;

use crate::core::clean::volumes::VolumesCleaner;
use crate::formatter::path::PathFormatter;
use crate::shared::constants::HOST_LOG_DIR;

pub struct LogCleaner {}

impl LogCleaner {
    pub async fn clean_all() -> Result<()> {
        tracing::info!("Cleaning all logs");

        VolumesCleaner::clean_dir(HOST_LOG_DIR, false).await?;

        Ok(())
    }

    pub async fn clean_space(space_name: &str) -> Result<()> {
        tracing::info!("Cleaning space: {}", space_name);

        let space_dir = PathFormatter::log_space_dir(space_name);
        VolumesCleaner::clean_dir(&space_dir, true).await?;

        Ok(())
    }

    pub async fn clean_project(space_name: &str, project_name: &str) -> Result<()> {
        tracing::info!("Cleaning space: {}, project: {}", space_name, project_name);

        let project_log_dir = PathFormatter::log_project_dir(space_name, project_name);

        VolumesCleaner::clean_dir(&project_log_dir, true).await?;

        Ok(())
    }

    pub async fn clean_task(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
        tracing::info!(
            "Cleaning space: {}, project: {}, task: {}",
            space_name,
            project_name,
            task_id
        );

        let project_log_dir = PathFormatter::log_project_dir(space_name, project_name);
        // 遍历日期目录
        let date_dirs = match fs::read_dir(&project_log_dir) {
            Ok(dirs) => dirs,
            Err(_) => return Ok(()),
        };

        for date_entry in date_dirs {
            let date_entry = match date_entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if !date_entry.path().is_dir() {
                continue;
            }

            let task_dirs = match fs::read_dir(date_entry.path()) {
                Ok(dirs) => dirs,
                Err(_) => continue,
            };

            for task_entry in task_dirs {
                let task_entry = match task_entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let dir_name = task_entry.file_name();
                let dir_name = dir_name.to_string_lossy();

                // 匹配目录名后缀中的 task_id
                if dir_name.ends_with(task_id) {
                    VolumesCleaner::clean_dir(task_entry.path().to_str().unwrap(), true).await?;
                }
            }
        }
        Ok(())
    }
}
