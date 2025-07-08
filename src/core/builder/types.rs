use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 构建器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderConfig {
    pub name: String,
    pub image: String,
    pub dockerfile: Option<String>,
    pub required: bool,
    pub health_check: Option<String>,
    pub volumes: Vec<String>,
    pub environment: HashMap<String, String>,
    pub ports: Vec<String>,
}

/// 构建器状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuilderStatus {
    NotCreated,
    Created,
    Running,
    Stopped,
    Error,
}

/// 健康检查状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub message: String,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// 构建器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderInfo {
    pub name: String,
    pub status: BuilderStatus,
    pub config: BuilderConfig,
    pub container_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_health_check: Option<HealthStatus>,
}

/// Docker Compose 文件结构
#[derive(Debug, Clone, Deserialize)]
pub struct DockerCompose {
    pub services: HashMap<String, ComposeService>,
    #[serde(default)]
    pub volumes: HashMap<String, serde_yaml::Value>,
}

/// Docker Compose 服务定义
#[derive(Debug, Clone, Deserialize)]
pub struct ComposeService {
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub build: Option<ComposeBuild>,
    #[serde(default)]
    pub container_name: Option<String>,
    #[serde(default)]
    pub volumes: Vec<String>,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub restart: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub working_dir: Option<String>,
}

/// Docker Compose 构建配置
#[derive(Debug, Clone, Deserialize)]
pub struct ComposeBuild {
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub dockerfile: Option<String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl ComposeService {
    /// 从 labels 中解析构建器类型
    pub fn get_builder_type(&self) -> Option<String> {
        self.labels.iter().find_map(|label| {
            if label.starts_with("builder.type=") {
                Some(label.replace("builder.type=", ""))
            } else {
                None
            }
        })
    }

    /// 从 labels 中解析构建器版本
    pub fn get_builder_version(&self) -> Option<String> {
        self.labels.iter().find_map(|label| {
            if label.starts_with("builder.version=") {
                Some(label.replace("builder.version=", ""))
            } else {
                None
            }
        })
    }

    /// 生成镜像名称（优先使用 image，其次使用 build context 生成）
    pub fn get_image_name(&self) -> String {
        if let Some(image) = &self.image {
            image.clone()
        } else if let Some(build) = &self.build {
            if let Some(context) = &build.context {
                if let Some(dockerfile) = &build.dockerfile {
                    format!(
                        "{}:{}",
                        context.replace("./", ""),
                        dockerfile.replace("Dockerfile.", "")
                    )
                } else {
                    format!("{}:latest", context.replace("./", ""))
                }
            } else {
                "unknown:latest".to_string()
            }
        } else {
            "unknown:latest".to_string()
        }
    }
}
