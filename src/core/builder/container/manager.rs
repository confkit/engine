//! 构建器容器管理器
//!
//! 基于 docker-compose.yml 管理构建器容器的生命周期

use crate::core::builder::types::{ComposeService, DockerCompose};
use crate::core::executor::{DockerExecutor, ExecutionResult, Executor};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// 构建器容器管理器
pub struct ContainerManager {
    compose: DockerCompose,
    docker_executor: DockerExecutor,
}

impl ContainerManager {
    /// 从 docker-compose.yml 文件创建管理器
    pub async fn from_compose_file<P: AsRef<Path>>(compose_path: P) -> Result<Self> {
        let compose_path = compose_path.as_ref();

        if !compose_path.exists() {
            return Err(anyhow::anyhow!(
                "docker-compose.yml 文件不存在: {}",
                compose_path.display()
            ));
        }

        let content = fs::read_to_string(compose_path)?;
        let compose: DockerCompose = serde_yaml::from_str(&content)?;
        let docker_executor = DockerExecutor::new();

        tracing::info!("从 {} 加载了 {} 个服务", compose_path.display(), compose.services.len());

        Ok(Self { compose, docker_executor })
    }

    /// 从当前目录的 docker-compose.yml 创建管理器
    pub async fn from_current_dir() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let compose_path = current_dir.join("docker-compose.yml");
        Self::from_compose_file(compose_path).await
    }

    /// 列出所有可用的服务名称
    pub fn list_service_names(&self) -> Vec<String> {
        self.compose.services.keys().cloned().collect()
    }

    /// 获取指定服务的配置
    pub fn get_service(&self, service_name: &str) -> Result<&ComposeService> {
        self.compose.services.get(service_name).ok_or_else(|| {
            anyhow::anyhow!("服务 '{}' 在 docker-compose.yml 中不存在", service_name)
        })
    }

    /// 创建构建器容器
    pub async fn create_builder(&self, service_name: &str) -> Result<BuilderContainer> {
        let service = self.get_service(service_name)?;

        tracing::info!("创建构建器容器: {}", service_name);

        // 1. 验证服务配置
        self.validate_service(service)?;

        // 2. 检查镜像是否存在
        self.check_image_availability(service).await?;

        // 3. 检查容器是否已存在
        if self.container_exists(service_name).await? {
            return Err(anyhow::anyhow!(
                "容器 '{}' 已存在，使用 --force 强制重新创建",
                service_name
            ));
        }

        // 4. 创建容器
        let container_info = self.create_container(service_name, service).await?;

        tracing::info!("构建器容器创建成功: {}", service_name);
        Ok(container_info)
    }

    /// 强制创建构建器容器（删除已存在的容器）
    pub async fn create_builder_force(&self, service_name: &str) -> Result<BuilderContainer> {
        let service = self.get_service(service_name)?;

        tracing::info!("强制创建构建器容器: {}", service_name);

        // 1. 验证服务配置
        self.validate_service(service)?;

        // 2. 检查镜像是否存在
        self.check_image_availability(service).await?;

        // 3. 如果容器存在，先删除
        if self.container_exists(service_name).await? {
            self.remove_container_internal(service_name, true).await?;
        }

        // 4. 创建容器
        let container_info = self.create_container(service_name, service).await?;

        tracing::info!("构建器容器强制创建成功: {}", service_name);
        Ok(container_info)
    }

    /// 列出所有构建器容器状态
    pub async fn list_builders(&self) -> Result<Vec<BuilderContainer>> {
        let mut builders = Vec::new();

        for service_name in self.list_service_names() {
            let container = self.get_container_info(&service_name).await?;
            builders.push(container);
        }

        Ok(builders)
    }

    /// 启动构建器容器
    pub async fn start_builder(&self, service_name: &str) -> Result<()> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        tracing::info!("启动构建器容器: {}", service_name);

        // 使用 docker start 命令
        let args = vec!["start".to_string(), container_name.clone()];
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!("启动容器失败: {}", result.stderr));
        }

        tracing::info!("构建器容器启动成功: {}", service_name);
        Ok(())
    }

    /// 停止构建器容器
    pub async fn stop_builder(&self, service_name: &str) -> Result<()> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        tracing::info!("停止构建器容器: {}", service_name);

        // 使用 docker stop 命令
        let args = vec!["stop".to_string(), container_name.clone()];
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!("停止容器失败: {}", result.stderr));
        }

        tracing::info!("构建器容器停止成功: {}", service_name);
        Ok(())
    }

    /// 删除构建器容器
    pub async fn remove_builder(&self, service_name: &str, force: bool) -> Result<()> {
        tracing::info!("删除构建器容器: {} (force: {})", service_name, force);
        self.remove_container_internal(service_name, force).await
    }

    /// 获取构建器容器日志
    pub async fn get_builder_logs(
        &self,
        service_name: &str,
        lines: Option<usize>,
    ) -> Result<String> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        let mut args = vec!["logs".to_string()];

        if let Some(lines) = lines {
            args.extend(vec!["--tail".to_string(), lines.to_string()]);
        }

        args.push(container_name);

        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!("获取容器日志失败: {}", result.stderr));
        }

        Ok(result.stdout)
    }

    /// 检查构建器健康状态
    pub async fn check_builder_health(&self, service_name: &str) -> Result<BuilderHealth> {
        let container = self.get_container_info(service_name).await?;

        let healthy = matches!(container.status, ContainerStatus::Running);
        let status = format!("{:?}", container.status);

        Ok(BuilderHealth { healthy, status, last_check: Utc::now() })
    }

    // === 内部辅助方法 ===

    /// 验证服务配置
    fn validate_service(&self, service: &ComposeService) -> Result<()> {
        if service.image.is_none() {
            return Err(anyhow::anyhow!("服务必须指定 image 字段"));
        }
        Ok(())
    }

    /// 检查镜像是否存在
    async fn check_image_availability(&self, service: &ComposeService) -> Result<()> {
        if let Some(image) = &service.image {
            if !self.docker_executor.image_exists(image).await? {
                return Err(anyhow::anyhow!(
                    "镜像 '{}' 不存在，请先运行 'confkit builder image create'",
                    image
                ));
            }
        }
        Ok(())
    }

    /// 检查容器是否存在
    async fn container_exists(&self, service_name: &str) -> Result<bool> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        // 使用 docker inspect 检查容器是否存在
        let args = vec!["inspect".to_string(), container_name];
        let result = self.execute_docker_command(&args).await?;

        Ok(result.success)
    }

    /// 创建容器
    async fn create_container(
        &self,
        service_name: &str,
        service: &ComposeService,
    ) -> Result<BuilderContainer> {
        let args = self.compose_service_to_docker_run(service)?;

        tracing::debug!("Docker 创建命令: docker {}", args.join(" "));

        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!("创建容器失败: {}", result.stderr));
        }

        // 创建成功后获取容器信息
        self.get_container_info(service_name).await
    }

    /// 获取容器信息
    async fn get_container_info(&self, service_name: &str) -> Result<BuilderContainer> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        // 使用 docker inspect 获取容器详细信息
        let args = vec![
            "inspect".to_string(),
            "--format".to_string(),
            "{{.State.Status}}|{{.Created}}".to_string(),
            container_name.clone(),
        ];

        let result = self.execute_docker_command(&args).await?;

        let (status, created_at) = if result.success {
            let output = result.stdout.trim();
            let parts: Vec<&str> = output.split('|').collect();

            let status = if parts.len() > 0 {
                self.parse_container_status(parts[0])
            } else {
                ContainerStatus::NotCreated
            };

            let created_at = if parts.len() > 1 {
                // 简化处理，实际可以解析 Docker 时间格式
                Some(Utc::now())
            } else {
                None
            };

            (status, created_at)
        } else {
            (ContainerStatus::NotCreated, None)
        };

        Ok(BuilderContainer {
            service_name: service_name.to_string(),
            container_name,
            image: service.image.clone().unwrap_or_default(),
            status,
            created_at,
            ports: Vec::new(),   // TODO: 解析端口映射
            volumes: Vec::new(), // TODO: 解析卷挂载
        })
    }

    /// 删除容器（内部方法）
    async fn remove_container_internal(&self, service_name: &str, force: bool) -> Result<()> {
        let service = self.get_service(service_name)?;
        let container_name = self.get_container_name(service);

        let mut args = vec!["rm".to_string()];

        if force {
            args.push("-f".to_string());
        }

        args.push(container_name);

        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!("删除容器失败: {}", result.stderr));
        }

        tracing::info!("构建器容器删除成功: {}", service_name);
        Ok(())
    }

    /// 获取容器名称
    fn get_container_name(&self, service: &ComposeService) -> String {
        service.container_name.clone().unwrap_or_else(|| "unknown".to_string())
    }

    /// 将 ComposeService 转换为 docker run 命令参数
    fn compose_service_to_docker_run(&self, service: &ComposeService) -> Result<Vec<String>> {
        let mut args = vec!["run".to_string(), "-d".to_string()];

        // 容器名称
        if let Some(name) = &service.container_name {
            args.extend(vec!["--name".to_string(), name.clone()]);
        }

        // 重启策略
        if let Some(restart) = &service.restart {
            args.extend(vec!["--restart".to_string(), restart.clone()]);
        }

        // 卷挂载
        for volume in &service.volumes {
            args.extend(vec!["-v".to_string(), volume.clone()]);
        }

        // 端口映射
        for port in &service.ports {
            args.extend(vec!["-p".to_string(), port.clone()]);
        }

        // 工作目录
        if let Some(workdir) = &service.working_dir {
            args.extend(vec!["-w".to_string(), workdir.clone()]);
        }

        // 标签
        for label in &service.labels {
            args.extend(vec!["--label".to_string(), label.clone()]);
        }

        // 镜像名称
        if let Some(image) = &service.image {
            args.push(image.clone());
        } else {
            return Err(anyhow::anyhow!("服务必须指定 image 字段"));
        }

        // 命令
        if let Some(command) = &service.command {
            args.extend(command.split_whitespace().map(|s| s.to_string()));
        }

        Ok(args)
    }

    /// 解析容器状态
    fn parse_container_status(&self, status_str: &str) -> ContainerStatus {
        match status_str.to_lowercase().as_str() {
            "running" => ContainerStatus::Running,
            "exited" => ContainerStatus::Exited,
            "created" => ContainerStatus::Created,
            "paused" => ContainerStatus::Paused,
            "restarting" => ContainerStatus::Restarting,
            "removing" => ContainerStatus::Removing,
            "dead" => ContainerStatus::Dead,
            _ => ContainerStatus::NotCreated,
        }
    }

    /// 执行 Docker 命令（通过 executor 的接口）
    async fn execute_docker_command(&self, args: &[String]) -> Result<ExecutionResult> {
        // 构造完整的命令
        let full_cmd = self.docker_executor.build_full_command(args);

        // 使用 std::process::Command 执行（静默模式）
        let output = Command::new(&full_cmd[0]).args(&full_cmd[1..]).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(ExecutionResult {
            exit_code,
            stdout,
            stderr,
            duration_ms: 0, // 简化处理
            success,
        })
    }
}

/// 构建器容器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderContainer {
    pub service_name: String,
    pub container_name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub ports: Vec<String>,   // 简化为字符串列表
    pub volumes: Vec<String>, // 简化为字符串列表
}

/// 容器状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContainerStatus {
    NotCreated,
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Dead,
    Exited,
}

/// 构建器健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderHealth {
    pub healthy: bool,
    pub status: String,
    pub last_check: DateTime<Utc>,
}
