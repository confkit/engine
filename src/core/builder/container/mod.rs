//! 容器管理模块
//!
//! 负责构建器容器相关的功能，包括：
//! - 容器创建和生命周期管理
//! - 容器状态监控
//! - 容器日志管理

pub mod manager;

// 重新导出主要的结构和功能
pub use manager::ContainerManager;
