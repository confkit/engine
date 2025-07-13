use super::InteractiveEngine;
use anyhow::Result;
use inquire::Confirm;

impl InteractiveEngine {
    /// 显示主菜单帮助
    pub async fn show_main_help(&mut self) -> Result<()> {
        println!("📖 ConfKit 交互式模式帮助");
        println!("{}", "=".repeat(50));
        println!();
        println!("ConfKit 是一个强大的构建工具，提供以下功能:");
        println!();
        println!("🔧 Builder 管理:");
        println!("   • 管理 Docker 构建环境容器");
        println!("   • 任意配置语言和框架");
        println!("   • 统一的构建环境配置");
        println!();
        println!("▶ Run 管理:");
        println!("   • 执行项目构建任务");
        println!("   • 支持多种构建配置");
        println!("   • 自动化构建流程");
        println!();
        println!("📋 Log 管理:");
        println!("   • 查看项目构建日志");
        println!("   • 支持文件名和任务ID匹配");
        println!("   • 实时跟踪日志输出");
        println!();
        println!("※ Task 管理 (即将推出):");
        println!("   • 定义和执行构建任务");
        println!("   • 任务依赖管理");
        println!("   • 并行执行支持");
        println!();
        println!("💡 交互提示:");
        println!("   • 使用 ↑↓ 方向键在选项间移动");
        println!("   • 按 Enter 键确认选择");
        println!("   • 按 Ctrl+C 随时退出或返回");
        println!("   • 本模式支持引导式操作，适合新用户");
        println!("   • 也支持直接命令输入，如 'confkit builder list --verbose'");
        println!();

        self.pause_for_user().await?;
        Ok(())
    }

    /// 暂停等待用户
    pub async fn pause_for_user(&self) -> Result<()> {
        let _confirm = Confirm::new("按 Enter 键继续...")
            .with_default(true)
            .with_help_message("按任意键继续")
            .prompt()
            .unwrap_or(true);
        Ok(())
    }
}
