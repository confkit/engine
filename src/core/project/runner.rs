//! 项目执行引擎
//!
//! 负责执行项目的构建步骤，集成日志记录和容器管理

use super::types::{
    ProjectConfig, SpaceConfig, StepConfig, StepResult, TaskContext, TaskResult, TaskStatus,
};
use crate::core::builder::ContainerManager;
use crate::core::executor::{DockerExecutor, ExecutionResult, Executor};
use crate::core::task::TaskManager;
use crate::infrastructure::logging::LoggingManager;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

/// 项目执行引擎
pub struct ProjectRunner {
    task_manager: TaskManager,
    container_manager: ContainerManager,
    docker_executor: DockerExecutor,
    logging_manager: LoggingManager,
}

impl ProjectRunner {
    /// 创建新的项目执行引擎
    pub async fn new() -> Result<Self> {
        let task_manager = TaskManager::new(4); // 最大并发数，虽然我们是顺序执行
        let container_manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let docker_executor = DockerExecutor::new();
        let logging_manager = LoggingManager::with_default();

        Ok(Self { task_manager, container_manager, docker_executor, logging_manager })
    }

    /// 执行项目
    pub async fn run_project(
        &self,
        space_name: String,
        project_name: String,
        space_config: SpaceConfig,
        project_config: ProjectConfig,
        options: RunOptions,
    ) -> Result<TaskResult> {
        // 使用项目配置文件中的项目名生成任务 ID
        let task_id = TaskManager::generate_task_id();

        // 创建任务上下文
        let mut context = TaskContext::new(
            task_id.clone(),
            space_name,
            project_name,
            project_config,
            space_config,
        );

        // 处理命令行选项
        if let Some(git_branch) = options.git_branch {
            context.environment.insert("GIT_BRANCH".to_string(), git_branch);
        }

        // 初始化任务结果
        let mut task_result = TaskResult::new(task_id.clone());
        task_result.start();

        // 初始化日志
        self.logging_manager.initialize()?;

        // 记录任务开始
        self.log_info(&format!(
            "开始执行项目: {}/{} ({})",
            context.space_name, context.project_name, context.task_id
        ))
        .await?;

        self.log_info("========================================").await?;

        // 预检查模式
        if options.dry_run {
            return self.dry_run_project(&context).await;
        }

        // 创建工作空间和产物目录
        self.prepare_workspace(&context).await?;

        let total_steps = context.project_config.steps.len();

        // 逐步执行所有步骤
        for (index, step) in context.project_config.steps.iter().enumerate() {
            let step_number = index + 1;

            self.log_info(&format!("[{}/{}] {}", step_number, total_steps, step.name)).await?;

            let step_result = self.execute_step(&context, step, step_number).await;

            match &step_result {
                Ok(result) => {
                    if result.status == TaskStatus::Success {
                        self.log_info(&format!(
                            "执行成功 (耗时: {:.1}s)",
                            result.duration_ms().unwrap_or(0) as f64 / 1000.0
                        ))
                        .await?;
                    } else {
                        self.log_error(&format!(
                            "执行失败 (耗时: {:.1}s)",
                            result.duration_ms().unwrap_or(0) as f64 / 1000.0
                        ))
                        .await?;

                        if let Some(error) = &result.error {
                            self.log_error(&format!("错误信息: {}", error)).await?;
                        }
                    }

                    task_result.add_step_result(result.clone());

                    // 如果步骤失败且不允许继续
                    if result.status == TaskStatus::Failed {
                        let continue_on_error = step.continue_on_error.unwrap_or(false);
                        if !continue_on_error {
                            task_result.finish(TaskStatus::Failed);
                            self.log_error("任务执行失败，停止后续步骤").await?;
                            return Ok(task_result);
                        } else {
                            self.log_info("步骤失败但配置为继续执行").await?;
                        }
                    }
                }
                Err(e) => {
                    self.log_error(&format!("步骤执行出错: {}", e)).await?;

                    let mut failed_result = StepResult::new(step.name.clone());
                    failed_result.failure(-1, String::new(), e.to_string());
                    task_result.add_step_result(failed_result);

                    task_result.finish(TaskStatus::Failed);
                    return Ok(task_result);
                }
            }

            self.log_info("").await?; // 空行分隔
        }

        // 任务完成
        task_result.finish(TaskStatus::Success);

        self.log_info(&format!(
            "项目执行完成! 总耗时: {:.1}s",
            task_result.total_duration_ms.unwrap_or(0) as f64 / 1000.0
        ))
        .await?;

        self.log_info(&format!("产物保存在: {}", context.artifacts_dir)).await?;

        Ok(task_result)
    }

