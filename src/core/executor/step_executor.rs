use anyhow::Result;
use chrono::Utc;
use std::time::Instant;
use tracing;

use super::command_executor::CommandExecutor;
use super::context::ExecutionContext;
use super::types::{StepResult, StepStatus};
use crate::types::config::ConfKitStepConfig;

/// 步骤执行器
pub struct StepExecutor {
    context: ExecutionContext,
}

impl StepExecutor {
    pub fn new(context: ExecutionContext) -> Self {
        Self { context }
    }

    /// 执行单个步骤
    pub async fn execute_step(
        &self,
        step: &ConfKitStepConfig,
        step_number: usize,
        total_steps: usize,
    ) -> Result<StepResult> {
        let mut result = StepResult {
            name: step.name.clone(),
            status: StepStatus::Running,
            started_at: Utc::now(),
            finished_at: None,
            duration_ms: None,
            exit_code: None,
            output: String::new(),
            error: None,
        };

        let start_time = Instant::now();

        // 记录步骤详情
        self.log_step_details(step, step_number, total_steps).await;

        // commands 长度为 0 时，直接跳过
        if step.commands.is_empty() {
            result.status = StepStatus::Skipped;
            result.exit_code = Some(0);
            result.output = String::new();
            result.error = None;

            return Ok(result);
        }
        // 执行命令
        let execution_result = if let Some(container) = &step.container {
            CommandExecutor::execute_in_container(
                &self.context,
                container,
                step.working_dir.as_deref(),
                &step.commands,
            )
            .await?
        } else {
            CommandExecutor::execute_locally(
                &self.context,
                step.working_dir.as_deref(),
                &step.commands,
            )
            .await?
        };

        let duration = start_time.elapsed();
        result.duration_ms = Some(duration.as_millis() as u64);
        result.finished_at = Some(Utc::now());

        // 设置结果
        if execution_result == 0 {
            result.status = StepStatus::Success;
            result.exit_code = Some(execution_result);
            result.output = String::new();
        } else {
            result.status = StepStatus::Failed;
            result.exit_code = Some(execution_result);
            result.output = String::new();
            result.error = Some(String::new());
        }

        Ok(result)
    }

    /// 记录步骤详情
    async fn log_step_details(
        &self,
        step: &ConfKitStepConfig,
        step_number: usize,
        total_steps: usize,
    ) {
        tracing::info!("Step Details:");
        tracing::info!("  Container: {}", step.container.as_deref().unwrap_or("Host"));
        tracing::info!("  Working Directory: {}", step.working_dir.as_deref().unwrap_or("Default"));
        tracing::info!("  Command Count: {}", step.commands.len());
        if let Some(timeout) = &step.timeout {
            tracing::info!("  Timeout: {}", timeout);
        }
        // ConfKitStepConfig 没有 continue_on_error 字段，暂时跳过
    }

    /// 记录步骤结果
    pub async fn log_step_result(
        &self,
        result: &StepResult,
        step_number: usize,
        total_steps: usize,
    ) {
        match result.status {
            StepStatus::Success => {
                tracing::info!(
                    "[{}/{}] ✓ Step '{}' executed successfully (Duration: {:.1}s)",
                    step_number,
                    total_steps,
                    result.name,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                );
            }
            StepStatus::Failed => {
                tracing::error!(
                    "[{}/{}] ✗ Step '{}' executed failed (Duration: {:.1}s)",
                    step_number,
                    total_steps,
                    result.name,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                );
                if let Some(error) = &result.error {
                    tracing::error!("Error Message: {}", error);
                }
            }
            _ => {}
        }
    }
}
