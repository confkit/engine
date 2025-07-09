use anyhow::Result;
use clap::{Args, Subcommand};

use crate::core::builder::{BuilderConfig, BuilderManager};

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
        /// 构建器名称
        name: String,
        /// Dockerfile路径
        #[arg(long)]
        dockerfile: String,
        /// 指定Docker镜像名称（可选，默认从构建器名称推断）
        #[arg(long)]
        image: Option<String>,
        /// 构建上下文路径（默认为Dockerfile所在目录）
        #[arg(long)]
        context: Option<String>,
        /// 构建参数（格式: KEY=VALUE）
        #[arg(long)]
        build_arg: Vec<String>,
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
            BuilderSubcommand::Create { name, dockerfile, image, context, build_arg } => {
                create_builder(name, dockerfile, image, context, build_arg).await?;
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
async fn create_builder(
    name: String,
    dockerfile: String,
    image: Option<String>,
    context: Option<String>,
    build_args: Vec<String>,
) -> Result<()> {
    tracing::info!("创建构建器（构建镜像）: {} ({})", name, dockerfile);

    // 创建构建器配置
    let mut config = BuilderConfig::new(name.clone(), dockerfile);

    // 如果指定了镜像名称，使用指定的镜像
    if let Some(image_name) = image {
        config = config.with_image(image_name);
    }

    // 如果指定了构建上下文，使用指定的上下文
    if let Some(context_path) = context {
        config = config.with_context(context_path);
    }

    // 解析和添加构建参数
    for build_arg in build_args {
        if let Some((key, value)) = parse_build_arg(&build_arg) {
            config = config.with_build_arg(key, value);
        } else {
            println!("警告: 忽略无效的构建参数格式: {}", build_arg);
        }
    }

    // 创建构建器管理器
    let mut manager = BuilderManager::new();

    // 调用 core 层的业务逻辑创建构建器
    match manager.create_builder(&name, &config).await {
        Ok(()) => {
            println!("✓ 构建器 '{}' 创建成功", name);
            println!("  镜像: {}", config.image);
            println!("  Dockerfile: {}", config.dockerfile);
            println!("  构建上下文: {}", config.context);
            if !config.build_args.is_empty() {
                println!("  构建参数: {} 个", config.build_args.len());
                for (key, value) in &config.build_args {
                    println!("    {}={}", key, value);
                }
            }
        }
        Err(e) => {
            println!("✗ 创建构建器失败: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// 解析构建参数字符串 (KEY=VALUE)
fn parse_build_arg(build_arg: &str) -> Option<(String, String)> {
    if let Some(pos) = build_arg.find('=') {
        let key = build_arg[..pos].trim().to_string();
        let value = build_arg[pos + 1..].trim().to_string();
        if !key.is_empty() {
            return Some((key, value));
        }
    }
    None
}
