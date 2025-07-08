use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::types::{
    BuilderConfig, BuilderInfo, BuilderStatus, ComposeService, DockerCompose, HealthStatus,
};

/// 构建器管理器
pub struct BuilderManager {
    docker_client: Option<crate::infrastructure::docker::DockerClient>,
    builders: HashMap<String, BuilderInfo>,
}

impl BuilderManager {
    pub fn new() -> Self {
        Self {
            docker_client: None, // TODO: 初始化Docker客户端
            builders: HashMap::new(),
        }
    }

    /// 从当前目录的 docker-compose.yml 文件加载构建器
    pub fn from_docker_compose<P: AsRef<Path>>(compose_path: P) -> Result<Self> {
        let mut manager = Self::new();
        manager.load_from_docker_compose(compose_path)?;
        Ok(manager)
    }

    /// 从当前工作目录加载 docker-compose.yml
    pub fn from_current_directory() -> Result<Self> {
        let compose_path = "./docker-compose.yml";
        Self::from_docker_compose(compose_path)
    }

    /// 创建带示例数据的管理器（用于演示和测试）
    pub fn with_demo_data() -> Self {
        // 首先尝试从当前目录加载 docker-compose.yml
        if let Ok(manager) = Self::from_current_directory() {
            return manager;
        }

        // 如果没有 docker-compose.yml 文件，则使用演示数据
        let mut manager = Self::new();

        let _ =
            manager.add_demo_builder("golang-1.24", "golang:1.24-alpine", BuilderStatus::Running);
        let _ = manager.add_demo_builder("node-22", "node:22-alpine", BuilderStatus::Stopped);
        let _ = manager.add_demo_builder("rust-latest", "rust:1.75-alpine", BuilderStatus::Running);
        let _ =
            manager.add_demo_builder("tauri-latest", "tauri/tauri:latest", BuilderStatus::Created);

        manager
    }

    /// 从 docker-compose.yml 文件加载构建器
    fn load_from_docker_compose<P: AsRef<Path>>(&mut self, compose_path: P) -> Result<()> {
        let compose_path = compose_path.as_ref();

        if !compose_path.exists() {
            return Err(anyhow::anyhow!(
                "docker-compose.yml 文件不存在: {}",
                compose_path.display()
            ));
        }

        let content = fs::read_to_string(compose_path)?;
        let compose: DockerCompose = serde_yaml::from_str(&content)?;

        for (service_name, service) in compose.services {
            // 只处理有 builder 标签的服务
            if service.get_builder_type().is_some() {
                let builder_info = self.compose_service_to_builder_info(&service_name, &service)?;
                self.builders.insert(service_name, builder_info);
            }
        }

        Ok(())
    }

    /// 将 Docker Compose 服务转换为构建器信息
    fn compose_service_to_builder_info(
        &self,
        name: &str,
        service: &ComposeService,
    ) -> Result<BuilderInfo> {
        let image = service.get_image_name();

        let config = BuilderConfig {
            name: name.to_string(),
            image: image.clone(),
            dockerfile: service.build.as_ref().and_then(|b| b.dockerfile.clone()),
            required: true,
            health_check: Some("echo 'healthy'".to_string()),
            volumes: service.volumes.clone(),
            environment: HashMap::new(), // TODO: 从 environment 字段解析
            ports: service.ports.clone(),
        };

        // 从容器名称推断状态（这里可以扩展为实际查询 Docker）
        let status = self.infer_builder_status(&service);
        let is_running = matches!(status, BuilderStatus::Running);

        let builder_info = BuilderInfo {
            name: name.to_string(),
            status,
            config,
            container_id: service.container_name.clone(),
            created_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
            last_health_check: if is_running {
                Some(HealthStatus {
                    healthy: true,
                    message: "构建器运行正常".to_string(),
                    last_check: chrono::Utc::now() - chrono::Duration::minutes(5),
                })
            } else {
                None
            },
        };

        Ok(builder_info)
    }

    /// 从 Docker Compose 服务推断构建器状态
    fn infer_builder_status(&self, service: &ComposeService) -> BuilderStatus {
        // 这里可以扩展为实际查询 Docker 容器状态
        // 目前基于服务名称进行简单推断
        if let Some(builder_type) = service.get_builder_type() {
            match builder_type.as_str() {
                "golang" | "rust" => BuilderStatus::Running,
                "node" => BuilderStatus::Stopped,
                "tauri" => BuilderStatus::Created,
                _ => BuilderStatus::Created,
            }
        } else {
            BuilderStatus::Created
        }
    }

