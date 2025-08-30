//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Volumes cleaner implementation

use crate::shared::constants::{HOST_ARTIFACTS_ROOT_DIR, HOST_WORKSPACE_DIR};
use anyhow::Result;
use std::{fs, path::Path};

pub struct VolumesCleaner;

impl VolumesCleaner {
    /// 清空 workspace, 但保留 workspace 目录
    pub async fn clean_workspace() -> Result<()> {
        Self::clean_dir(HOST_WORKSPACE_DIR, false).await
    }

    /// 清空 artifacts, 但保留 artifacts 目录
    pub async fn clean_artifacts() -> Result<()> {
        Self::clean_dir(HOST_ARTIFACTS_ROOT_DIR, false).await
    }

    /// 清空目录
    ///
    /// # 参数
    /// * `dir` - 目录路径
    /// * `remove_dir` - 是否删除目录本身，true 删除整个目录，false 只清空目录内容
    pub async fn clean_dir(dir: &str, remove_dir: bool) -> Result<()> {
        let dir_path = Path::new(dir);

        if !dir_path.exists() {
            return Ok(());
        }

        if remove_dir {
            // 删除整个目录
            fs::remove_dir_all(dir_path)?;
        } else {
            // 只清空目录内容，保留目录本身
            if dir_path.is_dir() {
                for entry in fs::read_dir(dir_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        fs::remove_dir_all(&path)?;
                    } else {
                        fs::remove_file(&path)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// 删除文件
    pub async fn remove_file(file: &str) -> Result<()> {
        let file_path = Path::new(file);

        if !file_path.exists() {
            return Ok(());
        }

        fs::remove_file(file_path)?;
        Ok(())
    }
}
