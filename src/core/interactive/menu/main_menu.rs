use super::super::{InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示主菜单
    pub async fn show_main_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[BUILDER] Builder 管理 - 管理构建镜像和环境",
            "[TASK] Task 管理 - 管理构建任务 (即将推出)",
            "[CONFIG] Config 管理 - 管理项目配置 (即将推出)",
            "[GIT] Git 管理 - 管理 Git 仓库 (即将推出)",
            "[HELP] 帮助信息",
            "[QUIT] 退出程序",
        ];

        let selection = Select::new("请选择要执行的操作:", options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[BUILDER]") => {
                    self.current_mode = InteractiveMode::BuilderMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[TASK]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[CONFIG]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[GIT]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[HELP]") => {
                    self.show_main_help().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[QUIT]") => Ok(false),
                _ => Ok(true),
            },
            Err(_) => {
                // 用户按了 Ctrl+C 或其他中断
                Ok(false)
            }
        }
    }
}
