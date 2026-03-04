//! Author: xiaoYown
//! Created: 2025-08-05
//! Description: Path formatter

use crate::shared::constants::HOST_LOG_DIR;

pub struct PathFormatter;

impl PathFormatter {
    // 格式化任务路径
    pub fn get_task_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        format!("--{space_name}--{project_name}-{task_id}")
    }

    /// 获取日志日期目录: volumes/logs/{date}
    pub fn log_date_dir(date: &str) -> String {
        format!("{}/{}", HOST_LOG_DIR, date)
    }

    /// 获取日志文件路径: volumes/logs/{date}/{task_id}.log
    pub fn log_file_path(date: &str, task_id: &str) -> String {
        format!("{}/{}.log", Self::log_date_dir(date), task_id)
    }

    /// 获取元数据文件路径: volumes/logs/{date}/{task_id}.meta.json
    pub fn log_meta_path(date: &str, task_id: &str) -> String {
        format!("{}/{}.meta.json", Self::log_date_dir(date), task_id)
    }

    /// 获取存入 DB 的相对路径: {date}/{task_id}.log
    pub fn log_relative_path(date: &str, task_id: &str) -> String {
        format!("{}/{}.log", date, task_id)
    }
}
