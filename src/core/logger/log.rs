//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log implementation

use anyhow::Result;
use std::fs;

use crate::core::executor::types::TaskMetadata;
use crate::formatter::path::PathFormatter;
use crate::shared::constants::{TASK_LOG_FILE, TASK_META_FILE};

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
            let metadata_path = format!("{}/{}", task_path, TASK_META_FILE);

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

/// 收集指定项目的所有任务条目，返回 (显示文本, task_dir_name) 列表，按时间倒序
pub fn collect_task_entries(
    space_name: &str,
    project_name: &str,
) -> Result<Vec<(String, String)>> {
    let project_dir = PathFormatter::log_project_dir(space_name, project_name);

    let date_dirs = match fs::read_dir(&project_dir) {
        Ok(dir) => dir,
        Err(_) => return Ok(vec![]),
    };

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

    let mut result = vec![];

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

        for task_dir_name in task_entries {
            let task_path =
                PathFormatter::log_task_dir(space_name, project_name, date, &task_dir_name);
            let metadata_path = format!("{}/{}", task_path, TASK_META_FILE);

            let label = if let Ok(content) = fs::read_to_string(&metadata_path) {
                if let Ok(meta) = serde_json::from_str::<TaskMetadata>(&content) {
                    let duration_str = match meta.duration_ms {
                        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
                        None => "-".to_string(),
                    };
                    format!(
                        "{}/{}  [{:?}]  {}  ({} steps)",
                        date,
                        task_dir_name,
                        meta.status,
                        duration_str,
                        meta.steps.len()
                    )
                } else {
                    format!("{}/{}", date, task_dir_name)
                }
            } else {
                format!("{}/{}", date, task_dir_name)
            };

            result.push((label, task_dir_name));
        }
    }

    // 倒序，最新的在前面
    result.reverse();

    Ok(result)
}

/// 根据 task_id 查找任务目录路径
fn find_task_dir(
    space_name: &str,
    project_name: &str,
    task_id: &str,
) -> Result<Option<std::path::PathBuf>> {
    let project_dir = PathFormatter::log_project_dir(space_name, project_name);
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

            if dir_name.ends_with(task_id) {
                return Ok(Some(task_entry.path()));
            }
        }
    }

    Ok(None)
}

/// 打印指定任务的日志内容
pub fn print_task_log(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
    let task_dir = match find_task_dir(space_name, project_name, task_id)? {
        Some(path) => path,
        None => {
            tracing::warn!("Log not found for task '{}'", task_id);
            return Ok(());
        }
    };

    // 打印 metadata 摘要
    let metadata_path = task_dir.join(TASK_META_FILE);
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
    let log_path = task_dir.join(TASK_LOG_FILE);
    match fs::read_to_string(&log_path) {
        Ok(content) => {
            tracing::info!("{}", content);
        }
        Err(_) => {
            tracing::warn!("Log file not found at: {}", log_path.display());
        }
    }

    Ok(())
}

/// 打印指定任务的元数据信息
pub fn print_task_info(space_name: &str, project_name: &str, task_id: &str) -> Result<()> {
    let task_dir = match find_task_dir(space_name, project_name, task_id)? {
        Some(path) => path,
        None => {
            tracing::warn!("Task not found: '{}'", task_id);
            return Ok(());
        }
    };

    let metadata_path = task_dir.join(TASK_META_FILE);
    let content = match fs::read_to_string(&metadata_path) {
        Ok(c) => c,
        Err(_) => {
            tracing::warn!("Metadata not found for task '{}'", task_id);
            return Ok(());
        }
    };

    let meta: TaskMetadata = serde_json::from_str(&content)?;

    let duration_str = match meta.duration_ms {
        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
        None => "-".to_string(),
    };

    tracing::info!("Task ID:      {}", meta.task_id);
    tracing::info!("Space:        {}", meta.space_name);
    tracing::info!("Project:      {}", meta.project_name);
    tracing::info!("Status:       {:?}", meta.status);
    tracing::info!("Started at:   {}", meta.started_at);
    tracing::info!("Finished at:  {}", meta.finished_at.as_deref().unwrap_or("-"));
    tracing::info!("Duration:     {}", duration_str);
    tracing::info!("Steps:        {}", meta.steps.len());

    if !meta.steps.is_empty() {
        tracing::info!("");
        for (i, step) in meta.steps.iter().enumerate() {
            let step_duration = match step.duration_ms {
                Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
                None => "-".to_string(),
            };
            tracing::info!(
                "  [Step {}] {}  [{:?}]  {}",
                i + 1,
                step.name,
                step.status,
                step_duration
            );
            if let Some(err) = &step.error {
                tracing::info!("           Error: {}", err);
            }
        }
    }

    Ok(())
}