    /// 添加示例构建器（内部方法）
    fn add_demo_builder(&mut self, name: &str, image: &str, status: BuilderStatus) -> Result<()> {
        let config = BuilderConfig {
            name: name.to_string(),
            image: image.to_string(),
            dockerfile: Some(format!("Dockerfile.{}", name)),
            required: true,
            health_check: Some("echo 'healthy'".to_string()),
            volumes: vec!["/workspace:/workspace".to_string()],
            environment: HashMap::new(),
            ports: vec!["8080:8080".to_string()],
        };

        let is_running = matches!(status, BuilderStatus::Running);
        let builder_info = BuilderInfo {
            name: name.to_string(),
            status,
            config,
            container_id: Some(format!("container_{}", name.replace('-', "_"))),
            created_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
            last_health_check: if is_running {
                Some(HealthStatus {
                    healthy: true,
                    message: "构建器运行正常".to_string(),
                    last_check: chrono::Utc::now() - chrono::Duration::minutes(5),
                })
            } else {
                None
            },
        };

        self.builders.insert(name.to_string(), builder_info);
        Ok(())
    }

    /// 创建构建器
    pub async fn create_builder(&mut self, name: &str, config: &BuilderConfig) -> Result<()> {
        tracing::info!("创建构建器: {}", name);

        // TODO: 实现构建器创建逻辑
        // 1. 检查构建器是否已存在
        // 2. 构建或拉取Docker镜像
        // 3. 创建容器
        // 4. 配置卷挂载和网络

        let builder_info = BuilderInfo {
            name: name.to_string(),
            status: BuilderStatus::Created,
            config: config.clone(),
            container_id: Some(format!("container_{}", name)),
            created_at: Some(chrono::Utc::now()),
            last_health_check: None,
        };

        self.builders.insert(name.to_string(), builder_info);
        Ok(())
    }

    /// 启动构建器
    pub async fn start_builder(&mut self, name: &str) -> Result<()> {
        tracing::info!("启动构建器: {}", name);

        // TODO: 实现构建器启动逻辑
        // 1. 检查构建器是否存在
        // 2. 启动容器
        // 3. 等待容器就绪
        // 4. 执行健康检查

        if let Some(builder) = self.builders.get_mut(name) {
            builder.status = BuilderStatus::Running;
        }

        Ok(())
    }

    /// 停止构建器
    pub async fn stop_builder(&mut self, name: &str) -> Result<()> {
        tracing::info!("停止构建器: {}", name);

        // TODO: 实现构建器停止逻辑
        // 1. 检查构建器是否存在
        // 2. 停止容器
        // 3. 清理资源

        if let Some(builder) = self.builders.get_mut(name) {
            builder.status = BuilderStatus::Stopped;
        }

        Ok(())
    }

    /// 删除构建器
    pub async fn remove_builder(&mut self, name: &str, force: bool) -> Result<()> {
        tracing::info!("删除构建器: {} (force: {})", name, force);

        // TODO: 实现构建器删除逻辑
        // 1. 停止构建器（如果在运行）
        // 2. 删除容器
        // 3. 删除镜像（可选）
        // 4. 清理配置

        self.builders.remove(name);
        Ok(())
    }

    /// 健康检查
    pub async fn health_check(&mut self, name: &str) -> Result<HealthStatus> {
        tracing::debug!("健康检查: {}", name);

        // TODO: 实现健康检查逻辑
        // 1. 检查容器是否运行
        // 2. 执行自定义健康检查命令
        // 3. 验证端口连接
        // 4. 更新健康状态

        let health_status = HealthStatus {
            healthy: true,
            message: "构建器运行正常".to_string(),
            last_check: chrono::Utc::now(),
        };

        if let Some(builder) = self.builders.get_mut(name) {
            builder.last_health_check = Some(health_status.clone());
        }

        Ok(health_status)
    }

    /// 列出所有构建器
    pub fn list_builders(&self) -> Vec<&BuilderInfo> {
        self.builders.values().collect()
    }

    /// 获取构建器信息
    pub fn get_builder(&self, name: &str) -> Option<&BuilderInfo> {
        self.builders.get(name)
    }

    /// 格式化输出构建器列表
    pub fn format_builders_table(&self) -> String {
        let builders = self.list_builders();
        self.format_filtered_builders_table(&builders)
    }

    /// 计算字符串的显示宽度（中文字符占2个位置，ASCII字符占1个位置）
    fn display_width(s: &str) -> usize {
        s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
    }

