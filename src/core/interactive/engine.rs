use anyhow::Result;
use std::io::{self, Write};

use super::types::{Command, CommandContext, CommandResult, InteractiveConfig};
use crate::core::builder::BuilderManager;

/// 交互式引擎
pub struct InteractiveEngine {
    /// 命令上下文
    context: CommandContext,
    /// 构建器管理器
    builder_manager: BuilderManager,
}

impl InteractiveEngine {
    /// 创建新的交互式引擎
    pub fn new(config: InteractiveConfig) -> Result<Self> {
        let context = CommandContext::new(config);

        // 尝试从当前目录加载构建器，失败则使用演示数据
        let builder_manager = BuilderManager::from_current_directory()
            .unwrap_or_else(|_| BuilderManager::with_demo_data());

        Ok(Self {
            context,
            builder_manager,
        })
    }

    /// 启动交互式会话
    pub async fn run(&mut self) -> Result<()> {
        println!("欢迎使用 ConfKit 交互式模式!");
        println!("输入 'help' 查看可用命令，输入 'exit' 或 'quit' 退出");
        println!();

        loop {
            print!("confkit> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // 添加到历史记录
            self.context.add_to_history(input.to_string());

            match self.execute_command(input).await {
                Ok(CommandResult::Continue) => continue,
                Ok(CommandResult::Exit) => {
                    println!("再见!");
                    break;
                }
                Ok(CommandResult::Help(help_text)) => {
                    println!("{}", help_text);
                }
                Ok(CommandResult::Error(error)) => {
                    println!("错误: {}", error);
                }
                Err(e) => {
                    println!("错误: {}", e);
                }
            }
        }

        Ok(())
    }

    /// 执行命令
    async fn execute_command(&mut self, input: &str) -> Result<CommandResult> {
        match Command::parse(input) {
            Ok(command) => self.handle_command(command).await,
            Err(e) => Ok(CommandResult::Error(format!(
                "{}. 输入 'help' 查看可用命令",
                e
            ))),
        }
    }

    /// 处理解析后的命令
    async fn handle_command(&mut self, command: Command) -> Result<CommandResult> {
        match command {
            Command::Help => Ok(CommandResult::Help(self.generate_help_text())),
            Command::Exit => Ok(CommandResult::Exit),
            Command::Clear => {
                print!("\x1B[2J\x1B[1;1H");
                Ok(CommandResult::Continue)
            }
            Command::BuilderList {
                verbose,
                status_filter,
            } => {
                self.list_builders(verbose, status_filter).await?;
                Ok(CommandResult::Continue)
            }
        }
    }

    /// 生成帮助文本
    fn generate_help_text(&self) -> String {
        let mut help = String::new();
        help.push_str("可用命令:\n");
        help.push_str("  help, h           - 显示此帮助信息\n");
        help.push_str("  exit, quit, q     - 退出交互式模式\n");
        help.push_str("  clear, cls        - 清屏\n");
        help.push_str("\n");
        help.push_str("构建器命令:\n");
        help.push_str("  builder list      - 列出所有构建器\n");
        help.push_str("  builder list -v   - 列出构建器(详细模式)\n");
        help.push_str("  builder list --status <状态>\n");
        help.push_str("                    - 按状态过滤构建器\n");
        help.push_str("  b list            - builder list 的简写\n");
        help.push_str("\n");
        help.push_str("可用状态:\n");
        help.push_str("  running     - 运行中\n");
        help.push_str("  stopped     - 已停止\n");
        help.push_str("  created     - 已创建\n");
        help.push_str("  notcreated  - 未创建\n");
        help.push_str("  error       - 错误\n");
        help.push_str("\n");
        help.push_str("示例:\n");
        help.push_str("  builder list --status running\n");
        help.push_str("  b list -v\n");
        help.push_str("  builder list --status stopped\n");
        help
    }

    /// 列出构建器
    async fn list_builders(&self, verbose: bool, status_filter: Option<String>) -> Result<()> {
        let builders = self.builder_manager.list_builders();

        if builders.is_empty() {
            println!("没有找到任何构建器");
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

        // 显示构建器表格
        println!("构建器列表:");
        println!(
            "{}",
            self.builder_manager
                .format_filtered_builders_table(&filtered_builders)
        );

        // 显示统计信息
        let stats = self.builder_manager.get_filtered_stats(&filtered_builders);
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
}
