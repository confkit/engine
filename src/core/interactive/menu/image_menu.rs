use super::super::{InteractiveEngine, InteractiveMode};
use crate::core::builder::{BuilderLoader, ImageBuilder, ImageCheckResult, ImageInspector};
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

    /// 显示镜像删除参数选择界面
    pub async fn show_image_remove_params(&mut self) -> Result<bool> {
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

        // 检查每个镜像的状态并收集可用的 tag
        println!("• 检查镜像状态...");
        let mut image_tags = Vec::new();

        for (builder_name, config) in &configs {
            // 检查镜像是否存在
            let image_status = match ImageInspector::check_target_image(&config.image).await {
                Ok(ImageCheckResult::Exists(info)) => {
                    // 镜像存在，添加其 tag 信息
                    let status_text = format!("✓ 已构建 - {} ({})", info.size, info.created_at);
                    (true, status_text, Some(info))
                }
                Ok(ImageCheckResult::NotExists) => {
                    // 镜像不存在
                    (false, "未构建".to_string(), None)
                }
                Err(_) => {
                    // 检查出错
                    (false, "状态未知".to_string(), None)
                }
            };

            // 解析镜像名称和标签
            let (repo, tag) = if let Some(colon_pos) = config.image.find(':') {
                let repo = &config.image[..colon_pos];
                let tag = &config.image[colon_pos + 1..];
                (repo.to_string(), tag.to_string())
            } else {
                (config.image.clone(), "latest".to_string())
            };

            image_tags.push((
                builder_name.clone(),
                repo,
                tag,
                config.image.clone(),
                image_status.0, // is_built
                image_status.1, // status_text
                image_status.2, // image_info
            ));
        }

        // 按镜像仓库分组显示
        let mut repo_groups: std::collections::HashMap<String, Vec<_>> =
            std::collections::HashMap::new();
        for item in image_tags {
            repo_groups.entry(item.1.clone()).or_default().push(item);
        }

        // 准备选择选项 - 简化版本
        let mut options = Vec::new();
        let mut image_data = Vec::new();

        for (repo, tags) in repo_groups.iter() {
            // 只显示已构建的镜像
            let built_tags: Vec<_> =
                tags.iter().filter(|(_, _, _, _, is_built, _, _)| *is_built).collect();

            if !built_tags.is_empty() {
                // 添加仓库分组标题
                options.push(format!("--- {} 镜像 ({} 个) ---", repo, built_tags.len()));
                image_data.push(None);

                // 添加该仓库下的所有已构建标签
                for (builder_name, _repo, tag, full_image, _is_built, status_text, _info) in
                    built_tags
                {
                    let display_text = format!("[{}] - {}", tag, status_text);
                    options.push(display_text);
                    image_data.push(Some((builder_name.clone(), full_image.clone())));
                }
            }
        }

        if options.is_empty() {
            println!("! 未找到任何已构建的镜像");
            println!();
            self.pause_for_user().await?;
            self.current_mode = InteractiveMode::ImageMenu;
            return Ok(true);
        }

        // 使用标准的多选界面
        let selections = MultiSelect::new("请选择要删除的镜像:", options.clone())
            .with_help_message("空格键选择/取消选择，回车键确认选择")
            .with_page_size(20)
            .prompt();

        let selected_images = match selections {
            Ok(choices) => {
                let mut selected_images = Vec::new();

                // 处理选中的选项
                for choice in &choices {
                    // 跳过分组标题行
                    if choice.starts_with("--- ") && choice.ends_with(" ---") {
                        continue;
                    }

                    // 查找对应的镜像数据
                    if let Some(index) = options.iter().position(|opt| opt == choice) {
                        if let Some(Some((builder_name, full_image))) = image_data.get(index) {
                            selected_images.push((builder_name.clone(), full_image.clone()));
                        }
                    }
                }

                selected_images
            }
            Err(_) => {
                // 用户中断，回到镜像管理菜单
                self.current_mode = InteractiveMode::ImageMenu;
                return Ok(true);
            }
        };

        if selected_images.is_empty() {
            println!("! 未选择任何有效的镜像");
            println!();
            self.pause_for_user().await?;
            return Ok(true);
        }

        // 显示选择的镜像
        println!();
        println!("▶ 选择的镜像:");
        for (builder_name, full_image) in &selected_images {
            println!("  • {} ({})", builder_name, full_image);
        }

        // 询问删除模式
        let force_options = vec![
            "● 普通删除 - 安全删除（如果镜像正在使用会失败）",
            "※ 强制删除 - 强制删除（即使正在使用）",
        ];

        let force_selection = Select::new("请选择删除模式:", force_options)
            .with_help_message("选择删除模式")
            .prompt();

        let force = match force_selection {
            Ok(choice) => choice.starts_with("※ 强制删除"),
            Err(_) => {
                self.current_mode = InteractiveMode::ImageMenu;
                return Ok(true);
            }
        };

        // 确认删除
        let confirm_msg = if force {
            format!("确认强制删除 {} 个镜像？这个操作不可恢复！", selected_images.len())
        } else {
            format!("确认删除 {} 个镜像？", selected_images.len())
        };

        let confirm = Confirm::new(&confirm_msg).with_default(false).prompt();

        match confirm {
            Ok(true) => {
                // 执行删除
                println!();
                println!("▶ 开始删除镜像...");
                if force {
                    println!("⚠️  使用强制删除模式");
                }

                let mut success_count = 0;
                let mut failed_count = 0;

                for (builder_name, full_image) in selected_images {
                    println!();
                    println!("→ 删除镜像: {} ({})", builder_name, full_image);

                    match self.execute_image_remove(&builder_name, force).await {
                        Ok(()) => {
                            println!("✓ 删除成功");
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("✗ 删除失败: {}", e);
                            failed_count += 1;
                        }
                    }
                }

                // 显示总结
                println!();
                println!("▶ 删除完成:");
                println!("  成功: {} 个", success_count);
                if failed_count > 0 {
                    println!("  失败: {} 个", failed_count);
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

    /// 执行镜像删除的核心逻辑
    async fn execute_image_remove(&mut self, name: &str, force: bool) -> Result<()> {
        // 从 builder.yml 加载构建器配置
        let config = match BuilderLoader::find_builder_config(name) {
            Ok(config) => {
                println!("✓ 找到构建镜像配置: {}", name);
                println!("  目标镜像: {}", config.image);
                config
            }
            Err(e) => {
                println!("✗ 加载构建镜像配置失败: {}", e);
                println!("  尝试直接删除镜像名称: {}", name);
                // 如果找不到配置，尝试直接使用提供的名称作为镜像名
                use crate::core::builder::BuilderConfig;
                BuilderConfig {
                    name: name.to_string(),
                    image: name.to_string(),
                    base_image: String::new(),
                    dockerfile: String::new(),
                    context: String::new(),
                    build_args: std::collections::HashMap::new(),
                }
            }
        };

        // 检查镜像是否存在
        println!();
        match ImageInspector::check_target_image(&config.image).await {
            Ok(ImageCheckResult::Exists(info)) => {
                println!("✓ 找到镜像:");
                println!("  镜像ID: {}", info.id);
                println!("  仓库: {}", info.repository);
                println!("  标签: {}", info.tag);
                println!("  创建时间: {}", info.created_at);
                println!("  大小: {}", info.size);
            }
            Ok(ImageCheckResult::NotExists) => {
                println!("! 镜像不存在: {}", config.image);
                return Ok(());
            }
            Err(e) => {
                println!("✗ 检查镜像时出错: {}", e);
                return Err(e);
            }
        }

        // 执行删除操作
        println!();
        println!("▶ 正在删除 Docker 镜像...");
        println!("→ 镜像: {}", config.image);

        match ImageInspector::remove_image(&config.image, force).await {
            Ok(()) => {
                println!();
                println!("✓ 镜像 '{}' 删除成功！", name);
                println!("→ 已删除镜像: {}", config.image);
                if force {
                    println!("→ 使用强制删除模式");
                }
                Ok(())
            }
            Err(e) => {
                println!();
                println!("✗ 构建镜像删除失败: {}", e);
                if !force {
                    println!("提示: 如果镜像正在被使用，请先停止相关容器");
                    println!("      或者选择强制删除模式");
                }
                Err(e)
            }
        }
    }
}
