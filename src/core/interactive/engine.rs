use super::builder::*;
use super::help::*;
use super::menu::*;
use super::types::{
    BuilderStatusOption, Command, CommandContext, InteractiveConfig, InteractiveMode, MenuItem,
};
use crate::core::builder::BuilderManager;
use anyhow::Result;

/// 交互式引擎
pub struct InteractiveEngine {
    /// 命令上下文
    context: CommandContext,
    /// 构建器管理器
    pub(crate) builder_manager: BuilderManager,
    /// 当前交互模式
    pub(crate) current_mode: InteractiveMode,
}

impl InteractiveEngine {
    /// 创建新的交互式引擎
    pub fn new(config: InteractiveConfig) -> Result<Self> {
        let context = CommandContext::new(config);
        let builder_manager = BuilderManager::from_current_directory()
            .unwrap_or_else(|_| BuilderManager::with_demo_data());
        Ok(Self { context, builder_manager, current_mode: InteractiveMode::MainMenu })
    }

    /// 启动交互式会话
    pub async fn run(&mut self) -> Result<()> {
        println!("🚀 欢迎使用 ConfKit 交互式模式!");
        println!("使用 ↑↓ 方向键选择，Enter 确认，Ctrl+C 退出");
        println!();
        loop {
            match &self.current_mode.clone() {
                InteractiveMode::MainMenu => {
                    if !self.show_main_menu().await? {
                        break;
                    }
                }
                InteractiveMode::BuilderMenu => {
                    if !self.show_builder_menu().await? {
                        break;
                    }
                }
                InteractiveMode::BuilderListParams { verbose, status_filter } => {
                    if !self.show_builder_list_params(*verbose, status_filter.clone()).await? {
                        break;
                    }
                }
            }
        }
        println!("👋 再见!");
        Ok(())
    }

    /// 处理传统命令 (向后兼容)
    pub async fn handle_legacy_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Help => {
                self.show_main_help().await?;
            }
            Command::Clear => {
                print!("\x1B[2J\x1B[1;1H");
            }
            Command::BuilderList { verbose, status_filter } => {
                match self.builder_manager.list_builders_with_filter(verbose, status_filter) {
                    Ok(output) => {
                        println!("{}", output);
                    }
                    Err(e) => {
                        println!("❌ 获取构建器列表失败: {}", e);
                    }
                }
            }
            Command::Exit => {}
        }
        Ok(())
    }
}
