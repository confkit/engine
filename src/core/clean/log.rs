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

        let log_dir = HOST_LOG_DIR;

        // 检查目录是否存在
        if !std::path::Path::new(&log_dir).exists() {
            tracing::error!("Log directory does not exist: {}", log_dir);
            return Ok(());
        }

        // 获取 log_dir 下的所有目录
        let dirs = fs::read_dir(log_dir)?;

        // 遍历 dirs 下的所有目录, 删除所有目录
        for dir in dirs {
            if dir.is_err() {
                continue;
            }
            let dir = dir.unwrap();
            let dir_name = dir.file_name().to_string_lossy().to_string();
            // 匹配 <space_name>
            if !dir_name.starts_with(format!("<{space_name}>").as_str()) {
                continue;
            }
            // 删除目录
            VolumesCleaner::clean_dir(&format!("{log_dir}/{dir_name}"), true).await?;
        }
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
        // 获取 log_dir 下的所有目录
        let dirs = fs::read_dir(project_log_dir)?;

        // 遍历 dirs 下的所有目录, 删除所有目录
        for dir in dirs {
            if dir.is_err() {
                continue;
            }
            let dir = dir.unwrap();
            let dir_name = dir.file_name().to_string_lossy().to_string();
            // 匹配 <task_id>
            if !dir_name.ends_with(format!("{task_id}.log").as_str()) {
                continue;
            }
            // 删除文件
            VolumesCleaner::remove_file(dir.path().to_str().unwrap()).await?;
        }
        Ok(())
    }
}
