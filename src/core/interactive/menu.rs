use anyhow::Result;

use super::ui::InteractiveUI;

pub struct InteractiveMenu {
    pub ui: InteractiveUI,
}

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
                // InteractiveUI::Builder => {
                //     if !self.builder().await? {
                //         break;
                //     }
                // }
                InteractiveUI::Image => {
                    if !self.image().await? {
                        break;
                    }
                }
                // InteractiveUI::Log => {
                //     if !self.log().await? {
                //         break;
                //     }
                // }
                // InteractiveUI::Quit => {
                //     break;
                // }
                _ => todo!(),
            }
        }
        Ok(())
    }
}
