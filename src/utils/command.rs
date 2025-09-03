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

pub type LogCallback = Arc<dyn Fn(&str) + Send + Sync>;
pub type LogCallbackBox = Box<dyn Fn(&str) + Send + Sync>;

pub struct CommandUtil;

impl CommandUtil {
    // 异步读取命令输出并输出日志
    pub fn spawn_log_reader<R>(
        reader: R,
        callback: Option<LogCallback>,
        cancel_token: CancellationToken,
    ) -> tokio::task::JoinHandle<()>
    where
        R: tokio::io::AsyncRead + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();
            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        tracing::debug!("Log reader cancelled");
                        break;
                    }
                    line_result = lines.next_line() => {
                        match line_result {
                            Ok(Some(line)) => {
                                if let Some(ref cb) = callback {
                                    cb(&line);
                                }
                            }
                            Ok(None) => {
                                tracing::debug!("Log reader reached EOF");
                                break;
                            }
                            Err(e) => {
                                tracing::error!("Error reading line: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        })
    }

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

        let mut log_handles = Vec::new();

        if let Some(stdout) = child.stdout.take() {
            let callback = on_stdout.map(|f| Arc::from(f) as LogCallback);
            let handle = Self::spawn_log_reader(stdout, callback, cancel_token.clone());
            log_handles.push(handle);
        }

        if let Some(stderr) = child.stderr.take() {
            let callback = on_stderr.map(|f| Arc::from(f) as LogCallback);
            let handle = Self::spawn_log_reader(stderr, callback, cancel_token.clone());
            log_handles.push(handle);
        }

        // 设置信号处理
        let cancel_token_clone = cancel_token.clone();
        let child_handle = std::sync::Arc::new(tokio::sync::Mutex::new(Some(child)));
        
        #[cfg(unix)]
        {
            let child_handle_clone = child_handle.clone();
            tokio::spawn(async move {
                let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).expect("Failed to create SIGINT handler");
                let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).expect("Failed to create SIGTERM handler");
                
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

        // 取消日志读取任务
        cancel_token.cancel();
        
        // 等待日志读取任务完成
        for handle in log_handles {
            let _ = handle.await;
        }

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
