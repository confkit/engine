//! Author: xiaoYown
//! Created: 2025-01-09
//! Description: Task logger implementation

use anyhow::Result;

use crate::infra::event_hub::{Event, EventHub, LogLevel};

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
        EventHub::global().publish(
            Event::new_log(level, message.to_string(), "task".to_string())
                .with_metadata("log_path".to_string(), self.log_path.clone()),
        )?;

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
}
