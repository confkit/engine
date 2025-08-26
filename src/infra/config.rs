//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: Configuration loader with caching

use std::path::Path;

use anyhow::Result;
use tokio::fs::read_to_string;

use crate::shared::constants::CONFKIT_CONFIG_FILE;
use crate::shared::global::CONFIG;
use crate::types::config::{
    ConfKitConfig, ConfKitImageConfig, ConfKitProjectConfig, ConfKitSourceConfig,
    ConfKitSpaceConfig, EngineComposeConfig,
};
use crate::utils::fs::get_yaml_files_in_dir;

pub struct ConfKitConfigLoader;

impl ConfKitConfigLoader {
    // ================================================ Config ================================================

    // 判断配置文件是否存在
    pub async fn is_config_file_exists() -> bool {
        Path::new(CONFKIT_CONFIG_FILE).exists()
    }

    // 设置全局配置文件
    pub async fn set_config() -> Result<()> {
        let config = Self::from_file(CONFKIT_CONFIG_FILE).await?;
        let mut guard = CONFIG.write().unwrap();
        *guard = Some(config);
        Ok(())
    }

    /// 从YAML文件加载配置
    pub async fn from_file(path: &str) -> Result<ConfKitConfig> {
        let content = read_to_string(path).await?;
        let config: ConfKitConfig = serde_yaml::from_str(&content)?;
        Self::validate(&config)?;
        Ok(config)
    }

    /// 获取配置
    pub fn get_config() -> ConfKitConfig {
        let guard = CONFIG.read().unwrap();
        match &*guard {
            Some(config) => config.clone(),
            None => unreachable!("ConfKitConfig should be initialized"),
        }
    }

    // ================================================ Spaces ================================================

    // 获取空间配置
    pub async fn get_project_config(
        space_name: &str,
        poject_name: &str,
    ) -> Result<Option<ConfKitProjectConfig>> {
        let project_config_list = Self::get_project_config_list(space_name).await?;

        let project_config = project_config_list.iter().find(|project| project.name == poject_name);

        Ok(project_config.cloned())
    }

    // 获取空间配置列表
    pub async fn get_space_list() -> Result<Vec<ConfKitSpaceConfig>> {
        let spaces_config = Self::get_config();
        Ok(spaces_config.spaces)
    }

    // 获取 space 配置信息
    pub async fn get_space_config(space_name: &str) -> Result<Option<ConfKitSpaceConfig>> {
        let space_list = Self::get_space_list().await?;
        let space_config = space_list.iter().find(|space| space.name == space_name);
        Ok(space_config.cloned())
    }

    // 获取项目源信息
    pub async fn get_project_source_info(
        space_name: &str,
        project_name: &str,
    ) -> Result<Option<ConfKitSourceConfig>> {
        let project_config = Self::get_project_config(space_name, project_name).await?;

        if project_config.is_none() {
            return Ok(None);
        }

        let project_config = project_config.unwrap();
        Ok(project_config.source)
    }

    // 获取 space 下的所有项目配置
    pub async fn get_project_config_list(space_name: &str) -> Result<Vec<ConfKitProjectConfig>> {
        let space_config = Self::get_space_config(space_name).await?;

        if space_config.is_none() {
            return Ok(vec![]);
        }

        let space_config = space_config.unwrap();

        let yaml_files = get_yaml_files_in_dir(&space_config.path)?;

        let mut project_config_list: Vec<ConfKitProjectConfig> = vec![];

        for file_name in yaml_files {
            let file_path = Path::new(&space_config.path).join(&file_name);
            let project_config = read_to_string(&file_path).await?;

            match serde_yaml::from_str::<ConfKitProjectConfig>(&project_config) {
                Ok(project_config) => project_config_list.push(project_config),
                Err(e) => tracing::warn!("Failed to parse project config: {}", e),
            }
        }

        Ok(project_config_list)
    }

    // ================================================ Docker Compose ================================================

    // 获取 Engine Compose 配置文件
    pub async fn get_engine_compose_config() -> Result<EngineComposeConfig> {
        let config_file_path = Self::get_config().engine_compose.file;
        let config = read_to_string(config_file_path).await?;
        let config: EngineComposeConfig = serde_yaml::from_str(&config)?;

        Ok(config)
    }

    // 获取镜像配置
    pub async fn get_image_config(name: &str, tag: &str) -> Result<Option<ConfKitImageConfig>> {
        let config = Self::get_config();
        let image = config.images.iter().find(|image| image.name == name && image.tag == tag);

        if image.is_none() {
            return Ok(None);
        }

        Ok(image.cloned())
    }

    /// 验证配置
    fn validate(_config: &ConfKitConfig) -> Result<()> {
        // TODO: 实现配置验证逻辑
        tracing::debug!("验证项目配置 - 待实现");
        Ok(())
    }
}
