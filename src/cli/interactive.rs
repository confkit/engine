//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Interactive mode subcommand implementation

use anyhow::Result;

use crate::core::interactive::menu::InteractiveMenu;

pub struct InteractiveCommand;

impl InteractiveCommand {
    pub async fn execute() -> Result<()> {
        tracing::info!("Starting interactive mode...");

        let mut menu = InteractiveMenu::new();

        menu.execute().await?;

        Ok(())
    }
}
