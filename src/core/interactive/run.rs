//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Run menu implementation

use std::collections::HashMap;

use anyhow::Result;
use inquire::Select;
use regex::Regex;

use crate::{core::executor::runner::Runner, infra::config::ConfKitConfigLoader};

use super::{
    custom::process_interactive_environments,
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

        options.sort();

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
            selection.replace(format!("<{space_name}>").as_str(), "").trim().to_string();

        tracing::info!("space_name: {space_name}, project_name: {project_name}");

        // 获取项目配置
        let project_config =
            ConfKitConfigLoader::get_project_config(space_name, &project_name).await?;

        // 处理交互式环境变量
        let mut environment_from_args = HashMap::new();

        // 如果项目配置中有交互式环境变量配置，则处理
        if let Some(project_config) = project_config {
            if let Some(interactive_configs) = &project_config.environment_from_args {
                if !interactive_configs.is_empty() {
                    tracing::info!("发现交互式环境变量配置，开始处理...");
                    environment_from_args =
                        process_interactive_environments(interactive_configs).await?;
                }
            }
        }

        let mut runner = Runner::new(space_name, &project_name, environment_from_args).await?;

        runner.start().await?;

        self.ui = InteractiveUI::Run;

        Ok(true)
    }
}
