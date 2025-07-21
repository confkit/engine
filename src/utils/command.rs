//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command utility

use anyhow::Result;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub type LogCallback = Arc<dyn Fn(&str) + Send + Sync>;

pub struct CommandUtil;

impl CommandUtil {
    // 异步读取命令输出并输出日志
    pub fn spawn_log_reader<R>(reader: R, callback: Option<LogCallback>)
    where
        R: tokio::io::AsyncRead + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();
            while let Some(line_result) = lines.next_line().await.transpose() {
                match line_result {
                    Ok(line) => {
                        if let Some(ref cb) = callback {
                            cb(&line);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error reading line: {}", e);
                    }
                }
            }
        });
    }

    // 执行命令并输出日志
    pub async fn execute_command_with_output(
        cmd: &mut Command,
        on_stdout: Option<Box<dyn Fn(&str) + Send + Sync>>,
        on_stderr: Option<Box<dyn Fn(&str) + Send + Sync>>,
    ) -> Result<i32> {
        let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

        if let Some(stdout) = child.stdout.take() {
            let callback = on_stdout.map(|f| Arc::from(f) as LogCallback);
            Self::spawn_log_reader(stdout, callback);
        }

        if let Some(stderr) = child.stderr.take() {
            let callback = on_stderr.map(|f| Arc::from(f) as LogCallback);
            Self::spawn_log_reader(stderr, callback);
        }

        let status = child.wait().await?;
        let exit_code = status.code().unwrap_or(-1);

        // 返回退出码，让调用者决定如何处理
        Ok(exit_code)
    }
}
