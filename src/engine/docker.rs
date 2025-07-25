//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Docker engine implementation

use anyhow::Result;
use std::process::Command;

use crate::{
    infra::config::ConfKitConfigLoader,
    types::config::{
        ContainerStatus, EngineContainerInfo, EngineImageInfo, EngineServiceConfig, ImageStatus,
    },
    utils::command::CommandUtil,
};

pub struct DockerEngine;

impl DockerEngine {
    // ================================================ Engine Basic ================================================

    // 检测当前宿主机是否安装了 Docker
    pub async fn check_engine() -> Result<()> {
        // 检测当前宿主机是否安装了 Docker
        let output = Command::new("docker").arg("--version").output();
        if output.is_err() {
            return Err(anyhow::anyhow!("Docker not installed"));
        }

        Ok(())
    }

    // ================================================ Image ================================================

    // 检查镜像是否存在
    pub async fn check_image_exists(image: &str, tag: &str) -> Result<bool> {
        let output = Command::new("docker")
            .arg("images")
            .arg("-q")
            .arg(format!("{}:{}", image, tag))
            .output()?;

        // 检查输出是否为空，空输出表示镜像不存在
        let output_str = String::from_utf8_lossy(&output.stdout);

        match output_str.trim() {
            "" => Ok(false),
            _ => Ok(true),
        }
    }

    // 获取镜像信息
    pub async fn get_image_info(image: &str, tag: &str) -> Result<EngineImageInfo> {
        let output = Command::new("docker")
            .arg("images")
            .arg("--format")
            .arg("{{.ID}}\t{{.Tag}}\t{{.CreatedAt}}\t{{.Size}}")
            .arg(format!("{}:{}", image, tag))
            .output()?;

        if !output.status.success() {
            return Ok(EngineImageInfo {
                id: "".to_string(),
                name: image.to_string(),
                tag: tag.to_string(),
                created_at: "".to_string(),
                size: "".to_string(),
                status: ImageStatus::Unbuilt,
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let line = output_str.trim();

        if line.is_empty() {
            return Ok(EngineImageInfo {
                id: "".to_string(),
                name: image.to_string(),
                tag: tag.to_string(),
                created_at: "".to_string(),
                size: "".to_string(),
                status: ImageStatus::Unbuilt,
            });
        }

        let parts: Vec<&str> = line.split('\t').collect();

        Ok(EngineImageInfo {
            id: parts[0].to_string(),
            name: image.to_string(),
            tag: tag.to_string(),
            created_at: parts[2].to_string(),
            size: parts[3].to_string(),
            status: ImageStatus::Built,
        })
    }

    // 拉取远程镜像到本地进行缓存
    pub async fn pull_image(image: &str, tag: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("pull").arg(format!("{}:{}", image, tag));

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| {
                // 根据内容判断是进度还是错误
                if line.contains("ERROR") || line.contains("FAILED") {
                    tracing::error!("✗ {}", line);
                } else {
                    tracing::info!("● {}", line);
                }
            })),
        )
        .await?;

        Ok(())
    }

