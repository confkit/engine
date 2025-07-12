use super::types::*;
use anyhow::Result;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;

/// Docker 执行引擎
pub struct DockerExecutor {
    /// Docker 可执行文件路径
    docker_path: String,
    /// 命令构建器
    command_builder: DockerCommandBuilder,
}

impl DockerExecutor {
    /// 创建新的 Docker 执行引擎
    pub fn new() -> Self {
        let docker_path = "docker".to_string();
        let command_builder = DockerCommandBuilder::new_with_docker_path(&docker_path);
        Self { docker_path, command_builder }
    }

    /// 使用自定义 Docker 路径创建执行引擎
    pub fn with_docker_path(docker_path: impl Into<String>) -> Self {
        let docker_path = docker_path.into();
        let command_builder = DockerCommandBuilder::new_with_docker_path(&docker_path);
        Self { docker_path, command_builder }
    }

    /// 获取命令构建器的引用（用于调试和命令生成）
    pub fn command_builder(&self) -> &DockerCommandBuilder {
        &self.command_builder
    }

    /// 构建完整的 Docker 命令（包含 docker 可执行文件路径）
    pub fn build_full_command(&self, args: &[String]) -> Vec<String> {
        let mut full_cmd = vec![self.docker_path.clone()];
        full_cmd.extend(args.iter().cloned());
        full_cmd
    }

