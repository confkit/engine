//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Log cleaner implementation

use anyhow::Result;
use std::path::Path;

use crate::core::clean::volumes::VolumesCleaner;
use crate::infra::db::TaskDb;
use crate::shared::constants::HOST_LOG_DIR;

pub struct LogCleaner {}

impl LogCleaner {
    pub async fn clean_all() -> Result<()> {
        tracing::info!("Cleaning all logs");

        let db = TaskDb::open()?;
        db.delete_all()?;

        VolumesCleaner::clean_dir(HOST_LOG_DIR, false).await?;

        Ok(())
    }

    pub async fn clean_space(space_name: &str) -> Result<()> {
        tracing::info!("Cleaning space: {}", space_name);

        let db = TaskDb::open()?;
        let paths = db.list_all_by_space(space_name)?;
        Self::remove_log_files(&paths).await;
        db.delete_by_space(space_name)?;

        Ok(())
    }

    pub async fn clean_project(space_name: &str, project_name: &str) -> Result<()> {
        tracing::info!("Cleaning space: {}, project: {}", space_name, project_name);

        let db = TaskDb::open()?;
        let paths = db.list_all_by_project(space_name, project_name)?;
        Self::remove_log_files(&paths).await;
        db.delete_by_project(space_name, project_name)?;

        Ok(())
    }

    pub async fn clean_task(task_id: &str) -> Result<()> {
        tracing::info!("Cleaning task: {}", task_id);

        let db = TaskDb::open()?;
        if let Some(record) = db.get_task(task_id)? {
            Self::remove_log_files(&[record.log_path]).await;
            db.delete_task(task_id)?;
        } else {
            tracing::warn!("Task not found: {}", task_id);
        }

        Ok(())
    }

    /// 根据相对路径列表删除日志文件及对应的 meta.json
    async fn remove_log_files(relative_paths: &[String]) {
        for rel_path in relative_paths {
            let log_path = format!("{}/{}", HOST_LOG_DIR, rel_path);
            let meta_path = log_path.replace(".log", ".meta.json");

            if Path::new(&log_path).exists() {
                let _ = std::fs::remove_file(&log_path);
            }
            if Path::new(&meta_path).exists() {
                let _ = std::fs::remove_file(&meta_path);
            }
        }
    }
}
