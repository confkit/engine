use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::types::{
    BuilderConfig, BuilderInfo, BuilderStatus, ComposeService, DockerCompose, HealthStatus,
};

/// 构建器加载器
pub struct BuilderLoader;

impl BuilderLoader {
    /// 从 docker-compose.yml 文件加载构建器
    pub fn load_from_docker_compose<P: AsRef<Path>>(
        compose_path: P,
    ) -> Result<HashMap<String, BuilderInfo>> {
        let compose_path = compose_path.as_ref();

        if !compose_path.exists() {
            return Err(anyhow::anyhow!(
                "docker-compose.yml 文件不存在: {}",
                compose_path.display()
            ));
        }

        let content = fs::read_to_string(compose_path)?;
        let compose: DockerCompose = serde_yaml::from_str(&content)?;

        let mut builders = HashMap::new();

        for (service_name, service) in compose.services {
            // 只处理有 builder 标签的服务
            if service.get_builder_type().is_some() {
                let builder_info = Self::compose_service_to_builder_info(&service_name, &service)?;
                builders.insert(service_name, builder_info);
            }
        }

        Ok(builders)
    }

    /// 将 Docker Compose 服务转换为构建器信息
    fn compose_service_to_builder_info(
        name: &str,
        service: &ComposeService,
    ) -> Result<BuilderInfo> {
        let image = service.get_image_name();

        // 从 build 配置创建构建器配置
        let config = if let Some(build) = &service.build {
            BuilderConfig {
                name: name.to_string(),
                image: image.clone(),
                dockerfile: build.dockerfile.clone().unwrap_or_else(|| "Dockerfile".to_string()),
                context: build.context.clone().unwrap_or_else(|| ".".to_string()),
                build_args: build.args.clone(),
            }
        } else {
            // 如果没有 build 配置，创建基本配置
            BuilderConfig {
                name: name.to_string(),
                image: image.clone(),
                dockerfile: "Dockerfile".to_string(),
                context: ".".to_string(),
                build_args: HashMap::new(),
            }
        };

        // 推断构建器状态
        let status = Self::infer_builder_status(&service);

        let builder_info = BuilderInfo {
            name: name.to_string(),
            status,
            config,
            image_id: None, // 需要实际查询 Docker 获取
            created_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
            build_logs: None,
        };

        Ok(builder_info)
    }

    /// 推断构建器状态
    fn infer_builder_status(service: &ComposeService) -> BuilderStatus {
        // 如果有容器名称，尝试查询容器状态
        if let Some(container_name) = &service.container_name {
            if let Ok(status) = Self::query_docker_container_status(container_name) {
                return status;
            }
        }

        // 默认状态
        BuilderStatus::NotCreated
    }

    /// 查询 Docker 容器状态
    fn query_docker_container_status(container_name: &str) -> Result<BuilderStatus> {
        use std::process::Command;

        let output = Command::new("docker")
            .args(&[
                "ps",
                "-a",
                "--filter",
                &format!("name={}", container_name),
                "--format",
                "{{.State}}",
            ])
            .output()?;

        if !output.status.success() {
            return Ok(BuilderStatus::NotCreated);
        }

        let status_str = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();

        let status = match status_str.as_str() {
            "running" => BuilderStatus::Running,
            "exited" => BuilderStatus::Stopped,
            "created" => BuilderStatus::Created,
            "paused" => BuilderStatus::Stopped,
            "restarting" => BuilderStatus::Running,
            _ => BuilderStatus::NotCreated,
        };

        tracing::debug!("容器 {} 状态: {} -> {:?}", container_name, status_str, status);
        Ok(status)
    }

    /// 创建演示数据
    pub fn create_demo_builders() -> HashMap<String, BuilderInfo> {
        let mut builders = HashMap::new();

        let demo_configs = vec![
            ("golang-1.24", "golang:1.24-alpine", BuilderStatus::Running),
            ("node-22", "node:22-alpine", BuilderStatus::Stopped),
            ("rust-latest", "rust:1.75-alpine", BuilderStatus::Running),
            ("tauri-latest", "tauri/tauri:latest", BuilderStatus::Created),
        ];

        for (name, image, status) in demo_configs {
            let config = BuilderConfig {
                name: name.to_string(),
                image: image.to_string(),
                dockerfile: format!("Dockerfile.{}", name),
                context: ".".to_string(),
                build_args: HashMap::new(),
            };

            let builder_info = BuilderInfo {
                name: name.to_string(),
                status,
                config,
                image_id: Some(format!("img_{}", name)),
                created_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
                build_logs: Some("Build completed successfully".to_string()),
            };

            builders.insert(name.to_string(), builder_info);
        }

        builders
    }
}
