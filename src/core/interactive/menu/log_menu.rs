//! 日志管理交互式菜单
//!
//! 提供日志查看操作的交互式界面

use super::super::{InteractiveEngine, InteractiveMode};
use crate::cli::log::{handle_log_list, handle_log_show, scan_log_directory, LogFileInfo};
use crate::core::project::ProjectLoader;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use std::path::PathBuf;

impl InteractiveEngine {
    /// 显示日志管理菜单 - 直接进入space选择
    pub async fn show_log_menu(&mut self) -> Result<bool> {
        if let Some((space, project)) = self.select_space_and_project().await? {
            self.current_mode = InteractiveMode::LogFileSelection { space, project };
            Ok(true)
        } else {
            // 返回主菜单
            self.current_mode = InteractiveMode::MainMenu;
            Ok(true)
        }
    }

    /// 显示日志文件选择和内容
    pub async fn show_log_file_selection(
        &mut self,
        space: String,
        project: String,
    ) -> Result<bool> {
        // 构建根日志目录路径
        let mut root_logs_dir = PathBuf::from("./volumes/logs");

        // 如果当前路径不存在，尝试 examples 目录下的路径
        if !root_logs_dir.exists() {
            root_logs_dir = PathBuf::from("./examples/volumes/logs");
        }

        // 构建具体的项目日志目录路径用于验证
        let project_logs_dir = root_logs_dir.join(&space).join(&project);

        // 添加调试信息
        println!("● 调试信息:");
        println!(
            "  当前工作目录: {:?}",
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("unknown"))
        );
        println!("  根日志目录: {}", root_logs_dir.display());
        println!("  项目日志目录: {}", project_logs_dir.display());
        println!("  项目日志目录是否存在: {}", project_logs_dir.exists());

        if !project_logs_dir.exists() {
            println!("※ 未找到空间 {} 项目 {} 的日志目录", space, project);
            println!("   路径: {}", project_logs_dir.display());

            // 尝试列出可能的路径
            println!("  正在检查可能的路径...");
            let possible_paths = vec![
                "./volumes/logs",
                "./examples/volumes/logs",
                "volumes/logs",
                "examples/volumes/logs",
            ];

            for path in possible_paths {
                let test_path = PathBuf::from(path);
                if test_path.exists() {
                    println!("  ✓ 找到路径: {}", test_path.display());
                } else {
                    println!("  ✗ 路径不存在: {}", test_path.display());
                }
            }

            self.current_mode = InteractiveMode::MainMenu;
            return Ok(true);
        }

        // 扫描日志文件 - 使用根目录，让scan_log_directory自己遍历
        let mut log_files = Vec::new();
        if let Err(e) = scan_log_directory(
            &root_logs_dir,
            &mut log_files,
            &Some(project.clone()),
            &Some(space.clone()),
        ) {
            println!("※ 扫描日志目录失败: {}", e);
            self.current_mode = InteractiveMode::MainMenu;
            return Ok(true);
        }

        if log_files.is_empty() {
            println!("※ 空间 {} 项目 {} 下没有找到日志文件", space, project);

            // 尝试直接列出目录内容作为调试信息
            println!("  调试: 直接列出目录内容:");
            if let Ok(entries) = std::fs::read_dir(&project_logs_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("    文件: {}", entry.file_name().to_string_lossy());
                    }
                }
            }

            self.current_mode = InteractiveMode::MainMenu;
            return Ok(true);
        }

        // 创建选择项
        let mut options: Vec<String> = log_files
            .iter()
            .map(|f| {
                let filename = f.path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
                format!("{} ({})", filename, f.size_mb)
            })
            .collect();

        options.push("[BACK] 返回主菜单".to_string());

        let selection =
            Select::new(&format!("选择要查看的日志文件 [{}:{}]:", space, project), options)
                .with_help_message("选择日志文件查看详细内容")
                .prompt();

        match selection {
            Ok(choice) => {
                if choice.starts_with("[BACK]") {
                    self.current_mode = InteractiveMode::MainMenu;
                } else {
                    // 提取文件名（去掉大小括号和大小信息）
                    let filename = choice.split(" (").next().unwrap_or(&choice).to_string();

                    // 直接显示日志内容
                    println!("\n▶ 正在显示日志文件: {}", filename);
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                    if let Err(e) = handle_log_show(
                        space.clone(),
                        project.clone(),
                        filename,
                        false, // follow
                        100,   // lines
                        true,  // timestamps
                        None,  // step
                        None,  // level
                    )
                    .await
                    {
                        println!("※ 显示日志失败: {}", e);
                    }

                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                    // 询问是否继续查看其他日志
                    let continue_choice =
                        Confirm::new("是否查看其他日志文件?").with_default(true).prompt();

                    match continue_choice {
                        Ok(true) => {
                            // 继续在当前日志选择界面
                            self.current_mode =
                                InteractiveMode::LogFileSelection { space, project };
                        }
                        _ => {
                            // 返回主菜单
                            self.current_mode = InteractiveMode::MainMenu;
                        }
                    }
                }
                Ok(true)
            }
            Err(_) => {
                self.current_mode = InteractiveMode::MainMenu;
                Ok(true)
            }
        }
    }

    /// 选择空间和项目
    async fn select_space_and_project(&mut self) -> Result<Option<(String, String)>> {
        // 获取项目列表
        let loader = match ProjectLoader::from_current_dir() {
            Ok(loader) => loader,
            Err(e) => {
                println!("✗ 无法创建项目加载器: {}", e);
                self.pause_for_user().await?;
                return Ok(None);
            }
        };

        let spaces = match loader.list_spaces().await {
            Ok(spaces) => spaces,
            Err(e) => {
                println!("✗ 无法列出空间: {}", e);
                self.pause_for_user().await?;
                return Ok(None);
            }
        };

        if spaces.is_empty() {
            println!("※ 未找到任何空间配置");
            self.pause_for_user().await?;
            return Ok(None);
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

        if project_options.is_empty() {
            println!("※ 未找到任何项目");
            self.pause_for_user().await?;
            return Ok(None);
        }

        // 选择项目
        let selection = Select::new("请选择要查看日志的项目:", project_options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        let selected_project = match selection {
            Ok(choice) => choice,
            Err(_) => {
                return Ok(None);
            }
        };

        // 解析选择的项目
        let parts: Vec<&str> = selected_project.split('/').collect();
        if parts.len() != 2 {
            println!("✗ 无效的项目选择");
            self.pause_for_user().await?;
            return Ok(None);
        }

        let space = parts[0].to_string();
        let project = parts[1].to_string();

        Ok(Some((space, project)))
    }
}