    /// 执行 Docker 命令并返回结果（支持实时输出）
    async fn execute_docker_command(&self, args: &[String]) -> Result<ExecutionResult> {
        let start_time = Instant::now();
        let full_cmd = self.build_full_command(args);

        tracing::info!("执行 Docker 命令: {}", full_cmd.join(" "));

        // 使用 tokio::process::Command 来支持异步和实时输出
        let mut child = TokioCommand::new(&full_cmd[0])
            .args(&full_cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let mut stdout_lines = stdout_reader.lines();
        let mut stderr_lines = stderr_reader.lines();

        let mut stdout_output = String::new();
        let mut stderr_output = String::new();

        // 实时读取并输出 stdout 和 stderr
        loop {
            tokio::select! {
                line = stdout_lines.next_line() => {
                    match line? {
                        Some(line) => {
                            println!("▶ {}", line);  // 实时输出到控制台
                            stdout_output.push_str(&line);
                            stdout_output.push('\n');
                        }
                        None => break,
                    }
                }
                line = stderr_lines.next_line() => {
                    match line? {
                        Some(line) => {
                            eprintln!("● {}", line);  // 实时输出错误信息到控制台
                            stderr_output.push_str(&line);
                            stderr_output.push('\n');
                        }
                        None => break,
                    }
                }
                else => break,
            }
        }

        // 等待进程结束
        let exit_status = child.wait().await?;
        let duration_ms = start_time.elapsed().as_millis() as u64;

        let exit_code = exit_status.code().unwrap_or(-1);
        let success = exit_status.success();

        if success {
            tracing::info!("Docker 命令执行成功 (耗时: {}ms)", duration_ms);
        } else {
            tracing::error!("Docker 命令执行失败 (退出码: {}, 耗时: {}ms)", exit_code, duration_ms);
        }

        Ok(ExecutionResult {
            exit_code,
            stdout: stdout_output,
            stderr: stderr_output,
            duration_ms,
            success,
        })
    }

    /// 执行 Docker 命令（静默模式，不显示实时输出）
    async fn execute_docker_command_silent(&self, args: &[String]) -> Result<ExecutionResult> {
        let start_time = Instant::now();
        let full_cmd = self.build_full_command(args);

        tracing::debug!("执行 Docker 命令（静默）: {:?}", full_cmd);

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
        let args = vec!["version".to_string()];
        let result = self.execute_docker_command_silent(&args).await?;
        Ok(result.success)
    }

    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult> {
        let args = self.command_builder.build_run_command(execution)?;
        // 容器执行通常需要看到实时输出
        self.execute_docker_command(&args).await
    }

    async fn build_image(&self, params: &ImageBuildParams) -> Result<String> {
        let args = self.command_builder.build_build_command(params)?;

        // 显示即将执行的构建命令
        let build_command = self.command_builder.build_command(params)?;
        println!("※ 开始构建镜像: {}", params.tag);
        println!("※ 构建命令: {}", build_command);
        println!("※ 构建过程:");

        // 镜像构建需要实时输出
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像构建失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        println!("✓ 镜像构建完成: {}", params.tag);

        // 构建成功后获取镜像 ID（静默）
        let inspect_args = self.command_builder.build_inspect_command(&params.tag);
        let inspect_result = self.execute_docker_command_silent(&inspect_args).await?;

        if inspect_result.success {
            // 简单返回镜像标签，实际项目中可以解析 JSON 获取真实的镜像 ID
            Ok(params.tag.clone())
        } else {
            Err(anyhow::anyhow!("无法获取构建后的镜像信息"))
        }
    }

    async fn pull_image(&self, image: &str) -> Result<()> {
        let args = self.command_builder.build_pull_command(image);

        // 显示即将执行的拉取命令
        let pull_command = self.command_builder.pull_command(image);
        println!("※ 开始拉取镜像: {}", image);
        println!("※ 拉取命令: {}", pull_command);
        println!("※ 拉取过程:");

        // 镜像拉取需要实时输出
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像拉取失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        println!("✓ 镜像拉取完成: {}", image);
        Ok(())
    }

    async fn remove_image(&self, params: &ImageOperationParams) -> Result<()> {
        let args = self.command_builder.build_remove_image_command(params);

        // 显示即将执行的删除命令
        let remove_command = self.command_builder.remove_command(params);
        println!("※ 删除镜像: {}", params.image);
        println!("※ 删除命令: {}", remove_command);

        // 镜像删除通常很快，但也可能需要看到过程
        let result = self.execute_docker_command(&args).await?;

        if !result.success {
            return Err(anyhow::anyhow!(
                "Docker 镜像删除失败 (退出码: {}): {}",
                result.exit_code,
                result.stderr
            ));
        }

        println!("✓ 镜像删除完成: {}", params.image);
        Ok(())
    }

    async fn list_images(&self) -> Result<Vec<String>> {
        let args = self.command_builder.build_list_images_command();
        // 列出镜像是快速操作，使用静默模式
        let result = self.execute_docker_command_silent(&args).await?;

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
        let args = self.command_builder.build_inspect_command(image);
        // 检查镜像是否存在是快速操作，使用静默模式
        let result = self.execute_docker_command_silent(&args).await?;
        Ok(result.success)
    }
}

/// Docker 命令构建器 - 提供便捷的命令构建方法
pub struct DockerCommandBuilder {
    docker_path: String,
}

impl DockerCommandBuilder {
    /// 创建新的命令构建器
    pub fn new() -> Self {
        Self { docker_path: "docker".to_string() }
    }

    /// 使用自定义 Docker 路径创建命令构建器
    pub fn new_with_docker_path(docker_path: &str) -> Self {
        Self { docker_path: docker_path.to_string() }
    }

