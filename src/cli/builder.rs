use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::BuilderManager;

#[derive(Args)]
pub struct BuilderCommand {
    #[command(subcommand)]
    command: BuilderSubcommand,
}

#[derive(Subcommand)]
pub enum BuilderSubcommand {
    /// 列出所有构建器
    List {
        /// 显示详细信息
        #[arg(long)]
        verbose: bool,
        /// 仅显示指定状态的构建器
        #[arg(long)]
        status: Option<String>,
    },
    /// 创建新的构建器
    Create {
        /// 构建器名称
        name: String,
        /// Dockerfile路径
        #[arg(long)]
        dockerfile: String,
    },
    /// 启动构建器
    Start {
        /// 构建器名称
        name: String,
    },
    /// 停止构建器
    Stop {
        /// 构建器名称
        name: String,
    },
    /// 删除构建器
    Remove {
        /// 构建器名称
        name: String,
        /// 强制删除
        #[arg(long)]
        force: bool,
    },
    /// 健康检查
    Health {
        /// 构建器名称
        name: Option<String>,
    },
}

impl BuilderCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            BuilderSubcommand::List { verbose, status } => {
                list_builders(verbose, status).await?;
            }
            BuilderSubcommand::Create { name, dockerfile } => {
                tracing::info!("创建构建器: {} ({})", name, dockerfile);
                println!("暂未实现 - builder create 命令");
            }
            BuilderSubcommand::Start { name } => {
                tracing::info!("启动构建器: {}", name);
                println!("暂未实现 - builder start 命令");
            }
            BuilderSubcommand::Stop { name } => {
                tracing::info!("停止构建器: {}", name);
                println!("暂未实现 - builder stop 命令");
            }
            BuilderSubcommand::Remove { name, force } => {
                tracing::info!("删除构建器: {} (force: {})", name, force);
                println!("暂未实现 - builder remove 命令");
            }
            BuilderSubcommand::Health { name } => {
                tracing::info!("健康检查: {:?}", name);
                println!("暂未实现 - builder health 命令");
            }
        }
        Ok(())
    }
}

/// 列出构建器
async fn list_builders(verbose: bool, status_filter: Option<String>) -> Result<()> {
    tracing::info!(
        "列出构建器 (verbose: {}, status: {:?})",
        verbose,
        status_filter
    );

    // 创建带示例数据的构建器管理器
    let manager = BuilderManager::with_demo_data();

    // 调用 core 层的业务逻辑
    let output = manager.list_builders_with_filter(verbose, status_filter)?;

    // 直接输出结果
    println!("{}", output);

    Ok(())
}
