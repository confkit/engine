use chrono::Local;

use crate::shared::constants::HOST_LOG_DIR;

pub struct PathFormatter;

impl PathFormatter {
    // 格式化任务路径
    pub fn get_task_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        format!("[{}]-{}-{}", space_name, project_name, task_id)
    }

    pub fn get_log_project_dir_name(space_name: &str, project_name: &str) -> String {
        format!("[{}]-{}", space_name, project_name)
    }

    pub fn format_log_project_path(space_name: &str, project_name: &str) -> String {
        format!("{}/{}", HOST_LOG_DIR, Self::get_log_project_dir_name(space_name, project_name))
    }

    pub fn get_project_log_path(space_name: &str, project_name: &str) -> String {
        format!("{}/[{}]-{}/", HOST_LOG_DIR, space_name, project_name)
    }

    pub fn get_task_log_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        let timestamp = Local::now().format("%Y.%m.%d-%H:%M:%S%.3f");

        let project_log_path = Self::get_project_log_path(space_name, project_name);

        let host_log_path = format!("{}/[{}]{}.log", project_log_path, timestamp, &task_id);

        host_log_path
    }
}
