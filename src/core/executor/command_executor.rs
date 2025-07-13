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
        working_dir: Option<&str>,
        commands: &[String],
    ) -> Result<i32> {
        for cmd in commands {
            let mut command = Command::new("docker");

            command.args(&["exec", "-i"]);

            context.resolve_container_variables(&mut command);

            if let Some(working_dir) = working_dir {
                command.args(&["-w", context.resolve_working_dir(working_dir).as_str()]);
            } else {
                command.args(&["-w", context.container_workspace_dir.as_str()]);
            }

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
        working_dir: Option<&str>,
        commands: &[String],
    ) -> Result<i32> {
        for cmd in commands {
            let mut command = Command::new("sh");
            command.arg("-c");
            command.arg(cmd);

            if let Some(working_dir) = working_dir {
                command.current_dir(context.resolve_working_dir(working_dir).as_str());
            } else {
                command.current_dir(context.host_workspace_dir.as_str());
            }

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
