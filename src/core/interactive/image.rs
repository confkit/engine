use anyhow::Result;
use inquire::Select;

use crate::core::{builder::image::ImageBuilder, interactive::ui::InteractiveUI};

use super::{menu::InteractiveMenu, ui::InteractiveImageUI};

impl InteractiveMenu {
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
                    ImageBuilder::print_image_list().await?;
                    Ok(true)
                }
                InteractiveImageUI::Create => Ok(true),
                InteractiveImageUI::Remove => Ok(true),
                InteractiveImageUI::Back => {
                    self.ui = InteractiveUI::Main;
                    Ok(true)
                }
                _ => Ok(true),
            },
            Err(_) => {
                // 用户按了 Ctrl+C 或其他中断
                Ok(false)
            }
        }
    }
}
