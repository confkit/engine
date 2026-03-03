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

        // 创建任务
        let mut task = Task::new(&host_log_dir);
        let task_id = task.id.clone();

        // 创建执行上下文
        let context = ExecutionContext::new(
            task_id,
            space_name.to_string(),
            project_name.to_string(),
            &project_config,
            environment_from_args,
        )
        .await?;

        // 设置上下文和项目配置
        task.context = Some(context);
        task.project_config = Some(project_config);

        Ok(Self { task })
    }

    pub async fn start(&mut self) -> Result<()> {
        // 立即输出 task id，方便外部调用方获取
        self.task.info(&format!("Task ID: {}", self.task.id))?;

        // 写入初始 metadata
        self.task.write_initial_metadata()?;

        self.task.prepare().await?;
        self.task.execute_steps().await?;
        self.task.cleanup().await?;

        // 完成并写入最终 metadata
        self.task.finalize_metadata()?;

        // 输出执行摘要
        self.task.print_summary()?;

        // 确保所有日志消息都已写入
        self.task.flush_logger().await?;

        Ok(())
    }
}
