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
        /// Force create (even if it already exists).
        #[arg(long)]
        force: bool,
    },
    /// Remove image.
    Remove {
        /// Image name.
        #[arg(short, long)]
        name: String,
        /// Image tag.
        #[arg(short, long)]
        tag: String,
        /// Force remove (even if it is being used).
        #[arg(long)]
        force: bool,
    },
}

impl ImageCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ImageSubcommand::List { status } => {
                ImageBuilder::print_list().await?;
            }
            ImageSubcommand::Create { all, name, tag, force } => {
                create_image(all, name, tag, force).await?;
            }
            ImageSubcommand::Remove { name, tag, force } => {
                ImageBuilder::remove(&name, &tag, force).await?;
            }
        }

        Ok(())
    }
}

/// 创建镜像（从 builder.yml 配置构建镜像）
async fn create_image(all: bool, name: String, tag: String, force: bool) -> Result<()> {
    if all {
        ImageBuilder::build_all(force).await?;
    } else {
        ImageBuilder::build(&name, &tag, force).await?;
    }

    Ok(())
}
