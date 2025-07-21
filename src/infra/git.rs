//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Git client implementation

use anyhow::Result;
use std::{path::Path, process::Command};

use crate::infra::config::ConfKitConfigLoader;

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
}

pub struct GitClient {
    pub space_name: String,
    pub project_name: String,
    pub git_info: Option<GitInfo>,
}

impl GitClient {
    // TODO: 后期优化
    pub async fn new(space_name: &str, project_name: &str) -> Result<Self> {
        let git_info = Self::get_envs(space_name, project_name).await?;

        Ok(Self {
            space_name: space_name.to_string(),
            project_name: project_name.to_string(),
            git_info,
        })
    }

    pub async fn get_envs(space_name: &str, project_name: &str) -> Result<Option<GitInfo>> {
        let project_config =
            ConfKitConfigLoader::get_project_config(space_name, project_name).await?;

        if project_config.is_none() {
            tracing::info!("Project config not found for project: {}", project_name);
            return Ok(None);
        }

        let project_config = project_config.unwrap();
        let source = project_config.source.unwrap();

        let source_hash = match Self::get_source_hash(&source.git_repo, &source.git_branch).await {
            Ok(source_hash) => source_hash,
            Err(e) => {
                tracing::error!("Failed to get source hash: {}", e);
                return Err(e);
            }
        };

        let git_info = GitInfo {
            repo_url: source.git_repo.clone(),
            branch: source.git_branch.clone(),
            commit_hash: source_hash.commit_hash,
            commit_hash_short: source_hash.commit_hash_short,
        };

        Ok(Some(git_info))
    }

    pub async fn get_source_hash(git_repo: &str, git_branch: &str) -> Result<SourceHash> {
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
}
