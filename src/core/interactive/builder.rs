use super::{BuilderStatusOption, InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示Builder List参数选择
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
                        // 执行命令 - 直接调用core层能力
                        println!("• 正在获取构建器列表...");
                        println!();

                        match self.builder_manager.list_builders_with_filter(
                            current_verbose,
                            current_status_filter.clone(),
                        ) {
                            Ok(output) => {
                                println!("{}", output);
                            }
                            Err(e) => {
                                println!("✗ 获取构建器列表失败: {}", e);
                            }
                        }

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