    // 构建镜像
    pub async fn build_image(
        name: &str,
        tag: &str,
        dockerfile: &str,
        context: Option<&str>,
    ) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command
            .arg("build")
            .arg("-t")
            .arg(format!("{}:{}", name, tag))
            .arg("-f")
            .arg(dockerfile)
            .arg(context.unwrap_or("."));

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| {
                // 根据内容判断是进度还是错误
                if line.contains("ERROR") || line.contains("FAILED") {
                    tracing::error!("✗ {}", line);
                } else {
                    tracing::info!("● {}", line);
                }
            })),
        )
        .await?;

        Ok(())
    }

    // 移除镜像
    pub async fn remove_image(image: &str, tag: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("rmi").arg(format!("{}:{}", image, tag));

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::error!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // ================================================ Container ================================================

    // 检查容器是否存在
    pub async fn check_container_exists(name: &str) -> Result<bool> {
        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .arg("--filter")
            .arg(format!("name=^{}$", name))
            .arg("--quiet")
            .output()?;

        // 如果找到容器，输出不为空；如果没找到，输出为空
        Ok(!output.stdout.is_empty())
    }

    // 创建容器
    pub async fn create_container(name: &str) -> Result<()> {
        let confkit_config = ConfKitConfigLoader::get_config();
        let engine_compose_file_path = confkit_config.engine_compose.file;
        let project_name = confkit_config.engine_compose.project;

        let mut command = tokio::process::Command::new("docker");

        command
            .arg("compose")
            .arg("-p")
            .arg(project_name)
            .arg("-f")
            .arg(engine_compose_file_path.as_str())
            .arg("create")
            .arg(name);

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::info!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // 移除容器
    pub async fn remove_container(name: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("rm").arg(name);

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::info!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // 启动容器
    pub async fn start_container(name: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("start").arg(name);

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::error!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // 停止容器
    pub async fn stop_container(name: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("stop").arg(name);

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::error!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // 重启容器
    pub async fn restart_container(name: &str) -> Result<()> {
        let mut command = tokio::process::Command::new("docker");
        command.arg("restart").arg(name);

        tracing::info!(" --------- Restarting container: {} ---------", name);

        CommandUtil::execute_command_with_output(
            &mut command,
            Some(Box::new(|line| tracing::info!("{}", line))),
            Some(Box::new(|line| tracing::error!("{}", line))),
        )
        .await?;

        Ok(())
    }

    // 获取容器信息
    pub async fn get_container_info(name: &str) -> Result<EngineContainerInfo> {
        // 检查容器是否存在
        if !Self::check_container_exists(name).await? {
            return Ok(EngineContainerInfo {
                id: "".to_string(),
                name: name.to_string(),
                image: "".to_string(),
                created_at: "".to_string(),
                size: "".to_string(),
                working_dir: None,
                status: ContainerStatus::Unbuilt,
            });
        }

        let service_config = Self::get_compose_service_config_by_container_name(name).await?;
        let service_config = service_config.unwrap();

        tracing::debug!("name: {}, image: {}", name, service_config.image);

        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .arg("--filter")
            .arg(format!("name={}", name))
            .arg("--format")
            .arg("{{.ID}}\t{{.Image}}\t{{.Status}}\t{{.CreatedAt}}\t{{.Size}}")
            .output()?;

        if !output.status.success() {
            return Ok(EngineContainerInfo {
                id: "".to_string(),
                name: name.to_string(),
                image: service_config.image,
                created_at: "".to_string(),
                size: "".to_string(),
                working_dir: service_config.working_dir,
                status: ContainerStatus::Unbuilt,
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.split('\n').collect();

        if lines.is_empty() {
            return Ok(EngineContainerInfo {
                id: "".to_string(),
                name: name.to_string(),
                image: service_config.image,
                created_at: "".to_string(),
                size: "".to_string(),
                working_dir: service_config.working_dir,
                status: ContainerStatus::Unbuilt,
            });
        }

        let line = lines[0];
        let mut parts: Vec<&str> = line.split('\t').collect();

        // line 长度补全
        while parts.clone().len() < 5 {
            parts.push("");
        }

        let status = match parts[2] {
            is_up if is_up.contains("Up") => ContainerStatus::Up,
            is_exited if is_exited.contains("Exited") => ContainerStatus::Exited,
            is_created if is_created.contains("Created") => ContainerStatus::Created,
            is_paused if is_paused.contains("Paused") => ContainerStatus::Paused,
            is_restarting if is_restarting.contains("Restarting") => ContainerStatus::Restarting,
            is_dead if is_dead.contains("Dead") => ContainerStatus::Dead,
            is_removing if is_removing.contains("Removing") => ContainerStatus::Removing,
            _ => ContainerStatus::Unbuilt,
        };

        let image = match parts[1] {
            img if img.is_empty() => service_config.image,
            img => img.to_string(),
        };

        Ok(EngineContainerInfo {
            id: parts[0].to_string(),
            name: name.to_string(),
            image,
            created_at: parts[3].to_string(),
            size: parts[4].to_string(),
            working_dir: service_config.working_dir,
            status,
        })
    }

    // ================================================ Docker Compose ================================================

    // 获取 Docker Compose 服务列表
    pub async fn get_compose_services() -> Result<Vec<EngineServiceConfig>> {
        let config = ConfKitConfigLoader::get_engine_compose_config().await?;
        let mut services = vec![];

        for (service_name, value) in config.services {
            services.push(EngineServiceConfig {
                service_name,
                container_name: value.container_name,
                image: value.image,
                working_dir: value.working_dir,
                ports: value.ports,
                environment: value.environment,
                volumes: value.volumes,
                depends_on: value.depends_on,
                other: value.other,
            });
        }

        Ok(services)
    }

    pub async fn get_compose_service_config_by_container_name(
        name: &str,
    ) -> Result<Option<EngineServiceConfig>> {
        let services = Self::get_compose_services().await?;

        for service in services {
            if service.container_name == name {
                return Ok(Some(service));
            }
        }

        Ok(None)
    }
}
