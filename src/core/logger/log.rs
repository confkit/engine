//! Author: xiaoYown
//! Created: 2025-07-24
//! Description: Log implementation

use anyhow::Result;
use std::fs;

use crate::infra::db::task_db::{PageParams, TaskFilter};
use crate::infra::db::TaskDb;
use crate::shared::constants::HOST_LOG_DIR;

/// 列出任务日志（DB 分页查询）
pub fn list_task_logs(filter: &TaskFilter, page_params: &PageParams) -> Result<()> {
    let db = TaskDb::open()?;
    let result = db.list_tasks(filter, page_params)?;

    if result.tasks.is_empty() {
        tracing::info!("No logs found");
        return Ok(());
    }

    let total_pages = result.total.div_ceil(result.size);

    tracing::info!(
        "Task logs (page {}/{}, total {} tasks):",
        result.page,
        total_pages,
        result.total
    );

    for task in &result.tasks {
        let duration_str = match task.duration_ms {
            Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
            None => "-".to_string(),
        };
        tracing::info!(
            "  {}  {}/{}  {}  [{}]  {}  ({} steps)",
            task.task_id,
            task.space_name,
            task.project_name,
            task.started_at,
            task.status,
            duration_str,
            task.steps.len()
        );
    }

    Ok(())
}

/// 按过滤条件收集任务条目（交互模式用）
pub fn collect_task_entries_filtered(filter: &TaskFilter) -> Result<Vec<(String, String)>> {
    let db = TaskDb::open()?;
    db.collect_entries_filtered(filter)
}

/// 打印指定任务的日志内容
pub fn print_task_log(task_id: &str) -> Result<()> {
    let db = TaskDb::open()?;
    let record = match db.get_task(task_id)? {
        Some(r) => r,
        None => {
            tracing::warn!("Log not found for task '{}'", task_id);
            return Ok(());
        }
    };

    // 打印 metadata 摘要
    let meta = &record.metadata;
    let duration_str = match meta.duration_ms {
        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
        None => "-".to_string(),
    };
    tracing::info!(
        "Task: {} | Status: {} | Duration: {} | Steps: {}",
        meta.task_id,
        meta.status,
        duration_str,
        meta.steps.len()
    );
    tracing::info!("---");

    // 打印日志内容
    let log_path = format!("{}/{}", HOST_LOG_DIR, record.log_path);
    match fs::read_to_string(&log_path) {
        Ok(content) => {
            tracing::info!("{}", content);
        }
        Err(_) => {
            tracing::warn!("Log file not found at: {}", log_path);
        }
    }

    Ok(())
}

/// 打印指定任务的元数据信息
pub fn print_task_info(task_id: &str) -> Result<()> {
    let db = TaskDb::open()?;
    let record = match db.get_task(task_id)? {
        Some(r) => r,
        None => {
            tracing::warn!("Task not found: '{}'", task_id);
            return Ok(());
        }
    };

    let meta = &record.metadata;
    let duration_str = match meta.duration_ms {
        Some(ms) => format!("{:.1}s", ms as f64 / 1000.0),
        None => "-".to_string(),
    };

    tracing::info!("Task ID:      {}", meta.task_id);
    tracing::info!("Space:        {}", meta.space_name);
    tracing::info!("Project:      {}", meta.project_name);
    tracing::info!("Status:       {}", meta.status);
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