    /// 执行单个步骤
    async fn execute_step(
        &self,
        context: &TaskContext,
        step: &StepConfig,
        step_number: usize,
    ) -> Result<StepResult> {
        let mut step_result = StepResult::new(step.name.clone());
        step_result.start();

        let start_time = Instant::now();

        // 记录步骤详情
        if let Some(container) = &step.container {
            self.log_info(&format!("容器: {}", container)).await?;
            let working_dir = context.get_container_working_dir(step);
            self.log_info(&format!("工作目录: {}", working_dir)).await?;
        } else {
            let working_dir = context.get_working_dir(step);
            self.log_info(&format!("工作目录: {}", working_dir)).await?;
        }

        if let Some(timeout) = &step.timeout {
            self.log_info(&format!("超时: {}", timeout)).await?;
        }

        // 执行命令
        let execution_result = if let Some(container_name) = &step.container {
            // 容器内执行
            self.execute_in_container(context, step, container_name).await?
        } else {
            // 本地执行
            self.execute_locally(context, step).await?
        };

        let duration = start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        // 记录执行输出
        if !execution_result.stdout.is_empty() {
            self.log_info("输出:").await?;
            for line in execution_result.stdout.lines() {
                self.log_info(&format!("  {}", line)).await?;
            }
        }

        if !execution_result.stderr.is_empty() {
            self.log_info("错误输出:").await?;
            for line in execution_result.stderr.lines() {
                self.log_info(&format!("  {}", line)).await?;
            }
        }

        // 设置步骤结果
        if execution_result.success {
            step_result.success(execution_result.exit_code, execution_result.stdout);
        } else {
            step_result.failure(
                execution_result.exit_code,
                execution_result.stdout,
                execution_result.stderr,
            );
        }

        Ok(step_result)
    }

