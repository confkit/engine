//! Author: xiaoYown
//! Created: 2025-01-09
//! Description: Task logger implementation

use std::fs::{self, OpenOptions};
use std::io::Write;

use anyhow::Result;
use chrono::Local;

use super::LogLevel;

/// 任务日志记录器 - 轻量级日志接口
#[derive(Debug, Clone)]
pub struct TaskLogger {
    log_path: String,
}

impl TaskLogger {
    pub fn new(log_path: String) -> Self {
        Self { log_path }
    }

    /// 记录指定级别的日志
    pub fn log_with_level(&self, message: &str, level: LogLevel) -> Result<(), anyhow::Error> {
        // tracing 输出到终端
        match level {
            LogLevel::Error => tracing::error!("{}", message),
            LogLevel::Warn => tracing::warn!("{}", message),
            LogLevel::Info => tracing::info!("{}", message),
            LogLevel::Debug => tracing::debug!("{}", message),
            LogLevel::Trace => tracing::trace!("{}", message),
        }

        // 同步写入日志文件
        let log_path = std::path::Path::new(&self.log_path);
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{}][{}] {}\n", timestamp, level, message);
        let mut file = OpenOptions::new().create(true).append(true).open(log_path)?;
        file.write_all(line.as_bytes())?;

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
