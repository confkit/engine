use super::super::{InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示主菜单
    pub async fn show_main_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[RUN] Run 管理 - 执行项目构建任务",
            "[BUILDER] Builder 管理 - 管理构建镜像和环境",
            "[LOG] Log 管理 - 查看和管理日志文件",
            "[HELP] 帮助信息",
            "[QUIT] 退出程序",
        ];

        let selection = Select::new("请选择要执行的操作:", options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[RUN]") => {
                    self.current_mode = InteractiveMode::RunMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[BUILDER]") => {
                    self.current_mode = InteractiveMode::BuilderMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[LOG]") => {
                    self.current_mode = InteractiveMode::LogMenu;
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
