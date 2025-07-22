//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Run menu implementation

use anyhow::Result;
use inquire::Select;
use regex::Regex;

use crate::{core::executor::runner::Runner, infra::config::ConfKitConfigLoader};

use super::{
    menu::InteractiveMenu,
    ui::{InteractiveOptionUI, InteractiveUI},
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

        options.push(InteractiveOptionUI::Back.to_string());

        let selection = Select::new("Please select a space:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        if selection == InteractiveOptionUI::Back.to_string() {
            self.ui = InteractiveUI::Main;
            return Ok(true);
        }

        // 匹配出 space: 以 < 开头，以 > 结尾
        let re = Regex::new(r"^<([^>]+)>\s").unwrap();
        let caps = re.captures(&selection).unwrap();
        let space_name = caps.get(1).unwrap().as_str();

        // 匹配出 project: 从 selection 中移除掉 <space_name>
        let project_name =
            selection.replace(format!("<{}>", space_name).as_str(), "").trim().to_string();

        tracing::info!("space_name: {}, project_name: {}", space_name, project_name);

        let mut runner = Runner::new(space_name, &project_name).await?;

        runner.start().await?;

        self.ui = InteractiveUI::Run;

        Ok(true)
    }
}
