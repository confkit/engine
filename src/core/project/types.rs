//! 项目管理相关类型定义

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 空间配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceConfig {
    pub name: String,
    pub description: String,
    pub projects: Vec<ProjectRef>,
}

/// 项目引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRef {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub config: String,
    pub source: Option<SourceConfig>,
}

/// 源代码配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub git_repo: String,
    pub git_branch: Option<String>,
    pub clone_depth: Option<u32>,
}

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    pub source: Option<SourceConfig>,
    pub environment: Option<HashMap<String, String>>,
    pub steps: Vec<StepConfig>,
    pub step_options: Option<StepOptions>,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub description: String,
}

/// 步骤配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepConfig {
    pub name: String,
    pub container: Option<String>,
    pub working_dir: Option<String>,
    pub commands: Vec<String>,
    pub timeout: Option<String>,
    pub continue_on_error: Option<bool>,
    pub parallel_group: Option<String>,
    // 注意：忽略 depends_on 字段，即使配置文件中存在
}

/// 步骤选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepOptions {
    pub retry: Option<u32>,
    pub timeout: Option<String>,
}

/// 任务执行上下文
#[derive(Debug, Clone)]
pub struct TaskContext {
    pub task_id: String,
    pub space_name: String,
    pub project_name: String,
    pub project_config: ProjectConfig,
    pub space_config: SpaceConfig,
    pub environment: HashMap<String, String>,
    pub workspace_dir: String,
    pub artifacts_dir: String,
    pub started_at: DateTime<Utc>,
}

/// 任务执行状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

/// 步骤执行结果
#[derive(Debug, Clone)]
pub struct StepResult {
    pub step_name: String,
    pub status: TaskStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub output: String,
    pub error: Option<String>,
}

/// 任务执行结果
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub step_results: Vec<StepResult>,
    pub total_duration_ms: Option<u64>,
}

impl TaskContext {
    /// 创建新的任务上下文
    pub fn new(
        task_id: String,
        space_name: String,
        project_name: String,
        project_config: ProjectConfig,
        space_config: SpaceConfig,
    ) -> Self {
        let mut environment = HashMap::new();

        // 使用项目配置文件中的项目名，而不是CLI参数中的项目名
        let actual_project_name = &project_config.project.name;

        // 添加基本环境变量
        environment.insert("TASK_ID".to_string(), task_id.clone());
        environment.insert("PROJECT_NAME".to_string(), actual_project_name.clone());
        environment.insert("SPACE_NAME".to_string(), space_name.clone());

        // 优先级：项目配置 > 空间配置 > 默认值
        // 首先添加项目配置中的环境变量（最高优先级）
        if let Some(project_env) = &project_config.environment {
            environment.extend(project_env.clone());
        }

        // 添加 Git 相关环境变量
        if let Some(source) = &project_config.source {
            environment.insert("GIT_REPO".to_string(), source.git_repo.clone());
            if let Some(branch) = &source.git_branch {
                environment.insert("GIT_BRANCH".to_string(), branch.clone());
            }
        }

        let workspace_dir = "./volumes/workspace".to_string();
        let artifacts_dir = format!("./volumes/artifacts/{}", task_id);

        Self {
            task_id,
            space_name,
            project_name: actual_project_name.clone(), // 使用项目配置中的名称
            project_config,
            space_config,
            environment,
            workspace_dir,
            artifacts_dir,
            started_at: Utc::now(),
        }
    }

    /// 解析环境变量
    pub fn resolve_variables(&self, text: &str) -> String {
        let mut result = text.to_string();

        for (key, value) in &self.environment {
            let pattern = format!("${{{}}}", key);
            result = result.replace(&pattern, value);
        }

        result
    }

    /// 获取步骤的工作目录
    pub fn get_working_dir(&self, step: &StepConfig) -> String {
        if let Some(working_dir) = &step.working_dir {
            self.resolve_variables(working_dir)
        } else {
            self.workspace_dir.clone()
        }
    }

    /// 获取容器内的工作目录
    pub fn get_container_working_dir(&self, step: &StepConfig) -> String {
        if let Some(working_dir) = &step.working_dir {
            self.resolve_variables(working_dir)
        } else {
            // 容器内的工作空间路径
            "/workspace".to_string()
        }
    }
}

impl StepResult {
    /// 创建新的步骤结果
    pub fn new(step_name: String) -> Self {
        Self {
            step_name,
            status: TaskStatus::Pending,
            started_at: Utc::now(),
            finished_at: None,
            exit_code: None,
            output: String::new(),
            error: None,
        }
    }

    /// 标记步骤开始
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Utc::now();
    }

    /// 标记步骤成功完成
    pub fn success(&mut self, exit_code: i32, output: String) {
        self.status = TaskStatus::Success;
        self.finished_at = Some(Utc::now());
        self.exit_code = Some(exit_code);
        self.output = output;
    }

    /// 标记步骤失败
    pub fn failure(&mut self, exit_code: i32, output: String, error: String) {
        self.status = TaskStatus::Failed;
        self.finished_at = Some(Utc::now());
        self.exit_code = Some(exit_code);
        self.output = output;
        self.error = Some(error);
    }

    /// 获取步骤执行时长（毫秒）
    pub fn duration_ms(&self) -> Option<u64> {
        if let Some(finished_at) = self.finished_at {
            let duration = finished_at.signed_duration_since(self.started_at);
            Some(duration.num_milliseconds() as u64)
        } else {
            None
        }
    }
}

impl TaskResult {
    /// 创建新的任务结果
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            status: TaskStatus::Pending,
            started_at: Utc::now(),
            finished_at: None,
            step_results: Vec::new(),
            total_duration_ms: None,
        }
    }

    /// 标记任务开始
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Utc::now();
    }

    /// 标记任务完成
    pub fn finish(&mut self, status: TaskStatus) {
        self.status = status;
        self.finished_at = Some(Utc::now());

        if let Some(finished_at) = self.finished_at {
            let duration = finished_at.signed_duration_since(self.started_at);
            self.total_duration_ms = Some(duration.num_milliseconds() as u64);
        }
    }

    /// 添加步骤结果
    pub fn add_step_result(&mut self, step_result: StepResult) {
        self.step_results.push(step_result);
    }
}
