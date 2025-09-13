//! Author: xiaoYown
//! Created: 2025-09-13
//! Description: Project configuration types

use super::interactive::ConfKitEnvironmentInteractiveConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ConfKit main configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitConfig {
    pub version: String,
    pub engine: super::engine::Engine,
    pub engine_compose: super::engine::EngineComposeConfig,
    pub spaces: Vec<ConfKitSpaceConfig>,
    pub images: Vec<ConfKitImageConfig>,
    pub shell: Option<super::engine::ShellConfig>,
}

/// ConfKit space configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitSpaceConfig {
    pub name: String,
    pub description: String,
    pub path: String,
}

/// ConfKit project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub source: Option<ConfKitSourceConfig>,
    pub environment_files: Option<Vec<ConfKitEnvironmentFileConfig>>,
    pub environment: Option<HashMap<String, String>>,
    pub environment_from_args: Option<Vec<ConfKitEnvironmentInteractiveConfig>>,
    pub steps: Vec<ConfKitStepConfig>,
    pub cleaner: Option<ConfKitCleanerConfig>,
}

/// ConfKit cleaner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitCleanerConfig {
    pub workspace: Option<bool>,
}

/// ConfKit source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitSourceConfig {
    pub git_repo: String,
    pub git_branch: Option<String>,
    pub git_tag: Option<String>,
    pub git_commit: Option<String>,
    pub local_path: Option<String>,
}

/// ConfKit step configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitStepConfig {
    pub name: String,
    pub container: Option<String>,
    pub working_dir: Option<String>,
    pub commands: Vec<String>,
    pub timeout: Option<u64>,
    pub continue_on_error: Option<bool>,
    pub condition: Option<String>,
}

/// ConfKit environment file configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEnvironmentFileConfig {
    pub format: String,
    pub path: String,
}

/// ConfKit image configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitImageConfig {
    pub name: String,
    pub base_image: String,
    pub tag: String,
    pub context: String,
    pub engine_file: String,
}

/// ConfKit image info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitImageInfo {
    pub name: String,
    pub base_image: String,
    pub tag: String,
    pub context: String,
    pub engine_file: String,
    pub full_name: String,
    pub base_full_name: String,
    pub status: super::engine::ImageStatus,
    pub size: Option<String>,
    pub created_at: Option<String>,
    pub id: Option<String>,
}

/// ConfKit engine compose configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfKitEngineComposeConfig {
    pub project: Option<String>,
    pub file: String,
}
