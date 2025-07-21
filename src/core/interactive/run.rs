//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Run menu implementation

use anyhow::Result;
use inquire::Select;

use crate::infra::config::ConfKitConfigLoader;

use super::{
    menu::InteractiveMenu,
    ui::{InteractiveMainUI, InteractiveUI},
};

impl InteractiveMenu {
    pub async fn run(&mut self) -> Result<bool> {
        // 获取 spaces
        let spaces = ConfKitConfigLoader::get_space_list().await?;

        let mut options: Vec<String> = vec![];

        for space in spaces {
            let projects = ConfKitConfigLoader::get_project_config_list(&space.name).await?;

            for project in projects {
                let option = format!("<{}> {}", space.name, project.name);
                options.push(option);
            }
        }

        options.push(InteractiveMainUI::Quit.to_string());

        let selection = Select::new("Please select a space:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        if selection == InteractiveMainUI::Quit.to_string() {
            return Ok(true);
        }

        tracing::info!("Selected space: {}", selection);

        self.ui = InteractiveUI::Main;

        Ok(true)
    }
}
