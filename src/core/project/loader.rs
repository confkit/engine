//! 项目配置加载器
//!
//! 负责加载和解析空间配置和项目配置文件

use super::types::{ProjectConfig, ProjectRef, SpaceConfig};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// 项目配置加载器
pub struct ProjectLoader {
    base_path: PathBuf,
}

impl ProjectLoader {
    /// 创建新的项目加载器
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self { base_path: base_path.as_ref().to_path_buf() }
    }

    /// 从当前目录创建加载器
    pub fn from_current_dir() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        Ok(Self::new(current_dir))
    }

    /// 加载空间配置
    pub async fn load_space_config(&self, space_name: &str) -> Result<SpaceConfig> {
        let config_path =
            self.base_path.join(".confkit").join("spaces").join(space_name).join("config.yml");

        if !config_path.exists() {
            return Err(anyhow::anyhow!("空间配置文件不存在: {}", config_path.display()));
        }

        let content = fs::read_to_string(&config_path)?;
        let config: SpaceConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("解析空间配置失败: {}", e))?;

        tracing::info!("成功加载空间配置: {}", space_name);
        Ok(config)
    }

    /// 加载项目配置
    pub async fn load_project_config(
        &self,
        space_name: &str,
        project_name: &str,
    ) -> Result<(SpaceConfig, ProjectConfig)> {
        // 先加载空间配置
        let space_config = self.load_space_config(space_name).await?;

        // 查找项目引用
        let project_ref =
            space_config.projects.iter().find(|p| p.name == project_name).ok_or_else(|| {
                anyhow::anyhow!("项目 '{}' 在空间 '{}' 中不存在", project_name, space_name)
            })?;

        // 构建项目配置文件路径
        let project_config_path = self
            .base_path
            .join(".confkit")
            .join("spaces")
            .join(space_name)
            .join(&project_ref.config);

        if !project_config_path.exists() {
            return Err(anyhow::anyhow!("项目配置文件不存在: {}", project_config_path.display()));
        }

        // 加载项目配置
        let content = fs::read_to_string(&project_config_path)?;
        let mut project_config: ProjectConfig = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("解析项目配置失败: {}", e))?;

        // 如果项目配置中没有 source，使用空间配置中的 source
        if project_config.source.is_none() {
            project_config.source = project_ref.source.clone();
        }

        // 验证配置
        self.validate_project_config(&project_config)?;

        tracing::info!("成功加载项目配置: {}/{}", space_name, project_name);
        Ok((space_config, project_config))
    }

    /// 列出空间中的所有项目
    pub async fn list_projects(&self, space_name: &str) -> Result<Vec<ProjectRef>> {
        let space_config = self.load_space_config(space_name).await?;
        Ok(space_config.projects)
    }

    /// 列出所有可用的空间
    pub async fn list_spaces(&self) -> Result<Vec<String>> {
        let spaces_dir = self.base_path.join(".confkit").join("spaces");

        if !spaces_dir.exists() {
            return Ok(Vec::new());
        }

        let mut spaces = Vec::new();
        let entries = fs::read_dir(spaces_dir)?;

        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let space_name = entry.file_name().to_string_lossy().to_string();

                // 检查是否有 config.yml 文件
                let config_path = entry.path().join("config.yml");
                if config_path.exists() {
                    spaces.push(space_name);
                }
            }
        }

        spaces.sort();
        Ok(spaces)
    }

    /// 验证项目配置
    fn validate_project_config(&self, config: &ProjectConfig) -> Result<()> {
        // 检查基本字段
        if config.project.name.is_empty() {
            return Err(anyhow::anyhow!("项目名称不能为空"));
        }

        if config.steps.is_empty() {
            return Err(anyhow::anyhow!("项目必须至少包含一个步骤"));
        }

        // 验证每个步骤
        for (index, step) in config.steps.iter().enumerate() {
            if step.name.is_empty() {
                return Err(anyhow::anyhow!("步骤 {} 的名称不能为空", index + 1));
            }

            if step.commands.is_empty() {
                return Err(anyhow::anyhow!("步骤 '{}' 必须至少包含一个命令", step.name));
            }

            // 验证超时格式（如果存在）
            if let Some(timeout) = &step.timeout {
                self.validate_timeout_format(timeout)?;
            }
        }

        // 验证全局步骤选项中的超时格式
        if let Some(step_options) = &config.step_options {
            if let Some(timeout) = &step_options.timeout {
                self.validate_timeout_format(timeout)?;
            }
        }

        Ok(())
    }

    /// 验证超时格式
    fn validate_timeout_format(&self, timeout: &str) -> Result<()> {
        // 简单验证超时格式，支持 "3m", "30s", "1h" 等
        let timeout = timeout.trim();
        if timeout.is_empty() {
            return Err(anyhow::anyhow!("超时值不能为空"));
        }

        let last_char = timeout.chars().last().unwrap();
        if !['s', 'm', 'h'].contains(&last_char) {
            return Err(anyhow::anyhow!(
                "无效的超时格式: {}，支持的单位: s(秒), m(分钟), h(小时)",
                timeout
            ));
        }

        let number_part = &timeout[..timeout.len() - 1];
        if number_part.parse::<u64>().is_err() {
            return Err(anyhow::anyhow!("无效的超时格式: {}，数字部分无法解析", timeout));
        }

        Ok(())
    }

    /// 解析超时为秒数
    pub fn parse_timeout_to_seconds(&self, timeout: &str) -> Result<u64> {
        let timeout = timeout.trim();
        let last_char = timeout.chars().last().unwrap();
        let number_part = &timeout[..timeout.len() - 1];
        let number: u64 = number_part.parse()?;

        let seconds = match last_char {
            's' => number,
            'm' => number * 60,
            'h' => number * 3600,
            _ => return Err(anyhow::anyhow!("不支持的时间单位: {}", last_char)),
        };

        Ok(seconds)
    }
}
