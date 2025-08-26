//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Input interactive implementation

use anyhow::Result;
use inquire::validator::Validation;
use inquire::Text;

use crate::types::config::ConfKitEnvironmentInteractiveConfig;

/// 处理输入框交互
pub async fn handle_input(config: &ConfKitEnvironmentInteractiveConfig) -> Result<String> {
    let mut text = Text::new(&config.prompt);

    // 设置默认值
    if let Some(default_value) = &config.default {
        text = text.with_default(default_value);
    }

    // 如果必填，添加验证
    if config.required {
        // 克隆name字段以便在闭包中使用
        let name = config.name.clone();
        text = text.with_validator(move |input: &str| {
            if input.trim().is_empty() {
                return Ok(Validation::Invalid(format!("{name} 不能为空").into()));
            }
            Ok(Validation::Valid)
        });
    }

    // 执行交互
    let result = text.prompt()?;

    Ok(result)
}
