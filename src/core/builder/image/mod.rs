//! 镜像管理模块
//!
//! 负责构建器镜像相关的功能，包括：
//! - 镜像构建和拉取
//! - 镜像检查和验证
//! - 镜像信息管理

pub mod builder;
pub mod inspector;
pub mod manager;

// 重新导出主要的结构和功能
pub use builder::ImageBuilder;
pub use inspector::ImageInspector;
pub use manager::ImageManager;

// 重新导出类型（从上级模块导入）
pub use crate::core::builder::types::ImageCheckResult;
