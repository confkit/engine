//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Image builder implementation

use anyhow::Result;

use crate::{
    engine::engine::ConfKitEngine, formatter::builder_image::BuilderImageFormatter,
    infra::config::ConfKitConfigLoader, types::config::ConfKitImageInfo,
};

pub struct ImageBuilder;

impl ImageBuilder {
    // 获取镜像信息
    pub async fn get_info(name: &str, tag: &str) -> Result<Option<ConfKitImageInfo>> {
        let image_config = ConfKitConfigLoader::get_image_config(name, tag).await?;

        if image_config.is_none() {
            return Ok(None);
        }
        let image_config = image_config.unwrap();

        let image_info = ConfKitEngine::get_image_info(name, tag).await?;

        let confkit_image_info = ConfKitImageInfo {
            name: name.to_string(),
            base_image: image_config.base_image,
            tag: image_config.tag,
            context: image_config.context,
            engine_file: image_config.engine_file,
            status: image_info.status,
            id: Some(image_info.id),
            created_at: Some(image_info.created_at),
            size: Some(image_info.size),
        };
        Ok(Some(confkit_image_info))
    }

    // 获取配置文件镜像 builder 列表
    pub async fn get_list() -> Result<Vec<ConfKitImageInfo>> {
        let config = ConfKitConfigLoader::get_config();
        let mut images: Vec<ConfKitImageInfo> = Vec::new();

        for image in config.images {
            let engine_image_info = Self::get_info(&image.name, &image.tag).await?;

            if engine_image_info.is_none() {
                tracing::error!("Image {} not found", image.name);
                continue;
            }
            images.push(engine_image_info.unwrap());
        }

        Ok(images)
    }

    // 构建镜像
    pub async fn build(name: &str, tag: &str) -> Result<()> {
        // 检查目标镜像是否已经存在
        if ConfKitEngine::check_image_exists(&name, tag).await? {
            tracing::info!("Image {} already exists", name);
            return Ok(());
        }

        let config = ConfKitConfigLoader::get_image_config(name, tag).await?;

        // 检查配置中是否存在镜像
        let config = match config {
            Some(cfg) => cfg,
            None => return Err(anyhow::anyhow!("Image {} not found", name)),
        };

        let is_base_image_exists =
            ConfKitEngine::check_image_exists(&config.base_image, &tag).await?;

        tracing::debug!("Checking base image: {}", config.base_image);

        // 检查基础镜像是否存在, 不存在则拉取
        if !is_base_image_exists {
            tracing::info!("Pulling base image: {}", config.base_image);
            ConfKitEngine::pull_image(&config.base_image, &tag).await?;
        }

        tracing::debug!("Building image: {} with tag: {}", name, tag);

        // 构建镜像
        ConfKitEngine::build_image(&config.name, &config.tag, &config.engine_file, None).await?;

        let engine_image_info = Self::get_info(name, tag).await?;

        BuilderImageFormatter::print_image_info(engine_image_info.as_ref());

        Ok(())
    }

    // 构建所有镜像
    pub async fn build_all() -> Result<()> {
        let config = ConfKitConfigLoader::get_config();
        for image in config.images {
            Self::build(&image.name, &image.tag).await?;
        }
        Ok(())
    }

    // 移除镜像
    pub async fn remove(name: &str, tag: &str) -> Result<()> {
        // TODO: 检查镜像是否被使用，如果被使用，则提示用户是否强制移除
        // TODO: 强制移除, 需要先停止所有使用该镜像的容器, 并删除所有使用该镜像的容器

        ConfKitEngine::remove_image(&name, &tag).await?;

        Ok(())
    }

    // 移除所有镜像
    pub async fn remove_all() -> Result<()> {
        let config = ConfKitConfigLoader::get_config();
        for image in config.images {
            Self::remove(&image.name, &image.tag).await?;
        }
        Ok(())
    }

    pub async fn print_list() -> Result<()> {
        let images = Self::get_list().await?;
        BuilderImageFormatter::print_list(&images);
        Ok(())
    }
}
