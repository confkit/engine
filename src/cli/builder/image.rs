//! 镜像管理子命令实现

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::{BuilderManager, ImageCheckResult, ImageInspector};

#[derive(Args)]
pub struct ImageCommand {
    #[command(subcommand)]
    pub command: ImageSubcommand,
}

#[derive(Subcommand)]
pub enum ImageSubcommand {
    /// 列出所有构建器镜像
    List {
        /// 显示详细信息
        #[arg(long)]
        verbose: bool,
        /// 仅显示指定状态的构建器
        #[arg(long)]
        status: Option<String>,
    },
    /// 创建/拉取镜像
    Create {
        /// 镜像名称
        image: String,
    },
    /// 删除镜像
    Remove {
        /// 镜像名称
        image: String,
        /// 强制删除（即使正在被使用）
        #[arg(long)]
        force: bool,
    },
}

impl ImageCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ImageSubcommand::List { verbose, status } => {
                list_images(verbose, status).await?;
            }
            ImageSubcommand::Create { image } => {
                create_image(image).await?;
            }
            ImageSubcommand::Remove { image, force } => {
                remove_image(image, force).await?;
            }
        }
        Ok(())
    }
}

/// 列出构建器镜像
async fn list_images(verbose: bool, status_filter: Option<String>) -> Result<()> {
    use crate::core::builder::{BuilderFormatter, BuilderManager};

    println!("• 正在加载构建器信息...");

    let manager = BuilderManager::with_demo_data().await;
    let output = manager.list_builders_with_filter(verbose, status_filter)?;

    println!("{}", output);
    Ok(())
}

/// 创建镜像（从 builder.yml 配置构建镜像）
async fn create_image(name: String) -> Result<()> {
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
            println!("\n✓ 构建器镜像 '{}' 创建成功！", name);
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
            println!("\n✗ 构建器镜像创建失败: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// 删除镜像
async fn remove_image(name: String, force: bool) -> Result<()> {
    use crate::core::builder::{BuilderLoader, ImageInspector};

    println!("• 正在删除构建镜像: {}", name);

    // 从 builder.yml 加载构建器配置以获取镜像名称
    let config = match BuilderLoader::find_builder_config(&name) {
        Ok(config) => {
            println!("✓ 找到构建器配置: {}", name);
            println!("  目标镜像: {}", config.image);
            config
        }
        Err(e) => {
            println!("✗ 加载构建器配置失败: {}", e);
            println!("  尝试直接删除镜像名称: {}", name);
            // 如果找不到配置，尝试直接使用提供的名称作为镜像名
            use crate::core::builder::BuilderConfig;
            BuilderConfig {
                name: name.clone(),
                image: name.clone(),
                base_image: String::new(),
                dockerfile: String::new(),
                context: String::new(),
                build_args: std::collections::HashMap::new(),
            }
        }
    };

    // 检查镜像是否存在
    println!();
    match ImageInspector::check_target_image(&config.image).await {
        Ok(ImageCheckResult::Exists(info)) => {
            println!("✓ 找到镜像:");
            println!("  镜像ID: {}", info.id);
            println!("  仓库: {}", info.repository);
            println!("  标签: {}", info.tag);
            println!("  创建时间: {}", info.created_at);
            println!("  大小: {}", info.size);
        }
        Ok(ImageCheckResult::NotExists) => {
            println!("! 镜像不存在: {}", config.image);
            return Ok(());
        }
        Err(e) => {
            println!("✗ 检查镜像时出错: {}", e);
            return Err(e);
        }
    }

    // 执行删除操作
    println!("\n▶ 正在删除 Docker 镜像...");
    println!("→ 镜像: {}", config.image);

    match ImageInspector::remove_image(&config.image, force).await {
        Ok(()) => {
            println!("\n✓ 镜像 '{}' 删除成功！", name);
            println!("→ 已删除镜像: {}", config.image);
            if force {
                println!("→ 使用强制删除模式");
            }
        }
        Err(e) => {
            println!("\n✗ 构建镜像删除失败: {}", e);
            if !force {
                println!("提示: 如果镜像正在被使用，请先停止相关容器");
                println!("      或者使用 --force 参数强制删除");
            }
            return Err(e);
        }
    }

    Ok(())
}
