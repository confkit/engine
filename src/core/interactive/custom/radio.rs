//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Radio interactive implementation

use anyhow::{anyhow, Result};
use inquire::Select;

use crate::types::config::ConfKitEnvironmentInteractiveConfig;

/// 处理单选框交互
pub async fn handle_radio(config: &ConfKitEnvironmentInteractiveConfig) -> Result<String> {
    // 检查是否有选项
    let options = match &config.options {
        Some(opts) if !opts.is_empty() => opts,
        _ => return Err(anyhow!("单选框 {} 没有提供选项", config.name)),
    };

    // 创建选择器
    let mut select = Select::new(&config.prompt, options.clone());

    // 设置默认值
    if let Some(default_value) = &config.default {
        if let Some(default_index) = options.iter().position(|opt| opt == default_value) {
            select = select.with_starting_cursor(default_index);
        }
    }

    // 执行交互
    let result = select.prompt()?;

    Ok(result.to_string())
}
