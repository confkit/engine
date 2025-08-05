//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Step executor implementation

use anyhow::Result;
use chrono::Utc;
use std::time::Instant;

use super::command_executor::CommandExecutor;
use super::context::ExecutionContext;
use super::types::{StepResult, StepStatus};
use crate::infra::log::Log;
use crate::types::config::ConfKitStepConfig;

/// 步骤执行器
pub struct StepExecutor {
    context: ExecutionContext,
    logger: Log,
}

impl StepExecutor {
    pub fn new(context: ExecutionContext, logger: Log) -> Self {
        Self { context, logger }
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
            )
            .await?
        } else {
            CommandExecutor::execute_locally(&self.context, &working_dir, &step.commands).await?
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

        self.log_step_result(&result, step_number, total_steps).await?;

        Ok(result)
    }

    /// 记录步骤详情
    async fn log_step_details(&self, step: &ConfKitStepConfig, working_dir: &str) -> Result<()> {
        self.logger.info("Step Details:")?;
        self.logger
            .info(&format!(" - Container: {}", step.container.as_deref().unwrap_or("Host")))?;
        self.logger.info(&format!(" - Working Directory: {working_dir}"))?;
        self.logger.info(&format!(" - Command Count: {}", step.commands.len()))?;

        if let Some(timeout) = &step.timeout {
            // TODO: 超时后，需要记录日志，并返回错误码
            self.logger.info(&format!(" - Timeout: {timeout}"))?;

            return Ok(());
        }
        Ok(())
    }

    /// 记录步骤结果
    async fn log_step_result(
        &self,
        result: &StepResult,
        step_number: usize,
        total_steps: usize,
    ) -> Result<()> {
        match result.status {
            StepStatus::Success => {
                self.logger.info(&format!(
                    "[{}/{}] ✓ Step '{}' executed successfully (Duration: {:.1}s)",
                    step_number,
                    total_steps,
                    result.name,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                ))?;

                Ok(())
            }
            StepStatus::Failed => {
                self.logger.error(&format!(
                    "[{}/{}] ✗ Step '{}' executed failed (Duration: {:.1}s)",
                    step_number,
                    total_steps,
                    result.name,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                ))?;

                if let Some(error) = &result.error {
                    self.logger.error(&format!("Error Message: {error}"))?;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
