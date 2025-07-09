use anyhow::Result;
use std::process::Command;

use super::types::{BuilderConfig, BuilderInfo, BuilderStatus};

/// 镜像构建器
pub struct ImageBuilder;

impl ImageBuilder {
    /// 构建 Docker 镜像
    pub async fn build_image(config: &BuilderConfig) -> Result<BuilderInfo> {
        tracing::info!("开始构建 Docker 镜像: {}", config.name);

        // 1. 验证构建配置
        Self::validate_build_config(config)?;

        // 2. 执行 Docker 构建
        let (image_id, build_logs) = Self::execute_docker_build(config).await?;

        // 3. 创建构建器信息
        let builder_info = BuilderInfo {
            name: config.name.clone(),
            status: BuilderStatus::Created,
            config: config.clone(),
            image_id: Some(image_id),
            created_at: Some(chrono::Utc::now()),
            build_logs: Some(build_logs),
        };

        tracing::info!("Docker 镜像构建成功: {} -> {}", config.name, config.image);
        Ok(builder_info)
    }

    /// 验证构建配置
    fn validate_build_config(config: &BuilderConfig) -> Result<()> {
        // 检查名称是否为空
        if config.name.trim().is_empty() {
            return Err(anyhow::anyhow!("构建器名称不能为空"));
        }

        // 检查 Dockerfile 是否存在
        if !std::path::Path::new(&config.dockerfile).exists() {
            return Err(anyhow::anyhow!("Dockerfile 文件不存在: {}", config.dockerfile));
        }

        // 检查构建上下文是否存在
        if !std::path::Path::new(&config.context).exists() {
            return Err(anyhow::anyhow!("构建上下文目录不存在: {}", config.context));
        }

        tracing::debug!("构建配置验证通过: {}", config.name);
        Ok(())
    }

    /// 执行 Docker 构建
    async fn execute_docker_build(config: &BuilderConfig) -> Result<(String, String)> {
        tracing::info!("执行 Docker 构建: {}", config.image);

        // 构建 docker build 命令
        let mut cmd = Command::new("docker");
        cmd.arg("build").arg("-t").arg(&config.image).arg("-f").arg(&config.dockerfile);

        // 添加构建参数
        for (key, value) in &config.build_args {
            cmd.arg("--build-arg").arg(format!("{}={}", key, value));
        }

        // 添加构建上下文
        cmd.arg(&config.context);

        tracing::debug!("Docker 构建命令: {:?}", cmd);

        // 执行命令
        let output = cmd.output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Docker 构建失败: {}", error_msg));
        }

        // 获取构建日志
        let build_logs = String::from_utf8_lossy(&output.stdout).to_string();

        // 获取镜像 ID
        let image_id = Self::get_image_id(&config.image).await?;

        Ok((image_id, build_logs))
    }

    /// 获取镜像 ID
    async fn get_image_id(image_name: &str) -> Result<String> {
        let output = Command::new("docker").args(&["images", "-q", image_name]).output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("无法获取镜像 ID"));
        }

        let image_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if image_id.is_empty() {
            return Err(anyhow::anyhow!("镜像不存在或构建失败"));
        }

        Ok(image_id)
    }

    /// 检查镜像是否存在
    pub async fn image_exists(image_name: &str) -> Result<bool> {
        let output = Command::new("docker").args(&["images", "-q", image_name]).output()?;

        if !output.status.success() {
            return Ok(false);
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let image_id = output_str.trim();
        Ok(!image_id.is_empty())
    }

    /// 删除镜像
    pub async fn remove_image(image_name: &str, force: bool) -> Result<()> {
        let mut cmd = Command::new("docker");
        cmd.args(&["rmi", image_name]);

        if force {
            cmd.arg("--force");
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("删除镜像失败: {}", error_msg));
        }

        tracing::info!("镜像删除成功: {}", image_name);
        Ok(())
    }
}
