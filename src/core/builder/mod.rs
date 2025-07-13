//! Builder 核心模块
//!
//! 负责构建器相关的核心功能，包括：
//! - 镜像管理 (image)
//! - 容器管理 (container)
//! - 通用工具和类型定义

pub mod container;
pub mod image;

// 保留的通用模块
// mod formatter;
// mod loader;
// mod output_handler;
// mod puller;
// mod types;
// mod validator;

// 重新导出新模块的功能
// pub use container::{BuilderContainer, ContainerManager, ContainerStatus};
// pub use image::{ImageBuilder, ImageCheckResult, ImageInspector, ImageManager};
