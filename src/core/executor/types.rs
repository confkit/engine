//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Executor types

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
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Running,
    Success,
    Failed,
    Skipped,
}

/// 任务元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    pub task_id: String,
    pub space_name: String,
    pub project_name: String,
    pub status: TaskStatus,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub steps: Vec<StepMetadata>,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
}

/// 步骤元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepMetadata {
    pub name: String,
    pub status: StepStatus,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
}

impl From<&StepResult> for StepMetadata {
    fn from(result: &StepResult) -> Self {
        Self {
            name: result.name.clone(),
            status: result.status.clone(),
            started_at: Some(result.started_at.to_rfc3339()),
            finished_at: result.finished_at.map(|t| t.to_rfc3339()),
            duration_ms: result.duration_ms,
            exit_code: result.exit_code,
            error: result.error.clone(),
        }
    }
}
