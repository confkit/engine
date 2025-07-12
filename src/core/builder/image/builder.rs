use anyhow::Result;
use std::collections::HashMap;

use super::inspector::ImageInspector;
use crate::core::builder::{
    puller::ImagePuller,
    types::{BuilderConfig, BuilderInfo, BuilderStatus},
    validator::BuildValidator,
};
use crate::core::executor::{DockerExecutor, Executor, ImageBuildParams};

/// 镜像构建器
pub struct ImageBuilder;

impl ImageBuilder {
    /// 构建 Docker 镜像
    pub async fn build_image(config: &BuilderConfig) -> Result<BuilderInfo> {
        tracing::info!("开始构建 Docker 镜像: {}", config.name);

        // 1. 验证构建配置
        BuildValidator::validate_build_config(config)?;

        // 2. 检查并拉取基础镜像
        ImagePuller::ensure_base_image_available(&config.base_image).await?;

        // 3. 执行 Docker 构建
        let (image_id, build_logs) = Self::execute_docker_build(config).await?;

        // 4. 创建构建器信息
        let builder_info = BuilderInfo {
            name: config.name.clone(),
            status: BuilderStatus::Created,
            config: config.clone(),
            image_id: Some(image_id),
            created_at: Some(chrono::Utc::now()),
            build_logs: Some(build_logs),
        };

        tracing::info!("Docker 镜像构建成功: {}", config.name);
        Ok(builder_info)
    }

    /// 执行 Docker 构建
    async fn execute_docker_build(config: &BuilderConfig) -> Result<(String, String)> {
        tracing::info!("执行 Docker 构建: {}", config.name);

        // 创建 Docker 执行器
        let executor = DockerExecutor::new();

        // 构建镜像构建参数 - 目标镜像标签与基础镜像标签保持一致
        let target_tag = format!("{}:{}", config.name, config.tag);
        let build_params = ImageBuildParams {
            tag: target_tag,
            dockerfile: config.dockerfile.clone(),
            context: config.context.clone(),
            build_args: config.build_args.clone(),
            platform: None,  // 可以从配置中获取
            no_cache: false, // 可以从配置中获取
        };

        tracing::debug!("Docker 构建参数: {:?}", build_params);

        // 生成命令用于调试和日志记录
        let build_command = executor.command_builder().build_command(&build_params)?;
        tracing::info!("即将执行的 Docker 命令: {}", build_command);

        // 执行镜像构建
        let image_id = executor.build_image(&build_params).await?;

        // 构建成功，生成构建日志
        let build_logs = format!(
            "Docker 镜像构建成功\n执行命令: {}\n镜像标签: {}\n镜像ID: {}\nDockerfile: {}\n构建上下文: {}",
            build_command, config.name, image_id, config.dockerfile, config.context
        );

        Ok((image_id, build_logs))
    }

    // 为了保持向后兼容性，重新导出一些常用方法

    /// 检查镜像是否存在
    pub async fn image_exists(image_name: &str) -> Result<bool> {
        let executor = DockerExecutor::new();
        executor.image_exists(image_name).await
    }

    /// 删除镜像
    pub async fn remove_image(image_name: &str, force: bool) -> Result<()> {
        let executor = DockerExecutor::new();
        let params = crate::core::executor::ImageOperationParams {
            image: image_name.to_string(),
            force,
            extra_args: vec![],
        };
        executor.remove_image(&params).await
    }
}
