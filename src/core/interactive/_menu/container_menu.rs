//! 容器管理交互式菜单
//!
//! 提供容器创建、启动、停止等操作的交互式界面

use crate::core::builder::{BuilderFormatter, ContainerManager};
use crate::core::interactive::{InteractiveEngine, InteractiveMode};
use anyhow::Result;
use inquire::{Confirm, Select, Text};

impl InteractiveEngine {
    /// 显示容器管理菜单
    pub async fn show_container_menu(&mut self) -> Result<bool> {
        let options = vec![
            "[LIST] 查看容器列表",
            "[CREATE] 创建容器",
            "[START] 启动容器",
            "[STOP] 停止容器",
            "[REMOVE] 删除容器",
            "[LOGS] 查看容器日志",
            "[BACK] 返回上级菜单",
        ];

        let selection = Select::new("容器管理菜单:", options)
            .with_help_message("选择要执行的容器操作")
            .prompt();

        match selection {
            Ok(choice) => match choice {
                choice if choice.starts_with("[LIST]") => {
                    self.handle_container_list().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[CREATE]") => {
                    self.handle_container_create().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[START]") => {
                    self.handle_container_start().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[STOP]") => {
                    self.handle_container_stop().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[REMOVE]") => {
                    self.handle_container_remove().await?;
                    self.pause_for_user().await?;
                    Ok(true)
                }
                choice if choice.starts_with("[LOGS]") => {
                    self.handle_container_logs().await?;
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
                // 用户中断，回到上级菜单
                self.current_mode = InteractiveMode::BuilderMenu;
                Ok(true)
            }
        }
    }

    /// 显示容器列表
    async fn handle_container_list(&self) -> Result<()> {
        println!("\n📋 构建器容器列表");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 调用 core 模块获取容器列表
        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let containers = manager.list_builders().await?;

        if containers.is_empty() {
            println!("📭 暂无容器");
            return Ok(());
        }

        // 直接打印容器信息
        for container in &containers {
            println!(
                "• {} | {} | {:?} | {}",
                container.service_name, container.image, container.status, container.container_name
            );
        }

        Ok(())
    }

    /// 创建容器交互流程
    async fn handle_container_create(&self) -> Result<()> {
        println!("\n➕ 创建构建器容器");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 先显示可用的服务列表
        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let services = manager.list_service_names();

        if services.is_empty() {
            println!("❌ docker-compose.yml 中没有找到可用的服务");
            return Ok(());
        }

        // 添加 "所有服务" 选项
        let mut service_options = vec!["[ALL] 创建所有服务".to_string()];
        for service in &services {
            service_options.push(format!("[SERVICE] {}", service));
        }

        let service_selection = Select::new("选择要创建的服务:", service_options)
            .with_help_message("选择单个服务或创建所有服务")
            .prompt()?;

        let create_all = service_selection.starts_with("[ALL]");
        let service_name =
            if create_all { None } else { Some(service_selection.replace("[SERVICE] ", "")) };

        // 询问是否强制重新创建
        let force =
            Confirm::new("是否强制重新创建（删除已存在的容器）?").with_default(false).prompt()?;

        // 询问是否创建后自动启动
        let start = Confirm::new("创建后是否自动启动?").with_default(true).prompt()?;

        // 调用 core 模块创建容器
        if create_all {
            // 为所有服务创建容器
            for service in &services {
                if force {
                    manager.create_builder_force(service).await?;
                } else {
                    manager.create_builder(service).await?;
                }
                if start {
                    manager.start_builder(service).await?;
                }
            }
            println!("✓ 所有容器创建完成");
        } else if let Some(service) = service_name {
            if force {
                manager.create_builder_force(&service).await?;
            } else {
                manager.create_builder(&service).await?;
            }
            if start {
                manager.start_builder(&service).await?;
            }
            println!("✓ 容器 '{}' 创建完成", service);
        }

        Ok(())
    }

    /// 启动容器交互流程
    async fn handle_container_start(&self) -> Result<()> {
        println!("\n▶ 启动构建器容器");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let containers = manager.list_builders().await?;

        // 过滤出已停止的容器
        let stopped_containers: Vec<_> = containers
            .iter()
            .filter(|c| !matches!(c.status, crate::core::builder::ContainerStatus::Running))
            .collect();

        if stopped_containers.is_empty() {
            println!("📭 没有已停止的容器可以启动");
            return Ok(());
        }

        // 创建选择列表
        let container_options: Vec<String> = stopped_containers
            .iter()
            .map(|c| format!("{} ({:?})", c.service_name, c.status))
            .collect();

        let selection = Select::new("选择要启动的容器:", container_options)
            .with_help_message("选择一个已停止的容器进行启动")
            .prompt()?;

        // 找到对应的容器
        if let Some(container) =
            stopped_containers.iter().find(|c| selection.starts_with(&c.service_name))
        {
            manager.start_builder(&container.service_name).await?;
            println!("✓ 容器 '{}' 启动成功", container.service_name);
        }

        Ok(())
    }

    /// 停止容器交互流程
    async fn handle_container_stop(&self) -> Result<()> {
        println!("\n⏹ 停止构建器容器");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let containers = manager.list_builders().await?;

        // 过滤出正在运行的容器
        let running_containers: Vec<_> = containers
            .iter()
            .filter(|c| matches!(c.status, crate::core::builder::ContainerStatus::Running))
            .collect();

        if running_containers.is_empty() {
            println!("📭 没有正在运行的容器可以停止");
            return Ok(());
        }

        // 创建选择列表
        let container_options: Vec<String> = running_containers
            .iter()
            .map(|c| format!("{} ({:?})", c.service_name, c.status))
            .collect();

        let selection = Select::new("选择要停止的容器:", container_options)
            .with_help_message("选择一个正在运行的容器进行停止")
            .prompt()?;

        // 找到对应的容器
        if let Some(container) =
            running_containers.iter().find(|c| selection.starts_with(&c.service_name))
        {
            manager.stop_builder(&container.service_name).await?;
            println!("✓ 容器 '{}' 停止成功", container.service_name);
        }

        Ok(())
    }

    /// 删除容器交互流程
    async fn handle_container_remove(&self) -> Result<()> {
        println!("\n🗑 删除构建器容器");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let containers = manager.list_builders().await?;

        if containers.is_empty() {
            println!("📭 没有容器可以删除");
            return Ok(());
        }

        // 创建选择列表
        let container_options: Vec<String> =
            containers.iter().map(|c| format!("{} ({:?})", c.service_name, c.status)).collect();

        let selection = Select::new("选择要删除的容器:", container_options)
            .with_help_message("选择一个容器进行删除")
            .prompt()?;

        // 找到对应的容器
        if let Some(container) = containers.iter().find(|c| selection.starts_with(&c.service_name))
        {
            // 确认删除
            let confirm_msg = format!("确认删除容器 '{}'?", container.service_name);
            let confirmed = Confirm::new(&confirm_msg).with_default(false).prompt()?;

            if confirmed {
                let force =
                    matches!(container.status, crate::core::builder::ContainerStatus::Running);
                manager.remove_builder(&container.service_name, force).await?;
                println!("✓ 容器 '{}' 删除成功", container.service_name);
            } else {
                println!("❌ 取消删除操作");
            }
        }

        Ok(())
    }

    /// 查看容器日志
    async fn handle_container_logs(&self) -> Result<()> {
        println!("\n📜 查看容器日志");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let manager = ContainerManager::from_compose_file("docker-compose.yml").await?;
        let containers = manager.list_builders().await?;

        if containers.is_empty() {
            println!("📭 没有容器可以查看日志");
            return Ok(());
        }

        // 创建选择列表
        let container_options: Vec<String> =
            containers.iter().map(|c| format!("{} ({:?})", c.service_name, c.status)).collect();

        let selection = Select::new("选择要查看日志的容器:", container_options)
            .with_help_message("选择一个容器查看其日志")
            .prompt()?;

        // 找到对应的容器
        if let Some(container) = containers.iter().find(|c| selection.starts_with(&c.service_name))
        {
            // 询问日志行数
            let lines_input = Text::new("显示最近多少行日志?")
                .with_default("50")
                .with_help_message("输入数字，默认为 50 行")
                .prompt()?;

            let lines = lines_input.parse().unwrap_or(50);

            println!("\n📜 容器 '{}' 的最近 {} 行日志:", container.service_name, lines);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            let logs = manager.get_builder_logs(&container.service_name, Some(lines)).await?;
            println!("{}", logs);
        }

        Ok(())
    }
}
