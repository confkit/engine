//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Task implementation

use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::infra::event_hub::{Event, EventHub, LogLevel};

/// 任务ID生成器
pub struct Task {
    pub id: String,
    pub started_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
    pub log_path: String,
}

impl Task {
    pub fn new(log_dir: &str) -> Self {
        let task_id = Self::generate_task_id();
        let timestamp = Local::now().format("%Y.%m.%d-%H:%M:%S%.3f");
        let log_path = format!("{log_dir}/[{timestamp}]-{task_id}.log");

        Self { id: task_id, started_at: Local::now(), finished_at: None, log_path }
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

    /// 记录 Warn 级别日志的便捷方法
    pub fn warn(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Warn)
    }

    /// 记录 Debug 级别日志的便捷方法
    pub fn debug(&self, message: &str) -> Result<(), anyhow::Error> {
        self.log_with_level(message, LogLevel::Debug)
    }

    // 生成任务ID
    fn generate_task_id() -> String {
        let uuid = Uuid::new_v4();

        uuid.to_string()[..11].to_string()
    }

    pub fn finish(&mut self) {
        self.finished_at = Some(Local::now());
    }

    pub fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            started_at: self.started_at.clone(),
            finished_at: self.finished_at.clone(),
            log_path: self.log_path.clone(),
        }
    }
}
