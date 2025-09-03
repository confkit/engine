//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Task implementation

use anyhow::Result;
use chrono::{DateTime, Local};
use uuid::Uuid;

use super::context::ExecutionContext;
use super::step_executor::StepExecutor;
use super::types::{StepResult, StepStatus};
use crate::core::clean::volumes::VolumesCleaner;
use crate::formatter::log::LogFormatter;
use crate::infra::event_hub::{Event, EventHub, LogLevel};
use crate::infra::logger::TaskLogger;
use crate::types::config::ConfKitProjectConfig;
use crate::utils::fs::make_dir_with_permissions;

/// 任务执行器
pub struct Task {
    pub id: String,
    pub started_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
    pub log_path: String,

    // 新增字段用于业务逻辑执行
    pub context: Option<ExecutionContext>,
    pub project_config: Option<ConfKitProjectConfig>,
    pub step_results: Vec<StepResult>,
}

impl Task {
    pub fn new(log_dir: &str) -> Self {
        let task_id = Self::generate_task_id();
        let timestamp = Local::now().format("%Y.%m.%d-%H:%M:%S%.3f");
        let log_path = format!("{log_dir}/[{timestamp}]-{task_id}.log");

        Self {
            id: task_id,
            started_at: Local::now(),
            finished_at: None,
            log_path,
            context: None,
            project_config: None,
            step_results: Vec::new(),
        }
    }

    /// 创建带有执行上下文和项目配置的任务
    pub fn with_context(
        log_dir: &str,
        context: ExecutionContext,
        project_config: ConfKitProjectConfig,
    ) -> Self {
        let mut task = Self::new(log_dir);
        task.context = Some(context);
        task.project_config = Some(project_config);
        task
    }

    /// 记录指定级别的日志
    pub fn log_with_level(&self, message: &str, level: LogLevel) -> Result<(), anyhow::Error> {
        EventHub::global().publish(
            Event::new_log(level, message.to_string(), "task".to_string())
                .with_metadata("log_path".to_string(), self.log_path.clone()),
        )?;

        Ok(())
    }

