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

    // 获取日志项目目录
    pub fn log_project_dir(space_name: &str, project_name: &str) -> String {
        format!("{}/{}", HOST_LOG_DIR, log_project_dir_name(space_name, project_name))
    }
}

// 获取日志项目目录名称
fn log_project_dir_name(space_name: &str, project_name: &str) -> String {
    format!("[{space_name}]-{project_name}")
}