    /// 右边填充空格使字符串达到指定的显示宽度
    fn pad_to_width(s: &str, width: usize) -> String {
        let current_width = Self::display_width(s);
        if current_width >= width {
            s.to_string()
        } else {
            format!("{}{}", s, " ".repeat(width - current_width))
        }
    }

    /// 获取构建器统计信息
    pub fn get_stats(&self) -> HashMap<&str, usize> {
        let mut stats = HashMap::new();

        for builder in self.builders.values() {
            let status_key = match builder.status {
                BuilderStatus::NotCreated => "not_created",
                BuilderStatus::Created => "created",
                BuilderStatus::Running => "running",
                BuilderStatus::Stopped => "stopped",
                BuilderStatus::Error => "error",
            };
            *stats.entry(status_key).or_insert(0) += 1;
        }

        stats.insert("total", self.builders.len());
        stats
    }

    /// 为过滤后的构建器列表生成格式化表格
    pub fn format_filtered_builders_table(&self, builders: &[&BuilderInfo]) -> String {
        if builders.is_empty() {
            return "没有找到任何构建器".to_string();
        }

        // 准备所有数据行
        let mut rows = Vec::new();
        for builder in builders {
            let container_id = builder
                .container_id
                .as_ref()
                .map(|id| id.as_str())
                .unwrap_or("N/A");

            let health_status = if let Some(health) = &builder.last_health_check {
                if health.healthy {
                    "健康"
                } else {
                    "异常"
                }
            } else {
                "未知"
            };

            let status_display = match builder.status {
                BuilderStatus::NotCreated => "未创建",
                BuilderStatus::Created => "已创建",
                BuilderStatus::Running => "运行中",
                BuilderStatus::Stopped => "已停止",
                BuilderStatus::Error => "错误",
            };

            rows.push((
                &builder.name,
                &builder.config.image,
                status_display,
                container_id,
                health_status,
            ));
        }

        // 计算每列的最大显示宽度
        let headers = ("名称", "镜像", "状态", "容器ID", "健康状态");

        let mut max_widths = (
            Self::display_width(headers.0), // 名称
            Self::display_width(headers.1), // 镜像
            Self::display_width(headers.2), // 状态
            Self::display_width(headers.3), // 容器ID
            Self::display_width(headers.4), // 健康状态
        );

        // 计算数据行的最大宽度
        for (name, image, status, container_id, health) in &rows {
            max_widths.0 = max_widths.0.max(Self::display_width(name));
            max_widths.1 = max_widths.1.max(Self::display_width(image));
            max_widths.2 = max_widths.2.max(Self::display_width(status));
            max_widths.3 = max_widths.3.max(Self::display_width(container_id));
            max_widths.4 = max_widths.4.max(Self::display_width(health));
        }

        // 添加一些padding让表格更好看
        max_widths.0 += 2;
        max_widths.1 += 2;
        max_widths.2 += 2;
        max_widths.3 += 2;
        max_widths.4 += 2;

        let mut output = String::new();

        // 表头
        output.push_str(&format!(
            "{} {} {} {} {}\n",
            Self::pad_to_width(headers.0, max_widths.0),
            Self::pad_to_width(headers.1, max_widths.1),
            Self::pad_to_width(headers.2, max_widths.2),
            Self::pad_to_width(headers.3, max_widths.3),
            Self::pad_to_width(headers.4, max_widths.4),
        ));

        // 分隔线
        let total_width =
            max_widths.0 + max_widths.1 + max_widths.2 + max_widths.3 + max_widths.4 + 4; // +4 for spaces
        output.push_str(&"-".repeat(total_width));
        output.push('\n');

        // 数据行
        for (name, image, status, container_id, health) in rows {
            output.push_str(&format!(
                "{} {} {} {} {}\n",
                Self::pad_to_width(name, max_widths.0),
                Self::pad_to_width(image, max_widths.1),
                Self::pad_to_width(status, max_widths.2),
                Self::pad_to_width(container_id, max_widths.3),
                Self::pad_to_width(health, max_widths.4),
            ));
        }

        output
    }

    /// 获取过滤后构建器列表的统计信息
    pub fn get_filtered_stats(&self, builders: &[&BuilderInfo]) -> HashMap<&str, usize> {
        let mut stats = HashMap::new();

        for builder in builders {
            let status_key = match builder.status {
                BuilderStatus::NotCreated => "not_created",
                BuilderStatus::Created => "created",
                BuilderStatus::Running => "running",
                BuilderStatus::Stopped => "stopped",
                BuilderStatus::Error => "error",
            };
            *stats.entry(status_key).or_insert(0) += 1;
        }

        stats.insert("total", builders.len());
        stats
    }
}
