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

    /// 获取日志 space 目录: volumes/logs/{space_name}
    pub fn log_space_dir(space_name: &str) -> String {
        format!("{}/{}", HOST_LOG_DIR, space_name)
    }

    /// 获取日志项目目录: volumes/logs/{space_name}/{project_name}
    pub fn log_project_dir(space_name: &str, project_name: &str) -> String {
        format!("{}/{}/{}", HOST_LOG_DIR, space_name, project_name)
    }

    /// 获取日志日期目录: volumes/logs/{space}/{project}/{date}
    pub fn log_date_dir(space_name: &str, project_name: &str, date: &str) -> String {
        format!("{}/{}", Self::log_project_dir(space_name, project_name), date)
    }

    /// 获取任务日志目录: volumes/logs/{space}/{project}/{date}/{ts-task_id}
    pub fn log_task_dir(
        space_name: &str,
        project_name: &str,
        date: &str,
        task_dir_name: &str,
    ) -> String {
        format!("{}/{}", Self::log_date_dir(space_name, project_name, date), task_dir_name)
    }
}
