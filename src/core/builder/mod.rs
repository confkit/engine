//! Builder 核心模块
//!
//! 负责构建器相关的核心功能，包括：
//! - 镜像管理 (image)
//! - 容器管理 (container)
//! - 通用工具和类型定义

// 新的子模块
pub mod container;
pub mod image;

// 保留的通用模块
mod formatter;
mod loader;
mod manager;
mod output_handler;
mod puller;
mod types;
mod validator;

// 重新导出新模块的功能
pub use container::ContainerManager;
pub use image::{ImageBuilder, ImageCheckResult, ImageInspector, ImageManager};

// 重新导出通用功能（保持向后兼容）
pub use formatter::*;
pub use loader::*;
pub use manager::*;
pub use output_handler::*;
pub use puller::*;
pub use types::*;
pub use validator::*;
