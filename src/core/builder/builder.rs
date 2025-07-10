use anyhow::Result;
use std::io::BufReader;
use std::process::{Command, Stdio};

use super::{
    inspector::ImageInspector,
    output_handler::BuildOutputHandler,
    puller::ImagePuller,
    types::{BuilderConfig, BuilderInfo, BuilderStatus},
    validator::BuildValidator,
};

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

        tracing::info!("Docker 镜像构建成功: {} -> {}", config.name, config.image);
        Ok(builder_info)
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

        // 设置标准输出和错误输出管道
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        tracing::debug!("Docker 构建命令: {:?}", cmd);

        // 启动进程
        let mut child = cmd.spawn()?;

        // 获取标准输出和错误输出
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // 创建输出缓冲区
        let mut build_logs = String::new();

        // 实时读取和显示输出
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // 启动两个任务来并行读取输出
        let (stdout_logs, stderr_logs) = tokio::join!(
            BuildOutputHandler::read_and_display_output(stdout_reader, "OUT"),
            BuildOutputHandler::read_and_display_build_output(stderr_reader)
        );

        // 等待进程完成
        let status = child.wait()?;

        // 合并日志
        build_logs.push_str(&stdout_logs);
        build_logs.push_str(&stderr_logs);

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Docker 构建失败 (退出代码: {})\n{}",
                status.code().unwrap_or(-1),
                stderr_logs
            ));
        }

        // 获取镜像 ID
        let image_id = ImageInspector::get_image_id(&config.image).await?;

        Ok((image_id, build_logs))
    }

    // 为了保持向后兼容性，重新导出一些常用方法

    /// 检查镜像是否存在 (委托给 ImageInspector)
    pub async fn image_exists(image_name: &str) -> Result<bool> {
        ImageInspector::image_exists(image_name).await
    }

    /// 删除镜像 (委托给 ImageInspector)
    pub async fn remove_image(image_name: &str, force: bool) -> Result<()> {
        ImageInspector::remove_image(image_name, force).await
    }
}
