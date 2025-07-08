use anyhow::Result;
use regex::Regex;

/// 验证器trait
pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<()>;
}

/// 字符串验证器
pub struct StringValidator {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<Regex>,
    pub allowed_chars: Option<String>,
    pub required: bool,
}

impl Default for StringValidator {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            pattern: None,
            allowed_chars: None,
            required: true,
        }
    }
}

impl StringValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn pattern(mut self, pattern: &str) -> Result<Self> {
        self.pattern = Some(Regex::new(pattern)?);
        Ok(self)
    }

    pub fn allowed_chars(mut self, chars: &str) -> Self {
        self.allowed_chars = Some(chars.to_string());
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl Validator<String> for StringValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() && self.required {
            return Err(anyhow::anyhow!("字段不能为空"));
        }

        if value.is_empty() && !self.required {
            return Ok(());
        }

        if let Some(min) = self.min_length {
            if value.len() < min {
                return Err(anyhow::anyhow!("长度不能少于{}个字符", min));
            }
        }

        if let Some(max) = self.max_length {
            if value.len() > max {
                return Err(anyhow::anyhow!("长度不能超过{}个字符", max));
            }
        }

        if let Some(ref pattern) = self.pattern {
            if !pattern.is_match(value) {
                return Err(anyhow::anyhow!("格式不正确"));
            }
        }

        if let Some(ref allowed) = self.allowed_chars {
            for ch in value.chars() {
                if !allowed.contains(ch) {
                    return Err(anyhow::anyhow!("包含不允许的字符: {}", ch));
                }
            }
        }

        Ok(())
    }
}

/// URL验证器
pub struct UrlValidator {
    pub schemes: Vec<String>,
    pub required: bool,
}

impl Default for UrlValidator {
    fn default() -> Self {
        Self {
            schemes: vec!["http".to_string(), "https".to_string()],
            required: true,
        }
    }
}

impl UrlValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn schemes(mut self, schemes: Vec<&str>) -> Self {
        self.schemes = schemes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl Validator<String> for UrlValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() && self.required {
            return Err(anyhow::anyhow!("URL不能为空"));
        }

        if value.is_empty() && !self.required {
            return Ok(());
        }

        // 简单的URL格式验证
        if !value.contains("://") {
            return Err(anyhow::anyhow!("URL格式不正确"));
        }

        let parts: Vec<&str> = value.splitn(2, "://").collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("URL格式不正确"));
        }

        let scheme = parts[0];
        if !self.schemes.contains(&scheme.to_string()) {
            return Err(anyhow::anyhow!(
                "不支持的协议: {}，支持的协议: {:?}",
                scheme,
                self.schemes
            ));
        }

        Ok(())
    }
}

/// 项目名称验证器
pub struct ProjectNameValidator;

impl Validator<String> for ProjectNameValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() {
            return Err(anyhow::anyhow!("项目名称不能为空"));
        }

        // 项目名称只能包含字母、数字、连字符和下划线
        let pattern = Regex::new(r"^[a-zA-Z0-9_-]+$")?;
        if !pattern.is_match(value) {
            return Err(anyhow::anyhow!(
                "项目名称只能包含字母、数字、连字符和下划线"
            ));
        }

        // 长度限制
        if value.len() < 2 {
            return Err(anyhow::anyhow!("项目名称至少需要2个字符"));
        }

        if value.len() > 50 {
            return Err(anyhow::anyhow!("项目名称不能超过50个字符"));
        }

        // 不能以连字符开头或结尾
        if value.starts_with('-') || value.ends_with('-') {
            return Err(anyhow::anyhow!("项目名称不能以连字符开头或结尾"));
        }

        Ok(())
    }
}

/// 任务ID验证器
pub struct TaskIdValidator;

impl Validator<String> for TaskIdValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() {
            return Err(anyhow::anyhow!("任务ID不能为空"));
        }

        // 任务ID格式：项目名-时间戳-随机字符
        let pattern = Regex::new(r"^[a-zA-Z0-9_-]+-\d{8}-\d{6}-[a-f0-9]{8}$")?;
        if !pattern.is_match(value) {
            return Err(anyhow::anyhow!("任务ID格式不正确"));
        }

        Ok(())
    }
}

