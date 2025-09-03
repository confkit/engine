//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Runner implementation

use std::collections::HashMap;

use anyhow::Result;
use tracing;

use super::context::ExecutionContext;
use super::task::Task;
use crate::formatter::path::PathFormatter;
use crate::infra::config::ConfKitConfigLoader;

/// 主执行器
pub struct Runner {
    task: Task,
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
                tracing::error!("Project '{project_name}' not found in space '{space_name}'");
                return Err(anyhow::anyhow!("Project not found"));
            }
        };

        let host_log_dir = PathFormatter::log_project_dir(space_name, project_name);

        // 创建基础任务以获取 task_id
        let base_task = Task::new(&host_log_dir);
        let task_id = base_task.id.clone();

        // 创建执行上下文
        let context = ExecutionContext::new(
            task_id.clone(),
            space_name.to_string(),
            project_name.to_string(),
            &project_config,
            environment_from_args,
        )
        .await?;

        // 创建带上下文的任务
        let task = Task::with_context(&host_log_dir, context.clone(), project_config.clone());

        Ok(Self { task })
    }

    pub async fn start(&mut self) -> Result<()> {
        // 使用新的 Task 方法
        self.task.prepare().await?;
        self.task.execute_steps().await?;
        self.task.cleanup().await?;
        self.task.finish();

        // 输出执行摘要
        self.task.print_summary()?;

        Ok(())
    }
}
