//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Interactive menu implementation

use anyhow::Result;
use inquire::Select;

use crate::core::interactive::ui::InteractiveYesNoUI;

use super::ui::InteractiveUI;

pub struct InteractiveMenu {
    pub ui: InteractiveUI,
}

// 基础交互菜单
impl InteractiveMenu {
    pub fn new() -> Self {
        Self { ui: InteractiveUI::Main }
    }

    pub async fn execute(&mut self) -> Result<()> {
        loop {
            match &self.ui {
                InteractiveUI::Main => {
                    if !self.main().await? {
                        break;
                    }
                }
                InteractiveUI::Run => {
                    if !self.run().await? {
                        break;
                    }
                }
                InteractiveUI::Image => {
                    if !self.image().await? {
                        break;
                    }
                }
                InteractiveUI::Builder => {
                    if !self.builder().await? {
                        break;
                    }
                }
                InteractiveUI::Clean => {
                    if !self.clean().await? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

// 公用交互方法
impl InteractiveMenu {
    // 选择是否强制
    pub async fn select_force(&self) -> Result<InteractiveYesNoUI> {
        let options = vec![InteractiveYesNoUI::Yes, InteractiveYesNoUI::No];

        let selection = Select::new("Force operation?", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt();

        match selection {
            Ok(choice) => Ok(choice),
            Err(_) => Ok(InteractiveYesNoUI::No),
        }
    }
}
