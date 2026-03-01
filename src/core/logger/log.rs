//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log implementation

use anyhow::Result;
use std::fs;

use crate::formatter::path::PathFormatter;

/// 列出指定项目的所有日志文件
pub fn list_task_logs(space_name: &str, project_name: &str) -> Result<()> {
    let host_log_path = PathFormatter::log_project_dir(space_name, project_name);

    let dir = match fs::read_dir(&host_log_path) {
        Ok(dir) => dir,
        Err(_) => {
            tracing::info!("No logs found for space '{}' project '{}'", space_name, project_name);
            return Ok(());
        }
    };

    let mut entries: Vec<_> = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_str()?.to_string();
            if name.ends_with(".log") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if entries.is_empty() {
        tracing::info!("No logs found for space '{}' project '{}'", space_name, project_name);
        return Ok(());
    }

    // 按文件名排序（包含时间戳，天然按时间排序）
    entries.sort();

    tracing::info!("Logs for [{}] {}:", space_name, project_name);
    for entry in &entries {
        tracing::info!("  {}", entry);
    }
    tracing::info!("Total: {} log(s)", entries.len());

    Ok(())
}

/// 打印指定任务的日志内容
pub fn print_task_log(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
    let host_log_path = PathFormatter::log_project_dir(space_name, project_name);
    let files = fs::read_dir(&host_log_path)?;

    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();

        if file_name.contains(task_id) {
            let file_path = file.path();
            let file_content = fs::read_to_string(file_path)?;

            tracing::info!("{}", file_content);

            return Ok(());
        }
    }

    tracing::warn!("Log not found for task '{}'", task_id);
    Ok(())
}
