use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::{BuilderManager, ImageCheckResult, ImageInspector};

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
            BuilderSubcommand::List { verbose, status } => {
                list_builders(verbose, status).await?;
            }
            BuilderSubcommand::Create { name } => {
                create_builder(name).await?;
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
    tracing::info!("列出构建器 (verbose: {}, status: {:?})", verbose, status_filter);

    // 创建带示例数据的构建器管理器
    let manager = BuilderManager::with_demo_data();

    // 调用 core 层的业务逻辑
    let output = manager.list_builders_with_filter(verbose, status_filter)?;

    // 直接输出结果
    println!("{}", output);

    Ok(())
}

/// 创建构建器（构建镜像）
async fn create_builder(name: String) -> Result<()> {
    use crate::core::builder::{BuilderLoader, ImageBuilder};

    println!("• 正在从 builder.yml 加载构建器配置...");

    // 从 builder.yml 加载构建器配置
    let config = match BuilderLoader::find_builder_config(&name) {
        Ok(config) => {
            println!("✓ 找到构建器配置: {}", name);
            println!("  目标镜像: {}", config.image);
            println!("  基础镜像: {}", config.base_image);
            println!("  Dockerfile: {}", config.dockerfile);
            println!("  构建上下文: {}", config.context);
            if !config.build_args.is_empty() {
                println!("  构建参数: {} 个", config.build_args.len());
                for (key, value) in &config.build_args {
                    println!("    {}={}", key, value);
                }
            }
            config
        }
        Err(e) => {
            println!("✗ 加载构建器配置失败: {}", e);
            return Err(e);
        }
    };

    // 检查目标镜像是否已存在
    println!();
    match ImageInspector::check_target_image(&config.image).await {
        Ok(ImageCheckResult::Exists(_)) => {
            println!("● 跳过构建，直接使用现有镜像");
            return Ok(());
        }
        Ok(ImageCheckResult::NotExists) => {
            println!("▶ 开始构建镜像...");
        }
        Err(e) => {
            println!("! 检查镜像时出错: {}, 继续尝试构建", e);
        }
    }

    // 执行镜像构建
    println!("\n▶ 正在构建 Docker 镜像...");
    println!("→ Dockerfile: {}", config.dockerfile);
    println!("→ 构建上下文: {}", config.context);

    match ImageBuilder::build_image(&config).await {
        Ok(builder_info) => {
            println!("\n✓ 构建器 '{}' 创建成功！", name);
            println!("→ 镜像: {}", config.image);
            if let Some(image_id) = &builder_info.image_id {
                println!("→ 镜像ID: {}", image_id);
            }
            println!(
                "→ 创建时间: {}",
                builder_info.created_at.unwrap_or_default().format("%Y-%m-%d %H:%M:%S")
            );

            // 显示构建日志的最后几行
            if let Some(logs) = &builder_info.build_logs {
                let lines: Vec<&str> = logs.lines().collect();
                let last_lines = lines.iter().rev().take(5).rev();
                println!("\n※ 构建日志 (最后 5 行):");
                for line in last_lines {
                    if !line.trim().is_empty() {
                        println!("   {}", line);
                    }
                }
            }
        }
        Err(e) => {
            println!("\n✗ 构建器创建失败: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
