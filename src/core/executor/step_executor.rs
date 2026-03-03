//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Step executor implementation

use anyhow::Result;
use chrono::Utc;
use std::time::Instant;

use super::command_executor::CommandExecutor;
use super::context::ExecutionContext;
use super::types::{StepResult, StepStatus};
use crate::core::condition::evaluator::ConditionEvaluator;
use crate::infra::logger::TaskLogger;
use crate::types::config::ConfKitStepConfig;

/// 步骤执行器
pub struct StepExecutor {
    context: ExecutionContext,
    task_logger: TaskLogger,
}

impl StepExecutor {
    pub fn new(context: ExecutionContext, task_logger: TaskLogger) -> Self {
        Self { context, task_logger }
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

        // 检查步骤条件
        if let Some(condition) = &step.condition {
            let evaluator = ConditionEvaluator::new(self.context.environment.clone());
            match evaluator.evaluate_string(condition) {
                Ok(true) => {
                    self.task_logger.info(&format!(
                        "[Step {}/{}] Condition satisfied: {}",
                        step_number, total_steps, condition
                    ))?;
                }
                Ok(false) => {
                    result.status = StepStatus::Skipped;
                    result.exit_code = Some(0);
                    result.output = String::new();
                    result.error = None;
                    result.finished_at = Some(Utc::now());
                    result.duration_ms = Some(start_time.elapsed().as_millis() as u64);

                    // 跳过步骤并记录结果
                    self.log_step_result(
                        &result,
                        step_number,
                        total_steps,
                        Some(&format!("condition {condition}")),
                    )?;
                    return Ok(result);
                }
                Err(e) => {
                    self.task_logger.warn(&format!(
                        "[Step {}/{}] Failed to evaluate condition '{}': {}. Executing step anyway.",
                        step_number, total_steps, condition, e
                    ))?;
                }
            }
        }

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
        self.log_step_details(step_number, total_steps, step, &working_dir).await?;

        // commands 长度为 0 时，直接跳过
        if step.commands.is_empty() {
            result.status = StepStatus::Skipped;
            result.exit_code = Some(0);
            result.output = String::new();
            result.error = None;

            return Ok(result);
        }
        // 执行命令，在 step 级别应用超时
        let execution_future = async {
            if let Some(container) = &step.container {
                CommandExecutor::execute_in_container(
                    &self.context,
                    container,
                    &working_dir,
                    &step.commands,
                    &self.task_logger,
                )
                .await
            } else {
                CommandExecutor::execute_locally(
                    &self.context,
                    &working_dir,
                    &step.commands,
                    &self.task_logger,
                )
                .await
            }
        };

        let execution_result = if let Some(timeout_secs) = step.timeout {
            let timeout_duration = std::time::Duration::from_secs(timeout_secs);
            match tokio::time::timeout(timeout_duration, execution_future).await {
                Ok(result) => result?,
                Err(_) => {
                    self.task_logger.error(&format!(
                        "Step '{}' timed out after {} seconds",
                        step.name, timeout_secs
                    ))?;
                    -1 // 超时返回 -1
                }
            }
        } else {
            execution_future.await?
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
                self.log_step_result(
                    &result,
                    step_number,
                    total_steps,
                    Some(&format!("Step timeout {timeout} seconds")),
                )?;
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

        self.log_step_result(&result, step_number, total_steps, None)?;

        Ok(result)
    }

    /// 记录步骤详情
    async fn log_step_details(
        &self,
        step_number: usize,
        total_steps: usize,
        step: &ConfKitStepConfig,
        working_dir: &str,
    ) -> Result<()> {
        self.task_logger.info(&format!("[Step {}/{}] Details:", step_number, total_steps))?;
        self.task_logger
            .info(&format!("  - Container: {}", step.container.as_deref().unwrap_or("Host")))?;
        self.task_logger.info(&format!("  - Working Directory: {working_dir}"))?;
        self.task_logger.info(&format!("  - Command Count: {}", step.commands.len()))?;

        if let Some(condition) = &step.condition {
            self.task_logger.info(&format!("  - Condition: {}", condition))?;
        }

        if let Some(timeout) = &step.timeout {
            self.task_logger.info(&format!("  - Timeout: {timeout}"))?;
        }
        Ok(())
    }

    /// 记录步骤结果
    fn log_step_result(
        &self,
        result: &StepResult,
        step_number: usize,
        total_steps: usize,
        detail: Option<&str>,
    ) -> Result<()> {
        match result.status {
            StepStatus::Success => self.task_logger.info(&format!(
                "[Step {}/{}] Completed ({:.1}s)",
                step_number,
                total_steps,
                result.duration_ms.unwrap_or(0) as f64 / 1000.0
            ))?,
            StepStatus::Failed => {
                self.task_logger.error(&format!(
                    "[Step {}/{}] Failed ({:.1}s)",
                    step_number,
                    total_steps,
                    result.duration_ms.unwrap_or(0) as f64 / 1000.0
                ))?;

                if let Some(error) = &result.error {
                    self.task_logger
                        .error(&format!("[Step {}/{}] Error: {error}", step_number, total_steps))?;
                }
            }
            StepStatus::Skipped => {
                self.task_logger.info(&format!(
                    "[Step {}/{}] Skipped: {}",
                    step_number,
                    total_steps,
                    detail.unwrap_or("No reason provided")
                ))?;
            }
            StepStatus::Running => {
                // 这种情况通常不应该出现在结果记录中，但为了完整性处理
                self.task_logger
                    .info(&format!("[Step {}/{}] Running...", step_number, total_steps))?;
            }
        }
        Ok(())
    }
}
