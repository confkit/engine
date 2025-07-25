//! Author: xiaoYown
//! Created: 2025-07-13
//! Description: ConfKit Engine CLI

use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;

mod infra;
mod shared;
mod types;
mod utils;

mod cli;
mod core;
mod engine;
mod formatter;

use cli::Cli;
use engine::engine::ConfKitEngine;
use infra::config::ConfKitConfigLoader;
use shared::constants::{HOST_ARTIFACTS_DIR, HOST_LOG_DIR, HOST_WORKSPACE_DIR};

// 初始化所需目录
fn init_dirs() -> Result<()> {
    let dirs = [HOST_WORKSPACE_DIR, HOST_ARTIFACTS_DIR, HOST_LOG_DIR];

    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).without_time().init();

    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // 初始化所需目录
    init_dirs()?;

    tracing::debug!("Loading .confkit.yml...");
    // 加载全局配置文件
    if let Err(e) = ConfKitConfigLoader::set_config().await {
        tracing::error!("✗ .confkit.yml not found in current directory: {}", e);
        std::process::exit(1);
    }

    tracing::debug!("Setting engine...");
    // 设置当前宿主机使用的引擎
    if let Err(e) = ConfKitEngine::set_engine(ConfKitConfigLoader::get_config().engine).await {
        tracing::error!("✗ Failed to set engine: {}", e);
        std::process::exit(1);
    }

    // 解析命令行参数
    let cli = Cli::parse();

    tracing::debug!("Executing command...");
    // 执行命令
    cli.execute().await?;

    tracing::debug!("Done!");

    Ok(())
}