    /// 生成所有支持的命令示例（用于调试和文档）
    pub fn generate_command_examples(&self) -> Result<Vec<(String, String)>> {
        let mut examples = Vec::new();

        // 示例 1: 简单的 run 命令
        let mut simple_execution = CommandExecution::default();
        simple_execution.image = Some("alpine:latest".to_string());
        simple_execution.commands = vec!["echo 'Hello World'".to_string()];
        simple_execution.auto_remove = true;

        examples.push(("简单容器执行".to_string(), self.run_command(&simple_execution)?));

        // 示例 2: 复杂的 run 命令
        let mut complex_execution = CommandExecution::default();
        complex_execution.image = Some("golang:1.21".to_string());
        complex_execution.commands =
            vec!["go mod download".to_string(), "go build -o app".to_string(), "./app".to_string()];
        complex_execution.context.working_dir = Some("/workspace".to_string());
        complex_execution.context.environment.insert("CGO_ENABLED".to_string(), "0".to_string());
        complex_execution.context.environment.insert("GOOS".to_string(), "linux".to_string());
        complex_execution.auto_remove = true;
        complex_execution.container_name = Some("go-build-container".to_string());

        examples.push(("复杂 Go 构建".to_string(), self.run_command(&complex_execution)?));

        // 示例 3: 镜像构建
        let build_params = ImageBuildParams {
            tag: "my-app:v1.0.0".to_string(),
            dockerfile: "Dockerfile".to_string(),
            context: ".".to_string(),
            build_args: {
                let mut args = std::collections::HashMap::new();
                args.insert("VERSION".to_string(), "1.0.0".to_string());
                args.insert("BUILD_DATE".to_string(), "2024-01-01".to_string());
                args
            },
            platform: Some("linux/amd64".to_string()),
            no_cache: false,
        };

        examples.push(("镜像构建".to_string(), self.build_command(&build_params)?));

        // 示例 4: 镜像操作
        examples.push(("镜像拉取".to_string(), self.pull_command("nginx:alpine")));

        let remove_params = ImageOperationParams {
            image: "old-image:v1.0.0".to_string(),
            force: true,
            extra_args: vec!["--no-prune".to_string()],
        };

        examples.push(("强制删除镜像".to_string(), self.remove_command(&remove_params)));

        examples.push(("列出镜像".to_string(), self.list_images_command()));

        examples.push(("检查镜像".to_string(), self.inspect_command("nginx:alpine")));

        Ok(examples)
    }

    /// 构建 docker run 命令参数
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

    /// 构建 docker build 命令参数
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

    /// 构建 docker pull 命令参数
    pub fn build_pull_command(&self, image: &str) -> Vec<String> {
        vec!["pull".to_string(), image.to_string()]
    }

    /// 构建 docker rmi 命令参数
    pub fn build_remove_image_command(&self, params: &ImageOperationParams) -> Vec<String> {
        let mut cmd_args = vec!["rmi".to_string()];

        if params.force {
            cmd_args.push("-f".to_string());
        }

        cmd_args.push(params.image.clone());
        cmd_args.extend(params.extra_args.clone());

        cmd_args
    }

    /// 构建 docker images 命令参数
    pub fn build_list_images_command(&self) -> Vec<String> {
        vec!["images".to_string(), "--format".to_string(), "{{.Repository}}:{{.Tag}}".to_string()]
    }

    /// 构建 docker inspect 命令参数
    pub fn build_inspect_command(&self, image: &str) -> Vec<String> {
        vec!["inspect".to_string(), image.to_string()]
    }

    /// 构建完整的 Docker 命令字符串（用于调试和日志）
    pub fn build_full_command(&self, args: &[String]) -> Vec<String> {
        let mut full_cmd = vec![self.docker_path.clone()];
        full_cmd.extend(args.iter().cloned());
        full_cmd
    }

    /// 构建 run 命令字符串
    pub fn run_command(&self, execution: &CommandExecution) -> Result<String> {
        let args = self.build_run_command(execution)?;
        let full_cmd = self.build_full_command(&args);
        Ok(full_cmd.join(" "))
    }

    /// 构建 build 命令字符串
    pub fn build_command(&self, params: &ImageBuildParams) -> Result<String> {
        let args = self.build_build_command(params)?;
        let full_cmd = self.build_full_command(&args);
        Ok(full_cmd.join(" "))
    }

    /// 构建 pull 命令字符串
    pub fn pull_command(&self, image: &str) -> String {
        let args = self.build_pull_command(image);
        let full_cmd = self.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 remove 命令字符串
    pub fn remove_command(&self, params: &ImageOperationParams) -> String {
        let args = self.build_remove_image_command(params);
        let full_cmd = self.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 images 命令字符串
    pub fn list_images_command(&self) -> String {
        let args = self.build_list_images_command();
        let full_cmd = self.build_full_command(&args);
        full_cmd.join(" ")
    }

    /// 构建 inspect 命令字符串
    pub fn inspect_command(&self, image: &str) -> String {
        let args = self.build_inspect_command(image);
        let full_cmd = self.build_full_command(&args);
        full_cmd.join(" ")
    }
}

impl Default for DockerCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}
