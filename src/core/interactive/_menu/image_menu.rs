use super::super::{InteractiveEngine, InteractiveMode};
use crate::core::builder::{
    BuilderInfo, BuilderLoader, BuilderStatus, ImageBuilder, ImageCheckResult, ImageInspector,
};
use anyhow::Result;
use inquire::{Confirm, MultiSelect, Select};

impl InteractiveEngine {
    /// 显示镜像管理菜单
    pub async fn show_image_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[LIST] 列出镜像 - 查看所有构建镜像的状态",
            "[CREATE] 创建镜像 - 从 builder.yml 构建新镜像",
            "[REMOVE] 删除镜像 - 删除指定的构建镜像",
            "[BACK] 返回 Builder 菜单",
        ];

        let selection = Select::new("镜像管理菜单:", options)
            .with_help_message("选择要执行的镜像操作")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[LIST]") => {
                    self.current_mode =
                        InteractiveMode::ImageListParams { verbose: false, status_filter: None };
                    Ok(true)
                }
                choice if choice.starts_with("[CREATE]") => {
                    self.current_mode = InteractiveMode::ImageCreateParams;
                    Ok(true)
                }
                choice if choice.starts_with("[REMOVE]") => {
                    self.current_mode = InteractiveMode::ImageRemoveParams;
                    Ok(true)
                }
                choice if choice.starts_with("[BACK]") => {
                    self.current_mode = InteractiveMode::BuilderMenu;
                    Ok(true)
                }
                _ => Ok(true),
            },
            Err(_) => {
                // 用户中断，回到 Builder 菜单
                self.current_mode = InteractiveMode::BuilderMenu;
                Ok(true)
            }
        }
    }

    /// 显示镜像创建参数选择界面
    pub async fn show_image_create_params(&mut self) -> Result<bool> {
        println!("• 正在加载可用的构建镜像配置...");

        // 加载 builder.yml 中的配置
        let configs = match BuilderLoader::load_from_current_dir() {
            Ok(configs) => configs,
            Err(e) => {
                println!("✗ 加载构建镜像配置失败: {}", e);
                println!("   请确保当前目录存在 builder.yml 文件");
                println!();
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::ImageMenu;
                return Ok(true);
            }
        };

        if configs.is_empty() {
            println!("! 未找到任何构建镜像配置");
            println!("  请在 builder.yml 文件中添加构建镜像配置");
            println!();
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::ImageMenu;
            return Ok(true);
        }

        // 准备选项列表
        let mut options: Vec<String> = configs.keys().map(|name| name.clone()).collect();
        options.push("[BACK] 返回镜像管理菜单".to_string());

        let selection = Select::new("请选择要创建的构建镜像:", options)
            .with_help_message("选择镜像配置并开始构建过程")
            .prompt();

        match selection {
            Ok(choice) => {
                if choice.starts_with("[BACK]") {
                    self.current_mode = InteractiveMode::ImageMenu;
                    return Ok(true);
                }

                // 直接使用选择的构建器名称
                let builder_name = choice.clone();

                println!();
                println!("▶ 开始创建构建镜像: {}", builder_name);

                // 调用镜像创建的核心逻辑
                match self.execute_image_create(&builder_name).await {
                    Ok(()) => {
                        println!("✓ 构建镜像 '{}' 创建成功!", builder_name);
                    }
                    Err(e) => {
                        println!("✗ 构建镜像创建失败: {}", e);
                    }
                }

                println!();
                self.pause_for_user().await?;
                self.current_mode = InteractiveMode::ImageMenu;
                Ok(true)
            }
            Err(_) => {
                // 用户中断，回到镜像管理菜单
                self.current_mode = InteractiveMode::ImageMenu;
                Ok(true)
            }
        }
    }

    /// 执行镜像创建的核心逻辑
    async fn execute_image_create(&mut self, name: &str) -> Result<()> {
        // 从 builder.yml 加载构建器配置
        let config = BuilderLoader::find_builder_config(name)?;

        println!("✓ 找到构建镜像配置: {}", name);
        println!("  目标镜像: {}:{}", config.name, config.tag);
        println!("  基础镜像: {}", config.base_image);
        println!("  Dockerfile: {}", config.dockerfile);
        println!("  构建上下文: {}", config.context);
        if !config.build_args.is_empty() {
            println!("  构建参数: {} 个", config.build_args.len());
            for (key, value) in &config.build_args {
                println!("    {}={}", key, value);
            }
        }

        // 检查目标镜像是否已存在 - 使用完整的镜像名称（包含标签）
        let target_image = format!("{}:{}", config.name, config.tag);
        println!();
        match ImageInspector::check_target_image(&target_image).await {
            Ok(ImageCheckResult::Exists(_)) => {
                println!("● 跳过构建，直接使用现有镜像");
                return Ok(());
            }
            Ok(ImageCheckResult::NotExists) => {
                println!("▶ 开始构建镜像...");
            }
            Err(e) => {
                println!("! 检查镜像时出错: {}, 继续尝试构建", e);
            }
        }

        // 执行镜像构建
        println!();
        println!("▶ 正在构建 Docker 镜像...");
        println!("→ Dockerfile: {}", config.dockerfile);
        println!("→ 构建上下文: {}", config.context);

        match ImageBuilder::build_image(&config).await {
            Ok(builder_info) => {
                println!();
                println!("✓ 构建镜像 '{}' 创建成功！", name);
                println!("→ 镜像: {}:{}", config.name, config.tag);
                if let Some(image_id) = &builder_info.image_id {
                    println!("→ 镜像ID: {}", image_id);
                }
                println!(
                    "→ 创建时间: {}",
                    builder_info.created_at.unwrap_or_default().format("%Y-%m-%d %H:%M:%S")
                );

                // 显示构建日志的最后几行
                if let Some(logs) = &builder_info.build_logs {
                    let lines: Vec<&str> = logs.lines().collect();
                    let last_lines = lines.iter().rev().take(5).rev();
                    println!();
                    println!("※ 构建日志 (最后 5 行):");
                    for line in last_lines {
                        if !line.trim().is_empty() {
                            println!("   {}", line);
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                println!();
                println!("✗ 构建镜像创建失败: {}", e);
                Err(e)
            }
        }
    }

    /// 显示镜像删除参数选择界面
    pub async fn show_image_remove_params(&mut self) -> Result<bool> {
        println!("• 正在加载构建器镜像信息...");

        // 使用 ImageManager 获取镜像列表
        let builder_infos = self.image_manager.list_builders();

        if builder_infos.is_empty() {
            println!("! 未找到任何构建器镜像");
            println!("  请先创建一些构建器镜像");
            println!();
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::ImageMenu;
            return Ok(true);
        }

        // 过滤出已创建的镜像（有实际镜像存在的）
        let available_images: Vec<&BuilderInfo> = builder_infos
            .into_iter()
            .filter(|info| {
                matches!(
                    info.status,
                    BuilderStatus::Created | BuilderStatus::Running | BuilderStatus::Stopped
                )
            })
            .collect();

        if available_images.is_empty() {
            println!("! 未找到任何可删除的镜像");
            println!("  所有构建器都处于未创建状态");
            println!();
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::ImageMenu;
            return Ok(true);
        }

        println!("✓ 找到 {} 个可删除的镜像", available_images.len());
        println!();

        // 准备选择选项
        let mut options = Vec::new();
        for info in &available_images {
            let status_icon = match info.status {
                BuilderStatus::Created => "🟢",
                BuilderStatus::Running => "🔵",
                BuilderStatus::Stopped => "🟡",
                _ => "⚪",
            };

            let status_text = match info.status {
                BuilderStatus::Created => "已创建",
                BuilderStatus::Running => "运行中",
                BuilderStatus::Stopped => "已停止",
                _ => "未知",
            };

            let display_text = format!("{} {} ({})", status_icon, info.name, status_text);
            options.push(display_text);
        }
        options.push("← 返回镜像管理菜单".to_string());

        // 使用多选界面让用户选择要删除的镜像
        let selections = MultiSelect::new("请选择要删除的镜像:", options.clone())
            .with_help_message("空格键选择/取消选择，回车键确认选择。注意：删除操作不可恢复！")
            .with_page_size(15)
            .prompt();

        let selected_builders = match selections {
            Ok(choices) => {
                let mut selected_builders = Vec::new();

                for choice in &choices {
                    if choice.starts_with("←") {
                        continue;
                    }

                    // 从选择中提取构建器名称
                    // 格式: "🟢 name (status)"
                    if let Some(name_with_status) = choice.split(" (").next() {
                        // 去掉状态图标和空格
                        let name = name_with_status
                            .split_whitespace()
                            .skip(1)
                            .collect::<Vec<_>>()
                            .join(" ");

                        // 查找对应的构建器信息
                        if let Some(info) = available_images.iter().find(|info| info.name == name) {
                            selected_builders.push((*info).clone());
                        }
                    }
                }

                selected_builders
            }
            Err(_) => {
                // 用户中断，回到镜像管理菜单
                self.current_mode = InteractiveMode::ImageMenu;
                return Ok(true);
            }
        };

        if selected_builders.is_empty() {
            println!("! 未选择任何镜像");
            println!();
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::ImageMenu;
            return Ok(true);
        }

        // 显示选择的镜像
        println!();
        println!("▶ 选择的镜像 ({} 个):", selected_builders.len());
        for (i, info) in selected_builders.iter().enumerate() {
            println!("  {}. {}", i + 1, info.name);
        }

        // 询问删除模式
        let force_options = vec![
            "● 普通删除 - 安全删除（如果镜像正在使用会失败）",
            "⚠️  强制删除 - 强制删除（即使正在使用，谨慎选择）",
        ];

        let force_selection = Select::new("请选择删除模式:", force_options)
            .with_help_message("普通删除更安全，强制删除可能影响正在运行的容器")
            .prompt();

        let force = match force_selection {
            Ok(choice) => choice.starts_with("⚠️"),
            Err(_) => {
                self.current_mode = InteractiveMode::ImageMenu;
                return Ok(true);
            }
        };

        // 确认删除
        let confirm_msg = if force {
            format!(
                "⚠️  确认强制删除 {} 个镜像？这个操作不可恢复，可能影响正在运行的容器！",
                selected_builders.len()
            )
        } else {
            format!("确认删除 {} 个镜像？这个操作不可恢复！", selected_builders.len())
        };

        let confirm = Confirm::new(&confirm_msg)
            .with_default(false)
            .with_help_message("请仔细确认，删除后无法恢复")
            .prompt();

        match confirm {
            Ok(true) => {
                // 执行删除
                println!();
                println!("▶ 开始删除镜像...");
                if force {
                    println!("⚠️  使用强制删除模式");
                }
                println!();

                let mut success_count = 0;
                let mut failed_count = 0;

                for (i, info) in selected_builders.iter().enumerate() {
                    println!("→ [{}/{}] 删除镜像: {}", i + 1, selected_builders.len(), info.name);

                    // 使用 ImageManager 删除镜像
                    match self.image_manager.remove_builder(&info.name, force).await {
                        Ok(()) => {
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("✗ 删除失败: {}", e);
                            failed_count += 1;
                        }
                    }
                    println!();
                }

                // 显示总结
                println!("▶ 删除操作完成:");
                println!("  ✓ 成功: {} 个", success_count);
                if failed_count > 0 {
                    println!("  ✗ 失败: {} 个", failed_count);
                    if !force {
                        println!();
                        println!("提示: 如果镜像正在被容器使用，请先停止相关容器");
                        println!("      或者选择强制删除模式（谨慎使用）");
                    }
                } else {
                    println!("  🎉 所有镜像删除成功！");
                }
            }
            Ok(false) => {
                println!("• 取消删除操作");
            }
            Err(_) => {
                println!("• 取消删除操作");
            }
        }

        println!();
        self.pause_for_user().await?;
        self.current_mode = InteractiveMode::ImageMenu;
        Ok(true)
    }
}
