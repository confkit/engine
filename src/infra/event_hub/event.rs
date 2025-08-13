//! Author: xiaoyown
//! Created: 2025-08-14
//! Description: event.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 日志级别
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// 事件类型定义
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// 日志事件
    Log(LogLevel),
    /// 任务状态事件
    TaskStatus,
    /// 系统监控事件
    SystemMetrics,
    /// 自定义事件
    Custom(String),
}

impl EventType {
    /// 获取事件类型的字符串表示
    pub fn as_str(&self) -> &str {
        match self {
            EventType::Log(_) => "log",
            EventType::TaskStatus => "task_status",
            EventType::SystemMetrics => "system_metrics",
            EventType::Custom(name) => name,
        }
    }
}

/// 事件结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// 事件类型
    pub event_type: EventType,
    /// 事件负载（消息内容）
    pub payload: String,
    /// 事件时间戳
    pub timestamp: DateTime<Utc>,
    /// 事件来源
    pub source: String,
    /// 事件ID（用于去重和追踪）
    pub id: String,
    /// 额外的元数据
    pub metadata: HashMap<String, String>,
}

impl Event {
    /// 创建新的事件
    pub fn new(event_type: EventType, payload: String, source: String) -> Self {
        Self {
            event_type,
            payload,
            timestamp: Utc::now(),
            source,
            id: Uuid::new_v4().to_string(),
            metadata: HashMap::new(),
        }
    }

    /// 创建日志事件的便捷方法
    pub fn new_log(level: LogLevel, message: String, source: String) -> Self {
        Self::new(EventType::Log(level), message, source)
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// 获取事件类型字符串
    pub fn type_str(&self) -> &str {
        self.event_type.as_str()
    }
}
