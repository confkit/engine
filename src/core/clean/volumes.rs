//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Volumes cleaner implementation

use anyhow::Result;
use std::{fs, path::Path};

use crate::{formatter::path::PathFormatter, shared::constants::HOST_WORKSPACE_DIR};

pub struct VolumesCleaner;

impl VolumesCleaner {
    pub async fn clean_workspace(
        space_name: &str,
        project_name: &str,
        task_id: &str,
    ) -> Result<()> {
        let task_dir = PathFormatter::get_task_path(&space_name, &project_name, &task_id);

        let host_workspace_dir = format!("{}/{}", HOST_WORKSPACE_DIR, task_dir);

        let host_workspace_dir = Path::new(&host_workspace_dir);

        if host_workspace_dir.exists() {
            fs::remove_dir_all(host_workspace_dir)?;
        }

        Ok(())
    }
}