/// 文件路径验证器
pub struct PathValidator {
    pub must_exist: bool,
    pub must_be_file: bool,
    pub must_be_dir: bool,
    pub required: bool,
}

impl Default for PathValidator {
    fn default() -> Self {
        Self {
            must_exist: false,
            must_be_file: false,
            must_be_dir: false,
            required: true,
        }
    }
}

impl PathValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn must_exist(mut self) -> Self {
        self.must_exist = true;
        self
    }

    pub fn must_be_file(mut self) -> Self {
        self.must_be_file = true;
        self
    }

    pub fn must_be_dir(mut self) -> Self {
        self.must_be_dir = true;
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl Validator<String> for PathValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() && self.required {
            return Err(anyhow::anyhow!("路径不能为空"));
        }

        if value.is_empty() && !self.required {
            return Ok(());
        }

        let path = std::path::Path::new(value);

        if self.must_exist && !path.exists() {
            return Err(anyhow::anyhow!("路径不存在: {}", value));
        }

        if self.must_be_file && !path.is_file() {
            return Err(anyhow::anyhow!("路径不是文件: {}", value));
        }

        if self.must_be_dir && !path.is_dir() {
            return Err(anyhow::anyhow!("路径不是目录: {}", value));
        }

        Ok(())
    }
}

/// 环境变量名称验证器
pub struct EnvVarNameValidator;

impl Validator<String> for EnvVarNameValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.is_empty() {
            return Err(anyhow::anyhow!("环境变量名称不能为空"));
        }

        // 环境变量名称只能包含大写字母、数字和下划线
        let pattern = Regex::new(r"^[A-Z][A-Z0-9_]*$")?;
        if !pattern.is_match(value) {
            return Err(anyhow::anyhow!(
                "环境变量名称只能包含大写字母、数字和下划线，且必须以字母开头"
            ));
        }

        Ok(())
    }
}

/// 验证配置文件
pub fn validate_config_file(config: &crate::core::config::ProjectConfig) -> Result<()> {
    // 验证项目名称
    ProjectNameValidator.validate(&config.project.name)?;

    // 验证Git仓库URL
    UrlValidator::new()
        .schemes(vec!["http", "https", "git", "ssh"])
        .validate(&config.source.git_repo)?;

    // 验证分支名称
    StringValidator::new()
        .min_length(1)
        .max_length(100)
        .validate(&config.source.git_branch)?;

    // 验证步骤名称
    for step in &config.steps {
        StringValidator::new()
            .min_length(1)
            .max_length(100)
            .validate(&step.name)?;

        // 验证命令不为空
        if step.commands.is_empty() {
            return Err(anyhow::anyhow!("步骤 '{}' 的命令不能为空", step.name));
        }
    }

    Ok(())
}

/// 验证环境变量
pub fn validate_environment_variables(
    env_vars: &std::collections::HashMap<String, String>,
) -> Result<()> {
    for (key, _value) in env_vars {
        EnvVarNameValidator.validate(key)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validator() {
        let validator = StringValidator::new().min_length(3).max_length(10);

        assert!(validator.validate(&"hello".to_string()).is_ok());
        assert!(validator.validate(&"hi".to_string()).is_err());
        assert!(validator.validate(&"verylongstring".to_string()).is_err());
    }

    #[test]
    fn test_project_name_validator() {
        let validator = ProjectNameValidator;

        assert!(validator.validate(&"my-project".to_string()).is_ok());
        assert!(validator.validate(&"project_123".to_string()).is_ok());
        assert!(validator.validate(&"my project".to_string()).is_err());
        assert!(validator.validate(&"-project".to_string()).is_err());
        assert!(validator.validate(&"project-".to_string()).is_err());
    }

    #[test]
    fn test_url_validator() {
        let validator = UrlValidator::new();

        assert!(validator
            .validate(&"https://github.com/user/repo.git".to_string())
            .is_ok());
        assert!(validator
            .validate(&"http://example.com".to_string())
            .is_ok());
        assert!(validator.validate(&"invalid-url".to_string()).is_err());
        assert!(validator
            .validate(&"ftp://example.com".to_string())
            .is_err());
    }
}
