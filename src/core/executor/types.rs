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
pub enum StepStatus {
    Running,
    Success,
    Failed,
    Skipped,
}
