//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command executor implementation

use anyhow::Result;
use tokio::process::Command;
use tracing;

use super::context::ExecutionContext;
use crate::{
    core::executor::context::resolve_host_variables, engine::ConfKitEngine,
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
    ) -> Result<i32> {
        ConfKitEngine::execute_in_container(container, working_dir, commands, &context.environment)
            .await
    }

    /// 在本地执行命令
    pub async fn execute_locally(
        context: &ExecutionContext,
        working_dir: &str,
        commands: &[String],
    ) -> Result<i32> {
        for cmd in commands {
            let mut command = Command::new("sh");

            resolve_host_variables(&mut command, &context.environment);

            command.arg("-c");
            command.arg(cmd);

            command.current_dir(working_dir);

            let exit_code = CommandUtil::execute_command_with_output(
                &mut command,
                Some(Box::new(|line| tracing::info!("{}", line))),
                Some(Box::new(|line| tracing::info!("{}", line))),
            )
            .await?;

            if exit_code != 0 {
                return Ok(exit_code);
            }
        }

        Ok(0)
    }
}
