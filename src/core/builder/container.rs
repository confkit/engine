//! Author: xiaoYown
//! Created: 2025-07-16
//! Description: Builder Container Core

use anyhow::Result;

use crate::core::builder::image::ImageBuilder;
use crate::engine::ConfKitEngine;
use crate::formatter::builder_container::BuilderContainerFormatter;
use crate::types::config::{ContainerStatus, EngineContainerInfo};

pub struct ContainerBuilder;

impl ContainerBuilder {
    // 获取容器信息
    pub async fn get_info(name: &str) -> Result<EngineContainerInfo> {
        let container_info = ConfKitEngine::get_container_info(name).await?;
        Ok(container_info)
    }

    // 创建容器
    pub async fn create(name: &str, force: bool) -> Result<()> {
        if force {
            ConfKitEngine::remove_container(name).await?;
        }

        if ConfKitEngine::check_container_exists(name).await? {
            tracing::info!("Container \"{}\" already exists", name);
            return Ok(());
        }

        tracing::info!("Creating container \"{}\"", name);

        // 判断镜像是否存在
        let service_config =
            ConfKitEngine::get_compose_service_config_by_container_name(name).await?;
        let service_config = service_config.unwrap();

        let image_name = service_config.image.split(':').next().unwrap();
        let image_tag = service_config.image.split(':').nth(1).unwrap();

        let image_info = ImageBuilder::get_info(image_name, image_tag).await?;

        if image_info.is_none() {
            tracing::error!("Image \"{}\" not found", service_config.image);
            return Ok(());
        }

        // 尝试构建镜像
        ImageBuilder::build(image_name, image_tag).await?;

        // 创建容器
        ConfKitEngine::create_container(name).await?;

        let container_info = Self::get_info(name).await?;

        BuilderContainerFormatter::print_container_info(Some(&container_info));

        Ok(())
    }

    // 启动容器
    pub async fn start(name: &str) -> Result<()> {
        if !ConfKitEngine::check_container_exists(name).await? {
            Self::create(name, false).await?;
        }

        ConfKitEngine::start_container(name).await?;
        Ok(())
    }

    // 停止容器
    pub async fn stop(name: &str) -> Result<()> {
        if !ConfKitEngine::check_container_exists(name).await? {
            tracing::info!("Container \"{}\" not found", name);
            return Ok(());
        }

        ConfKitEngine::stop_container(name).await?;
        Ok(())
    }

    // 重启容器
    pub async fn restart(name: Option<String>, all: bool) -> Result<()> {
        if all {
            let containers = Self::get_list().await?;

            for container in containers {
                // 如果容器正在重启或者未构建, 则跳过
                let can_restart = !matches!(
                    container.status,
                    ContainerStatus::Restarting | ContainerStatus::Unbuilt
                );

                if can_restart {
                    ConfKitEngine::restart_container(container.name.as_str()).await?;
                }
            }
            return Ok(());
        }

        if name.is_none() {
            tracing::error!("Container name is required");
            return Ok(());
        }

        let container_name = name.unwrap();
        let container_name = container_name.as_str();

        if !ConfKitEngine::check_container_exists(container_name).await? {
            tracing::info!("Container \"{}\" not found", container_name);
            return Ok(());
        }

        ConfKitEngine::restart_container(container_name).await?;
        Ok(())
    }

    // 删除容器
    pub async fn remove(name: &str, force: bool) -> Result<()> {
        if !ConfKitEngine::check_container_exists(name).await? {
            tracing::info!("Container \"{}\" not found", name);
            return Ok(());
        }

        let container_info = Self::get_info(name).await?;

        if !force && container_info.status == ContainerStatus::Up {
            tracing::info!("Container {} is running, use --force to remove", name);
            return Ok(());
        }

        if force && container_info.status == ContainerStatus::Up {
            ConfKitEngine::stop_container(name).await?;
        }

        ConfKitEngine::remove_container(name).await?;

        tracing::info!("Container \"{}\" removed", name);

        Ok(())
    }

    pub async fn get_list() -> Result<Vec<EngineContainerInfo>> {
        let services = ConfKitEngine::get_compose_services().await?;
        let mut containers = vec![];

        for service in services {
            let container_info = match Self::get_info(service.container_name.as_str()).await {
                Ok(info) => info,
                Err(e) => {
                    tracing::error!("Failed to get container info: {}", e);
                    continue;
                }
            };

            containers.push(EngineContainerInfo {
                id: container_info.id.clone(),
                name: container_info.name.clone(),
                image: container_info.image.clone(),
                created_at: container_info.created_at.clone(),
                size: container_info.size.clone(),
                status: container_info.status.clone(),
                working_dir: service.working_dir.clone(),
            });
        }

        containers.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(containers)
    }

    // 创建所有容器
    pub async fn create_all(force: bool) -> Result<()> {
        let services = ConfKitEngine::get_compose_services().await?;

        for service in services {
            Self::create(service.container_name.as_str(), force).await?;
        }

        Ok(())
    }

    // 移除所有容器
    pub async fn remove_all(force: bool) -> Result<()> {
        let services = ConfKitEngine::get_compose_services().await?;

        for service in services {
            Self::remove(service.container_name.as_str(), force).await?;
        }

        Ok(())
    }

    // 启动所有容器
    pub async fn start_all() -> Result<()> {
        let services = ConfKitEngine::get_compose_services().await?;

        for service in services {
            Self::start(service.container_name.as_str()).await?;
        }

        Ok(())
    }

    // 停止所有容器
    pub async fn stop_all() -> Result<()> {
        let services = ConfKitEngine::get_compose_services().await?;

        for service in services {
            Self::stop(service.container_name.as_str()).await?;
        }

        Ok(())
    }

    // 重启所有容器
    pub async fn restart_all(force: bool) -> Result<()> {
        let services = ConfKitEngine::get_compose_services().await?;

        for service in services {
            Self::restart(Some(service.container_name.clone()), force).await?;
        }

        Ok(())
    }

    // 打印容器列表
    pub async fn print_list() -> Result<()> {
        let containers = Self::get_list().await?;
        BuilderContainerFormatter::print_container_list(&containers);
        Ok(())
    }
}
