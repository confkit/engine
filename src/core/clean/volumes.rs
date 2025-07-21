//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Volumes cleaner implementation

use anyhow::Result;
use std::{fs, path::Path};

use crate::{
    core::executor::task::Task,
    shared::constants::{HOST_ARTIFACTS_DIR, HOST_WORKSPACE_DIR},
};

pub struct VolumesCleaner;

impl VolumesCleaner {
    pub async fn clean(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
        let task_dir = Task::format_task_path(&space_name, &project_name, &task_id);

        let host_workspace_dir = format!("{}/{}", HOST_WORKSPACE_DIR, task_dir);
        let host_artifacts_dir = format!("{}/{}", HOST_ARTIFACTS_DIR, task_dir);

        let host_workspace_dir = Path::new(&host_workspace_dir);
        let host_artifacts_dir = Path::new(&host_artifacts_dir);

        if host_workspace_dir.exists() {
            fs::remove_dir_all(host_workspace_dir)?;
        }

        if host_artifacts_dir.exists() {
            fs::remove_dir_all(host_artifacts_dir)?;
        }

        Ok(())
    }
}
