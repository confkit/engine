use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 构建器配置（用于镜像构建）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderConfig {
    pub name: String,
    pub image: String,
    pub dockerfile: String,
    pub context: String,                     // 构建上下文路径
    pub build_args: HashMap<String, String>, // 构建参数
}

impl BuilderConfig {
    /// 从基本参数创建构建器配置
    pub fn new(name: String, dockerfile: String) -> Self {
        // 从 Dockerfile 路径推断镜像名称和构建上下文
        let (image, context) = Self::derive_image_and_context(&name, &dockerfile);

        Self { name, image, dockerfile, context, build_args: HashMap::new() }
    }

    /// 从名称和 Dockerfile 路径推断镜像名称和构建上下文
    fn derive_image_and_context(name: &str, dockerfile: &str) -> (String, String) {
        let path = std::path::Path::new(dockerfile);

        // 构建上下文是 Dockerfile 所在目录
        let context = if let Some(parent) = path.parent() {
            parent.to_string_lossy().to_string()
        } else {
            ".".to_string()
        };

        // 镜像名称格式: 构建器名:latest
        let image = format!("{}:latest", name);

        (image, context)
    }

    /// 添加构建参数
    pub fn with_build_arg(mut self, key: String, value: String) -> Self {
        self.build_args.insert(key, value);
        self
    }

    /// 设置镜像名称
    pub fn with_image(mut self, image: String) -> Self {
        self.image = image;
        self
    }

    /// 设置构建上下文
    pub fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }
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
    pub image_id: Option<String>, // 镜像 ID（替代容器 ID）
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub build_logs: Option<String>, // 构建日志（替代健康检查）
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
