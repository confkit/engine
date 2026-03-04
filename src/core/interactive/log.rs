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
use crate::infra::db::task_db::{PageParams, TaskFilter};

/// 日志查询范围
enum LogScope {
    /// 全部项目
    All,
    /// 指定 space
    Space(String),
    /// 指定 space + project
    Project(String, String),
}

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
                let scope = match self.select_log_scope().await? {
                    Some(s) => s,
                    None => return Ok(true),
                };
                let filter = scope_to_filter(&scope);
                let page_params = PageParams { page: 1, size: 20 };
                log::list_task_logs(&filter, &page_params)?;
            }
            InteractiveLogUI::Show => {
                let scope = match self.select_log_scope().await? {
                    Some(s) => s,
                    None => return Ok(true),
                };
                let task_id = match Self::select_task_entry_filtered(&scope)? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                log::print_task_log(&task_id)?;
            }
            InteractiveLogUI::Info => {
                let scope = match self.select_log_scope().await? {
                    Some(s) => s,
                    None => return Ok(true),
                };
                let task_id = match Self::select_task_entry_filtered(&scope)? {
                    Some(v) => v,
                    None => return Ok(true),
                };
                log::print_task_info(&task_id)?;
            }
            InteractiveLogUI::Back => {
                self.ui = InteractiveUI::Main;
            }
        }

        Ok(true)
    }

    /// 交互式选择日志查询范围：全部 / 指定 space / 指定 space+project
    async fn select_log_scope(&mut self) -> Result<Option<LogScope>> {
        let scope_options = vec![
            "[ALL] All projects".to_string(),
            "[SPACE] Filter by space".to_string(),
            "[PROJECT] Filter by space/project".to_string(),
            InteractiveOptionUI::Back.to_string(),
        ];

        let selection = Select::new("Select query scope:", scope_options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        if selection == InteractiveOptionUI::Back.to_string() {
            return Ok(None);
        }

        if selection.starts_with("[ALL]") {
            return Ok(Some(LogScope::All));
        }

        if selection.starts_with("[SPACE]") {
            let space = match self.select_space().await? {
                Some(s) => s,
                None => return Ok(None),
            };
            return Ok(Some(LogScope::Space(space)));
        }

        // [PROJECT]
        let (space, project) = match self.select_space_and_project().await? {
            Some(v) => v,
            None => return Ok(None),
        };
        Ok(Some(LogScope::Project(space, project)))
    }

    /// 交互式选择 space
    async fn select_space(&mut self) -> Result<Option<String>> {
        let spaces = ConfKitConfigLoader::get_space_list().await?;

        let mut options: Vec<String> = spaces.iter().map(|s| s.name.clone()).collect();
        options.push(InteractiveOptionUI::Back.to_string());

        let selection = Select::new("Select space:", options)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        if selection == InteractiveOptionUI::Back.to_string() {
            return Ok(None);
        }

        Ok(Some(selection))
    }

    /// 交互式选择 space 和 project，返回 None 表示用户选择了返回
    async fn select_space_and_project(&mut self) -> Result<Option<(String, String)>> {
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
            return Ok(None);
        }

        let re = regex::Regex::new(r"^<([^>]+)>\s").unwrap();
        let caps = re.captures(&selection).unwrap();
        let space_name = caps.get(1).unwrap().as_str().to_string();
        let project_name =
            selection.replace(format!("<{space_name}>").as_str(), "").trim().to_string();

        Ok(Some((space_name, project_name)))
    }

    /// 根据查询范围交互式选择任务，返回 task_id
    fn select_task_entry_filtered(scope: &LogScope) -> Result<Option<String>> {
        let filter = scope_to_filter(scope);
        let entries = log::collect_task_entries_filtered(&filter)?;

        if entries.is_empty() {
            let desc = match scope {
                LogScope::All => "any project".to_string(),
                LogScope::Space(s) => format!("space '{}'", s),
                LogScope::Project(s, p) => format!("{}/{}", s, p),
            };
            tracing::info!("No logs found for {}", desc);
            return Ok(None);
        }

        let labels: Vec<&str> = entries.iter().map(|(label, _)| label.as_str()).collect();

        let selection = Select::new("Select task log:", labels)
            .with_help_message("Use ↑↓ to navigate, Enter to confirm")
            .prompt()?;

        let task_id =
            entries.iter().find(|(label, _)| label == selection).map(|(_, id)| id.clone());

        Ok(task_id)
    }
}

fn scope_to_filter(scope: &LogScope) -> TaskFilter {
    match scope {
        LogScope::All => TaskFilter { space_name: None, project_name: None },
        LogScope::Space(s) => TaskFilter { space_name: Some(s.clone()), project_name: None },
        LogScope::Project(s, p) => {
            TaskFilter { space_name: Some(s.clone()), project_name: Some(p.clone()) }
        }
    }
}
