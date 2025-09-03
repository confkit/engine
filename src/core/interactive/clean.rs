//! Author: xiaoYown
//! Created: 2025-01-09
//! Description: Clean menu implementation

use anyhow::Result;
use inquire::Select;

use super::{
    menu::InteractiveMenu,
    ui::{InteractiveCleanUI, InteractiveUI, InteractiveYesNoUI},
};
use crate::core::clean::{log::LogCleaner, volumes::VolumesCleaner};

impl InteractiveMenu {
    pub async fn clean(&mut self) -> Result<bool> {
        let options = vec![
            InteractiveCleanUI::Logs,
            InteractiveCleanUI::Workspace,
            InteractiveCleanUI::Artifacts,
            InteractiveCleanUI::All,
            InteractiveCleanUI::Back,
        ];

        let selection = Select::new("Select clean option:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        match selection {
            InteractiveCleanUI::Logs => {
                if self.confirm_action("clean all logs").await? {
                    LogCleaner::clean_all().await?;
                    println!("✓ All logs cleaned successfully");
                }
            },
            InteractiveCleanUI::Workspace => {
                if self.confirm_action("clean workspace directories").await? {
                    VolumesCleaner::clean_workspace().await?;
                    println!("✓ Workspace cleaned successfully");
                }
            },
            InteractiveCleanUI::Artifacts => {
                if self.confirm_action("clean build artifacts").await? {
                    VolumesCleaner::clean_artifacts().await?;
                    println!("✓ Artifacts cleaned successfully");
                }
            },
            InteractiveCleanUI::All => {
                if self.confirm_action("clean ALL (logs, workspace, artifacts)").await? {
                    println!("Cleaning workspace...");
                    VolumesCleaner::clean_workspace().await?;
                    
                    println!("Cleaning artifacts...");
                    VolumesCleaner::clean_artifacts().await?;
                    
                    println!("Cleaning all logs...");
                    LogCleaner::clean_all().await?;
                    
                    println!("✓ All resources cleaned successfully");
                }
            },
            InteractiveCleanUI::Back => {
                self.ui = InteractiveUI::Main;
            }
        }
        
        Ok(true)
    }


    async fn confirm_action(&self, action: &str) -> Result<bool> {
        let options = vec![InteractiveYesNoUI::Yes, InteractiveYesNoUI::No];
        let selection = Select::new(&format!("Confirm to {}?", action), options)
            .with_help_message("⚠️  This action cannot be undone!")
            .prompt()?;
        Ok(selection == InteractiveYesNoUI::Yes)
    }

}