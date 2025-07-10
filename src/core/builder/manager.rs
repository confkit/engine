use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use super::{BuilderConfig, BuilderFormatter, BuilderInfo, BuilderLoader, ImageBuilder};

/// 构建器管理器
pub struct BuilderManager {
    builders: HashMap<String, BuilderInfo>,
}

impl BuilderManager {
    pub fn new() -> Self {
        Self { builders: HashMap::new() }
    }

    /// 从当前目录的 docker-compose.yml 文件加载构建器（保留向后兼容）
    pub fn from_docker_compose<P: AsRef<Path>>(compose_path: P) -> Result<Self> {
        let builders = BuilderLoader::load_from_docker_compose(compose_path)?;
        Ok(Self { builders })
    }

    /// 从当前工作目录加载 builder.yml
    pub fn from_current_directory() -> Result<Self> {
        let builders = BuilderLoader::load_builder_infos_from_current_dir()?;
        Ok(Self { builders })
    }

    /// 创建带示例数据的管理器（用于演示和测试）
    pub fn with_demo_data() -> Self {
        // 首先尝试从当前目录加载 builder.yml
        if let Ok(manager) = Self::from_current_directory() {
            return manager;
        }

        // 如果没有 builder.yml 文件，则使用演示数据
        let builders = BuilderLoader::create_demo_builders();
        Self { builders }
    }

    /// 创建新的构建器（构建镜像）
    pub async fn create_builder(&mut self, name: &str, config: &BuilderConfig) -> Result<()> {
        tracing::info!("创建构建器（构建镜像）: {}", name);

        // 1. 检查构建器是否已存在
        if self.builders.contains_key(name) {
            return Err(anyhow::anyhow!("构建器 '{}' 已存在", name));
        }

        // 2. 使用 ImageBuilder 构建镜像
        let builder_info = ImageBuilder::build_image(config).await?;

        // 3. 保存构建器信息
        self.builders.insert(name.to_string(), builder_info);

        tracing::info!("构建器 '{}' 创建成功", name);
        Ok(())
    }

    /// 删除构建器（删除镜像）
    pub async fn remove_builder(&mut self, name: &str, force: bool) -> Result<()> {
        tracing::info!("删除构建器: {} (force: {})", name, force);

        let builder =
            self.builders.get(name).ok_or_else(|| anyhow::anyhow!("构建器 '{}' 不存在", name))?;

        // 删除镜像
        ImageBuilder::remove_image(&builder.config.image, force).await?;

        // 从管理器中移除
        self.builders.remove(name);

        tracing::info!("构建器 '{}' 删除成功", name);
        Ok(())
    }

    /// 列出构建器（带过滤和格式化）
    pub fn list_builders_with_filter(
        &self,
        verbose: bool,
        status_filter: Option<String>,
    ) -> Result<String> {
        let builders: Vec<&BuilderInfo> = self.builders.values().collect();
        let output = BuilderFormatter::format_builders_list(&builders, verbose, status_filter);
        Ok(output)
    }

    /// 列出所有构建器
    pub fn list_builders(&self) -> Vec<&BuilderInfo> {
        self.builders.values().collect()
    }

    /// 获取构建器信息
    pub fn get_builder(&self, name: &str) -> Option<&BuilderInfo> {
        self.builders.get(name)
    }

    /// 检查镜像是否存在
    pub async fn image_exists(&self, name: &str) -> Result<bool> {
        if let Some(builder) = self.builders.get(name) {
            ImageBuilder::image_exists(&builder.config.image).await
        } else {
            Ok(false)
        }
    }
}
