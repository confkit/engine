//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Execution context implementation

use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};
use tokio::process::Command;

use crate::{
    formatter::path::PathFormatter,
    infra::git::{GitClient, GitInfo},
    shared::constants::{CONTAINER_WORKSPACE_DIR, HOST_WORKSPACE_DIR},
    types::config::ConfKitProjectConfig,
};

/// 执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub task_id: String,
    pub space_name: String,
    pub project_name: String,
    pub project_config: ConfKitProjectConfig,
    pub environment: HashMap<String, String>,
    pub clean_workspace: bool,
    /// 主机工作空间目录
    pub host_workspace_dir: String,
    /// 容器工作空间目录
    pub container_workspace_dir: String,
    /// 日志路径
    pub host_log_path: String,
    /// Git 信息
    pub git_info: Option<GitInfo>,
}

impl ExecutionContext {
    /// 创建新的执行上下文
    pub async fn new(
        task_id: String,
        space_name: String,
        project_name: String,
        project_config: &ConfKitProjectConfig,
    ) -> Result<Self> {
        let task_path_identify = PathFormatter::get_task_path(&space_name, &project_name, &task_id);

        let host_workspace_dir = format!("{HOST_WORKSPACE_DIR}/{task_path_identify}");
        let container_workspace_dir = format!("{CONTAINER_WORKSPACE_DIR}/{task_path_identify}");

        let host_log_path = PathFormatter::get_task_log_path(&space_name, &project_name, &task_id);

        let git_client = GitClient::new(&space_name, &project_name).await?;

        let environment = Self::build_environment(
            &task_id,
            &space_name,
            &project_name,
            &project_config,
            &host_workspace_dir,
            &container_workspace_dir,
            &git_client.git_info,
        );

        let clean_workspace = if let Some(cleaner) = &project_config.cleaner {
            cleaner.workspace.unwrap_or(true)
        } else {
            true
        };

        Ok(Self {
            task_id: task_id.clone(),
            space_name,
            project_name: project_name.clone(),
            project_config: project_config.clone(),
            environment,
            host_workspace_dir,
            container_workspace_dir,
            host_log_path,
            git_info: git_client.git_info,
            clean_workspace,
        })
    }

    pub fn resolve_working_dir(&self, working_dir: &str) -> String {
        let mut result = working_dir.to_string();
        for (key, value) in &self.environment {
            let pattern = format!("${{{key}}}");
            result = result.replace(&pattern, value);
        }

        result
    }
}

impl ExecutionContext {
    /// 构建环境变量
    fn build_environment(
        task_id: &str,
        space_name: &str,
        project_name: &str,
        project_config: &ConfKitProjectConfig,
        host_workspace_dir: &str,
        container_workspace_dir: &str,
        git_info: &Option<GitInfo>,
    ) -> HashMap<String, String> {
        let mut env = HashMap::new();

        // 环境文件解析
        if project_config.environment_files.is_some() {
            let environment_files = project_config.environment_files.as_ref().unwrap();
            for file_path in environment_files {
                // yaml 文件解析
                if file_path.format == "yaml" {
                    let file_path = Path::new(&file_path.path);
                    let file_content = fs::read_to_string(file_path).unwrap();
                    let yaml_data: HashMap<String, String> =
                        serde_yaml::from_str(&file_content).unwrap();
                    for (key, value) in yaml_data {
                        env.insert(key, value);
                    }
                }
                // env 文件解析
                if file_path.format == "env" {
                    let file_path = Path::new(&file_path.path);
                    let file_content = fs::read_to_string(file_path).unwrap();
                    let env_data: HashMap<String, String> =
                        serde_yaml::from_str(&file_content).unwrap();
                    for (key, value) in env_data {
                        env.insert(key, value);
                    }
                }
            }
        }

        // 基础环境变量
        env.insert("TASK_ID".to_string(), task_id.to_string());
        env.insert("PROJECT_NAME".to_string(), project_name.to_string());
        env.insert("SPACE_NAME".to_string(), space_name.to_string());

        // 主机工作空间目录
        env.insert("HOST_WORKSPACE_DIR".to_string(), host_workspace_dir.to_string());
        // 容器工作空间目录
        env.insert("CONTAINER_WORKSPACE_DIR".to_string(), container_workspace_dir.to_string());

        // Git 相关变量
        if let Some(git_info) = git_info {
            env.insert("GIT_REPO".to_string(), git_info.repo_url.clone());
            env.insert("GIT_BRANCH".to_string(), git_info.branch.clone());
            env.insert("GIT_HASH".to_string(), git_info.commit_hash.clone());
            env.insert("GIT_HASH_SHORT".to_string(), git_info.commit_hash_short.clone());
            env.insert("PROJECT_VERSION".to_string(), git_info.project_version.clone());
        }

        // 项目环境变量
        if let Some(project_env) = &project_config.environment {
            for (key, value) in project_env {
                env.insert(key.clone(), value.clone());
            }
        }

        env
    }
}

/// 注入环境变量
pub fn resolve_host_variables(command: &mut Command, environment: &HashMap<String, String>) {
    for (key, value) in environment {
        command.env(key, value);
    }
}

/// 注入容器环境变量
pub fn resolve_container_variables(command: &mut Command, environment: &HashMap<String, String>) {
    for (key, value) in environment {
        command.args(["-e", &format!("{key}={value}")]);
    }
}
