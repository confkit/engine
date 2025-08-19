//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: ConfKit Config Types

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// ================================================ Engine infra types ================================================
/// engine image info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineImageInfo {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub created_at: String,
    pub size: String,
    #[serde(default = "default_image_unbuilt")]
    pub status: ImageStatus,
}

/// engine container info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub created_at: String,
    pub size: String,
    #[serde(default = "default_container_unbuilt")]
    pub status: ContainerStatus,
    pub working_dir: Option<String>,
}

/// ================================================ Engine Compose Config ================================================
/// engine compose config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineComposeConfig {
    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub services: HashMap<String, EngineServiceConfig>,

    // 忽略其他所有字段（networks, volumes, etc.）
    #[serde(flatten)]
    pub other: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineServiceConfig {
    #[serde(default)]
    pub service_name: String,

    #[serde(default)]
    pub container_name: String,

    // 保留一些常用字段，但都设为可选
    #[serde(default)]
    pub image: String,

    #[serde(default)]
    pub working_dir: Option<String>,

    #[serde(default)]
    pub ports: Option<Vec<String>>,

    #[serde(default)]
    pub environment: Option<HashMap<String, String>>,

    #[serde(default)]
    pub volumes: Option<Vec<serde_yaml::Value>>,

    #[serde(default)]
    pub depends_on: Option<Vec<String>>,

    // 忽略所有其他字段
    #[serde(flatten)]
    pub other: HashMap<String, serde_yaml::Value>,
}

/// ================================================ ConfKit Config ================================================
/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitConfig {
    pub version: String,
    #[serde(default = "default_engine")]
    pub engine: Engine,
    pub engine_compose: ConfKitEngineComposeConfig,
    pub spaces: Vec<ConfKitSpaceConfig>,
    pub images: Vec<ConfKitImageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitSpaceConfig {
    pub name: String,
    pub description: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitProjectConfig {
    pub name: String,
    pub description: String,
    #[serde(default = "default_shell")]
    pub shell: ConfKitShellConfig,
    pub source: Option<ConfKitSourceConfig>,
    /// 环境变量文件, 优先级最低
    pub environment_files: Option<Vec<ConfKitEnvironmentFileConfig>>,
    /// 环境变量, 优先级次之
    pub environment: Option<HashMap<String, String>>,
    /// 来自参数的环境变量, 优先级最高(仅 interfactive 模式下生效)
    pub environment_from_args: Option<Vec<ConfKitEnvironmentInteractiveConfig>>,
    pub cleaner: Option<ConfKitCleanerConfig>,
    pub steps: Vec<ConfKitStepConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitCleanerConfig {
    pub workspace: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitSourceConfig {
    pub git_repo: String,
    pub git_branch: String,
    // 项目语言: javascript, rust
    pub language: Option<String>,
    // 项目配置文件: javascript: package.json, rust: Cargo.toml
    pub manifest_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitStepConfig {
    pub name: String,
    pub container: Option<String>,
    pub working_dir: Option<String>,
    pub commands: Vec<String>,
    /// 超时时间，单位：秒
    pub timeout: Option<u64>,
    #[serde(default)]
    pub continue_on_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEnvironmentFileConfig {
    /// yaml, env
    pub format: String,
    pub path: String,
}

/// === ConfKit Config file image config ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitImageConfig {
    pub name: String,
    pub base_image: String,
    pub tag: String,
    pub context: String,
    pub engine_file: String,
}

/// === ConfKit Config file image info ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitImageInfo {
    // === ConfKit Config file image config ===
    pub name: String,
    pub base_image: String,
    pub tag: String,
    pub context: String,
    pub engine_file: String,
    // === Engine image info ===
    #[serde(default = "default_image_unbuilt")]
    pub status: ImageStatus,
    // Information from engine
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEngineComposeConfig {
    #[serde(default = "default_project")]
    pub project: String,
    #[serde(default)]
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitShellConfig {
    #[serde(default = "default_bash")]
    pub host: String,
    #[serde(default = "default_bash")]
    pub container: String,
}

/// 宿主机使用的引擎
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Docker,
    Podman,
}

fn default_engine() -> Engine {
    Engine::Docker
}

fn default_project() -> String {
    "confkit".to_string()
}

fn default_bash() -> String {
    "bash".to_string()
}

fn default_shell() -> ConfKitShellConfig {
    ConfKitShellConfig { host: "bash".to_string(), container: "bash".to_string() }
}

fn default_image_unbuilt() -> ImageStatus {
    ImageStatus::Unbuilt
}

fn default_container_unbuilt() -> ContainerStatus {
    ContainerStatus::Unbuilt
}

// 镜像状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageStatus {
    Unbuilt,
    Built,
}

impl fmt::Display for ImageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageStatus::Unbuilt => write!(f, "N/A"),
            ImageStatus::Built => write!(f, "Built"),
        }
    }
}

// 容器状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerStatus {
    Up,
    Exited,
    Created,
    Paused,
    Restarting,
    Dead,
    Removing,
    Unbuilt,
}

impl fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainerStatus::Up => write!(f, "Up"),
            ContainerStatus::Exited => write!(f, "Exited"),
            ContainerStatus::Created => write!(f, "Created"),
            ContainerStatus::Paused => write!(f, "Paused"),
            ContainerStatus::Restarting => write!(f, "Restarting"),
            ContainerStatus::Dead => write!(f, "Dead"),
            ContainerStatus::Removing => write!(f, "Removing"),
            ContainerStatus::Unbuilt => write!(f, "N/A"),
        }
    }
}

/// 交互式环境变量类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfKitInteractiveType {
    /// 输入框
    Input,
    /// 单选框
    Radio,
    /// 复选框
    Checkbox,
    /// 确认框
    Confirm,
}

/// 交互式环境变量配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEnvironmentInteractiveConfig {
    /// 环境变量名称
    pub name: String,
    /// 交互方式(input/radio/checkbox/confirm)
    #[serde(rename = "type")]
    pub interactive_type: ConfKitInteractiveType,
    /// 交互提示
    pub prompt: String,
    /// 默认值
    pub default: Option<String>,
    /// 是否必填
    #[serde(default = "default_required")]
    pub required: bool,
    /// 选项列表(仅对radio和checkbox有效)
    pub options: Option<Vec<String>>,
}

/// 默认为必填
fn default_required() -> bool {
    true
}
