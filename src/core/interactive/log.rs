//! Author: xiaoYown
//! Created: 2025-03-03
//! Description: Log menu implementation

use anyhow::Result;
use inquire::Select;

use super::{
    menu::InteractiveMenu,
    ui::{InteractiveLogUI, InteractiveOptionUI, InteractiveUI},
};
use crate::core::logger::log;
use crate::infra::config::ConfKitConfigLoader;

impl InteractiveMenu {
    pub async fn log(&mut self) -> Result<bool> {
        let options = vec![
            InteractiveLogUI::List,
            InteractiveLogUI::Show,
            InteractiveLogUI::Info,
            InteractiveLogUI::Back,
        ];

        let selection = Select::new("Select log option:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        match selection {
            InteractiveLogUI::List => {
                let (space, project) = match self.select_space_project().await? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                log::list_task_logs(&space, &project)?;
            }
            InteractiveLogUI::Show => {
                let (space, project) = match self.select_space_project().await? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                let task_id = match Self::select_task_entry(&space, &project)? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                log::print_task_log(&space, &project, &task_id)?;
            }
            InteractiveLogUI::Info => {
                let (space, project) = match self.select_space_project().await? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                let task_id = match Self::select_task_entry(&space, &project)? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                log::print_task_info(&space, &project, &task_id)?;
            }
            InteractiveLogUI::Back => {
                self.ui = InteractiveUI::Main;
            }
        }

        Ok(true)
    }

    /// 交互式选择 space 和 project，返回 None 表示用户选择了返回
    async fn select_space_project(&mut self) -> Result<Option<(String, String)>> {
        let spaces = ConfKitConfigLoader::get_space_list().await?;

        let mut options: Vec<String> = vec![];
        for space in spaces {
            let projects = ConfKitConfigLoader::get_project_config_list(&space.name).await?;
            for project in projects {
                options.push(format!("<{}> {}", space.name, project.name));
            }
        }

        options.push(InteractiveOptionUI::Back.to_string());
        options.sort();

        let selection = Select::new("Select space/project:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        if selection == InteractiveOptionUI::Back.to_string() {
            self.ui = InteractiveUI::Main;
            return Ok(None);
        }

        let re = regex::Regex::new(r"^<([^>]+)>\s").unwrap();
        let caps = re.captures(&selection).unwrap();
        let space_name = caps.get(1).unwrap().as_str().to_string();
        let project_name =
            selection.replace(format!("<{space_name}>").as_str(), "").trim().to_string();

        Ok(Some((space_name, project_name)))
    }

    /// 交互式选择任务日志条目，返回 task_dir_name，None 表示无日志或用户返回
    fn select_task_entry(space: &str, project: &str) -> Result<Option<String>> {
        let entries = log::collect_task_entries(space, project)?;

        if entries.is_empty() {
            tracing::info!("No logs found for {}/{}", space, project);
            return Ok(None);
        }

        let labels: Vec<&str> = entries.iter().map(|(label, _)| label.as_str()).collect();

        let selection = Select::new("Select task log:", labels)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        // 找到选中项对应的 task_dir_name
        let task_dir_name = entries
            .iter()
            .find(|(label, _)| label == selection)
            .map(|(_, dir_name)| dir_name.clone());

        Ok(task_dir_name)
    }
}
