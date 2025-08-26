//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Git client implementation

use anyhow::Result;
use std::{fs, process::Command};
use uuid::Uuid;

use crate::{
    infra::config::ConfKitConfigLoader, shared::constants::HOST_CACHE_DIR,
    types::config::ConfKitSourceConfig,
};

#[derive(Debug, Clone)]
pub struct SourceHash {
    pub commit_hash: String,
    pub commit_hash_short: String,
}

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub repo_url: String,
    pub branch: String,
    pub commit_hash: String,
    pub commit_hash_short: String,
    pub project_version: String,
}

pub struct GitClient {
    pub git_info: Option<GitInfo>,
}

impl GitClient {
    // TODO: 后期优化
    pub async fn new(space_name: &str, project_name: &str) -> Result<Self> {
        let git_info = Self::get_envs(space_name, project_name).await?;

        Ok(Self { git_info })
    }

    // 从仓库直接获取相关环境变量
    pub async fn get_envs(space_name: &str, project_name: &str) -> Result<Option<GitInfo>> {
        let source = ConfKitConfigLoader::get_project_source_info(space_name, project_name).await?;

        let source = match source {
            Some(source) => source,
            None => {
                tracing::info!("Project source not found for project: {}", project_name);
                return Ok(None);
            }
        };

        let source_hash = match Self::get_source_hash(&source.git_repo, &source.git_branch).await {
            Ok(source_hash) => source_hash,
            Err(e) => {
                tracing::error!("Failed to get source hash: {}", e);
                return Err(e);
            }
        };

        let project_version = match Self::get_source_project_version(&source).await {
            Ok(project_version) => project_version,
            Err(e) => {
                tracing::warn!("Failed to get source project version: {}", e);
                "".to_string()
            }
        };

        let git_info = GitInfo {
            repo_url: source.git_repo.clone(),
            branch: source.git_branch.clone(),
            commit_hash: source_hash.commit_hash,
            commit_hash_short: source_hash.commit_hash_short,
            project_version,
        };

        Ok(Some(git_info))
    }

    // 获取远程仓库的 commit hash 和 commit hash 短版本
    async fn get_source_hash(git_repo: &str, git_branch: &str) -> Result<SourceHash> {
        tracing::debug!("Getting source hash for git repo: {} branch: {}", git_repo, git_branch);

        let source_hash = Command::new("git").args(["ls-remote", git_repo, git_branch]).output()?;

        if !source_hash.status.success() {
            tracing::error!(
                "Failed to get source hash for git repo: {} branch: {}",
                git_repo,
                git_branch
            );
            return Err(anyhow::anyhow!("Failed to get source hash"));
        }

        tracing::debug!("Source hash: {}", String::from_utf8_lossy(&source_hash.stdout));

        let output = String::from_utf8_lossy(&source_hash.stdout);
        let first_line = output.lines().next().unwrap();
        let commit_hash = first_line.split_whitespace().next().unwrap().to_string();

        let commit_hash_short = commit_hash[..8].to_string();

        Ok(SourceHash { commit_hash, commit_hash_short })
    }

    // 获取远程仓库指定文件内容
    async fn get_source_file_content(
        git_repo: &str,
        git_branch: &str,
        file_path: &str,
    ) -> Result<String> {
        // 创建临时目录, uuid 命名
        let temp_dir = format!("{HOST_CACHE_DIR}/{}", &Uuid::new_v4().to_string()[..8]);

        tracing::debug!("Temp directory: {temp_dir}");

        fs::create_dir_all(&temp_dir)?;

        tracing::debug!("Initializing git directory: {temp_dir}");
        // git 初始化目录
        Command::new("git").args(["init"]).current_dir(&temp_dir).output()?;

        tracing::debug!("Adding remote repository: {git_repo}");
        // git 添加远程仓库
        Command::new("git")
            .args(["remote", "add", "origin", git_repo])
            .current_dir(&temp_dir)
            .output()?;

        tracing::debug!("Configuring sparse checkout: {file_path}");
        // 启用稀疏检出
        Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&temp_dir)
            .output()?;

        // 创建稀疏检出目录
        fs::create_dir_all(format!("{temp_dir}/.git/info"))?;

        // 写入稀疏检出配置
        fs::write(format!("{temp_dir}/.git/info/sparse-checkout"), file_path)?;

        tracing::debug!("Pulling origin: {git_branch}");
        // 仅 clone 指定文件
        Command::new("git").args(["pull", "origin", git_branch]).current_dir(&temp_dir).output()?;

        tracing::debug!("Getting source file content: {file_path}");
        // 获取文件内容
        let source_file = Command::new("cat").args([file_path]).current_dir(&temp_dir).output()?;

        // 删除临时目录
        fs::remove_dir_all(&temp_dir)?;

        if !source_file.status.success() {
            tracing::error!(
                "Failed to get source file content for git repo: {git_repo} branch: {git_branch}"
            );
            return Ok("".to_string());
        }

        let output = String::from_utf8_lossy(&source_file.stdout);
        Ok(output.to_string())
    }

    // 根据编程语言解析远程仓库项目配置文件内容, 获取对应配置信息
    async fn get_source_project_version(source: &ConfKitSourceConfig) -> Result<String> {
        // 早期返回，避免嵌套
        let language = match source.language.as_ref() {
            Some(language) => language,
            None => {
                tracing::warn!("Language not found");
                return Ok("".to_string());
            }
        };

        let manifest_file = match source.manifest_file.as_ref() {
            Some(manifest_file) => manifest_file,
            None => {
                tracing::warn!("Source file not found");
                return Ok("".to_string());
            }
        };

        let manifest_file_content =
            Self::get_source_file_content(&source.git_repo, &source.git_branch, manifest_file)
                .await?;

        // 根据语言类型解析配置文件
        match language.as_str() {
            "javascript" => Self::parse_javascript_config(&manifest_file_content),
            "rust" => Self::parse_rust_config(&manifest_file_content),
            _ => {
                tracing::warn!("Unsupported language: {language}");
                Ok("".to_string())
            }
        }
    }

    // 解析 JavaScript 配置文件 (package.json)
    fn parse_javascript_config(content: &str) -> Result<String> {
        let config: serde_json::Value = serde_json::from_str(content)?;
        Ok(config["version"].as_str().unwrap_or("").to_string())
    }

    // 解析 Rust 配置文件 (Cargo.toml)
    fn parse_rust_config(content: &str) -> Result<String> {
        let config: toml::Value = toml::from_str(content)?;
        Ok(config["package"]["version"].as_str().unwrap_or("").to_string())
    }
}
