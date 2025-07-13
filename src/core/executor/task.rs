use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 任务ID生成器
pub struct Task {
    pub id: String,
    pub space_name: String,
    pub project_name: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(space_name: &str, project_name: &str) -> Self {
        Self {
            id: Self::generate_task_id(),
            space_name: space_name.to_string(),
            project_name: project_name.to_string(),
            started_at: Utc::now(),
            finished_at: None,
        }
    }

    // 生成任务ID
    fn generate_task_id() -> String {
        let uuid = Uuid::new_v4();
        let short_uuid = uuid.to_string()[..11].to_string();
        short_uuid
    }
}

impl Task {
    // 格式化任务路径
    pub fn format_task_path(space_name: &str, project_name: &str, task_id: &str) -> String {
        format!("<{}>-{}-{}", space_name, project_name, task_id)
    }
}
