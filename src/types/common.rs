//! Author: xiaoYown
//! Created: 2025-09-13
//! Description: Common types and type aliases

use std::sync::Arc;

/// 日志回调函数类型 (Box版本，用于移动语义)
pub type LogCallback = Box<dyn Fn(&str) + Send + Sync>;

/// 日志回调函数类型 (Arc版本，用于共享引用)
pub type LogCallbackArc = Arc<dyn Fn(&str) + Send + Sync>;

// /// 错误处理回调函数类型
// pub type ErrorCallback = Box<dyn FnOnce(anyhow::Error) + Send + Sync>;

// /// 通用事件回调函数类型
// pub type EventCallback<T> = Box<dyn Fn(T) + Send + Sync>;

// /// 输出处理回调函数类型
// pub type OutputCallback = Box<dyn Fn(&str) + Send + Sync>;
