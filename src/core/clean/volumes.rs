//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Volumes cleaner implementation

use anyhow::Result;
use std::{fs, path::Path};

pub struct VolumesCleaner;

impl VolumesCleaner {
    /// 清空目录
    pub async fn clean_dir(dir: &str) -> Result<()> {
        let dir = Path::new(dir);
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        Ok(())
    }
}
