use super::super::{InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示Builder菜单
    pub async fn show_builder_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[IMAGE] 镜像管理 - 管理构建镜像",
            "[CONTAINER] 容器管理 - 管理构建器容器",
            "[BACK] 返回主菜单",
        ];

        let selection = Select::new("Builder 管理菜单:", options)
            .with_help_message("选择要执行的 Builder 操作")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[IMAGE]") => {
                    self.current_mode = InteractiveMode::ImageMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[CONTAINER]") => {
                    self.current_mode = InteractiveMode::ContainerMenu;
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

    /// 显示 Builder Create 参数选择界面 (保留向后兼容)
    pub async fn show_builder_create_params(&mut self) -> Result<bool> {
        // 直接重定向到新的镜像创建参数选择界面
        self.current_mode = InteractiveMode::ImageCreateParams;
        Ok(true)
    }
}
