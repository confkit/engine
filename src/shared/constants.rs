//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Constants

/// 配置文件名
pub const CONFKIT_CONFIG_FILE: &str = ".confkit.yml";

/// Volumes 根目录
pub const HOST_VOLUMES_DIR: &str = "volumes";

/// 工作目录
pub const HOST_WORKSPACE_DIR: &str = "volumes/workspace";

/// 产物目录
pub const HOST_ARTIFACTS_ROOT_DIR: &str = "volumes/artifacts";

/// 日志目录
pub const HOST_LOG_DIR: &str = "volumes/logs";

/// 容器工作空间目录
pub const CONTAINER_WORKSPACE_DIR: &str = "/workspace";

/// 容器产物目录
pub const CONTAINER_ARTIFACTS_ROOT_DIR: &str = "/artifacts";

/// 缓存目录
pub const HOST_CACHE_DIR: &str = "volumes/cache";

/// 临时目录(用于临时文件存储, 仓库信息获取等)
pub const HOST_TEMP_DIR: &str = "volumes/temp";

/// 任务日志文件名
pub const TASK_LOG_FILE: &str = "task.log";

/// 任务元数据文件名
pub const TASK_META_FILE: &str = "task.meta.json";

/// 任务数据库文件名
pub const TASK_DB_FILE: &str = "tasks.db";

// /// 容器缓存目录
// pub const CONTAINER_CACHE_DIR: &str = "/cache";
