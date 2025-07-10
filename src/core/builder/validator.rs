use super::types::BuilderConfig;
use anyhow::Result;

/// 构建验证器
pub struct BuildValidator;

impl BuildValidator {
    /// 验证构建配置
    pub fn validate_build_config(config: &BuilderConfig) -> Result<()> {
        // 检查名称是否为空
        if config.name.trim().is_empty() {
            return Err(anyhow::anyhow!("构建器名称不能为空"));
        }

        // 检查 Dockerfile 是否存在
        Self::validate_dockerfile_exists(&config.dockerfile)?;

        // 检查构建上下文是否存在
        Self::validate_context_exists(&config.context)?;

        tracing::debug!("构建配置验证通过: {}", config.name);
        Ok(())
    }

    /// 验证 Dockerfile 文件是否存在
    pub fn validate_dockerfile_exists(dockerfile_path: &str) -> Result<()> {
        if !std::path::Path::new(dockerfile_path).exists() {
            return Err(anyhow::anyhow!("Dockerfile 文件不存在: {}", dockerfile_path));
        }
        Ok(())
    }

    /// 验证构建上下文目录是否存在
    pub fn validate_context_exists(context_path: &str) -> Result<()> {
        if !std::path::Path::new(context_path).exists() {
            return Err(anyhow::anyhow!("构建上下文目录不存在: {}", context_path));
        }
        Ok(())
    }
}
