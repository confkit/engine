use super::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;
use std::time::Instant;

/// Docker 执行引擎
pub struct DockerExecutor {
    /// Docker 可执行文件路径
    docker_path: String,
}

impl DockerExecutor {
    /// 创建新的 Docker 执行引擎
    pub fn new() -> Self {
        Self { docker_path: "docker".to_string() }
    }

    /// 使用自定义 Docker 路径创建执行引擎
    pub fn with_docker_path(docker_path: impl Into<String>) -> Self {
        Self { docker_path: docker_path.into() }
    }

    /// 构建 docker run 命令
    pub fn build_run_command(&self, execution: &CommandExecution) -> Result<Vec<String>> {
        let mut cmd_args = vec!["run".to_string()];

        // 自动删除容器
        if execution.auto_remove {
            cmd_args.push("--rm".to_string());
        }

        // 交互式模式
        cmd_args.push("-i".to_string());

        // 工作目录
        if let Some(working_dir) = &execution.context.working_dir {
            cmd_args.push("-w".to_string());
            cmd_args.push(working_dir.clone());
        }

        // 环境变量
        for (key, value) in &execution.context.environment {
            cmd_args.push("-e".to_string());
            cmd_args.push(format!("{}={}", key, value));
        }

        // 用户
        if let Some(user) = &execution.context.user {
            cmd_args.push("--user".to_string());
            cmd_args.push(user.clone());
        }

        // 网络模式
        if let Some(network) = &execution.context.network {
            cmd_args.push("--network".to_string());
            cmd_args.push(network.clone());
        }

        // 卷挂载
        for volume in &execution.context.volumes {
            cmd_args.push("-v".to_string());
            let mount_str = if volume.read_only {
                format!("{}:{}:ro", volume.host_path, volume.container_path)
            } else {
                format!("{}:{}", volume.host_path, volume.container_path)
            };
            cmd_args.push(mount_str);
        }

        // 端口映射
        for port in &execution.context.ports {
            cmd_args.push("-p".to_string());
            cmd_args.push(format!("{}:{}", port.host_port, port.container_port));
        }

        // 容器名称
        if let Some(container_name) = &execution.container_name {
            cmd_args.push("--name".to_string());
            cmd_args.push(container_name.clone());
        }

        // 镜像名称
        if let Some(image) = &execution.image {
            cmd_args.push(image.clone());
        } else {
            return Err(anyhow::anyhow!("Docker 执行需要指定镜像"));
        }

        // 执行的命令
        if execution.commands.is_empty() {
            return Err(anyhow::anyhow!("没有指定要执行的命令"));
        }

        // 如果只有一个命令，直接添加
        if execution.commands.len() == 1 {
            // 使用 sh -c 来执行命令，支持管道和重定向
            cmd_args.push("sh".to_string());
            cmd_args.push("-c".to_string());
            cmd_args.push(execution.commands[0].clone());
        } else {
            // 多个命令，组合成一个 shell 脚本
            cmd_args.push("sh".to_string());
            cmd_args.push("-c".to_string());
            let combined_cmd = execution.commands.join(" && ");
            cmd_args.push(combined_cmd);
        }

        Ok(cmd_args)
    }

    /// 构建 docker build 命令
    pub fn build_build_command(&self, params: &ImageBuildParams) -> Result<Vec<String>> {
        let mut cmd_args = vec!["build".to_string()];

        // 镜像标签
        cmd_args.push("-t".to_string());
        cmd_args.push(params.tag.clone());

        // Dockerfile 路径
        cmd_args.push("-f".to_string());
        cmd_args.push(params.dockerfile.clone());

        // 构建参数
        for (key, value) in &params.build_args {
            cmd_args.push("--build-arg".to_string());
            cmd_args.push(format!("{}={}", key, value));
        }

        // 目标平台
        if let Some(platform) = &params.platform {
            cmd_args.push("--platform".to_string());
            cmd_args.push(platform.clone());
        }

        // 无缓存构建
        if params.no_cache {
            cmd_args.push("--no-cache".to_string());
        }

        // 构建上下文
        cmd_args.push(params.context.clone());

        Ok(cmd_args)
    }

    /// 构建 docker pull 命令
    pub fn build_pull_command(&self, image: &str) -> Vec<String> {
        vec!["pull".to_string(), image.to_string()]
    }

    /// 构建 docker rmi 命令
    pub fn build_remove_image_command(&self, params: &ImageOperationParams) -> Vec<String> {
        let mut cmd_args = vec!["rmi".to_string()];

        if params.force {
            cmd_args.push("-f".to_string());
        }

        cmd_args.push(params.image.clone());
        cmd_args.extend(params.extra_args.clone());

        cmd_args
    }

