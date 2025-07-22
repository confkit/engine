//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Image management subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::image::ImageBuilder;

#[derive(Args)]
pub struct ImageCommand {
    #[command(subcommand)]
    pub command: ImageSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ImageSubcommand {
    /// List all builder images.
    /// (eg: confkit builder image list)
    List {
        /// Only show builder images with the specified status.
        #[arg(long)]
        status: Option<String>,
    },
    /// Create/pull image.
    Create {
        /// Create all images.
        #[arg(long)]
        all: bool,
        /// Image name.
        #[arg(short, long)]
        name: String,
        /// Image tag.
        #[arg(short, long)]
        tag: String,
    },
    /// Remove image.
    Remove {
        /// Image name.
        #[arg(short, long)]
        name: String,
        /// Image tag.
        #[arg(short, long)]
        tag: String,
    },
}

impl ImageCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ImageSubcommand::List { status } => {
                ImageBuilder::print_list().await?;
            }
            ImageSubcommand::Create { all, name, tag } => {
                create_image(all, name, tag).await?;
            }
            ImageSubcommand::Remove { name, tag } => {
                ImageBuilder::remove(&name, &tag).await?;
            }
        }

        Ok(())
    }
}

/// 创建镜像（从 builder.yml 配置构建镜像）
async fn create_image(all: bool, name: String, tag: String) -> Result<()> {
    if all {
        ImageBuilder::build_all().await?;
    } else {
        ImageBuilder::build(&name, &tag).await?;
    }

    Ok(())
}
