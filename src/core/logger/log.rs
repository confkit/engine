//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log implementation

use anyhow::Result;
use std::fs;

use crate::formatter::path::PathFormatter;

pub fn print_task_log(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
    let host_log_path = PathFormatter::get_project_log_path(space_name, project_name);
    let files = fs::read_dir(&host_log_path)?;

    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();

        if file_name.contains(task_id) {
            let file_path = file.path();
            let file_content = fs::read_to_string(file_path)?;

            tracing::info!("{}", file_content);

            break;
        }
    }

    Ok(())
}
