//! Author: xiaoYown
//! Created: 2025-03-03
//! Description: Config subcommand implementation

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::infra::config::ConfKitConfigLoader;

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Show current configuration overview.
    Show,
    /// Validate configuration file.
    Validate,
}

impl ConfigCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ConfigSubcommand::Show => handle_show().await,
            ConfigSubcommand::Validate => handle_validate().await,
        }
    }
}

async fn handle_show() -> Result<()> {
    let config = ConfKitConfigLoader::get_config();

    tracing::info!("ConfKit Configuration");
    tracing::info!("{}", "=".repeat(50));

    // Engine
    tracing::info!("Engine:           {:?}", config.engine);
    tracing::info!("Version:          {}", config.version);
    tracing::info!("Compose file:     {}", config.engine_compose.file);
    tracing::info!("Compose project:  {}", config.engine_compose.project);

    // Images
    tracing::info!("");
    tracing::info!("Images ({}):", config.images.len());
    for image in &config.images {
        tracing::info!(
            "  - {}:{} (base: {}, context: {})",
            image.name,
            image.tag,
            image.base_image,
            image.context
        );
    }

    // Spaces
    tracing::info!("");
    tracing::info!("Spaces ({}):", config.spaces.len());
    for space in &config.spaces {
        tracing::info!("  [{}] {} (path: {})", space.name, space.description, space.path);

        let projects = ConfKitConfigLoader::get_project_config_list(&space.name).await?;
        for project in &projects {
            let step_count = project.steps.len();
            let source_info = match &project.source {
                Some(src) => format!("git: {}", src.git_repo),
                None => "no source".to_string(),
            };
            tracing::info!("    - {} | {} | {} steps", project.name, source_info, step_count);
        }
    }

    Ok(())
}

async fn handle_validate() -> Result<()> {
    tracing::info!("Validating .confkit.yml...");

    let config = ConfKitConfigLoader::get_config();
    let mut errors: Vec<String> = vec![];

    // 校验 spaces
    if config.spaces.is_empty() {
        errors.push("No spaces defined".to_string());
    }

    for space in &config.spaces {
        if space.name.is_empty() {
            errors.push("Space name is empty".to_string());
        }
        if space.path.is_empty() {
            errors.push(format!("Space '{}' has empty path", space.name));
        }
        if !std::path::Path::new(&space.path).exists() {
            errors.push(format!("Space '{}' path does not exist: {}", space.name, space.path));
        }

        // 校验 space 下的项目配置
        match ConfKitConfigLoader::get_project_config_list(&space.name).await {
            Ok(projects) => {
                if projects.is_empty() {
                    errors.push(format!(
                        "Space '{}' has no project configs in: {}",
                        space.name, space.path
                    ));
                }
                for project in &projects {
                    if project.steps.is_empty() {
                        errors.push(format!(
                            "Project '{}/{}' has no steps",
                            space.name, project.name
                        ));
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Failed to load projects for space '{}': {}", space.name, e));
            }
        }
    }

    // 校验 images
    for image in &config.images {
        if image.name.is_empty() {
            errors.push("Image name is empty".to_string());
        }
        if !std::path::Path::new(&image.context).exists() {
            errors.push(format!(
                "Image '{}:{}' context does not exist: {}",
                image.name, image.tag, image.context
            ));
        }
    }

    // 校验 compose file
    if !std::path::Path::new(&config.engine_compose.file).exists() {
        errors.push(format!("Compose file does not exist: {}", config.engine_compose.file));
    }

    if errors.is_empty() {
        tracing::info!("Configuration is valid");
    } else {
        tracing::warn!("Found {} issue(s):", errors.len());
        for (i, err) in errors.iter().enumerate() {
            tracing::warn!("  {}. {}", i + 1, err);
        }
    }

    Ok(())
}
