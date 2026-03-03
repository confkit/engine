//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Command utility

use anyhow::Result;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::types::common::{LogCallback, LogCallbackArc};

pub type LogCallbackBox = LogCallback;

pub struct CommandUtil;

impl CommandUtil {
    // 执行命令并输出日志
    pub async fn execute_command_with_output(
        cmd: &mut Command,
        on_stdout: Option<LogCallbackBox>,
        on_stderr: Option<LogCallbackBox>,
    ) -> Result<i32> {
        Self::execute_command_with_timeout(cmd, on_stdout, on_stderr, None).await
    }

    // 执行命令并输出日志，支持超时和取消
    pub async fn execute_command_with_timeout(
        cmd: &mut Command,
        on_stdout: Option<LogCallbackBox>,
        on_stderr: Option<LogCallbackBox>,
        command_timeout: Option<Duration>,
    ) -> Result<i32> {
        let cancel_token = CancellationToken::new();
        let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

        // 将 stdout 和 stderr 合并到单一 task 中顺序读取，避免两个独立 task 的竞争导致乱序
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let stdout_cb = on_stdout.map(|f| Arc::from(f) as LogCallbackArc);
        let stderr_cb = on_stderr.map(|f| Arc::from(f) as LogCallbackArc);
        let cancel_clone = cancel_token.clone();

        let log_handle = tokio::spawn(async move {
            let mut stdout_lines = stdout.map(|s| BufReader::new(s).lines());
            let mut stderr_lines = stderr.map(|s| BufReader::new(s).lines());
            let mut stdout_done = stdout_lines.is_none();
            let mut stderr_done = stderr_lines.is_none();

            loop {
                if stdout_done && stderr_done {
                    break;
                }

                tokio::select! {
                    biased;

                    _ = cancel_clone.cancelled() => {
                        tracing::debug!("Log reader cancelled");
                        break;
                    }

                    result = stdout_lines.as_mut().unwrap().next_line(),
                        if !stdout_done =>
                    {
                        match result {
                            Ok(Some(line)) => {
                                if let Some(ref cb) = stdout_cb {
                                    cb(&line);
                                }
                            }
                            Ok(None) => {
                                tracing::debug!("Stdout reader reached EOF");
                                stdout_done = true;
                            }
                            Err(e) => {
                                tracing::error!("Error reading stdout: {}", e);
                                stdout_done = true;
                            }
                        }
                    }

                    result = stderr_lines.as_mut().unwrap().next_line(),
                        if !stderr_done =>
                    {
                        match result {
                            Ok(Some(line)) => {
                                if let Some(ref cb) = stderr_cb {
                                    cb(&line);
                                }
                            }
                            Ok(None) => {
                                tracing::debug!("Stderr reader reached EOF");
                                stderr_done = true;
                            }
                            Err(e) => {
                                tracing::error!("Error reading stderr: {}", e);
                                stderr_done = true;
                            }
                        }
                    }
                }
            }
        });

        // 设置信号处理
        let cancel_token_clone = cancel_token.clone();
        let child_handle = std::sync::Arc::new(tokio::sync::Mutex::new(Some(child)));

        #[cfg(unix)]
        {
            let child_handle_clone = child_handle.clone();
            tokio::spawn(async move {
                let mut sigint =
                    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
                        .expect("Failed to create SIGINT handler");
                let mut sigterm =
                    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                        .expect("Failed to create SIGTERM handler");

                tokio::select! {
                    _ = sigint.recv() => {
                        tracing::info!("Received SIGINT, cancelling command execution");
                        cancel_token_clone.cancel();
                        if let Some(mut child) = child_handle_clone.lock().await.take() {
                            let _ = child.kill().await;
                        }
                    }
                    _ = sigterm.recv() => {
                        tracing::info!("Received SIGTERM, cancelling command execution");
                        cancel_token_clone.cancel();
                        if let Some(mut child) = child_handle_clone.lock().await.take() {
                            let _ = child.kill().await;
                        }
                    }
                }
            });
        }

        // 等待进程完成，支持超时
        let wait_result = {
            if let Some(mut child) = child_handle.lock().await.take() {
                if let Some(timeout_duration) = command_timeout {
                    timeout(timeout_duration, child.wait()).await
                } else {
                    Ok(child.wait().await)
                }
            } else {
                // 进程已被终止
                #[cfg(unix)]
                {
                    Ok(Ok(std::process::ExitStatus::from_raw(0)))
                }
                #[cfg(not(unix))]
                {
                    Ok(Err(anyhow::anyhow!("Process was terminated")))
                }
            }
        };

        // 等待日志读取任务完成（进程退出后 pipe 自然 EOF，reader 会读完所有缓冲数据）
        let _ = log_handle.await;

        // 所有日志读取完毕后再取消（清理信号处理任务）
        cancel_token.cancel();

        match wait_result {
            Ok(Ok(status)) => {
                let exit_code = status.code().unwrap_or(-1);
                Ok(exit_code)
            }
            Ok(Err(e)) => {
                tracing::error!("Command execution error: {}", e);
                Err(e.into())
            }
            Err(_) => {
                tracing::warn!("Command execution timed out, killing process");
                if let Some(mut child) = child_handle.lock().await.take() {
                    let _ = child.kill().await;
                }
                Ok(-1) // 超时返回 -1
            }
        }
    }
}
