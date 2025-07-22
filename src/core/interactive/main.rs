//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Main menu implementation

use anyhow::Result;
use inquire::Select;

use super::{
    menu::InteractiveMenu,
    ui::{InteractiveMainUI, InteractiveUI},
};

impl InteractiveMenu {
    pub async fn main(&mut self) -> Result<bool> {
        let options = vec![
            InteractiveMainUI::Run,
            InteractiveMainUI::Builder,
            InteractiveMainUI::Image,
            InteractiveMainUI::Log,
            InteractiveMainUI::Quit,
        ];

        let selection = Select::new("Please select an option:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                InteractiveMainUI::Run => {
                    self.ui = InteractiveUI::Run;
                    Ok(true)
                }
                InteractiveMainUI::Builder => {
                    self.ui = InteractiveUI::Builder;
                    Ok(true)
                }
                InteractiveMainUI::Image => {
                    self.ui = InteractiveUI::Image;
                    Ok(true)
                }
                InteractiveMainUI::Log => Ok(true),
                InteractiveMainUI::Quit => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }
}
