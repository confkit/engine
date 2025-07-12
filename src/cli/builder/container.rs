//! 容器管理子命令实现

use crate::core::builder::{BuilderContainer, ContainerManager, ContainerStatus};
use anyhow::Result;
use clap::Args;

/// Builder Create 命令参数
#[derive(Debug, Args)]
pub struct BuilderCreateArgs {
    /// 服务名称列表
    pub services: Option<Vec<String>>,

    /// 创建所有服务
    #[arg(long)]
    pub all: bool,

    /// 强制重新创建（删除已存在的容器）
    #[arg(long)]
    pub force: bool,

    /// 创建后自动启动
    #[arg(long)]
    pub start: bool,
}

/// Builder Start 命令参数
#[derive(Debug, Args)]
pub struct BuilderStartArgs {
    /// 服务名称
    pub service: String,
}

/// Builder Stop 命令参数
#[derive(Debug, Args)]
pub struct BuilderStopArgs {
    /// 服务名称
    pub service: String,
}

/// Builder Remove 命令参数
#[derive(Debug, Args)]
pub struct BuilderRemoveArgs {
    /// 服务名称
    pub service: String,

    /// 强制删除（即使容器正在运行）
    #[arg(long)]
    pub force: bool,
}

/// Builder Logs 命令参数
#[derive(Debug, Args)]
pub struct BuilderLogsArgs {
    /// 服务名称
    pub service: String,

    /// 显示的行数
    #[arg(long, short)]
    pub lines: Option<usize>,
}

/// 处理 builder create 命令
pub async fn handle_builder_create(args: &BuilderCreateArgs) -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    match &args.services {
        Some(services) => {
            // 创建指定的服务
            for service in services {
                create_single_builder(&manager, service, args.force, args.start).await?;
            }
        }
        None if args.all => {
            // 创建所有服务
            create_all_builders(&manager, args.force, args.start).await?;
        }
        None => {
            // 交互式选择
            interactive_select_and_create(&manager, args.force, args.start).await?;
        }
    }

    Ok(())
}

/// 处理 builder start 命令
pub async fn handle_builder_start(args: &BuilderStartArgs) -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    println!("• 启动构建器容器: {}", args.service);

    manager.start_builder(&args.service).await?;

    println!("✓ 构建器容器启动成功: {}", args.service);
    Ok(())
}

/// 处理 builder stop 命令
pub async fn handle_builder_stop(args: &BuilderStopArgs) -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    println!("• 停止构建器容器: {}", args.service);

    manager.stop_builder(&args.service).await?;

    println!("✓ 构建器容器停止成功: {}", args.service);
    Ok(())
}

/// 处理 builder remove 命令
pub async fn handle_builder_remove(args: &BuilderRemoveArgs) -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    println!("• 删除构建器容器: {}", args.service);

    manager.remove_builder(&args.service, args.force).await?;

    println!("✓ 构建器容器删除成功: {}", args.service);
    Ok(())
}

/// 处理 builder logs 命令
pub async fn handle_builder_logs(args: &BuilderLogsArgs) -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    println!("• 获取构建器容器日志: {}", args.service);

    let logs = manager.get_builder_logs(&args.service, args.lines).await?;

    if logs.trim().is_empty() {
        println!("! 没有找到日志内容");
    } else {
        println!("▶ 容器日志:");
        println!("{}", logs);
    }

    Ok(())
}

/// 处理 builder list 命令（容器列表）
pub async fn handle_builder_list() -> Result<()> {
    let manager = ContainerManager::from_current_dir().await?;

    println!("• 获取构建器容器列表...");

    let containers = manager.list_builders().await?;

    if containers.is_empty() {
        println!("! 未找到任何构建器容器");
        println!("  使用 'confkit builder create' 创建构建器容器");
        return Ok(());
    }

    // 显示容器列表
    display_containers_table(&containers);

    Ok(())
}

// === 内部辅助函数 ===

/// 创建单个构建器
async fn create_single_builder(
    manager: &ContainerManager,
    service_name: &str,
    force: bool,
    start: bool,
) -> Result<()> {
    println!("• 创建构建器容器: {}", service_name);

    // 检查服务是否存在
    if manager.get_service(service_name).is_err() {
        let available_services = manager.list_service_names();
        println!("✗ 服务 '{}' 不存在", service_name);
        println!("  可用的服务: {}", available_services.join(", "));
        return Ok(());
    }

    // 创建容器
    let container = if force {
        manager.create_builder_force(service_name).await?
    } else {
        match manager.create_builder(service_name).await {
            Ok(container) => container,
            Err(e) => {
                println!("✗ 创建失败: {}", e);
                return Ok(());
            }
        }
    };

    println!("✓ 构建器容器创建成功: {}", service_name);
    println!("  → 容器名称: {}", container.container_name);
    println!("  → 镜像: {}", container.image);
    println!("  → 状态: {:?}", container.status);

    // 可选：自动启动
    if start && !matches!(container.status, ContainerStatus::Running) {
        println!("▶ 自动启动容器...");
        match manager.start_builder(service_name).await {
            Ok(()) => println!("✓ 容器启动成功"),
            Err(e) => println!("✗ 容器启动失败: {}", e),
        }
    }

    Ok(())
}

