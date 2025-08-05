//! Author: xiaoYown
//! Created: 2025-07-22
//! Description: Log formatter

pub struct LogFormatter;

impl LogFormatter {
    pub fn header(title: &str) -> String {
        // 自动适应 80 长度
        let title_len = title.len();
        let padding = (78 - title_len) / 2;
        let padding_str = "=".repeat(padding);

        format!("\n{padding_str}[ {title} ]{padding_str}")
    }
}