    /// 在容器内执行命令
    async fn execute_in_container(
        &self,
        context: &TaskContext,
        step: &StepConfig,
        container_name: &str,
    ) -> Result<ExecutionResult> {
        // 确保容器正在运行
        self.log_info(&format!("检查容器 '{}' 状态", container_name)).await?;

        let containers = self.container_manager.list_builders().await?;
        let container = containers
            .iter()
            .find(|c| c.service_name == container_name)
            .ok_or_else(|| anyhow::anyhow!("容器 '{}' 不存在", container_name))?;

        // 如果容器未运行，尝试启动
        if !matches!(container.status, crate::core::builder::ContainerStatus::Running) {
            self.log_info(&format!("启动容器 '{}'", container_name)).await?;
            self.container_manager.start_builder(container_name).await?;
        }

        // 构建执行命令
        let working_dir = context.get_container_working_dir(step);
        let mut combined_command = String::new();

        // 切换到工作目录（如果指定了的话）
        if !working_dir.is_empty() && working_dir != "." {
            combined_command.push_str(&format!("cd {} && ", working_dir));
        }

        // 添加所有命令
        for (i, cmd) in step.commands.iter().enumerate() {
            let resolved_cmd = context.resolve_variables(cmd);
            if i > 0 {
                combined_command.push_str(" && ");
            }
            combined_command.push_str(&resolved_cmd);
        }

        // 使用 docker exec 执行命令
        let args = vec![
            "exec".to_string(),
            container.container_name.clone(),
            "sh".to_string(),
            "-c".to_string(),
            combined_command,
        ];

        // 使用 docker exec 执行命令
        let full_cmd = self.docker_executor.build_full_command(&args);
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

    /// 在本地执行命令
    async fn execute_locally(
        &self,
        context: &TaskContext,
        step: &StepConfig,
    ) -> Result<ExecutionResult> {
        let working_dir = context.get_working_dir(step);

        // 确保工作目录存在
        if !working_dir.is_empty() && working_dir != "." {
            fs::create_dir_all(&working_dir)?;
        }

        // 逐个执行命令
        let mut combined_output = String::new();
        let mut combined_stderr = String::new();
        let mut last_exit_code = 0;

        for cmd in &step.commands {
            let resolved_cmd = context.resolve_variables(cmd);

            let output = if working_dir.is_empty() || working_dir == "." {
                Command::new("sh").arg("-c").arg(&resolved_cmd).output()?
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(&resolved_cmd)
                    .current_dir(&working_dir)
                    .output()?
            };

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            combined_output.push_str(&stdout);
            combined_stderr.push_str(&stderr);

            last_exit_code = output.status.code().unwrap_or(-1);

            // 如果命令失败，停止执行后续命令
            if !output.status.success() {
                break;
            }
        }

        Ok(ExecutionResult {
            exit_code: last_exit_code,
            stdout: combined_output,
            stderr: combined_stderr,
            duration_ms: 0, // 简化处理
            success: last_exit_code == 0,
        })
    }

    /// 准备工作空间
    async fn prepare_workspace(&self, _context: &TaskContext) -> Result<()> {
        // volumes 目录已经在 main.rs 启动时创建，这里不需要做任何事情
        self.log_info("工作空间准备完成").await?;
        Ok(())
    }

    /// 确保目录权限正确，容器可以访问
    async fn ensure_directory_permissions(&self, dir_path: &str) -> Result<()> {
        let path = std::path::Path::new(dir_path);
        if path.exists() {
            // 跨平台的权限设置方法
            let mut perms = fs::metadata(path)?.permissions();

            // 对于volumes目录及其子目录，设置为可读写
            // 对于其他目录，设置为只读
            let is_volumes_dir = dir_path.contains("volumes");

            // 在Unix系统上设置具体权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = if is_volumes_dir {
                    0o777 // rwxrwxrwx - 全开放权限
                } else {
                    0o755 // rwxr-xr-x - 标准权限
                };
                perms.set_mode(mode);
                let mode_str = if mode == 0o777 { "777" } else { "755" };
                self.log_info(&format!("设置目录权限: {} -> {}", dir_path, mode_str)).await?;
            }

            // 在Windows系统上设置基本权限
            #[cfg(windows)]
            {
                // Windows上设置为可读写
                perms.set_readonly(false);
                let perm_str = if is_volumes_dir { "全开放" } else { "标准" };
                self.log_info(&format!("设置目录权限: {} -> {}", dir_path, perm_str)).await?;
            }

            // 在其他系统上的基本处理
            #[cfg(not(any(unix, windows)))]
            {
                perms.set_readonly(false);
                self.log_info(&format!("设置目录权限: {} -> 可读写", dir_path)).await?;
            }

            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// 预检查模式（干运行）
    async fn dry_run_project(&self, context: &TaskContext) -> Result<TaskResult> {
        let mut task_result = TaskResult::new(context.task_id.clone());
        task_result.start();

        self.log_info("=== 预检查模式 (不会实际执行命令) ===").await?;

        let total_steps = context.project_config.steps.len();

        for (index, step) in context.project_config.steps.iter().enumerate() {
            let step_number = index + 1;

            self.log_info(&format!("[{}/{}] {}", step_number, total_steps, step.name)).await?;

            if let Some(container) = &step.container {
                self.log_info(&format!("  容器: {}", container)).await?;
                let working_dir = context.get_container_working_dir(step);
                self.log_info(&format!("  工作目录: {}", working_dir)).await?;
            } else {
                let working_dir = context.get_working_dir(step);
                self.log_info(&format!("  工作目录: {}", working_dir)).await?;
            }

            self.log_info("  命令:").await?;
            for cmd in &step.commands {
                let resolved_cmd = context.resolve_variables(cmd);
                self.log_info(&format!("    {}", resolved_cmd)).await?;
            }

            // 创建模拟的成功结果
            let mut step_result = StepResult::new(step.name.clone());
            step_result.success(0, "dry-run mode".to_string());
            task_result.add_step_result(step_result);

            self.log_info("").await?;
        }

        task_result.finish(TaskStatus::Success);
        self.log_info("预检查完成").await?;

        Ok(task_result)
    }

    /// 记录信息日志
    async fn log_info(&self, message: &str) -> Result<()> {
        println!("{}", message);
        // TODO: 集成 LoggingManager 的实际日志写入
        Ok(())
    }

    /// 记录错误日志
    async fn log_error(&self, message: &str) -> Result<()> {
        eprintln!("{}", message);
        // TODO: 集成 LoggingManager 的实际日志写入
        Ok(())
    }
}

/// 运行选项
#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    /// 详细输出
    pub verbose: bool,
    /// 预检查模式
    pub dry_run: bool,
    /// Git 分支
    pub git_branch: Option<String>,
    /// 强制执行
    pub force: bool,
}
