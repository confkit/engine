//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Builder management implementation

use anyhow::Result;
use inquire::Select;

use crate::core::{
    builder::container::ContainerBuilder,
    interactive::ui::{InteractiveUI, InteractiveYesNoUI},
};

use super::{menu::InteractiveMenu, ui::InteractiveBuilderUI};

impl InteractiveMenu {
    // 构建管理菜单
    pub async fn builder(&mut self) -> Result<bool> {
        let options = vec![
            InteractiveBuilderUI::List,
            InteractiveBuilderUI::Start,
            InteractiveBuilderUI::Stop,
            InteractiveBuilderUI::Restart,
            InteractiveBuilderUI::Create,
            InteractiveBuilderUI::Remove,
            InteractiveBuilderUI::Back,
        ];

        let selection = Select::new("Please select an option:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                InteractiveBuilderUI::List => {
                    ContainerBuilder::print_list().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Start => {
                    self.start_builder().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Stop => {
                    self.stop_builder().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Restart => {
                    self.restart_builder().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Create => {
                    self.create_builder().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Remove => {
                    self.remove_builder().await?;
                    Ok(true)
                }
                InteractiveBuilderUI::Back => {
                    self.ui = InteractiveUI::Main;
                    Ok(true)
                }
            },
            Err(_) => {
                // 用户按了 Ctrl+C 或其他中断
                Ok(false)
            }
        }
    }

    // 创建 builder 容器
    pub async fn create_builder(&mut self) -> Result<bool> {
        let container_name = self.select_container().await?;

        if container_name == "Back" {
            self.ui = InteractiveUI::Builder;
            return Ok(true);
        }

        if container_name == "All" {
            let force = self.select_force().await?;
            let force = force == InteractiveYesNoUI::Yes;
            ContainerBuilder::create_all(force).await?;
            return Ok(true);
        }

        let force = self.select_force().await?;
        let force = force == InteractiveYesNoUI::Yes;

        ContainerBuilder::create(&container_name, force).await?;

        Ok(true)
    }

    // 移除 builder 容器
    pub async fn remove_builder(&mut self) -> Result<bool> {
        let container_name = self.select_container().await?;

        if container_name == "Back" {
            self.ui = InteractiveUI::Builder;
            return Ok(true);
        }

        if container_name == "All" {
            let force = self.select_force().await?;
            let force = force == InteractiveYesNoUI::Yes;
            ContainerBuilder::remove_all(force).await?;
            return Ok(true);
        }

        let force = self.select_force().await?;
        let force = force == InteractiveYesNoUI::Yes;

        ContainerBuilder::remove(&container_name, force).await?;

        Ok(true)
    }

    // 启动 builder 容器
    pub async fn start_builder(&mut self) -> Result<bool> {
        let container_name = self.select_container().await?;

        if container_name == "Back" {
            self.ui = InteractiveUI::Builder;
            return Ok(true);
        }

        if container_name == "All" {
            ContainerBuilder::start_all().await?;
            return Ok(true);
        }

        ContainerBuilder::start(&container_name).await?;
        Ok(true)
    }

    // 停止 builder 容器
    pub async fn stop_builder(&mut self) -> Result<bool> {
        let container_name = self.select_container().await?;

        if container_name == "Back" {
            self.ui = InteractiveUI::Builder;
            return Ok(true);
        }

        if container_name == "All" {
            ContainerBuilder::stop_all().await?;
            return Ok(true);
        }

        ContainerBuilder::stop(&container_name).await?;
        Ok(true)
    }

    // 重启 builder 容器
    pub async fn restart_builder(&mut self) -> Result<bool> {
        let container_name = self.select_container().await?;

        if container_name == "Back" {
            self.ui = InteractiveUI::Builder;
            return Ok(true);
        }

        if container_name == "All" {
            let force = self.select_force().await?;
            let force = force == InteractiveYesNoUI::Yes;
            ContainerBuilder::restart_all(force).await?;
            return Ok(true);
        }

        let force = self.select_force().await?;
        let force = force == InteractiveYesNoUI::Yes;

        ContainerBuilder::restart(Some(container_name), force).await?;
        Ok(true)
    }

    // 选择镜像
    pub async fn select_container(&self) -> Result<String> {
        let container_name = ContainerBuilder::get_list().await?;

        // 显示: name(status)
        let mut options: Vec<String> = container_name
            .iter()
            .map(|container| format!("{} ({})", container.name, container.status))
            .collect();

        // 添加全部选项
        options.insert(0, "All".to_string());

        // 添加返回选项
        options.push("Back".to_string());

        let selection = Select::new("Please select an container:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        let selection = match selection {
            Ok(selection) => selection,
            Err(_) => return Err(anyhow::anyhow!("Failed to select container")),
        };

        if selection == "All" {
            return Ok(String::from("All"));
        }

        if selection == "Back" {
            return Ok(String::from("Back"));
        }

        let container_name = selection.split(' ').next().unwrap().to_string();

        Ok(container_name)
    }
}
