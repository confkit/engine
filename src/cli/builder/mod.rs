//! Builder 命令模块
//!
//! 负责构建器相关的命令处理，包括：
//! - 镜像管理 (image)
//! - 容器管理 (container)

use anyhow::Result;
use clap::{Args, Subcommand};

pub mod container;
pub mod image;

// 重新导出主要的命令结构
pub use container::{
    BuilderCreateArgs, BuilderLogsArgs, BuilderRemoveArgs, BuilderStartArgs, BuilderStopArgs,
};
pub use image::ImageCommand;

#[derive(Args)]
pub struct BuilderCommand {
    #[command(subcommand)]
    command: BuilderSubcommand,
}

#[derive(Subcommand)]
pub enum BuilderSubcommand {
    /// 镜像管理
    Image(ImageCommand),
    /// 列出所有构建器
    List {
        /// 显示详细信息
        #[arg(long)]
        verbose: bool,
        /// 仅显示指定状态的构建器
        #[arg(long)]
        status: Option<String>,
    },
    /// 创建新的构建器容器（基于 docker-compose.yml）
    Create {
        /// 服务名称列表
        services: Option<Vec<String>>,
        /// 创建所有服务
        #[arg(long)]
        all: bool,
        /// 强制重新创建（删除已存在的容器）
        #[arg(long)]
        force: bool,
        /// 创建后自动启动
        #[arg(long)]
        start: bool,
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
            BuilderSubcommand::Image(cmd) => cmd.execute().await,
            BuilderSubcommand::List { verbose: _, status: _ } => {
                // 使用新的容器列表函数
                container::handle_builder_list().await
            }
            BuilderSubcommand::Create { services, all, force, start } => {
                // 使用新的容器创建函数
                let args = BuilderCreateArgs { services, all, force, start };
                container::handle_builder_create(&args).await
            }
            BuilderSubcommand::Start { name } => {
                // 使用新的容器启动函数
                let args = BuilderStartArgs { service: name };
                container::handle_builder_start(&args).await
            }
            BuilderSubcommand::Stop { name } => {
                // 使用新的容器停止函数
                let args = BuilderStopArgs { service: name };
                container::handle_builder_stop(&args).await
            }
            BuilderSubcommand::Remove { name, force } => {
                // 使用新的容器删除函数
                let args = BuilderRemoveArgs { service: name, force };
                container::handle_builder_remove(&args).await
            }
            BuilderSubcommand::Health { name: _ } => {
                // 健康检查功能暂时使用列表显示
                println!("• 健康检查功能，显示构建器状态:");
                container::handle_builder_list().await
            }
        }
    }
}
