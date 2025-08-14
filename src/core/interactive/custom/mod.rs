//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Custom interactive menu implementation

// 自定义交换
// 用于交互注入环境变量
// 目前支持
// 单选 - radio
// 多选 - checkbox
// 输入 - input

mod checkbox;
mod input;
mod radio;

use anyhow::Result;
use std::collections::HashMap;

use crate::types::config::{ConfKitEnvironmentInteractiveConfig, ConfKitInteractiveType};

pub use self::{checkbox::handle_checkbox, input::handle_input, radio::handle_radio};

/// 处理交互式环境变量配置
pub async fn process_interactive_environments(
    configs: &[ConfKitEnvironmentInteractiveConfig],
) -> Result<HashMap<String, String>> {
    let mut env_vars = HashMap::new();

    // 遍历所有交互配置
    for config in configs {
        tracing::info!("处理交互式环境变量: {}", config.name);

        // 根据交互类型调用不同的处理函数
        let value = match config.interactive_type {
            ConfKitInteractiveType::Input => handle_input(config).await?,
            ConfKitInteractiveType::Radio => handle_radio(config).await?,
            ConfKitInteractiveType::Checkbox => handle_checkbox(config).await?,
        };

        // 将结果添加到环境变量中
        env_vars.insert(config.name.clone(), value);
        tracing::debug!("设置环境变量 {}={}", config.name, env_vars.get(&config.name).unwrap());
    }

    Ok(env_vars)
}
