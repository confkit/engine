//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Runner implementation

use std::collections::HashMap;

use anyhow::Result;
use tracing;

use super::context::ExecutionContext;
use super::step_executor::StepExecutor;
use super::task::Task;
use super::types::{StepResult, StepStatus};
use crate::core::clean::volumes::VolumesCleaner;
use crate::formatter::log::LogFormatter;
use crate::formatter::path::PathFormatter;
use crate::infra::config::ConfKitConfigLoader;
use crate::types::config::ConfKitProjectConfig;
use crate::utils::fs::make_dir_with_permissions;

/// 主执行器
pub struct Runner {
    context: ExecutionContext,
    task: Task,
    space_name: String,
    project_name: String,
    project_config: ConfKitProjectConfig,
}

impl Runner {
    pub async fn new(
        space_name: &str,
        project_name: &str,
        environment_from_args: HashMap<String, String>,
    ) -> Result<Self> {
        // 获取项目配置
        let project_config =
            ConfKitConfigLoader::get_project_config(space_name, project_name).await?;

        let project_config = match project_config {
            Some(config) => config,
            None => {
                tracing::error!("Project '{}' not found in space '{}'", project_name, space_name);
                return Err(anyhow::anyhow!("Project not found"));
            }
        };

        let host_log_dir = PathFormatter::get_project_log_path(&space_name, &project_name);

        // 创建任务
        let task = Task::new(&host_log_dir);

        let task_id = task.id.clone();

        // 创建执行上下文
        let context = ExecutionContext::new(
            task_id.clone(),
            space_name.to_string(),
            project_name.to_string(),
            &project_config,
            environment_from_args,
        )
        .await?;

        Ok(Self {
            context,
            task,
            space_name: space_name.to_string(),
            project_name: project_name.to_string(),
            project_config,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        // TODO: 检查容器是否存在

        // 创建工作目录
        make_dir_with_permissions(&self.context.host_workspace_dir, 0o777)?;

        // 打印任务信息
        self.print_task_info()?;

        // 创建步骤执行器
        let executor = StepExecutor::new(self.context.clone(), self.task.clone());

        // 执行所有步骤
        let results = self.execute_all_steps(&self.project_config, &executor).await?;

        self.task.info("Cleaning workspace")?;

        if self.context.clean_workspace {
            VolumesCleaner::clean_workspace(&self.space_name, &self.project_name, &self.task.id)
                .await?;
        }

        self.task.finish();

        // 输出执行摘要
        self.print_execution_summary(&results)?;

        Ok(())
    }

    /// 执行所有步骤
    async fn execute_all_steps(
        &self,
        project_config: &ConfKitProjectConfig,
        executor: &StepExecutor,
    ) -> Result<Vec<StepResult>> {
        let mut results = Vec::new();
        let total_steps = project_config.steps.len();

        self.task.info(&LogFormatter::header("Execution Steps"))?;

        self.task.info(&format!(
            "Start to execute project: {} (total {} steps)",
            self.project_name, total_steps
        ))?;

        for (index, step) in project_config.steps.iter().enumerate() {
            let step_number = index + 1;
            let step_continue_on_error = step.continue_on_error.unwrap_or(false);

            self.task.info(&format!("[{}/{}] {}", step_number, total_steps, step.name))?;

            let result = executor.execute_step(step, step_number, total_steps).await?;

            results.push(result.clone());

            // 检查是否需要继续执行
            if result.status == StepStatus::Failed && !step_continue_on_error {
                // ConfKitStepConfig 没有 continue_on_error 字段，停止执行
                self.task.error(&format!("Step '{}' failed, stop execution", step.name))?;
                break;
            }
        }

        Ok(results)
    }

    /// 打印任务信息
    fn print_task_info(&self) -> Result<()> {
        self.task.info(&LogFormatter::header("Task Info"))?;
        // 打印任务信息
        self.task.info(&format!("Task: {}", self.context.task_id))?;
        self.task.info(&format!("Space: {}", self.context.space_name))?;
        self.task.info(&format!("Project: {}", self.context.project_name))?;
        self.task.info(&format!("Host workspace dir: {}", self.context.host_workspace_dir))?;
        self.task
            .info(&format!("Container workspace dir: {}", self.context.container_workspace_dir))?;

        // 打印 Git 信息
        self.task.info(&LogFormatter::header("Git Info"))?;
        self.task
            .info(&format!("Repository: {}", self.context.git_info.as_ref().unwrap().repo_url))?;
        self.task.info(&format!("Branch: {}", self.context.git_info.as_ref().unwrap().branch))?;
        self.task
            .info(&format!("Commit: {}", self.context.git_info.as_ref().unwrap().commit_hash))?;

        // 环境变量
        self.task.info(&LogFormatter::header("Environment"))?;
        for (key, value) in &self.context.environment {
            self.task.info(&format!("{key}: {value}"))?;
        }

        Ok(())
    }

    /// 打印执行摘要
    fn print_execution_summary(&self, results: &[StepResult]) -> Result<()> {
        self.task.info(&LogFormatter::header("Execution Summary"))?;
        self.task.info(&format!("Total steps: {}", results.len()))?;

        let successful = results.iter().filter(|r| r.status == StepStatus::Success).count();
        let failed = results.iter().filter(|r| r.status == StepStatus::Failed).count();
        let skipped = results.iter().filter(|r| r.status == StepStatus::Skipped).count();

        self.task.info(&format!("Success: {successful}, Failed: {failed}, Skipped: {skipped}"))?;

        self.task.info(&format!("Started at: {}", self.task.started_at))?;
        self.task.info(&format!(
            "Finished at: {}",
            self.task.finished_at.as_ref().map(|t| t.to_string()).as_deref().unwrap_or("-")
        ))?;

        let total_duration: u64 = results.iter().filter_map(|r| r.duration_ms).sum();

        self.task.info(&format!("Total duration: {:.1}s", total_duration as f64 / 1000.0))?;

        Ok(())
    }
}
