//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log implementation

use anyhow::Result;
use std::fs;

use crate::core::executor::types::TaskMetadata;
use crate::formatter::path::PathFormatter;

/// 列出指定项目的所有任务日志
pub fn list_task_logs(space_name: &str, project_name: &str) -> Result<()> {
    let project_dir = PathFormatter::log_project_dir(space_name, project_name);

    let date_dirs = match fs::read_dir(&project_dir) {
        Ok(dir) => dir,
        Err(_) => {
            tracing::info!("No logs found for space '{}' project '{}'", space_name, project_name);
            return Ok(());
        }
    };

    // 收集日期目录并排序
    let mut date_entries: Vec<_> = date_dirs
        .filter_map(|e| {
            let e = e.ok()?;
            if e.path().is_dir() {
                Some(e.file_name().to_str()?.to_string())
            } else {
                None
            }
        })
        .collect();
    date_entries.sort();

    let mut total_tasks = 0usize;

    tracing::info!("Logs for {}/{}:", space_name, project_name);

    for date in &date_entries {
        let date_path = PathFormatter::log_date_dir(space_name, project_name, date);
        let task_dirs = match fs::read_dir(&date_path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        let mut task_entries: Vec<_> = task_dirs
            .filter_map(|e| {
                let e = e.ok()?;
                if e.path().is_dir() {
                    Some(e.file_name().to_str()?.to_string())
                } else {
                    None
                }
            })
            .collect();
        task_entries.sort();

        if task_entries.is_empty() {
            continue;
        }

        tracing::info!("  {}/", date);

        for task_dir_name in &task_entries {
            total_tasks += 1;
            let task_path =
                PathFormatter::log_task_dir(space_name, project_name, date, task_dir_name);
            let metadata_path = format!("{}/metadata.json", task_path);

            if let Ok(content) = fs::read_to_string(&metadata_path) {
                if let Ok(meta) = serde_json::from_str::<TaskMetadata>(&content) {
                    let duration_str = match meta.duration_ms {
                        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
                        None => "-".to_string(),
                    };
                    let step_count = meta.steps.len();
                    tracing::info!(
                        "    {}  [{:?}]  {}  ({} steps)",
                        task_dir_name,
                        meta.status,
                        duration_str,
                        step_count
                    );
                    continue;
                }
            }
            // metadata 不可读时仅显示目录名
            tracing::info!("    {}", task_dir_name);
        }
    }

    if total_tasks == 0 {
        tracing::info!("No logs found for space '{}' project '{}'", space_name, project_name);
    } else {
        tracing::info!("Total: {} task(s)", total_tasks);
    }

    Ok(())
}

/// 打印指定任务的日志内容
pub fn print_task_log(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
    let project_dir = PathFormatter::log_project_dir(space_name, project_name);

    // 遍历日期目录 -> 任务目录，匹配 task_id
    let date_dirs = fs::read_dir(&project_dir)?;

    for date_entry in date_dirs {
        let date_entry = date_entry?;
        if !date_entry.path().is_dir() {
            continue;
        }

        let task_dirs = match fs::read_dir(date_entry.path()) {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        for task_entry in task_dirs {
            let task_entry = task_entry?;
            let dir_name = task_entry.file_name();
            let dir_name = dir_name.to_string_lossy();

            if !dir_name.ends_with(task_id) {
                continue;
            }

            // 打印 metadata 摘要
            let metadata_path = format!("{}/metadata.json", task_entry.path().display());
            if let Ok(content) = fs::read_to_string(&metadata_path) {
                if let Ok(meta) = serde_json::from_str::<TaskMetadata>(&content) {
                    let duration_str = match meta.duration_ms {
                        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
                        None => "-".to_string(),
                    };
                    tracing::info!(
                        "Task: {} | Status: {:?} | Duration: {} | Steps: {}",
                        meta.task_id,
                        meta.status,
                        duration_str,
                        meta.steps.len()
                    );
                    tracing::info!("---");
                }
            }

            // 打印日志内容
            let log_path = format!("{}/task.log", task_entry.path().display());
            match fs::read_to_string(&log_path) {
                Ok(content) => {
                    tracing::info!("{}", content);
                }
                Err(_) => {
                    tracing::warn!("Log file not found at: {}", log_path);
                }
            }
            return Ok(());
        }
    }

    tracing::warn!("Log not found for task '{}'", task_id);
    Ok(())
}
