//! Author: xiaoYown
//! Created: 2025-08-14
//! Description: Checkbox interactive implementation

use anyhow::{anyhow, Result};
use inquire::{list_option::ListOption, validator::Validation, MultiSelect};

use crate::types::config::ConfKitEnvironmentInteractiveConfig;

/// 处理复选框交互
pub async fn handle_checkbox(config: &ConfKitEnvironmentInteractiveConfig) -> Result<String> {
    // 检查是否有选项
    let options = match &config.options {
        Some(opts) if !opts.is_empty() => opts,
        _ => return Err(anyhow!("复选框 {} 没有提供选项", config.name)),
    };

    // 创建多选器
    let mut multi_select = MultiSelect::new(&config.prompt, options.clone());

    // 声明 default_indices 在外层作用域
    let mut default_indices: Vec<usize> = Vec::new();

    // 设置默认值
    if let Some(default_value) = &config.default {
        let default_values: Vec<&str> = default_value.split(',').collect();
        default_indices = default_values
            .iter()
            .filter_map(|val| options.iter().position(|opt| opt == val))
            .collect();

        if !default_indices.is_empty() {
            multi_select = multi_select.with_default(&default_indices);
        }
    }

    // 如果必填，添加验证
    if config.required {
        let name = config.name.clone(); // 克隆名称到本地变量
        multi_select = multi_select.with_validator(move |selections: &[ListOption<&String>]| {
            if selections.is_empty() {
                return Ok(Validation::Invalid(format!("{} 至少需要选择一项", name).into()));
            }
            Ok(Validation::Valid)
        });
    }

    // 执行交互
    let results = multi_select.prompt()?;

    // 将结果转换为逗号分隔的字符串
    let result_str = results.join(",");

    Ok(result_str)
}
