use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 任务ID类型
pub type TaskId = String;

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: TaskId,
    pub project_name: String,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub current_step: Option<String>,
    pub total_steps: usize,
    pub completed_steps: usize,
}

/// 任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub duration: chrono::Duration,
    pub steps_results: Vec<StepResult>,
    pub artifacts: Vec<String>,
    pub error_message: Option<String>,
}

/// 步骤结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub name: String,
    pub status: TaskStatus,
    pub duration: chrono::Duration,
    pub output: String,
    pub error_output: Option<String>,
    pub exit_code: Option<i32>,
}

/// 任务上下文
#[derive(Debug, Clone)]
pub struct TaskContext {
    pub task_id: TaskId,
    pub project_name: String,
    pub workspace_dir: String,
    pub artifacts_dir: String,
    pub environment: HashMap<String, String>,
}

/// 任务句柄
pub struct TaskHandle {
    pub info: TaskInfo,
    pub cancel_token: tokio_util::sync::CancellationToken,
}
