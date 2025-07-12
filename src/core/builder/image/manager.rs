//! 镜像管理器
//!
//! 负责镜像的统一管理，包括构建、检查、列表等功能

use crate::core::builder::types::{BuilderConfig, BuilderInfo};
use anyhow::Result;

use super::{ImageBuilder, ImageCheckResult, ImageInspector};

pub struct ImageManager {
    // 镜像管理器的状态
}

impl ImageManager {
    pub fn new() -> Self {
        Self {}
    }

    /// 构建镜像
    pub async fn build_image(&self, config: &BuilderConfig) -> Result<BuilderInfo> {
        ImageBuilder::build_image(config).await
    }

    /// 检查镜像是否存在
    pub async fn check_image(&self, image_name: &str) -> Result<ImageCheckResult> {
        ImageInspector::check_target_image(image_name).await
    }

    /// 列出镜像
    pub async fn list_images(&self) -> Result<Vec<String>> {
        // TODO: 实现镜像列表功能
        Ok(vec![])
    }

    /// 删除镜像
    pub async fn remove_image(&self, image_name: &str, force: bool) -> Result<()> {
        ImageInspector::remove_image(image_name, force).await
    }

    /// 检查镜像是否存在
    pub async fn image_exists(&self, image_name: &str) -> Result<bool> {
        ImageInspector::image_exists(image_name).await
    }
}
