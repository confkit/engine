//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command executor implementation

use anyhow::Result;
use tokio::process::Command;
use tracing;

use super::context::ExecutionContext;
use crate::utils::command::CommandUtil;

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
        for cmd in commands {
            let mut command = Command::new("docker");

            command.args(&["exec", "-i"]);

            context.resolve_container_variables(&mut command);

            command.args(&["-w", working_dir]);

            command.args(&[container, "sh", "-c", cmd]);

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

    /// 在本地执行命令
    pub async fn execute_locally(
        context: &ExecutionContext,
        working_dir: &str,
        commands: &[String],
    ) -> Result<i32> {
        for cmd in commands {
            let mut command = Command::new("sh");
            command.arg("-c");
            command.arg(cmd);

            command.current_dir(working_dir);

            context.resolve_host_variables(&mut command);
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
