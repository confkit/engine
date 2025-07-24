//! Author: xiaoYown
//! Created: 2025-07-22
//! Description: Log formatter

use chrono::Local;

use crate::shared::constants::HOST_LOG_DIR;

pub struct LogFormatter;

impl LogFormatter {
    pub fn header(title: &str) -> String {
        // 自动适应 80 长度
        let title_len = title.len();
        let padding = (78 - title_len) / 2;
        let padding_str = "=".repeat(padding);
        return format!("\n{}[ {} ]{}", padding_str, title, padding_str);
    }
}

impl LogFormatter {
    pub fn get_project_log_path(space_name: &str, project_name: &str) -> String {
        format!("{}/<{}>-{}/", HOST_LOG_DIR, space_name, project_name)
    }

    pub fn get_task_log_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        let timestamp = Local::now().format("%Y.%m.%d-%H:%M:%S%.3f");

        let project_log_path = Self::get_project_log_path(space_name, project_name);

        let host_log_path = format!("{}/[{}]{}.log", project_log_path, timestamp, &task_id);

        host_log_path
    }
}
