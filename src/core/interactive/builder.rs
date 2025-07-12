use super::{BuilderStatusOption, InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示镜像列表参数选择
    pub(super) async fn show_image_list_params(
        &mut self,
        current_verbose: bool,
        current_status_filter: Option<String>,
    ) -> Result<bool> {
        // 显示当前设置
        println!("• 构建镜像列表参数配置");
        println!();
        println!("当前设置:");
        println!("  详细模式: {}", if current_verbose { "✓ 开启" } else { "✗ 关闭" });

        let status_display = match &current_status_filter {
            Some(status) => match status.as_str() {
                "notcreated" => "未创建",
                "created" => "已创建",
                "running" => "运行中",
                "stopped" => "已停止",
                "error" => "错误",
                _ => "所有状态",
            },
            None => "所有状态",
        };
        println!("  状态过滤: {}", status_display);
        println!();

        let options = vec![
            "● 执行命令 - 使用当前设置列出构建镜像".to_string(),
            format!("※ 切换详细模式 - 当前: {}", if current_verbose { "开启" } else { "关闭" }),
            format!("• 选择状态过滤 - 当前: {}", status_display),
            "← 返回镜像管理菜单".to_string(),
        ];

        let selection = Select::new("请选择操作:", options)
            .with_help_message("配置构建镜像列表显示参数")
            .prompt();

        match selection {
            Ok(choice) => {
                match choice {
                    choice if choice.starts_with("● 执行命令") => {
                        // 执行命令 - 使用 ImageManager 获取镜像列表
                        println!("• 正在获取构建镜像列表...");
                        println!();

                        // 获取构建器列表
                        let builders = self.image_manager.list_builders();

                        // 应用状态过滤
                        let filtered_builders: Vec<_> =
                            if let Some(status_filter) = &current_status_filter {
                                builders
                                    .into_iter()
                                    .filter(|builder| match status_filter.as_str() {
                                        "notcreated" => matches!(
                                            builder.status,
                                            crate::core::builder::BuilderStatus::NotCreated
                                        ),
                                        "created" => matches!(
                                            builder.status,
                                            crate::core::builder::BuilderStatus::Created
                                        ),
                                        "running" => matches!(
                                            builder.status,
                                            crate::core::builder::BuilderStatus::Running
                                        ),
                                        "stopped" => matches!(
                                            builder.status,
                                            crate::core::builder::BuilderStatus::Stopped
                                        ),
                                        "error" => matches!(
                                            builder.status,
                                            crate::core::builder::BuilderStatus::Error
                                        ),
                                        _ => true,
                                    })
                                    .collect()
                            } else {
                                builders
                            };

                        // 显示结果
                        use crate::core::builder::BuilderFormatter;
                        let output = BuilderFormatter::format_builders_list(
                            &filtered_builders,
                            current_verbose,
                            current_status_filter.clone(),
                        );
                        println!("{}", output);

                        println!();
                        self.pause_for_user().await?;

                        self.current_mode = InteractiveMode::ImageMenu;
                        Ok(true)
                    }
                    choice if choice.starts_with("※ 切换详细模式") => {
                        self.current_mode = InteractiveMode::ImageListParams {
                            verbose: !current_verbose,
                            status_filter: current_status_filter,
                        };
                        Ok(true)
                    }
                    choice if choice.starts_with("• 选择状态过滤") => {
                        let new_status = self.select_status_filter().await?;
                        self.current_mode = InteractiveMode::ImageListParams {
                            verbose: current_verbose,
                            status_filter: new_status,
                        };
                        Ok(true)
                    }
                    choice if choice.starts_with("← 返回") => {
                        self.current_mode = InteractiveMode::ImageMenu;
                        Ok(true)
                    }
                    _ => Ok(true),
                }
            }
            Err(_) => {
                // 用户中断，回到镜像管理菜单
                self.current_mode = InteractiveMode::ImageMenu;
                Ok(true)
            }
        }
    }

    /// 显示Builder List参数选择 (保留向后兼容)
    pub(super) async fn show_builder_list_params(
        &mut self,
        current_verbose: bool,
        current_status_filter: Option<String>,
    ) -> Result<bool> {
        // 显示当前设置
        println!("• 构建器列表参数配置");
        println!();
        println!("当前设置:");
        println!("  详细模式: {}", if current_verbose { "✓ 开启" } else { "✗ 关闭" });

        let status_display = match &current_status_filter {
            Some(status) => match status.as_str() {
                "notcreated" => "未创建",
                "created" => "已创建",
                "running" => "运行中",
                "stopped" => "已停止",
                "error" => "错误",
                _ => "所有状态",
            },
            None => "所有状态",
        };
        println!("  状态过滤: {}", status_display);
        println!();

        let options = vec![
            "● 执行命令 - 使用当前设置列出构建器".to_string(),
            format!("※ 切换详细模式 - 当前: {}", if current_verbose { "开启" } else { "关闭" }),
            format!("• 选择状态过滤 - 当前: {}", status_display),
            "← 返回 Builder 菜单".to_string(),
        ];

        let selection = Select::new("请选择操作:", options)
            .with_help_message("配置构建器列表显示参数")
            .prompt();

        match selection {
            Ok(choice) => {
                match choice {
                    choice if choice.starts_with("● 执行命令") => {
                        // 执行命令 - 使用 ImageManager 获取构建器列表
                        println!("• 正在获取构建器列表...");
                        println!();

                        // 获取构建器列表
                        let builders = self.image_manager.list_builders();

                        // 显示结果
                        use crate::core::builder::BuilderFormatter;
                        let output = BuilderFormatter::format_builders_list(
                            &builders,
                            current_verbose,
                            current_status_filter.clone(),
                        );
                        println!("{}", output);

                        println!();
                        self.pause_for_user().await?;

                        self.current_mode = InteractiveMode::BuilderMenu;
                        Ok(true)
                    }
                    choice if choice.starts_with("※ 切换详细模式") => {
                        self.current_mode = InteractiveMode::BuilderListParams {
                            verbose: !current_verbose,
                            status_filter: current_status_filter,
                        };
                        Ok(true)
                    }
                    choice if choice.starts_with("• 选择状态过滤") => {
                        let new_status = self.select_status_filter().await?;
                        self.current_mode = InteractiveMode::BuilderListParams {
                            verbose: current_verbose,
                            status_filter: new_status,
                        };
                        Ok(true)
                    }
                    choice if choice.starts_with("← 返回") => {
                        self.current_mode = InteractiveMode::BuilderMenu;
                        Ok(true)
                    }
                    _ => Ok(true),
                }
            }
            Err(_) => {
                // 用户中断，回到Builder菜单
                self.current_mode = InteractiveMode::BuilderMenu;
                Ok(true)
            }
        }
    }

    /// 选择状态过滤
    pub(super) async fn select_status_filter(&mut self) -> Result<Option<String>> {
        let status_options = vec![
            ("• 所有状态", BuilderStatusOption::All),
            ("✗ 未创建", BuilderStatusOption::NotCreated),
            ("✓ 已创建", BuilderStatusOption::Created),
            ("▶ 运行中", BuilderStatusOption::Running),
            ("■ 已停止", BuilderStatusOption::Stopped),
            ("! 错误", BuilderStatusOption::Error),
        ];

        let options: Vec<String> =
            status_options.iter().map(|(display, _)| display.to_string()).collect();

        let selection = Select::new("选择状态过滤:", options)
            .with_help_message("选择要显示的构建器状态")
            .prompt();

        match selection {
            Ok(choice) => {
                // 找到对应的状态选项
                for (display, status_option) in status_options {
                    if choice == display {
                        return Ok(status_option.to_filter_string());
                    }
                }
                Ok(None)
            }
            Err(_) => {
                // 用户中断，保持当前设置
                Ok(None)
            }
        }
    }
}
