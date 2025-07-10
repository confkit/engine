use serde::{Deserialize, Serialize};
use std::fmt;

/// 自定义错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfkitError {
    /// 配置错误
    ConfigError { message: String, file_path: Option<String> },
    /// 任务错误
    TaskError { task_id: String, step_name: Option<String>, message: String },
    /// Docker错误
    DockerError { message: String, container_id: Option<String> },
    /// Git错误
    GitError { message: String, repo_url: Option<String> },
    /// 网络错误
    NetworkError { message: String, url: Option<String> },
    /// 存储错误
    StorageError { message: String, path: Option<String> },
    /// 验证错误
    ValidationError { field: String, message: String },
    /// 系统错误
    SystemError { message: String, source: Option<String> },
}

impl fmt::Display for ConfkitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfkitError::ConfigError { message, file_path } => {
                if let Some(path) = file_path {
                    write!(f, "配置错误 [{}]: {}", path, message)
                } else {
                    write!(f, "配置错误: {}", message)
                }
            }
            ConfkitError::TaskError { task_id, step_name, message } => {
                if let Some(step) = step_name {
                    write!(f, "任务错误 [{}::{}]: {}", task_id, step, message)
                } else {
                    write!(f, "任务错误 [{}]: {}", task_id, message)
                }
            }
            ConfkitError::DockerError { message, container_id } => {
                if let Some(id) = container_id {
                    write!(f, "Docker错误 [{}]: {}", id, message)
                } else {
                    write!(f, "Docker错误: {}", message)
                }
            }
            ConfkitError::GitError { message, repo_url } => {
                if let Some(url) = repo_url {
                    write!(f, "Git错误 [{}]: {}", url, message)
                } else {
                    write!(f, "Git错误: {}", message)
                }
            }
            ConfkitError::NetworkError { message, url } => {
                if let Some(u) = url {
                    write!(f, "网络错误 [{}]: {}", u, message)
                } else {
                    write!(f, "网络错误: {}", message)
                }
            }
            ConfkitError::StorageError { message, path } => {
                if let Some(p) = path {
                    write!(f, "存储错误 [{}]: {}", p, message)
                } else {
                    write!(f, "存储错误: {}", message)
                }
            }
            ConfkitError::ValidationError { field, message } => {
                write!(f, "验证错误 [{}]: {}", field, message)
            }
            ConfkitError::SystemError { message, source } => {
                if let Some(src) = source {
                    write!(f, "系统错误 [{}]: {}", src, message)
                } else {
                    write!(f, "系统错误: {}", message)
                }
            }
        }
    }
}

impl std::error::Error for ConfkitError {}

/// 错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub task_id: Option<String>,
    pub step_name: Option<String>,
    pub file_path: Option<String>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            task_id: None,
            step_name: None,
            file_path: None,
            additional_info: std::collections::HashMap::new(),
        }
    }
}

impl ErrorContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_task_id(mut self, task_id: String) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_step_name(mut self, step_name: String) -> Self {
        self.step_name = Some(step_name);
        self
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn with_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}

/// 结果类型别名
pub type ConfkitResult<T> = Result<T, ConfkitError>;

/// 错误转换trait
pub trait IntoConfkitError {
    fn into_confkit_error(self, context: ErrorContext) -> ConfkitError;
}

impl IntoConfkitError for std::io::Error {
    fn into_confkit_error(self, context: ErrorContext) -> ConfkitError {
        if let Some(path) = context.file_path {
            ConfkitError::StorageError { message: self.to_string(), path: Some(path) }
        } else {
            ConfkitError::SystemError {
                message: self.to_string(),
                source: Some("std::io::Error".to_string()),
            }
        }
    }
}

impl IntoConfkitError for serde_yaml::Error {
    fn into_confkit_error(self, context: ErrorContext) -> ConfkitError {
        ConfkitError::ConfigError {
            message: format!("YAML解析错误: {}", self),
            file_path: context.file_path,
        }
    }
}

/// 便利宏：创建配置错误
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        ConfkitError::ConfigError { message: $msg.to_string(), file_path: None }
    };
    ($msg:expr, $path:expr) => {
        ConfkitError::ConfigError { message: $msg.to_string(), file_path: Some($path.to_string()) }
    };
}

/// 便利宏：创建任务错误
#[macro_export]
macro_rules! task_error {
    ($task_id:expr, $msg:expr) => {
        ConfkitError::TaskError {
            task_id: $task_id.to_string(),
            step_name: None,
            message: $msg.to_string(),
        }
    };
    ($task_id:expr, $step:expr, $msg:expr) => {
        ConfkitError::TaskError {
            task_id: $task_id.to_string(),
            step_name: Some($step.to_string()),
            message: $msg.to_string(),
        }
    };
}

/// 便利宏：创建验证错误
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $msg:expr) => {
        ConfkitError::ValidationError { field: $field.to_string(), message: $msg.to_string() }
    };
}
