//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Image management implementation

use anyhow::Result;
use inquire::Select;

use crate::core::{builder::image::ImageBuilder, interactive::ui::InteractiveUI};

use super::{menu::InteractiveMenu, ui::InteractiveImageUI};

impl InteractiveMenu {
    // 镜像管理菜单
    pub async fn image(&mut self) -> Result<bool> {
        let options = vec![
            InteractiveImageUI::List,
            InteractiveImageUI::Create,
            InteractiveImageUI::Remove,
            InteractiveImageUI::Back,
        ];

        let selection = Select::new("Please select an option:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                InteractiveImageUI::List => {
                    ImageBuilder::print_list().await?;
                    Ok(true)
                }
                InteractiveImageUI::Create => {
                    self.create_image().await?;
                    Ok(true)
                }
                InteractiveImageUI::Remove => {
                    self.remove_image().await?;
                    Ok(true)
                }
                InteractiveImageUI::Back => {
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

    // 创建镜像
    pub async fn create_image(&mut self) -> Result<bool> {
        let (image_name, image_tag) = self.select_image().await?;

        // 全部创建
        if image_name == "All" && image_tag == "" {
            ImageBuilder::build_all().await?;
            return Ok(true);
        }

        if image_name == "Back" {
            self.ui = InteractiveUI::Image;
            return Ok(true);
        }

        ImageBuilder::build(&image_name, &image_tag).await?;

        Ok(true)
    }

    // 移除镜像
    pub async fn remove_image(&mut self) -> Result<bool> {
        let (image_name, image_tag) = self.select_image().await?;

        if image_name == "Back" {
            self.ui = InteractiveUI::Image;
            return Ok(true);
        }

        if image_name == "All" && image_tag == "" {
            ImageBuilder::remove_all().await?;
            return Ok(true);
        }

        ImageBuilder::remove(&image_name, &image_tag).await?;
        Ok(true)
    }

    // 选择镜像
    async fn select_image(&self) -> Result<(String, String)> {
        let images = ImageBuilder::get_list().await?;

        // 显示: name:tag(status)
        let mut options: Vec<String> = images
            .iter()
            .map(|image| format!("{}:{} ({})", image.name, image.tag, image.status))
            .collect();

        // 添加全部选项
        options.insert(0, "All".to_string());

        // 添加返回选项
        options.push("Back".to_string());

        let selection = Select::new("Please select an image:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        let selection = match selection {
            Ok(selection) => selection,
            Err(_) => return Err(anyhow::anyhow!("Failed to select image")),
        };

        if selection == "All" {
            return Ok((String::from("All"), String::from("")));
        }

        if selection == "Back" {
            return Ok((String::from("Back"), String::from("")));
        }

        // 提取 name 和 tag
        let mut parts = selection.split(':');
        let name = parts.next().unwrap().to_string();
        let tag = parts.next().unwrap().split(' ').next().unwrap().to_string();

        Ok((name, tag))
    }
}
