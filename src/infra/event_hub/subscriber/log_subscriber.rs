//! Author: xiaoyown
//! Created: 2025-08-14
//! Description: log_subscriber.rs

use async_trait::async_trait;
use chrono::Local;
use std::path::{Path, PathBuf};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

use super::EventSubscriber;
use crate::infra::event_hub::{Event, EventType, LogLevel};

/// 日志消息结构
#[derive(Debug, Clone)]
struct LogMessage {
    /// 日志级别
    level: LogLevel,
    /// 原始消息内容
    message: String,
    /// 日志文件路径
    log_path: PathBuf,
}

/// 日志订阅者
///
/// 负责将日志事件写入动态指定的文件中
/// 日志路径通过事件的 metadata 中的 "log_path" 字段传递
pub struct LogSubscriber {
    /// 默认日志文件路径（当事件中未指定路径时使用）
    default_log_path: Option<PathBuf>,
    // /// 是否启用时间戳
    // enable_timestamp: bool,
    // /// 日志格式模板
    // format_template: String,
}

impl LogSubscriber {
    // /// 创建新的日志订阅者（无默认路径）
    // ///
    // /// 日志路径必须通过事件的 metadata 中的 "log_path" 字段传递
    // pub fn new() -> Self {
    //     Self {
    //         default_log_path: None,
    //         enable_timestamp: true,
    //         format_template: "[{timestamp}] [{level}] {message}".to_string(),
    //     }
    // }

    /// 创建带默认日志路径的订阅者
    ///
    /// # 参数
    /// - `default_log_path`: 默认日志文件路径（当事件中未指定时使用）
    pub fn with_default_path<P: AsRef<Path>>(default_log_path: P) -> Self {
        Self {
            default_log_path: Some(default_log_path.as_ref().to_path_buf()),
            // enable_timestamp: true,
            // format_template: "[{timestamp}][{level}] {message}".to_string(),
        }
    }

    // /// 创建带自定义配置的日志订阅者
    // ///
    // /// # 参数
    // /// - `default_log_path`: 默认日志文件路径
    // /// - `enable_timestamp`: 是否启用时间戳
    // /// - `format_template`: 日志格式模板
    // pub fn with_config<P: AsRef<Path>>(
    //     default_log_path: Option<P>,
    //     enable_timestamp: bool,
    //     format_template: String,
    // ) -> Self {
    //     Self {
    //         default_log_path: default_log_path.map(|p| p.as_ref().to_path_buf()),
    //         enable_timestamp,
    //         format_template,
    //     }
    // }

    /// 从事件中获取日志路径
    ///
    /// 优先使用事件 metadata 中的 "log_path"，其次使用默认路径
    fn get_log_path(&self, event: &Event) -> anyhow::Result<PathBuf> {
        // 首先尝试从事件元数据中获取日志路径
        if let Some(log_path) = event.metadata.get("log_path") {
            return Ok(PathBuf::from(log_path));
        }

        // 其次使用默认路径
        if let Some(ref default_path) = self.default_log_path {
            return Ok(default_path.clone());
        }

        // 如果都没有，返回错误
        Err(anyhow::anyhow!(
            "No log path found: missing 'log_path' field in event metadata, and no default path set"
        ))
    }

    /// 判断是否应该记录该级别的日志
    fn should_log(&self, level: &LogLevel) -> bool {
        // 这里可以根据配置决定日志级别过滤
        // 目前记录所有级别的日志
        match level {
            LogLevel::Trace
            | LogLevel::Debug
            | LogLevel::Info
            | LogLevel::Warn
            | LogLevel::Error => true,
        }
    }

    /// 写入日志文件
    async fn write_log_to_file(&self, log_msg: &LogMessage) -> anyhow::Result<()> {
        // 确保目录存在
        if let Some(parent) = log_msg.log_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create log directory: {}", e))?;
        }

        // 格式化日志行（时间戳+级别+消息）
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let formatted_line = format!("[{}][{}] {}\n", timestamp, log_msg.level, log_msg.message);

        // 以追加模式打开文件并一次性写入完整的日志行
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_msg.log_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to open log file: {}", e))?;

        // 一次性写入完整的日志行
        file.write_all(formatted_line.as_bytes())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write log: {}", e))?;

        // 确保数据写入磁盘
        file.flush().await.map_err(|e| anyhow::anyhow!("Failed to flush log file: {}", e))?;

        Ok(())
    }
}

#[async_trait]
impl EventSubscriber for LogSubscriber {
    fn name(&self) -> &'static str {
        "log_subscriber"
    }

    fn interested_events(&self) -> Vec<&'static str> {
        vec!["log"]
    }

    async fn handle(&self, event: &Event) -> anyhow::Result<()> {
        // 只处理日志事件
        if let EventType::Log(level) = &event.event_type {
            // 检查是否应该记录该级别的日志
            if !self.should_log(level) {
                tracing::debug!("Skipping log level: {}", level);
                return Ok(());
            }

            // 获取日志路径
            let log_path = match self.get_log_path(event) {
                Ok(path) => path,
                Err(e) => {
                    tracing::error!("Failed to get log path: {}", e);
                    return Err(e);
                }
            };

            // 构造日志消息
            let log_message =
                LogMessage { level: level.clone(), message: event.payload.clone(), log_path };

            // 根据日志级别进行相应的 tracing 输出（只输出原始消息）
            match log_message.level {
                LogLevel::Error => tracing::error!("{}", log_message.message),
                LogLevel::Warn => tracing::warn!("{}", log_message.message),
                LogLevel::Info => tracing::info!("{}", log_message.message),
                LogLevel::Debug => tracing::debug!("{}", log_message.message),
                LogLevel::Trace => tracing::trace!("{}", log_message.message),
            }

            // 写入日志文件
            self.write_log_to_file(&log_message).await?;
        } else {
            tracing::warn!("Log subscriber received non-log event: {}", event.type_str());
        }

        Ok(())
    }

    fn is_interested(&self, event: &Event) -> bool {
        matches!(event.event_type, EventType::Log(_))
    }
}
