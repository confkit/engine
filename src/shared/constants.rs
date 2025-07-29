//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Constants

/// 配置文件名
pub const CONFKIT_CONFIG_FILE: &str = ".confkit.yml";

/// 工作目录
pub const HOST_WORKSPACE_DIR: &str = "volumes/workspace";

/// 日志目录
pub const HOST_LOG_DIR: &str = "volumes/logs";

/// 容器工作空间目录
pub const CONTAINER_WORKSPACE_DIR: &str = "/workspace";

/// 缓存目录
pub const HOST_CACHE_DIR: &str = "volumes/cache";

/// 容器缓存目录
pub const CONTAINER_CACHE_DIR: &str = "/cache";
