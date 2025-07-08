use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    pub source: SourceConfig,
    pub environment: HashMap<String, String>,
    pub steps: Vec<StepConfig>,
    pub step_options: Option<StepOptions>,
    pub notifications: Option<NotificationConfig>,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

/// 源码配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub git_repo: String,
    pub git_branch: String,
    pub git_tag: Option<String>,
    pub clone_depth: Option<u32>,
}

/// 步骤配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepConfig {
    pub name: String,
    pub container: Option<String>,
    pub working_dir: Option<String>,
    pub commands: Vec<String>,
    pub depends_on: Option<Vec<String>>,
    pub parallel_group: Option<String>,
    pub retry: Option<u32>,
    pub timeout: Option<String>,
    pub continue_on_error: Option<bool>,
}

/// 步骤默认选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepOptions {
    pub retry: Option<u32>,
    pub timeout: Option<String>,
    pub continue_on_error: Option<bool>,
    pub parallel: Option<bool>,
    pub shell: Option<String>,
}

/// 通知配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub on_success: Option<Vec<NotificationTarget>>,
    pub on_failure: Option<Vec<NotificationTarget>>,
}

/// 通知目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTarget {
    #[serde(rename = "type")]
    pub notification_type: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub payload: Option<String>,
}
