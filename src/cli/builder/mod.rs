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
pub use container::ContainerCommand;
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
    /// 创建新的构建器（构建镜像）
    Create {
        /// 构建器名称（从 builder.yml 中读取配置）
        name: String,
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
            BuilderSubcommand::List { verbose, status } => {
                let container_cmd = ContainerCommand {
                    command: container::ContainerSubcommand::List { verbose, status },
                };
                container_cmd.execute().await
            }
            BuilderSubcommand::Create { name } => {
                let container_cmd =
                    ContainerCommand { command: container::ContainerSubcommand::Create { name } };
                container_cmd.execute().await
            }
            BuilderSubcommand::Start { name } => {
                let container_cmd =
                    ContainerCommand { command: container::ContainerSubcommand::Start { name } };
                container_cmd.execute().await
            }
            BuilderSubcommand::Stop { name } => {
                let container_cmd =
                    ContainerCommand { command: container::ContainerSubcommand::Stop { name } };
                container_cmd.execute().await
            }
            BuilderSubcommand::Remove { name, force } => {
                let container_cmd = ContainerCommand {
                    command: container::ContainerSubcommand::Remove { name, force },
                };
                container_cmd.execute().await
            }
            BuilderSubcommand::Health { name } => {
                let container_cmd =
                    ContainerCommand { command: container::ContainerSubcommand::Health { name } };
                container_cmd.execute().await
            }
        }
    }
}
