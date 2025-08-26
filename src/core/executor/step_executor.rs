//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Step executor implementation

use anyhow::Result;
use chrono::Utc;
use std::time::Instant;

use super::command_executor::CommandExecutor;
use super::context::ExecutionContext;
use super::types::{StepResult, StepStatus};
use crate::core::executor::task::Task;
use crate::types::config::ConfKitStepConfig;

/// 步骤执行器
pub struct StepExecutor {
    context: ExecutionContext,
    task: Task,
}

impl StepExecutor {
    pub fn new(context: ExecutionContext, task: Task) -> Self {
        Self { context, task }
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

        let working_dir = match &step.working_dir {
            Some(working_dir) => self.context.resolve_working_dir(working_dir),
            None => {
                if step.container.is_some() {
                    self.context.container_workspace_dir.clone()
                } else {
                    self.context.host_workspace_dir.clone()
                }
            }
        };

        // 记录步骤详情
        self.log_step_details(step, &working_dir).await?;

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
                &working_dir,
                &step.commands,
                &self.task,
            )
            .await?
        } else {
            CommandExecutor::execute_locally(
                &self.context,
                &working_dir,
                &step.commands,
                &self.task,
            )
            .await?
        };

        let duration = start_time.elapsed();
        result.duration_ms = Some(duration.as_millis() as u64);
        result.finished_at = Some(Utc::now());

        // 设置结果
        result.exit_code = Some(execution_result);
        result.output = String::new();

        // 检查是否超时
        if let Some(timeout) = &step.timeout {
            if duration.as_secs() > *timeout {
                result.status = StepStatus::Failed;
                result.exit_code = Some(1);
                result.error = Some(format!("Step timeout: {timeout} seconds"));
                self.log_step_result(&result, step_number, total_steps)?;
                return Ok(result);
            }
        }

        // 根据执行结果设置状态
        if execution_result == 0 {
            result.status = StepStatus::Success;
        } else {
            result.status = StepStatus::Failed;
            result.error = Some(format!("Command failed with exit code: {execution_result}"));
        }

        self.log_step_result(&result, step_number, total_steps)?;

        Ok(result)
    }

    /// 记录步骤详情
    async fn log_step_details(&self, step: &ConfKitStepConfig, working_dir: &str) -> Result<()> {
        self.task.info("Step Details:")?;
        self.task
            .info(&format!(" - Container: {}", step.container.as_deref().unwrap_or("Host")))?;
        self.task.info(&format!(" - Working Directory: {working_dir}"))?;
        self.task.info(&format!(" - Command Count: {}", step.commands.len()))?;

        if let Some(timeout) = &step.timeout {
            self.task.info(&format!(" - Timeout: {timeout}"))?;
        }
        Ok(())
    }

    /// 记录步骤结果
    fn log_step_result(
        &self,
        result: &StepResult,
        step_number: usize,
        total_steps: usize,
    ) -> Result<()> {
        match result.status {
            StepStatus::Success => self.task.info(&format!(
                "[{}/{}] ✓ Step '{}' executed successfully (Duration: {:.1}s)",
                step_number,
                total_steps,
                result.name,
                result.duration_ms.unwrap_or(0) as f64 / 1000.0
            ))?,
            StepStatus::Failed => {
                self.task.error(&format!(
                    "[{}/{}] ✗ Step '{}' executed failed (Duration: {:.1}s)",
                    step_number,
                    total_steps,
                    result.name,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                ))?;

                if let Some(error) = &result.error {
                    self.task.error(&format!("Error Message: {error}"))?;
                }
            }
            StepStatus::Skipped => {
                self.task.info(&format!(
                    "[{}/{}] ○ Step '{}' skipped",
                    step_number, total_steps, result.name
                ))?;
            }
            StepStatus::Running => {
                // 这种情况通常不应该出现在结果记录中，但为了完整性处理
                self.task.info(&format!(
                    "[{}/{}] ▶ Step '{}' is running...",
                    step_number, total_steps, result.name
                ))?;
            }
        }
        Ok(())
    }
}
