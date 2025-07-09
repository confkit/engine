use super::{InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示主菜单
    pub async fn show_main_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[BUILDER] Builder 管理 - 管理构建环境容器",
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

    /// 显示Builder菜单
    pub async fn show_builder_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[LIST] 列出构建器 - 查看所有构建器的状态",
            "[NEW] 创建构建器 - 创建新的构建器 (即将推出)",
            "[START] 启动构建器 - 启动指定的构建器 (即将推出)",
            "[STOP] 停止构建器 - 停止指定的构建器 (即将推出)",
            "[DEL] 删除构建器 - 删除指定的构建器 (即将推出)",
            "[HEALTH] 健康检查 - 检查构建器健康状态 (即将推出)",
            "[BACK] 返回主菜单",
        ];

        let selection = Select::new("Builder 管理菜单:", options)
            .with_help_message("选择要执行的 Builder 操作")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[LIST]") => {
                    self.current_mode =
                        InteractiveMode::BuilderListParams { verbose: false, status_filter: None };
                    Ok(true)
                }
                choice if choice.starts_with("[NEW]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[START]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[STOP]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[DEL]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[HEALTH]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[BACK]") => {
                    self.current_mode = InteractiveMode::MainMenu;
                    Ok(true)
                }
                _ => Ok(true),
            },
            Err(_) => {
                // 用户中断，回到主菜单
                self.current_mode = InteractiveMode::MainMenu;
                Ok(true)
            }
        }
    }
}
