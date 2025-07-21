//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Log implementation

use chrono::Local;
use std::io::Write;
use std::{fs, path::Path};

use anyhow::Result;

pub struct Log {
    file_path: String,
}

impl Log {
    pub fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }

    // 修改成 tracing api 的格式
    pub fn info(&self, message: &str) -> Result<()> {
        tracing::info!("{}", message);
        self.write(message)?;
        Ok(())
    }

    pub fn warn(&self, message: &str) -> Result<()> {
        tracing::warn!("{}", message);
        self.write(message)?;
        Ok(())
    }

    pub fn error(&self, message: &str) -> Result<()> {
        tracing::error!("{}", message);
        self.write(message)?;
        Ok(())
    }

    pub fn debug(&self, message: &str) -> Result<()> {
        tracing::debug!("{}", message);
        self.write(message)?;
        Ok(())
    }

    pub fn write(&self, message: &str) -> Result<()> {
        let file_path = Path::new(&self.file_path);

        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 以追加模式打开文件，如果不存在则创建
        let mut file = fs::OpenOptions::new().create(true).append(true).open(file_path)?;

        // 写入消息并添加换行符
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        if let Err(e) = file.write_all(format!("[{}] {}\n", timestamp, message).as_bytes()) {
            tracing::error!("Failed to write log: {}", e);
            return Ok(());
        }

        Ok(())
    }
}
