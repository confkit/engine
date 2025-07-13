use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 步骤执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub name: String,
    pub status: StepStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
    pub exit_code: Option<i32>,
    pub output: String,
    pub error: Option<String>,
}

/// 步骤状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepStatus {
    Running,
    Success,
    Failed,
    Skipped,
}

/// 执行摘要
#[derive(Debug, Clone)]
pub struct ExecutionSummary {
    pub total_steps: usize,
    pub successful_steps: usize,
    pub failed_steps: usize,
    pub skipped_steps: usize,
    pub total_duration_ms: u64,
    pub task_id: String,
    pub project_name: String,
}
