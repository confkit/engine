//! 镜像管理器
//!
//! 负责镜像的统一管理，包括构建、检查、列表等功能

use crate::core::builder::types::{BuilderConfig, BuilderInfo};
use crate::core::builder::BuilderLoader;
use anyhow::Result;
use std::collections::HashMap;

use super::{ImageBuilder, ImageCheckResult, ImageInspector};

pub struct ImageManager {
    // 镜像管理器的状态
    builders: HashMap<String, BuilderInfo>,
}

impl ImageManager {
    pub fn new() -> Self {
        Self { builders: HashMap::new() }
    }

    /// 从当前目录加载构建器信息
    pub async fn from_current_directory() -> Result<Self> {
        let builders = BuilderLoader::load_builder_infos_from_current_dir().await?;
        Ok(Self { builders })
    }

    /// 从当前目录或默认空状态创建管理器
    pub async fn new_auto() -> Self {
        // 尝试从当前目录加载配置
        Self::from_current_directory().await.unwrap_or_else(|_| Self::new())
    }

    /// 列出所有构建器
    pub fn list_builders(&self) -> Vec<&BuilderInfo> {
        self.builders.values().collect()
    }

    /// 获取构建器信息
    pub fn get_builder(&self, name: &str) -> Option<&BuilderInfo> {
        self.builders.get(name)
    }

    /// 删除构建器（删除镜像）
    pub async fn remove_builder(&mut self, name: &str, force: bool) -> Result<()> {
        tracing::info!("删除构建器: {} (force: {})", name, force);

        let builder =
            self.builders.get(name).ok_or_else(|| anyhow::anyhow!("构建器 '{}' 不存在", name))?;

        // 删除镜像 - 使用完整的镜像名称（包含标签）
        let target_image = format!("{}:{}", builder.config.name, builder.config.tag);
        self.remove_image(&target_image, force).await?;

        // 从管理器中移除
        self.builders.remove(name);

        tracing::info!("构建器 '{}' 删除成功", name);
        Ok(())
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
