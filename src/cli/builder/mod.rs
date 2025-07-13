use anyhow::Result;
use clap::{Args, Subcommand};

pub mod image;

// // 重新导出主要的命令结构
// pub use container::{
//     BuilderCreateArgs, BuilderLogsArgs, BuilderRemoveArgs, BuilderStartArgs, BuilderStopArgs,
// };
pub use image::ImageCommand;

use crate::core::builder::container::ContainerBuilder;

#[derive(Args)]
pub struct BuilderCommand {
    #[command(subcommand)]
    command: BuilderSubcommand,
}

#[derive(Subcommand)]
pub enum BuilderSubcommand {
    /// Image management.
    Image(ImageCommand),
    /// List all builder containers.
    List {},
    /// Create and start builder container(based on docker-compose.yml)
    Create {
        /// Create all services
        #[arg(short, long)]
        all: bool,
        // services: Option<Vec<String>>,
        #[arg(short, long)]
        name: String,
        /// Force to recreate(remove existing container)
        #[arg(short, long)]
        force: bool,
    },
    /// Start builder container
    Start {
        /// Builder name
        #[arg(short, long)]
        name: String,
    },
    /// Stop builder container
    Stop {
        /// Builder name
        #[arg(short, long)]
        name: String,
    },
    /// Restart builder container
    Restart {
        /// Builder name
        #[arg(short, long)]
        name: Option<String>,

        /// Restart all services
        #[arg(short, long, default_value = "false")]
        all: bool,
    },
    /// Remove builder container
    Remove {
        /// Builder name
        #[arg(short, long)]
        name: String,
        /// Force to remove
        #[arg(short, long)]
        force: bool,
    },
    // /// 健康检查
    // Health {
    //     /// 构建器名称
    //     name: Option<String>,
    // },
}

impl BuilderCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            BuilderSubcommand::Image(cmd) => cmd.execute().await,
            BuilderSubcommand::List {} => {
                ContainerBuilder::print_list().await?;
                Ok(())
            }
            BuilderSubcommand::Create { all, name, force } => {
                if all {
                    ContainerBuilder::create_all(force).await?;
                } else {
                    ContainerBuilder::create(&name, force).await?;
                }
                Ok(())
            }
            BuilderSubcommand::Remove { name, force } => {
                ContainerBuilder::remove(&name, force).await?;
                Ok(())
            }
            BuilderSubcommand::Start { name } => {
                ContainerBuilder::start(&name).await?;
                Ok(())
            }
            BuilderSubcommand::Stop { name } => {
                ContainerBuilder::stop(&name).await?;
                Ok(())
            }
            BuilderSubcommand::Restart { name, all } => {
                ContainerBuilder::restart(name, all).await?;
                Ok(())
            } // BuilderSubcommand::Health { name: _ } => {
              //     // 健康检查功能暂时使用列表显示
              //     println!("• 健康检查功能，显示构建器状态:");
              //     container::handle_builder_list().await
              // }
        }
    }
}
