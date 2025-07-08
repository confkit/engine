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

    // 获取构建器列表
    let builders = manager.list_builders();

    if builders.is_empty() {
        println!("没有找到任何构建器");
        println!("\n提示：使用 'confkit builder create' 命令创建新的构建器");
        return Ok(());
    }

    // 应用状态过滤
    let filtered_builders: Vec<_> = if let Some(status) = status_filter {
        let status_lower = status.to_lowercase();
        builders
            .into_iter()
            .filter(|builder| {
                let builder_status = match builder.status {
                    crate::core::builder::BuilderStatus::NotCreated => "notcreated",
                    crate::core::builder::BuilderStatus::Created => "created",
                    crate::core::builder::BuilderStatus::Running => "running",
                    crate::core::builder::BuilderStatus::Stopped => "stopped",
                    crate::core::builder::BuilderStatus::Error => "error",
                };
                builder_status == status_lower
            })
            .collect()
    } else {
        builders
    };

    if filtered_builders.is_empty() {
        println!("没有找到符合条件的构建器");
        return Ok(());
    }

    // 显示构建器表格（基于过滤后的列表）
    println!("构建器列表:");
    println!(
        "{}",
        manager.format_filtered_builders_table(&filtered_builders)
    );

    // 显示统计信息（基于过滤后的列表）
    let stats = manager.get_filtered_stats(&filtered_builders);
    println!("\n统计信息:");
    println!("  总数: {}", stats.get("total").unwrap_or(&0));
    println!("  运行中: {}", stats.get("running").unwrap_or(&0));
    println!("  已停止: {}", stats.get("stopped").unwrap_or(&0));
    println!("  已创建: {}", stats.get("created").unwrap_or(&0));

    if let Some(error_count) = stats.get("error") {
        if *error_count > 0 {
            println!("  错误: {}", error_count);
        }
    }

    // 详细信息模式
    if verbose {
        println!("\n详细信息:");
        for builder in filtered_builders {
            println!("\n构建器: {}", builder.name);
            println!("  镜像: {}", builder.config.image);
            println!("  状态: {:?}", builder.status);

            if let Some(container_id) = &builder.container_id {
                println!("  容器ID: {}", container_id);
            }

            if let Some(created_at) = builder.created_at {
                println!("  创建时间: {}", created_at.format("%Y-%m-%d %H:%M:%S UTC"));
            }

            if let Some(health) = &builder.last_health_check {
                println!(
                    "  健康状态: {} ({})",
                    if health.healthy { "健康" } else { "异常" },
                    health.message
                );
                println!(
                    "  最后检查: {}",
                    health.last_check.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }

            if !builder.config.volumes.is_empty() {
                println!("  卷挂载: {}", builder.config.volumes.join(", "));
            }

            if !builder.config.ports.is_empty() {
                println!("  端口映射: {}", builder.config.ports.join(", "));
            }
        }
    }

    Ok(())
}
