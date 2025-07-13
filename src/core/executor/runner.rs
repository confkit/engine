use anyhow::Result;
use tracing;

use super::context::ExecutionContext;
use super::step_executor::StepExecutor;
use super::task::Task;
use super::types::{StepResult, StepStatus};
use crate::core::clean::volumes::VolumesCleaner;
use crate::infra::config::ConfKitConfigLoader;
use crate::infra::log::Log;
use crate::types::config::ConfKitProjectConfig;
use crate::utils::fs::make_dir_with_permissions;

/// 主执行器
pub struct Runner {
    space_name: String,
    project_name: String,
}

impl Runner {
    pub fn new(space_name: &str, project_name: &str) -> Self {
        Self { space_name: space_name.to_string(), project_name: project_name.to_string() }
    }

    pub async fn start(&self) -> Result<()> {
        // TODO: 检查容器是否存在

        // 获取项目配置
        let project_config = ConfKitConfigLoader::get_project_config(
            self.space_name.as_str(),
            self.project_name.as_str(),
        )
        .await?;

        let project_config = match project_config {
            Some(config) => config,
            None => {
                tracing::error!(
                    "Project '{}' not found in space '{}'",
                    self.project_name,
                    self.space_name
                );
                return Err(anyhow::anyhow!("Project not found"));
            }
        };

        // 创建任务
        let task = Task::new(&self.space_name, &self.project_name);

        let task_id = task.id.clone();

        // 创建执行上下文
        let context = ExecutionContext::new(
            task_id.clone(),
            self.space_name.clone(),
            self.project_name.clone(),
            &project_config,
        )
        .await?;

        let logger = Log::new(&context.host_log_dir);

        // 创建工作目录
        make_dir_with_permissions(&context.host_workspace_dir, 0o777)?;

        // 创建产物目录
        make_dir_with_permissions(&context.host_artifacts_dir, 0o777)?;

        // 创建步骤执行器
        let executor = StepExecutor::new(context, logger);

        // 执行所有步骤
        let results = self.execute_all_steps(&project_config, &executor).await?;

        VolumesCleaner::clean(&self.space_name, &self.project_name, &task_id).await?;

        // 输出执行摘要
        self.print_execution_summary(&results);

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

        tracing::info!(
            "Start to execute project: {} (total {} steps)",
            self.project_name,
            total_steps
        );

        for (index, step) in project_config.steps.iter().enumerate() {
            let step_number = index + 1;

            tracing::info!("[{}/{}] {}", step_number, total_steps, step.name);

            let result = executor.execute_step(step, step_number, total_steps).await?;

            results.push(result.clone());

            // 检查是否需要继续执行
            if result.status == StepStatus::Failed {
                // ConfKitStepConfig 没有 continue_on_error 字段，暂时停止执行
                tracing::error!("Step '{}' failed, stop execution", step.name);
                break;
            }
        }

        Ok(results)
    }

    /// 打印执行摘要
    fn print_execution_summary(&self, results: &[StepResult]) {
        tracing::info!("Execution summary:");
        tracing::info!("Total steps: {}", results.len());

        let successful = results.iter().filter(|r| r.status == StepStatus::Success).count();
        let failed = results.iter().filter(|r| r.status == StepStatus::Failed).count();
        let skipped = results.iter().filter(|r| r.status == StepStatus::Skipped).count();

        tracing::info!("Success: {}, Failed: {}, Skipped: {}", successful, failed, skipped);

        let total_duration: u64 = results.iter().filter_map(|r| r.duration_ms).sum();

        tracing::info!("Total duration: {:.1}s", total_duration as f64 / 1000.0);
    }
}