    /// 构建 docker images 命令
    pub fn build_list_images_command(&self) -> Vec<String> {
        vec!["images".to_string(), "--format".to_string(), "{{.Repository}}:{{.Tag}}".to_string()]
    }

    /// 构建 docker inspect 命令
    pub fn build_inspect_command(&self, image: &str) -> Vec<String> {
        vec!["inspect".to_string(), image.to_string()]
    }

    /// 构建完整的 Docker 命令（包含 docker 可执行文件路径）
    pub fn build_full_command(&self, args: &[String]) -> Vec<String> {
        let mut full_cmd = vec![self.docker_path.clone()];
        full_cmd.extend(args.iter().cloned());
        full_cmd
    }

    /// 执行 Docker 命令并返回结果
    async fn execute_docker_command(&self, args: &[String]) -> Result<ExecutionResult> {
        let start_time = Instant::now();
        let full_cmd = self.build_full_command(args);

        tracing::debug!("执行 Docker 命令: {:?}", full_cmd);

        let output = Command::new(&full_cmd[0]).args(&full_cmd[1..]).output()?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        Ok(ExecutionResult { exit_code, stdout, stderr, duration_ms, success })
    }
}

impl Default for DockerExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Executor for DockerExecutor {
    fn executor_type(&self) -> ExecutorType {
        ExecutorType::Docker
    }

    async fn is_available(&self) -> Result<bool> {
        let result = self.execute_docker_command(&["version".to_string()]).await?;
        Ok(result.success)
    }

    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult> {
        let args = self.build_run_command(execution)?;
        self.execute_docker_command(&args).await
    }

    async fn build_image(&self, params: &ImageBuildParams) -> Result<String> {
        let args = self.build_build_command(params)?;
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像构建失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        // 构建成功后获取镜像 ID
        let inspect_args = self.build_inspect_command(&params.tag);
        let inspect_result = self.execute_docker_command(&inspect_args).await?;

        if inspect_result.success {
            // 简单返回镜像标签，实际项目中可以解析 JSON 获取真实的镜像 ID
            Ok(params.tag.clone())
        } else {
            Err(anyhow::anyhow!("无法获取构建后的镜像信息"))
        }
    }

    async fn pull_image(&self, image: &str) -> Result<()> {
        let args = self.build_pull_command(image);
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像拉取失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        Ok(())
    }

    async fn remove_image(&self, params: &ImageOperationParams) -> Result<()> {
        let args = self.build_remove_image_command(params);
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像删除失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        Ok(())
    }

    async fn list_images(&self) -> Result<Vec<String>> {
        let args = self.build_list_images_command();
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像列表获取失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        let images: Vec<String> = result
            .stdout
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.contains("<none>"))
            .map(|line| line.trim().to_string())
            .collect();

        Ok(images)
    }

    async fn image_exists(&self, image: &str) -> Result<bool> {
        let inspect_args = self.build_inspect_command(image);
        let result = self.execute_docker_command(&inspect_args).await?;
        Ok(result.success)
    }
}

/// Docker 命令构建器 - 提供便捷的命令构建方法
pub struct DockerCommandBuilder {
    executor: DockerExecutor,
}

impl DockerCommandBuilder {
    /// 创建新的命令构建器
    pub fn new() -> Self {
        Self { executor: DockerExecutor::new() }
    }

    /// 构建 run 命令字符串
    pub fn run_command(&self, execution: &CommandExecution) -> Result<String> {
        let args = self.executor.build_run_command(execution)?;
        let full_cmd = self.executor.build_full_command(&args);
        Ok(full_cmd.join(" "))
    }

    /// 构建 build 命令字符串
    pub fn build_command(&self, params: &ImageBuildParams) -> Result<String> {
        let args = self.executor.build_build_command(params)?;
        let full_cmd = self.executor.build_full_command(&args);
        Ok(full_cmd.join(" "))
    }

    /// 构建 pull 命令字符串
    pub fn pull_command(&self, image: &str) -> String {
        let args = self.executor.build_pull_command(image);
        let full_cmd = self.executor.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 remove 命令字符串
    pub fn remove_command(&self, params: &ImageOperationParams) -> String {
        let args = self.executor.build_remove_image_command(params);
        let full_cmd = self.executor.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 images 命令字符串
    pub fn list_images_command(&self) -> String {
        let args = self.executor.build_list_images_command();
        let full_cmd = self.executor.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 inspect 命令字符串
    pub fn inspect_command(&self, image: &str) -> String {
        let args = self.executor.build_inspect_command(image);
        let full_cmd = self.executor.build_full_command(&args);
        full_cmd.join(" ")
    }
}

impl Default for DockerCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}
