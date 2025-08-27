//! Author: xiaoYown
//! Created: 2025-07-13
//! Description: ConfKit Engine CLI

use std::{fs, path::Path, sync::Arc};

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
use engine::ConfKitEngine;
use infra::config::ConfKitConfigLoader;
use shared::constants::{
    HOST_ARTIFACTS_ROOT_DIR, HOST_CACHE_DIR, HOST_LOG_DIR, HOST_TEMP_DIR, HOST_WORKSPACE_DIR,
};

use crate::infra::event_hub::{EventHub, LogSubscriber};

// 初始化所需目录
fn init_dirs() -> Result<()> {
    let dirs =
        [HOST_ARTIFACTS_ROOT_DIR, HOST_WORKSPACE_DIR, HOST_LOG_DIR, HOST_CACHE_DIR, HOST_TEMP_DIR];

    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir)?;
        }
    }

    Ok(())
}

// 初始化事件中心
async fn init_event_hub() -> Result<()> {
    EventHub::global().subscribe(Arc::new(LogSubscriber::with_default_path("logs/app.log"))).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 解析全局令行参数
    let cli = Cli::parse_args();

    // 设置日志级别
    let mut log_level = tracing::Level::INFO;
    // 是否显示日志路径
    let mut show_path = false;

    // 开发模式
    if cfg!(debug_assertions) {
        log_level = tracing::Level::DEBUG;
        show_path = true;
    }

    // 初始化日志系统
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(log_level)
        .with_target(show_path)
        .with_level(!cli.hide_level)
        .init();

    // 检查配置文件是否存在
    if !ConfKitConfigLoader::is_config_file_exists().await {
        tracing::error!(
            "✗ .confkit.yml not found in current directory. This is not a confkit project."
        );
        std::process::exit(1);
    }

    // 初始化事件中心
    init_event_hub().await?;

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
