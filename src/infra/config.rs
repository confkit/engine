//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: Configuration loader with caching

use std::collections::HashMap;
use std::fs;
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

    // ================================================ Projects ================================================

    // 获取项目配置
    pub async fn get_project_config(
        space_name: &str,
        poject_name: &str,
    ) -> Result<Option<ConfKitProjectConfig>> {
        let project_config_list = Self::get_project_config_list(space_name).await?;

        let project_config = project_config_list.iter().find(|project| project.name == poject_name);

        Ok(project_config.cloned())
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

    // 获取项目 enviroment 信息
    pub async fn load_project_env(
        space_name: &str,
        project_name: &str,
    ) -> Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>)> {
        let project_config = Self::get_project_config(space_name, project_name).await?;

        if project_config.is_none() {
            return Ok((HashMap::new(), HashMap::new(), HashMap::new()));
        }
        let project_config = project_config.unwrap();
        let mut env_from_file = HashMap::new();
        let mut env_from_conf = HashMap::new();
        let mut env_mixed = HashMap::new();

        // 环境文件解析
        if let Some(environment_files) = &project_config.environment_files {
            for env_file in environment_files {
                let path = Path::new(&env_file.path);
                let file_content = match fs::read_to_string(path) {
                    Ok(content) => content,
                    Err(e) => {
                        tracing::warn!("Failed to read environment file '{}': {}", env_file.path, e);
                        continue;
                    }
                };

                let parsed = match env_file.format.as_str() {
                    "yaml" => match serde_yaml::from_str::<HashMap<String, String>>(&file_content) {
                        Ok(data) => data,
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse yaml environment file '{}': {}",
                                env_file.path,
                                e
                            );
                            continue;
                        }
                    },
                    "env" => Self::parse_env_file(&file_content),
                    _ => {
                        tracing::warn!(
                            "Unsupported environment file format '{}', skipping",
                            env_file.format
                        );
                        continue;
                    }
                };

                for (key, value) in parsed {
                    env_from_file.insert(key.clone(), value.clone());
                    env_mixed.insert(key, value);
                }
            }
        }

        // 项目环境变量
        if let Some(project_env) = project_config.environment {
            for (key, value) in project_env {
                env_from_conf.insert(key.clone(), value.clone());
                env_mixed.insert(key, value);
            }
        }

        Ok((env_mixed, env_from_conf, env_from_file))
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

    /// 解析 .env 格式文件内容
    ///
    /// 每行格式为 `KEY=VALUE`，忽略空行和 `#` 开头的注释行。
    fn parse_env_file(content: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                if !key.is_empty() {
                    map.insert(key.to_string(), value.to_string());
                }
            }
        }
        map
    }

    /// 验证配置
    fn validate(_config: &ConfKitConfig) -> Result<()> {
        // TODO: 实现配置验证逻辑
        tracing::debug!("验证项目配置 - 待实现");
        Ok(())
    }
}
