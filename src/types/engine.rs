//! Author: xiaoYown
//! Created: 2025-09-13  
//! Description: Engine infrastructure types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Engine image info
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

/// Engine container info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub created_at: String,
    pub size: String,
    #[serde(default = "default_container_unbuilt")]
    pub status: ContainerStatus,
}

/// Engine compose config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineComposeConfig {
    pub project: Option<String>,
    pub file: String,
    #[serde(default)]
    pub services: Vec<EngineServiceConfig>,
}

/// Engine service config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineServiceConfig {
    pub name: String,
    pub image: String,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub depends_on: Option<Vec<String>>,
    pub restart: Option<String>,
    pub command: Option<String>,
    pub working_dir: Option<String>,
    pub user: Option<String>,
    pub privileged: Option<bool>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub security_opt: Option<Vec<String>>,
    pub devices: Option<Vec<String>>,
    pub tmpfs: Option<Vec<String>>,
    pub shm_size: Option<String>,
    pub stdin_open: Option<bool>,
    pub tty: Option<bool>,
    pub labels: Option<Vec<String>>,
}

/// Engine type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Docker,
    Podman,
}

/// Image status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStatus {
    Built,
    Unbuilt,
    Building,
    Failed,
    Unknown,
}

/// Container status  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Running,
    Stopped,
    Created,
    Restarting,
    Paused,
    Exited,
    Dead,
    Unknown,
}

/// Shell config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub host: String,
    pub container: String,
}

// Default functions
fn default_image_unbuilt() -> ImageStatus {
    ImageStatus::Unbuilt
}

fn default_container_unbuilt() -> ContainerStatus {
    ContainerStatus::Unknown
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Engine::Docker => write!(f, "docker"),
            Engine::Podman => write!(f, "podman"),
        }
    }
}

impl fmt::Display for ImageStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageStatus::Built => write!(f, "built"),
            ImageStatus::Unbuilt => write!(f, "unbuilt"),
            ImageStatus::Building => write!(f, "building"),
            ImageStatus::Failed => write!(f, "failed"),
            ImageStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerStatus::Running => write!(f, "running"),
            ContainerStatus::Stopped => write!(f, "stopped"),
            ContainerStatus::Created => write!(f, "created"),
            ContainerStatus::Restarting => write!(f, "restarting"),
            ContainerStatus::Paused => write!(f, "paused"),
            ContainerStatus::Exited => write!(f, "exited"),
            ContainerStatus::Dead => write!(f, "dead"),
            ContainerStatus::Unknown => write!(f, "unknown"),
        }
    }
}