/// 创建所有构建器
async fn create_all_builders(manager: &ContainerManager, force: bool, start: bool) -> Result<()> {
    let service_names = manager.list_service_names();

    println!("• 创建所有构建器容器 ({} 个)", service_names.len());

    for (i, service_name) in service_names.iter().enumerate() {
        println!();
        println!("→ [{}/{}] {}", i + 1, service_names.len(), service_name);
        create_single_builder(manager, service_name, force, start).await?;
    }

    println!();
    println!("✓ 所有构建器容器创建完成");

    Ok(())
}

/// 交互式选择并创建
async fn interactive_select_and_create(
    manager: &ContainerManager,
    force: bool,
    start: bool,
) -> Result<()> {
    let service_names = manager.list_service_names();

    if service_names.is_empty() {
        println!("! 未找到任何服务配置");
        println!("  请检查 docker-compose.yml 文件");
        return Ok(());
    }

    println!("• 可用的构建器服务:");
    for (i, service) in service_names.iter().enumerate() {
        println!("  {}. {}", i + 1, service);
    }

    println!();
    println!("请选择要创建的服务（输入序号或服务名称，'all' 创建所有）:");

    use std::io::{self, Write};
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        println!("• 取消操作");
        return Ok(());
    }

    if input == "all" {
        create_all_builders(manager, force, start).await?;
    } else if let Ok(index) = input.parse::<usize>() {
        if index > 0 && index <= service_names.len() {
            let service_name = &service_names[index - 1];
            create_single_builder(manager, service_name, force, start).await?;
        } else {
            println!("✗ 无效的序号: {}", index);
        }
    } else if service_names.contains(&input.to_string()) {
        create_single_builder(manager, input, force, start).await?;
    } else {
        println!("✗ 无效的选择: {}", input);
    }

    Ok(())
}

/// 显示容器表格
fn display_containers_table(containers: &[BuilderContainer]) {
    println!("构建器容器列表:");
    println!();

    // 计算列宽
    let mut max_service_width = "服务名称".len();
    let mut max_container_width = "容器名称".len();
    let mut max_image_width = "镜像".len();
    let mut max_status_width = "状态".len();

    for container in containers {
        max_service_width = max_service_width.max(container.service_name.len());
        max_container_width = max_container_width.max(container.container_name.len());
        max_image_width = max_image_width.max(container.image.len());
        max_status_width = max_status_width.max(format!("{:?}", container.status).len());
    }

    // 表头
    println!(
        "  {} | {} | {} | {}",
        pad_to_width("服务名称", max_service_width),
        pad_to_width("容器名称", max_container_width),
        pad_to_width("镜像", max_image_width),
        pad_to_width("状态", max_status_width),
    );

    // 分隔线
    println!(
        "  {} | {} | {} | {}",
        "-".repeat(max_service_width),
        "-".repeat(max_container_width),
        "-".repeat(max_image_width),
        "-".repeat(max_status_width),
    );

    // 数据行
    for container in containers {
        let status_icon = match container.status {
            ContainerStatus::Running => "▶",
            ContainerStatus::Created => "●",
            ContainerStatus::Exited => "■",
            ContainerStatus::Paused => "⏸",
            ContainerStatus::NotCreated => "○",
            _ => "?",
        };

        let status_display = format!("{} {:?}", status_icon, container.status);

        println!(
            "  {} | {} | {} | {}",
            pad_to_width(&container.service_name, max_service_width),
            pad_to_width(&container.container_name, max_container_width),
            pad_to_width(&container.image, max_image_width),
            pad_to_width(&status_display, max_status_width),
        );
    }

    println!();

    // 统计信息
    let total = containers.len();
    let running =
        containers.iter().filter(|c| matches!(c.status, ContainerStatus::Running)).count();
    let created =
        containers.iter().filter(|c| matches!(c.status, ContainerStatus::Created)).count();
    let not_created =
        containers.iter().filter(|c| matches!(c.status, ContainerStatus::NotCreated)).count();

    println!("统计信息:");
    println!("  总数: {}", total);
    println!("  ▶ 运行中: {}", running);
    println!("  ● 已创建: {}", created);
    println!("  ○ 未创建: {}", not_created);
}

/// 右边填充空格使字符串达到指定宽度
fn pad_to_width(s: &str, width: usize) -> String {
    format!("{:<width$}", s, width = width)
}
