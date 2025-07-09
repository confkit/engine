use std::collections::HashMap;

use super::types::{BuilderInfo, BuilderStatus};

/// 构建器格式化器
pub struct BuilderFormatter;

impl BuilderFormatter {
    /// 格式化构建器列表输出
    pub fn format_builders_list(
        builders: &[&BuilderInfo],
        verbose: bool,
        status_filter: Option<String>,
    ) -> String {
        if builders.is_empty() {
            return "没有找到任何构建器\n\n提示：使用 'confkit builder create' 命令创建新的构建器"
                .to_string();
        }

        // 应用状态过滤
        let filtered_builders: Vec<&BuilderInfo> = if let Some(status) = status_filter {
            let status_lower = status.to_lowercase();
            builders
                .into_iter()
                .filter(|builder| {
                    let builder_status = match builder.status {
                        BuilderStatus::NotCreated => "notcreated",
                        BuilderStatus::Created => "created",
                        BuilderStatus::Running => "running",
                        BuilderStatus::Stopped => "stopped",
                        BuilderStatus::Error => "error",
                    };
                    builder_status == status_lower
                })
                .cloned()
                .collect()
        } else {
            builders.iter().cloned().collect()
        };

        if filtered_builders.is_empty() {
            return "没有找到符合条件的构建器".to_string();
        }

        let mut output = String::new();

        // 显示构建器表格
        output.push_str("构建器列表:\n");
        output.push_str(&Self::format_builders_table(&filtered_builders));

        // 显示统计信息
        let stats = Self::get_builders_stats(&filtered_builders);
        output.push_str("\n统计信息:\n");
        output.push_str(&format!("  总数: {}\n", stats.get("total").unwrap_or(&0)));
        output.push_str(&format!("  未创建: {}\n", stats.get("not_created").unwrap_or(&0)));
        output.push_str(&format!("  运行中: {}\n", stats.get("running").unwrap_or(&0)));
        output.push_str(&format!("  已停止: {}\n", stats.get("stopped").unwrap_or(&0)));
        output.push_str(&format!("  已创建: {}\n", stats.get("created").unwrap_or(&0)));

        if let Some(error_count) = stats.get("error") {
            if *error_count > 0 {
                output.push_str(&format!("  错误: {}\n", error_count));
            }
        }

        // 详细信息模式
        if verbose {
            output.push_str("\n详细信息:\n");
            for builder in filtered_builders {
                output.push_str(&Self::format_builder_details(builder));
            }
        }

        output
    }

    /// 格式化构建器表格
    fn format_builders_table(builders: &[&BuilderInfo]) -> String {
        if builders.is_empty() {
            return "没有找到任何构建器".to_string();
        }

        // 准备所有数据行
        let mut rows = Vec::new();
        for builder in builders {
            let image_id = builder
                .image_id
                .as_ref()
                .map(|id| &id[..8.min(id.len())]) // 显示前8位
                .unwrap_or("N/A");

            let build_status = match builder.status {
                BuilderStatus::NotCreated => "未构建",
                BuilderStatus::Created => "已构建",
                BuilderStatus::Running => "运行中",
                BuilderStatus::Stopped => "已停止",
                BuilderStatus::Error => "错误",
            };

            rows.push((&builder.name, &builder.config.image, build_status, image_id));
        }

        // 计算每列的最大显示宽度
        let headers = ("名称", "镜像", "状态", "镜像ID");

        let mut max_widths = (
            Self::display_width(headers.0), // 名称
            Self::display_width(headers.1), // 镜像
            Self::display_width(headers.2), // 状态
            Self::display_width(headers.3), // 镜像ID
        );

        // 计算数据行的最大宽度
        for (name, image, status, image_id) in &rows {
            max_widths.0 = max_widths.0.max(Self::display_width(name));
            max_widths.1 = max_widths.1.max(Self::display_width(image));
            max_widths.2 = max_widths.2.max(Self::display_width(status));
            max_widths.3 = max_widths.3.max(Self::display_width(image_id));
        }

        // 构建表格
        let mut table = String::new();

        // 表头
        table.push_str(&format!(
            "  {} | {} | {} | {}\n",
            Self::pad_to_width(headers.0, max_widths.0),
            Self::pad_to_width(headers.1, max_widths.1),
            Self::pad_to_width(headers.2, max_widths.2),
            Self::pad_to_width(headers.3, max_widths.3),
        ));

        // 分隔线
        table.push_str(&format!(
            "  {} | {} | {} | {}\n",
            "-".repeat(max_widths.0),
            "-".repeat(max_widths.1),
            "-".repeat(max_widths.2),
            "-".repeat(max_widths.3),
        ));

        // 数据行
        for (name, image, status, image_id) in rows {
            table.push_str(&format!(
                "  {} | {} | {} | {}\n",
                Self::pad_to_width(name, max_widths.0),
                Self::pad_to_width(image, max_widths.1),
                Self::pad_to_width(status, max_widths.2),
                Self::pad_to_width(image_id, max_widths.3),
            ));
        }

        table
    }

    /// 格式化构建器详细信息
    fn format_builder_details(builder: &BuilderInfo) -> String {
        let mut details = String::new();

        details.push_str(&format!("\n构建器: {}\n", builder.name));
        details.push_str(&format!("  镜像: {}\n", builder.config.image));
        details.push_str(&format!("  状态: {:?}\n", builder.status));
        details.push_str(&format!("  Dockerfile: {}\n", builder.config.dockerfile));
        details.push_str(&format!("  构建上下文: {}\n", builder.config.context));

        if let Some(image_id) = &builder.image_id {
            details.push_str(&format!("  镜像ID: {}\n", image_id));
        }

        if let Some(created_at) = builder.created_at {
            details
                .push_str(&format!("  创建时间: {}\n", created_at.format("%Y-%m-%d %H:%M:%S UTC")));
        }

        if !builder.config.build_args.is_empty() {
            details.push_str("  构建参数:\n");
            for (key, value) in &builder.config.build_args {
                details.push_str(&format!("    {}={}\n", key, value));
            }
        }

        if let Some(logs) = &builder.build_logs {
            if !logs.trim().is_empty() {
                details.push_str("  构建日志:\n");
                for line in logs.lines().take(5) {
                    details.push_str(&format!("    {}\n", line));
                }
                if logs.lines().count() > 5 {
                    details.push_str("    ...\n");
                }
            }
        }

        details
    }

    /// 计算字符串的显示宽度（中文字符占2个位置，ASCII字符占1个位置）
    fn display_width(s: &str) -> usize {
        s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
    }

    /// 右边填充空格使字符串达到指定的显示宽度
    fn pad_to_width(s: &str, width: usize) -> String {
        let current_width = Self::display_width(s);
        if current_width >= width {
            s.to_string()
        } else {
            format!("{}{}", s, " ".repeat(width - current_width))
        }
    }

    /// 获取构建器统计信息
    fn get_builders_stats(builders: &[&BuilderInfo]) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        for builder in builders {
            let status_key = match builder.status {
                BuilderStatus::NotCreated => "not_created".to_string(),
                BuilderStatus::Created => "created".to_string(),
                BuilderStatus::Running => "running".to_string(),
                BuilderStatus::Stopped => "stopped".to_string(),
                BuilderStatus::Error => "error".to_string(),
            };
            *stats.entry(status_key).or_insert(0) += 1;
        }

        stats.insert("total".to_string(), builders.len());
        stats
    }
}
