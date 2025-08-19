//! Author: xiaoYown
//! Created: 2025-08-19
//! Description: Confirm interactive implementation

use anyhow::Result;
use inquire::Confirm;

use crate::types::config::ConfKitEnvironmentInteractiveConfig;

/// 处理确认框交互
pub async fn handle_confirm(config: &ConfKitEnvironmentInteractiveConfig) -> Result<String> {
    // 创建确认框
    let mut confirm = Confirm::new(&config.prompt);

    // 设置默认值
    let default_value = if let Some(default_str) = &config.default {
        // 解析默认值，支持多种格式
        match default_str.to_lowercase().as_str() {
            "true" | "yes" | "y" | "1" => true,
            "false" | "no" | "n" | "0" => false,
            _ => {
                tracing::warn!("无效的默认值 '{}', 使用 false 作为默认值", default_str);
                false
            }
        }
    } else {
        false
    };

    confirm = confirm.with_default(default_value);

    // 执行交互
    let result = confirm.prompt()?;

    // 将布尔值转换为字符串返回
    Ok(if result { "true".to_string() } else { "false".to_string() })
}
