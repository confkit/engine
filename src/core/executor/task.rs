//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Task implementation

use chrono::{DateTime, Local};
use uuid::Uuid;

/// 任务ID生成器
pub struct Task {
    pub id: String,
    pub started_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new() -> Self {
        Self { id: Self::generate_task_id(), started_at: Local::now(), finished_at: None }
    }

    // 生成任务ID
    fn generate_task_id() -> String {
        let uuid = Uuid::new_v4();
        let short_uuid = uuid.to_string()[..11].to_string();
        short_uuid
    }

    pub fn finish(&mut self) {
        self.finished_at = Some(Local::now());
    }
}

impl Task {
    // 格式化任务路径
    pub fn format_task_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        format!("<{}>-{}-{}", space_name, project_name, task_id)
    }
}
