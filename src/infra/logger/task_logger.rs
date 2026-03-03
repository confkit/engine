//! Author: xiaoYown
//! Created: 2025-01-09
//! Description: Task logger implementation

use std::fs::{self, OpenOptions};
use std::io::Write;

use anyhow::Result;
use chrono::Local;
use tokio::sync::{mpsc, oneshot};

use super::LogLevel;

/// 内部日志命令
enum LogCommand {
    Log { message: String, level: LogLevel },
    Flush { responder: oneshot::Sender<()> },
}

/// 任务日志记录器 - 通过 mpsc 通道序列化日志输出
#[derive(Clone)]
pub struct TaskLogger {
    sender: mpsc::UnboundedSender<LogCommand>,
}

impl TaskLogger {
    pub fn new(log_path: String) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        // 启动后台消费者 task
        tokio::spawn(Self::consumer(receiver, log_path));

        Self { sender }
    }

    /// 后台消费者：顺序处理所有日志命令
    async fn consumer(mut receiver: mpsc::UnboundedReceiver<LogCommand>, log_path: String) {
        while let Some(cmd) = receiver.recv().await {
            match cmd {
                LogCommand::Log { message, level } => {
                    // tracing 输出到终端
                    match level {
                        LogLevel::Error => tracing::error!("{}", message),
                        LogLevel::Warn => tracing::warn!("{}", message),
                        LogLevel::Info => tracing::info!("{}", message),
                        LogLevel::Debug => tracing::debug!("{}", message),
                        LogLevel::Trace => tracing::trace!("{}", message),
                    }

                    // 同步写入日志文件
                    if let Err(e) = Self::write_to_file(&log_path, &message, &level) {
                        tracing::error!("Failed to write log file: {}", e);
                    }
                }
                LogCommand::Flush { responder } => {
                    // 所有之前的消息已处理完毕，通知调用方
                    let _ = responder.send(());
                }
            }
        }
    }

    /// 写入日志文件
    fn write_to_file(log_path: &str, message: &str, level: &LogLevel) -> Result<()> {
        let path = std::path::Path::new(log_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{}][{}] {}\n", timestamp, level, message);
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }

    /// 记录指定级别的日志
    pub fn log_with_level(&self, message: &str, level: LogLevel) -> Result<(), anyhow::Error> {
        self.sender
            .send(LogCommand::Log { message: message.to_string(), level })
            .map_err(|_| anyhow::anyhow!("Log channel closed"))?;
        Ok(())
    }

    /// 等待所有积压的日志消息处理完毕
    pub async fn flush(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(LogCommand::Flush { responder: tx })
            .map_err(|_| anyhow::anyhow!("Log channel closed"))?;
        rx.await.map_err(|_| anyhow::anyhow!("Log flush failed"))?;
        Ok(())
    }

    /// 记录 Info 级别日志的便捷方法
    pub fn info(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Info)
    }

    /// 记录 Error 级别日志的便捷方法
    pub fn error(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Error)
    }

    /// 记录 Warn 级别日志的便捷方法
    pub fn warn(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Warn)
    }
}
