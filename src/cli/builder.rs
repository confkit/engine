//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Builder management subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::container::ContainerBuilder;
use crate::engine::ConfKitEngine;

#[derive(Args)]
pub struct BuilderCommand {
    #[command(subcommand)]
    command: BuilderSubcommand,
}

#[derive(Subcommand)]
pub enum BuilderSubcommand {
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
    /// Check builder container health status.
    Health {
        /// Builder name (check all if not specified)
        #[arg(short, long)]
        name: Option<String>,
    },
}

impl BuilderCommand {
    pub async fn execute(self) -> Result<()> {
        ConfKitEngine::ensure_running().await?;

        match self.command {
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
            }
            BuilderSubcommand::Health { name } => {
                ContainerBuilder::print_health(name.as_deref()).await?;
                Ok(())
            }
        }
    }
}
