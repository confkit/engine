use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::types::{
    BuilderConfig, BuilderInfo, BuilderStatus, ComposeService, DockerCompose, HealthStatus,
};
use crate::core::executor::{DockerExecutor, Executor};

/// Builder YAML 配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderYaml {
    pub builders: Vec<BuilderYamlEntry>,
}

/// Builder YAML 配置项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderYamlEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub builder_type: String,
    pub base_image: String,
    pub tag: String,
    pub dockerfile: String,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub build_args: Option<HashMap<String, String>>,
}

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

    /// 从 builder.yml 文件加载构建器配置
    pub fn load_from_builder_yaml<P: AsRef<Path>>(
        yaml_path: P,
    ) -> Result<HashMap<String, BuilderConfig>> {
        let yaml_path = yaml_path.as_ref();

        if !yaml_path.exists() {
            return Err(anyhow::anyhow!("builder.yml 文件不存在: {}", yaml_path.display()));
        }

        let content = fs::read_to_string(yaml_path)?;
        let builder_yaml: BuilderYaml = serde_yaml::from_str(&content)?;

        let mut configs = HashMap::new();

        for entry in builder_yaml.builders {
            let config = Self::yaml_entry_to_builder_config(entry)?;
            configs.insert(config.name.clone(), config);
        }

        tracing::info!("从 builder.yml 加载了 {} 个构建器配置", configs.len());
        Ok(configs)
    }

    /// 从当前目录查找并加载 builder.yml
    pub fn load_from_current_dir() -> Result<HashMap<String, BuilderConfig>> {
        let current_dir = std::env::current_dir()?;
        let yaml_path = current_dir.join("builder.yml");
        Self::load_from_builder_yaml(yaml_path)
    }

    /// 将 YAML 配置项转换为构建器配置
    fn yaml_entry_to_builder_config(entry: BuilderYamlEntry) -> Result<BuilderConfig> {
        let context = entry.context.unwrap_or_else(|| ".".to_string());
        let build_args = entry.build_args.unwrap_or_default();

        // 组合基础镜像名称（用于拉取）
        let base_image = format!("{}:{}", entry.base_image, entry.tag);

        let config = BuilderConfig {
            name: entry.name, // name 字段就是构建器名称，不拼接标签
            base_image,
            tag: entry.tag, // 保存原始标签信息
            dockerfile: entry.dockerfile,
            context,
            build_args,
        };

        Ok(config)
    }

    /// 根据名称查找构建器配置
    pub fn find_builder_config(name: &str) -> Result<BuilderConfig> {
        let configs = Self::load_from_current_dir()?;

        if let Some(config) = configs.get(name) {
            Ok(config.clone())
        } else {
            let available_names: Vec<String> = configs.keys().cloned().collect();
            Err(anyhow::anyhow!(
                "未找到名为 '{}' 的构建器配置\n可用的构建器: {}",
                name,
                available_names.join(", ")
            ))
        }
    }

    /// 从 builder.yml 文件加载构建器信息（用于 list 命令）
    pub async fn load_builder_infos_from_current_dir() -> Result<HashMap<String, BuilderInfo>> {
        let configs = Self::load_from_current_dir()?;
        let mut builder_infos = HashMap::new();

        for (name, config) in configs {
            let builder_info = Self::config_to_builder_info(config).await?;
            builder_infos.insert(name, builder_info);
        }

        tracing::info!("从 builder.yml 加载了 {} 个构建器信息", builder_infos.len());
        Ok(builder_infos)
    }

    /// 将 BuilderConfig 转换为 BuilderInfo（用于列表显示）
    async fn config_to_builder_info(config: BuilderConfig) -> Result<BuilderInfo> {
        // 检查镜像是否存在来推断状态 - 使用完整的镜像名称（包含标签）
        let target_image = format!("{}:{}", config.name, config.tag);
        let status = match Self::check_image_status(&target_image).await {
            Ok(true) => BuilderStatus::Created,
            Ok(false) => BuilderStatus::NotCreated,
            Err(_) => BuilderStatus::NotCreated,
        };

        // 如果镜像存在，尝试获取镜像ID
        let image_id = if matches!(status, BuilderStatus::Created) {
            Self::get_image_id(&target_image).await.ok()
        } else {
            None
        };

        let builder_info = BuilderInfo {
            name: config.name.clone(),
            status,
            config,
            image_id,
            created_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)), // 模拟创建时间
            build_logs: None,
        };

        Ok(builder_info)
    }

    /// 检查镜像是否存在
    async fn check_image_status(image_name: &str) -> Result<bool> {
        let executor = DockerExecutor::new();
        executor.image_exists(image_name).await
    }

    /// 获取镜像ID
    async fn get_image_id(image_name: &str) -> Result<String> {
        // 通过检查镜像是否存在来获取ID
        let executor = DockerExecutor::new();
        if executor.image_exists(image_name).await? {
            // 简化处理：如果镜像存在，返回镜像名作为ID
            // 在实际应用中，可以通过 docker inspect 获取真实的镜像ID
            Ok(format!("img_{}", image_name.replace(":", "_")))
        } else {
            Err(anyhow::anyhow!("镜像不存在"))
        }
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
                name: name.to_string(), // name 字段就是构建器名称
                base_image: service.image.clone().unwrap_or_else(|| "alpine:latest".to_string()),
                tag: "latest".to_string(), // Docker Compose 默认使用 latest 标签
                dockerfile: build.dockerfile.clone().unwrap_or_else(|| "Dockerfile".to_string()),
                context: build.context.clone().unwrap_or_else(|| ".".to_string()),
                build_args: build.args.clone(),
            }
        } else {
            // 如果没有 build 配置，创建基本配置
            BuilderConfig {
                name: name.to_string(), // name 字段就是构建器名称
                base_image: service.image.clone().unwrap_or_else(|| "alpine:latest".to_string()),
                tag: "latest".to_string(), // Docker Compose 默认使用 latest 标签
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
}
