//! Author: xiaoYown
//! Created: 2025-07-13
//! Description: ConfKit CLI

use anyhow::Result;
use clap::Parser;

mod cli;
mod core;
mod engine;
mod formatter;
mod infra;
mod shared;
mod types;
mod utils;

use cli::Cli;
use engine::engine::ConfKitEngine;
use infra::config::ConfKitConfigLoader;
// use infra::storage::StorageManager;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).without_time().init();

    // TODO: 替换统一封装 logger
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // TODO: 替换统一封装 logger
    tracing::debug!("Loading .confkit.yml...");
    // 加载全局配置文件
    if let Err(e) = ConfKitConfigLoader::set_config().await {
        tracing::error!("✗ .confkit.yml not found in current directory: {}", e);
        std::process::exit(1);
    }

    // TODO: 替换统一封装 logger
    tracing::debug!("Setting engine...");
    // 设置当前宿主机使用的引擎
    if let Err(e) = ConfKitEngine::set_engine(ConfKitConfigLoader::get_config().engine).await {
        tracing::error!("✗ Failed to set engine: {}", e);
        std::process::exit(1);
    }

    // // 初始化存储目录结构
    // let storage_manager = StorageManager::with_default();
    // storage_manager.initialize().await?;

    // 解析命令行参数
    let cli = Cli::parse();

    // TODO: 替换统一封装 logger
    tracing::debug!("Executing command...");
    // 执行命令
    cli.execute().await?;

    // TODO: 替换统一封装 logger
    tracing::debug!("Done!");

    Ok(())
}
