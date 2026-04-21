//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command executor implementation

use anyhow::Result;
use tokio::process::Command;

use crate::types::common::LogCallback;

use super::context::ExecutionContext;
use crate::{
    core::executor::context::resolve_host_variables, engine::ConfKitEngine,
    infra::logger::TaskLogger, utils::command::CommandUtil,
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
        task_logger: &TaskLogger,
    ) -> Result<i32> {
        for (index, cmd) in commands.iter().enumerate() {
            let resolved = context.resolve_variables(cmd);
            task_logger.info(&format!("  [Cmd {}/{}] {resolved}", index + 1, commands.len()))?;

            let exit_code = ConfKitEngine::execute_in_container(
                container,
                &context.project_config.shell.container,
                working_dir,
                cmd,
                &context.environment,
                task_logger,
            )
            .await?;

            if exit_code != 0 {
                task_logger.error(&format!(
                    "  [Cmd {}/{}] Failed (exit code: {exit_code})",
                    index + 1,
                    commands.len()
                ))?;
                return Ok(exit_code);
            } else {
                task_logger.info(&format!("  [Cmd {}/{}] Done", index + 1, commands.len()))?;
            }
        }

        Ok(0)
    }

    /// 在本地执行命令
    pub async fn execute_locally(
        context: &ExecutionContext,
        working_dir: &str,
        commands: &[String],
        task_logger: &TaskLogger,
    ) -> Result<i32> {
        for (index, cmd) in commands.iter().enumerate() {
            // 创建命令
            let mut command = Command::new(&context.project_config.shell.host);

            resolve_host_variables(&mut command, &context.environment);

            command.arg("-c");
            command.arg(cmd);

            command.current_dir(working_dir);

            let resolved = context.resolve_variables(cmd);
            task_logger.info(&format!("  [Cmd {}/{}] {resolved}", index + 1, commands.len()))?;

            // 创建回调，避免重复代码
            let stdout_callback: Option<LogCallback> = {
                let task_logger = task_logger.clone();
                Some(Box::new(move |line| {
                    let _ = task_logger.info(&format!("    | {}", line));
                }))
            };

            let stderr_callback: Option<LogCallback> = {
                let task_logger = task_logger.clone();
                Some(Box::new(move |line| {
                    let _ = task_logger.info(&format!("    | {}", line));
                }))
            };

            let exit_code = CommandUtil::execute_command_with_output(
                &mut command,
                stdout_callback,
                stderr_callback,
            )
            .await?;

            if exit_code != 0 {
                task_logger.error(&format!(
                    "  [Cmd {}/{}] Failed (exit code: {exit_code})",
                    index + 1,
                    commands.len()
                ))?;
                return Ok(exit_code);
            } else {
                task_logger.info(&format!("  [Cmd {}/{}] Done", index + 1, commands.len()))?;
            }
        }

        Ok(0)
    }
}
