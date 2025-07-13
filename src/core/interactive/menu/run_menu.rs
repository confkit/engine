use super::super::{InteractiveEngine, InteractiveMode};
use crate::core::project::{ProjectLoader, ProjectRunner, RunOptions};
use anyhow::Result;
use inquire::{Confirm, Select};

impl InteractiveEngine {
    /// 显示运行菜单
    pub async fn show_run_menu(&mut self) -> Result<bool> {
        let options = vec!["[RUN] 运行项目", "[LIST] 列出可用项目", "[BACK] 返回主菜单"];

        let selection = Select::new("请选择运行操作:", options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[LIST]") => {
                    self.show_project_list().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[RUN]") => {
                    self.current_mode = InteractiveMode::RunProjectParams;
                    Ok(true)
                }
                choice if choice.starts_with("[BACK]") => {
                    self.current_mode = InteractiveMode::MainMenu;
                    Ok(true)
                }
                _ => Ok(true),
            },
            Err(_) => {
                self.current_mode = InteractiveMode::MainMenu;
                Ok(true)
            }
        }
    }

    /// 显示项目列表
    pub async fn show_project_list(&mut self) -> Result<()> {
        println!("● 扫描可用项目...");

        let loader = match ProjectLoader::from_current_dir() {
            Ok(loader) => loader,
            Err(e) => {
                println!("✗ 无法创建项目加载器: {}", e);
                return Ok(());
            }
        };

        let spaces = match loader.list_spaces().await {
            Ok(spaces) => spaces,
            Err(e) => {
                println!("✗ 无法列出空间: {}", e);
                return Ok(());
            }
        };

        if spaces.is_empty() {
            println!("※ 未找到任何空间配置");
            return Ok(());
        }

        println!("✓ 发现以下项目:");
        println!();

        for space_name in spaces {
            println!("▶ 空间: {}", space_name);

            let projects = match loader.list_projects(&space_name).await {
                Ok(projects) => projects,
                Err(e) => {
                    println!("  ✗ 无法列出空间 '{}' 中的项目: {}", space_name, e);
                    continue;
                }
            };

            if projects.is_empty() {
                println!("  ※ 该空间中没有项目");
            } else {
                for project in projects {
                    println!("  • {} ({})", project.name, project.description);
                }
            }
        }
        println!();

        Ok(())
    }

    /// 显示运行项目参数选择
    pub async fn show_run_project_params(&mut self) -> Result<bool> {
        // 获取项目列表
        let loader = match ProjectLoader::from_current_dir() {
            Ok(loader) => loader,
            Err(e) => {
                println!("✗ 无法创建项目加载器: {}", e);
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::RunMenu;
                return Ok(true);
            }
        };

        let spaces = match loader.list_spaces().await {
            Ok(spaces) => spaces,
            Err(e) => {
                println!("✗ 无法列出空间: {}", e);
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::RunMenu;
                return Ok(true);
            }
        };

        if spaces.is_empty() {
            println!("※ 未找到任何空间配置");
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::RunMenu;
            return Ok(true);
        }

        // 构建项目选择列表
        let mut project_options = Vec::new();
        for space_name in &spaces {
            let projects = match loader.list_projects(space_name).await {
                Ok(projects) => projects,
                Err(e) => {
                    println!("✗ 无法列出空间 '{}' 中的项目: {}", space_name, e);
                    continue;
                }
            };

            for project in projects {
                project_options.push(format!("{}/{}", space_name, project.name));
            }
        }

        // 选择项目
        let selection = Select::new("请选择要运行的项目:", project_options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        let selected_project = match selection {
            Ok(choice) => choice,
            Err(_) => {
                self.current_mode = InteractiveMode::RunMenu;
                return Ok(true);
            }
        };

        // 解析选择的项目
        let parts: Vec<&str> = selected_project.split('/').collect();
        if parts.len() != 2 {
            println!("✗ 无效的项目选择");
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::RunMenu;
            return Ok(true);
        }

        let space = parts[0].to_string();
        let project = parts[1].to_string();

        // 确认执行
        println!();
        println!("准备运行项目: {}/{}", space, project);
        println!();

        let confirm = Confirm::new("确认执行?").with_default(true).prompt().unwrap_or(false);

        if confirm {
            self.current_mode = InteractiveMode::RunProjectExecution {
                space,
                project,
                verbose: false,   // 使用默认值
                dry_run: false,   // 使用默认值
                git_branch: None, // 使用默认值
                force: false,     // 使用默认值
            };
        } else {
            self.current_mode = InteractiveMode::RunMenu;
        }

        Ok(true)
    }

    /// 执行运行项目
    pub async fn show_run_project_execution(
        &mut self,
        space: String,
        project: String,
        verbose: bool,
        dry_run: bool,
        git_branch: Option<String>,
        force: bool,
    ) -> Result<bool> {
        println!("● 开始执行项目: {}/{}", space, project);
        println!();

        // 创建项目加载器
        let loader = match ProjectLoader::from_current_dir() {
            Ok(loader) => loader,
            Err(e) => {
                println!("✗ 无法创建项目加载器: {}", e);
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::RunMenu;
                return Ok(true);
            }
        };

        // 加载项目配置
        let (space_config, project_config) =
            match loader.load_project_config(&space, &project).await {
                Ok(configs) => configs,
                Err(e) => {
                    println!("✗ 无法加载项目配置: {}", e);
                    self.pause_for_user().await?;
                    self.current_mode = InteractiveMode::RunMenu;
                    return Ok(true);
                }
            };

        // 创建运行选项
        let options = RunOptions { verbose, dry_run, git_branch, force };

        // 创建项目执行引擎
        let runner = match ProjectRunner::new().await {
            Ok(runner) => runner,
            Err(e) => {
                println!("✗ 无法创建项目执行引擎: {}", e);
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::RunMenu;
                return Ok(true);
            }
        };

        // 执行项目
        match runner.run_project(space, project, space_config, project_config, options).await {
            Ok(result) => {
                println!();
                match result.status {
                    crate::core::project::types::TaskStatus::Success => {
                        println!("✓ 项目执行成功! 任务ID: {}", result.task_id);
                        if let Some(duration) = result.total_duration_ms {
                            println!("  总耗时: {:.1}s", duration as f64 / 1000.0);
                        }
                    }
                    crate::core::project::types::TaskStatus::Failed => {
                        println!("✗ 项目执行失败! 任务ID: {}", result.task_id);
                    }
                    _ => {
                        println!("※ 项目执行状态异常: {:?}", result.status);
                    }
                }
            }
            Err(e) => {
                println!("✗ 项目执行出错: {}", e);
            }
        }

        println!();
        self.pause_for_user().await?;
        self.current_mode = InteractiveMode::RunMenu;
        Ok(true)
    }
}
