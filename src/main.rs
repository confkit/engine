//! Author: xiaoYown
//! Created: 2025-07-13
//! Description: ConfKit Engine CLI

use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;
use tokio::signal;

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

/// 等待系统终止信号
async fn wait_for_shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal, shutting down gracefully...");
        },
    }
}

/// 优雅关闭应用程序
async fn shutdown_gracefully() -> Result<()> {
    tracing::debug!("Shutting down application");
    tracing::debug!("Application shutdown completed");
    Ok(())
}

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
    // INFO/DEBUG/TRACE → stdout, WARN/ERROR → stderr
    use tracing_subscriber::fmt::writer::MakeWriterExt;
    let stdout = std::io::stdout.with_min_level(tracing::Level::INFO);
    let stderr = std::io::stderr.with_max_level(tracing::Level::WARN);

    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(log_level)
        .with_target(show_path)
        .with_level(!cli.hide_level)
        .with_writer(stdout.and(stderr))
        .init();

    // 检查配置文件是否存在
    if !ConfKitConfigLoader::is_config_file_exists().await {
        tracing::error!(
            "✗ .confkit.yml not found in current directory. This is not a confkit project."
        );
        std::process::exit(1);
    }

    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // 初始化所需目录
    init_dirs()?;

    // 初始化任务数据库（确保建表）
    if let Err(e) = infra::db::TaskDb::open() {
        tracing::warn!("Failed to initialize task database: {}", e);
    }

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

    // 使用 tokio::select 来同时等待命令执行和关闭信号
    tokio::select! {
        // 正常执行命令
        result = cli.execute() => {
            match result {
                Ok(_) => {
                    tracing::debug!("Command executed successfully");

                    // 正常完成时也需要优雅关闭 EventHub
                    if let Err(e) = shutdown_gracefully().await {
                        tracing::warn!("Graceful shutdown failed: {}", e);
                    }

                    Ok(())
                },
                Err(e) => {
                    tracing::error!("Command failed: {}", e);

                    // 即使失败也要优雅关闭
                    if let Err(shutdown_err) = shutdown_gracefully().await {
                        tracing::warn!("Graceful shutdown failed: {}", shutdown_err);
                    }

                    Err(e)
                }
            }
        },
        // 等待关闭信号
        _ = wait_for_shutdown_signal() => {
            // 收到信号，执行优雅关闭
            shutdown_gracefully().await?;
            Ok(())
        }
    }
}