    /// 记录 Info 级别日志的便捷方法
    pub fn info(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Info)
    }

    /// 记录 Error 级别日志的便捷方法
    pub fn error(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Error)
    }

    // /// 记录 Warn 级别日志的便捷方法
    // pub fn warn(&self, message: &str) -> Result<(), anyhow::Error> {
    //     self.log_with_level(message, LogLevel::Warn)
    // }

    // /// 记录 Debug 级别日志的便捷方法
    // pub fn debug(&self, message: &str) -> Result<(), anyhow::Error> {
    //     self.log_with_level(message, LogLevel::Debug)
    // }

    // 生成任务ID
    fn generate_task_id() -> String {
        let uuid = Uuid::new_v4();

        uuid.to_string()[..11].to_string()
    }

    pub fn finish(&mut self) {
        self.finished_at = Some(Local::now());
    }

    /// 创建轻量级的任务日志记录器
    pub fn logger(&self) -> TaskLogger {
        TaskLogger::new(self.log_path.clone())
    }

    // 任务生命周期管理方法 (稍后实现)

    /// 准备任务执行环境
    pub async fn prepare(&self) -> Result<()> {
        let context = match &self.context {
            Some(ctx) => ctx,
            None => {
                self.error("Task context not available for prepare")?;
                return Err(anyhow::anyhow!("Task context not available"));
            }
        };

        // 创建工作目录
        make_dir_with_permissions(&context.host_workspace_dir, 0o777)?;

        // 打印任务信息
        self.print_info()?;

        Ok(())
    }

    /// 执行所有步骤
    pub async fn execute_steps(&mut self) -> Result<()> {
        let (context, project_config) = match (&self.context, &self.project_config) {
            (Some(ctx), Some(cfg)) => (ctx, cfg),
            _ => {
                self.error("Task context or project config not available for execute_steps")?;
                return Err(anyhow::anyhow!("Task context or project config not available"));
            }
        };

        let total_steps = project_config.steps.len();

        self.info(&LogFormatter::header("Execution Steps"))?;

        self.info(&format!(
            "Start to execute project: {} (total {} steps)",
            context.project_name, total_steps
        ))?;

        // 创建步骤执行器
        let executor = StepExecutor::new(context.clone(), self.logger());

        for (index, step) in project_config.steps.iter().enumerate() {
            let step_number = index + 1;
            let step_continue_on_error = step.continue_on_error.unwrap_or(false);

            self.info(&format!("[{}/{}] {}", step_number, total_steps, step.name))?;

            let result = executor.execute_step(step, step_number, total_steps).await?;

            self.step_results.push(result.clone());

            // 检查是否需要继续执行
            if result.status == StepStatus::Failed && !step_continue_on_error {
                self.error(&format!("Step '{}' failed, stop execution", step.name))?;
                break;
            }
        }

        Ok(())
    }

    /// 清理任务资源
    pub async fn cleanup(&self) -> Result<()> {
        let context = match &self.context {
            Some(ctx) => ctx,
            None => {
                self.error("Task context not available for cleanup")?;
                return Ok(());
            }
        };

        self.info("Cleaning workspace")?;

        // 清理工作空间
        if context.clean_workspace {
            VolumesCleaner::clean_dir(&context.host_workspace_dir, true).await?;
        }

        Ok(())
    }

    // 信息输出方法 (稍后实现)

    /// 打印任务信息
    pub fn print_info(&self) -> Result<()> {
        let context = match &self.context {
            Some(ctx) => ctx,
            None => {
                self.error("Task context not available for print_info")?;
                return Ok(());
            }
        };

        self.info(&LogFormatter::header("Task Info"))?;
        // 打印任务信息
        self.info(&format!("Space: {}", context.space_name))?;
        self.info(&format!("Project: {}", context.project_name))?;
        self.info(&format!("Task: {}", context.task_id))?;
        self.info(&format!("Host workspace dir: {}", context.host_workspace_dir))?;
        self.info(&format!("Container workspace dir: {}", context.container_workspace_dir))?;

        // 打印 Git 信息
        self.info(&LogFormatter::header("Git Info"))?;
        if let Some(git_info) = &context.git_info {
            self.info(&format!("Repository: {}", git_info.repo_url))?;
            self.info(&format!("Branch: {}", git_info.branch))?;
            self.info(&format!("Commit: {}", git_info.commit_hash))?;
        } else {
            self.info("Git information not available")?;
        }

        // 环境变量
        self.info(&LogFormatter::header("Environment"))?;
        for (key, value) in &context.environment {
            self.info(&format!("{key}: {value}"))?;
        }

        Ok(())
    }

    /// 打印执行摘要
    pub fn print_summary(&self) -> Result<()> {
        let context = match &self.context {
            Some(ctx) => ctx,
            None => {
                self.error("Task context not available for print_summary")?;
                return Ok(());
            }
        };

        self.info(&LogFormatter::header("Execution Summary"))?;
        self.info(&format!("Space: {}", context.space_name))?;
        self.info(&format!("Project: {}", context.project_name))?;
        self.info(&format!("Task: {}", self.id))?;
        self.info(&format!("Total steps: {}", self.step_results.len()))?;

        let (successful, failed, skipped) = self.get_execution_stats();
        self.info(&format!("Success: {successful}, Failed: {failed}, Skipped: {skipped}"))?;

        self.info(&format!("Started at: {}", self.started_at))?;
        self.info(&format!(
            "Finished at: {}",
            self.finished_at.as_ref().map(|t| t.to_string()).as_deref().unwrap_or("-")
        ))?;

        let total_duration = self.get_total_duration();
        self.info(&format!("Total duration: {:.1}s", total_duration as f64 / 1000.0))?;

        Ok(())
    }

    /// 获取执行统计信息 (成功, 失败, 跳过)
    pub fn get_execution_stats(&self) -> (usize, usize, usize) {
        let successful =
            self.step_results.iter().filter(|r| r.status == StepStatus::Success).count();
        let failed = self.step_results.iter().filter(|r| r.status == StepStatus::Failed).count();
        let skipped = self.step_results.iter().filter(|r| r.status == StepStatus::Skipped).count();
        (successful, failed, skipped)
    }

    /// 获取总执行时长（毫秒）
    pub fn get_total_duration(&self) -> u64 {
        self.step_results.iter().filter_map(|r| r.duration_ms).sum()
    }
}
