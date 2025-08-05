//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: ConfKit Engine 统一引擎调用

use std::collections::HashMap;

use crate::engine::docker::DockerEngine;
use crate::engine::podman::PodmanEngine;
use crate::shared::global::ENGINE;
use crate::types::config::{Engine, EngineContainerInfo, EngineImageInfo, EngineServiceConfig};
use anyhow::Result;

mod docker;
mod podman;

pub struct ConfKitEngine;

impl ConfKitEngine {
    // 获取当前宿主机使用的引擎
    pub async fn get_engine() -> Result<Engine> {
        let guard = ENGINE.read().unwrap();
        match &*guard {
            Some(engine) => Ok(engine.clone()),
            None => unreachable!("Engine should be initialized"),
        }
    }

    // 检测当前宿主机是否支持引擎
    pub async fn check_engine(engine: Engine) -> Result<()> {
        match engine {
            Engine::Docker => DockerEngine::check_engine().await,
            Engine::Podman => PodmanEngine::check_engine().await,
        }
    }

    // 设置当前宿主机使用的引擎
    pub async fn set_engine(engine: Engine) -> Result<()> {
        // 检测当前宿主机是否支持引擎
        Self::check_engine(engine.clone()).await?;

        let mut guard = ENGINE.write().unwrap();
        *guard = Some(engine);
        Ok(())
    }

    // ================================================ Image ================================================

    // 检查镜像是否存在
    pub async fn check_image_exists(image: &str, tag: &str) -> Result<bool> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::check_image_exists(image, tag).await,
            Engine::Podman => PodmanEngine::check_image_exists(image, tag).await,
        }
    }

    // 获取镜像信息
    pub async fn get_image_info(image: &str, tag: &str) -> Result<EngineImageInfo> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::get_image_info(image, tag).await,
            Engine::Podman => PodmanEngine::get_image_info(image, tag).await,
        }
    }

    // 拉取远程镜像到本地进行缓存
    pub async fn pull_image(image: &str, tag: &str) -> Result<()> {
        tracing::info!("Pulling image \"{}\"", image);

        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::pull_image(image, tag).await,
            Engine::Podman => PodmanEngine::pull_image(image, tag).await,
        }
    }

    // 构建镜像
    pub async fn build_image(
        name: &str,
        tag: &str,
        dockerfile: &str,
        context: Option<&str>,
    ) -> Result<()> {
        tracing::info!("Building image \"{}\"", name);

        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::build_image(name, tag, dockerfile, context).await,
            Engine::Podman => PodmanEngine::build_image(name, tag, dockerfile, context).await,
        }
    }

    // 移除镜像
    pub async fn remove_image(image: &str, tag: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::remove_image(image, tag).await,
            Engine::Podman => PodmanEngine::remove_image(image, tag).await,
        }
    }

    // ================================================ Container ================================================

    // 检查容器是否存在
    pub async fn check_container_exists(name: &str) -> Result<bool> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::check_container_exists(name).await,
            Engine::Podman => PodmanEngine::check_container_exists(name).await,
        }
    }

    // 创建容器
    pub async fn create_container(name: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::create_container(name).await,
            Engine::Podman => PodmanEngine::create_container(name).await,
        }
    }

    // 移除容器
    pub async fn remove_container(name: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::remove_container(name).await,
            Engine::Podman => PodmanEngine::remove_container(name).await,
        }
    }

    // 启动容器
    pub async fn start_container(name: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::start_container(name).await,
            Engine::Podman => PodmanEngine::start_container(name).await,
        }
    }

    // 停止容器
    pub async fn stop_container(name: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::stop_container(name).await,
            Engine::Podman => PodmanEngine::stop_container(name).await,
        }
    }

    // 重启容器
    pub async fn restart_container(name: &str) -> Result<()> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::restart_container(name).await,
            Engine::Podman => PodmanEngine::restart_container(name).await,
        }
    }

    // 获取容器信息
    pub async fn get_container_info(name: &str) -> Result<EngineContainerInfo> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::get_container_info(name).await,
            Engine::Podman => PodmanEngine::get_container_info(name).await,
        }
    }

    // 在容器中执行命令
    pub async fn execute_in_container(
        container: &str,
        working_dir: &str,
        commands: &[String],
        environment: &HashMap<String, String>,
    ) -> Result<i32> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => {
                DockerEngine::execute_in_container(container, working_dir, commands, environment)
                    .await
            }
            Engine::Podman => {
                PodmanEngine::execute_in_container(container, working_dir, commands, environment)
                    .await
            }
        }
    }

    // ================================================ Docker Compose ================================================

    // 获取 Docker Compose 服务列表
    pub async fn get_compose_services() -> Result<Vec<EngineServiceConfig>> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => DockerEngine::get_compose_services().await,
            Engine::Podman => PodmanEngine::get_compose_services().await,
        }
    }

    // 获取 Docker Compose 服务配置
    pub async fn get_compose_service_config_by_container_name(
        name: &str,
    ) -> Result<Option<EngineServiceConfig>> {
        let engine = Self::get_engine().await?;
        match engine {
            Engine::Docker => {
                DockerEngine::get_compose_service_config_by_container_name(name).await
            }
            Engine::Podman => {
                PodmanEngine::get_compose_service_config_by_container_name(name).await
            }
        }
    }
}
