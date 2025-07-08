use anyhow::Result;

use super::types::ProjectConfig;

impl ProjectConfig {
    /// 从YAML文件加载配置
    pub async fn from_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: ProjectConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // TODO: 实现配置验证逻辑
        tracing::debug!("验证项目配置: {}", self.project.name);
        Ok(())
    }
}
