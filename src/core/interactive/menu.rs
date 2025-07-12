use super::{InteractiveEngine, InteractiveMode};
use crate::core::builder::{BuilderLoader, ImageBuilder, ImageCheckResult, ImageInspector};
use anyhow::Result;
use inquire::Select;

impl InteractiveEngine {
    /// 显示主菜单
    pub async fn show_main_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[BUILDER] Builder 管理 - 管理构建镜像和环境",
            "[TASK] Task 管理 - 管理构建任务 (即将推出)",
            "[CONFIG] Config 管理 - 管理项目配置 (即将推出)",
            "[GIT] Git 管理 - 管理 Git 仓库 (即将推出)",
            "[HELP] 帮助信息",
            "[QUIT] 退出程序",
        ];

        let selection = Select::new("请选择要执行的操作:", options)
            .with_help_message("使用 ↑↓ 方向键选择，Enter 确认")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[BUILDER]") => {
                    self.current_mode = InteractiveMode::BuilderMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[TASK]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[CONFIG]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[GIT]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[HELP]") => {
                    self.show_main_help().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[QUIT]") => Ok(false),
                _ => Ok(true),
            },
            Err(_) => {
                // 用户按了 Ctrl+C 或其他中断
                Ok(false)
            }
        }
    }

    /// 显示Builder菜单
    pub async fn show_builder_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[IMAGE] 镜像管理 - 管理构建镜像",
            "[CONTAINER] 构建器管理 - 管理构建器实例 (即将推出)",
            "[BACK] 返回主菜单",
        ];

        let selection = Select::new("Builder 管理菜单:", options)
            .with_help_message("选择要执行的 Builder 操作")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[IMAGE]") => {
                    self.current_mode = InteractiveMode::ImageMenu;
                    Ok(true)
                }
                choice if choice.starts_with("[CONTAINER]") => {
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[BACK]") => {
                    self.current_mode = InteractiveMode::MainMenu;
                    Ok(true)
                }
                _ => Ok(true),
            },
            Err(_) => {
                // 用户中断，回到主菜单
                self.current_mode = InteractiveMode::MainMenu;
                Ok(true)
            }
        }
    }

    /// 显示镜像管理菜单
    pub async fn show_image_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[LIST] 列出镜像 - 查看所有构建镜像的状态",
            "[CREATE] 创建镜像 - 从 builder.yml 构建新镜像",
            "[REMOVE] 删除镜像 - 删除指定的构建镜像 (即将推出)",
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
                    println!("※ 该功能即将推出，敬请期待!");
                    println!();
                    self.pause_for_user().await?;
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
        let mut options: Vec<String> = configs
            .keys()
            .map(|name| format!("{} - {}", name, configs.get(name).unwrap().image))
            .collect();
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

                // 提取构建器名称（在 " - " 之前的部分）
                let builder_name = choice.split(" - ").next().unwrap_or(&choice).to_string();

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
        println!("  目标镜像: {}", config.image);
        println!("  基础镜像: {}", config.base_image);
        println!("  Dockerfile: {}", config.dockerfile);
        println!("  构建上下文: {}", config.context);
        if !config.build_args.is_empty() {
            println!("  构建参数: {} 个", config.build_args.len());
            for (key, value) in &config.build_args {
                println!("    {}={}", key, value);
            }
        }

        // 检查目标镜像是否已存在
        println!();
        match ImageInspector::check_target_image(&config.image).await {
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
                println!("→ 镜像: {}", config.image);
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

    /// 显示 Builder Create 参数选择界面 (保留向后兼容)
    pub async fn show_builder_create_params(&mut self) -> Result<bool> {
        // 直接重定向到新的镜像创建参数选择界面
        self.current_mode = InteractiveMode::ImageCreateParams;
        Ok(true)
    }
}
