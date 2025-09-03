//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command executor implementation

use anyhow::Result;
use tokio::process::Command;

use super::context::ExecutionContext;
use crate::{
    core::executor::{context::resolve_host_variables, task::Task},
    engine::ConfKitEngine,
    utils::command::CommandUtil,
};

/// 命令执行器
pub struct CommandExecutor;

impl CommandExecutor {
    /// 在容器中执行命令
    pub async fn execute_in_container(
        context: &ExecutionContext,
        container: &str,
        working_dir: &str,
        commands: &[String],
        task: &Task,
    ) -> Result<i32> {
        ConfKitEngine::execute_in_container(
            container,
            &context.project_config.shell.container,
            working_dir,
            commands,
            &context.environment,
            &task.clone(),
        )
        .await
    }

    /// 在本地执行命令
    pub async fn execute_locally(
        context: &ExecutionContext,
        working_dir: &str,
        commands: &[String],
        task: &Task,
    ) -> Result<i32> {
        Self::execute_locally_with_timeout(context, working_dir, commands, task, None).await
    }

    /// 在本地执行命令，支持超时
    pub async fn execute_locally_with_timeout(
        context: &ExecutionContext,
        working_dir: &str,
        commands: &[String],
        task: &Task,
        timeout: Option<std::time::Duration>,
    ) -> Result<i32> {
        for (index, cmd) in commands.iter().enumerate() {
            // 创建命令
            let mut command = Command::new(&context.project_config.shell.host);

            resolve_host_variables(&mut command, &context.environment);

            command.arg("-c");
            command.arg(cmd);

            command.current_dir(working_dir);

            task.info(&format!(
                "Executing host command ({}/{}): '{cmd}' in directory: {working_dir}",
                index + 1,
                commands.len()
            ))?;

            let exit_code = if timeout.is_some() {
                CommandUtil::execute_command_with_timeout(
                    &mut command,
                    {
                        let task = task.clone();
                        Some(Box::new(move |line| {
                            let _ = task.info(line);
                        }))
                    },
                    {
                        let task = task.clone();
                        Some(Box::new(move |line| {
                            let _ = task.info(line);
                        }))
                    },
                    timeout,
                )
                .await?
            } else {
                CommandUtil::execute_command_with_output(
                    &mut command,
                    {
                        let task = task.clone();
                        Some(Box::new(move |line| {
                            let _ = task.info(line);
                        }))
                    },
                    {
                        let task = task.clone();
                        Some(Box::new(move |line| {
                            let _ = task.info(line);
                        }))
                    },
                )
                .await?
            };

            if exit_code != 0 {
                task.error(&format!("Command failed with exit code {exit_code}: '{cmd}'"))?;
                return Ok(exit_code);
            } else {
                task.info(&format!(
                    "Command completed successfully ({}/{}): '{cmd}'",
                    index + 1,
                    commands.len()
                ))?;
            }
        }

        Ok(0)
    }
}
