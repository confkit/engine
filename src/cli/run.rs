//! Author: xiaoYown
//! Created: 2025-07-21
//! Description: Run build task subcommand implementation

use std::collections::HashMap;

use crate::core::condition::evaluator::ConditionEvaluator;
use crate::core::executor::runner::Runner;
use crate::infra::config::ConfKitConfigLoader;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// space name
    #[arg(short, long)]
    pub space: String,

    /// project name
    #[arg(short, long)]
    pub project: String,

    /// environments - not required
    #[arg(short, long)]
    pub environments: Option<Vec<String>>,

    /// Preview steps without executing
    #[arg(long)]
    pub dry_run: bool,
}

/// 处理 run 命令
pub async fn handle_run(args: &RunArgs) -> Result<()> {
    if args.dry_run {
        return handle_dry_run(args).await;
    }

    let environment_from_args = parse_environments(args.environments.clone()).await?;

    let mut runner =
        Runner::new(args.space.as_str(), args.project.as_str(), environment_from_args).await?;

    runner.start().await?;

    Ok(())
}

/// dry-run: 预览步骤，不实际执行
async fn handle_dry_run(args: &RunArgs) -> Result<()> {
    let project_config =
        ConfKitConfigLoader::get_project_config(&args.space, &args.project).await?;

    let project_config = match project_config {
        Some(config) => config,
        None => {
            tracing::error!(
                "Project '{}' not found in space '{}'",
                args.project,
                args.space
            );
            return Ok(());
        }
    };

    // 加载环境变量用于条件求值
    let (mut env_mixed, _, _) =
        ConfKitConfigLoader::load_project_env(&args.space, &args.project).await?;

    // 合并命令行传入的环境变量
    let env_from_args = parse_environments(args.environments.clone()).await?;
    for (key, value) in env_from_args {
        env_mixed.insert(key, value);
    }

    let evaluator = ConditionEvaluator::new(env_mixed);
    let total = project_config.steps.len();

    tracing::info!("Dry run: {}/{}", args.space, args.project);
    tracing::info!("{}", "=".repeat(50));
    tracing::info!("Steps ({}):", total);

    for (i, step) in project_config.steps.iter().enumerate() {
        let step_num = i + 1;
        let target = step.container.as_deref().unwrap_or("host");

        // 求值条件
        let condition_result = if let Some(condition) = &step.condition {
            match evaluator.evaluate_string(condition) {
                Ok(true) => format!("condition: {} -> PASS", condition),
                Ok(false) => format!("condition: {} -> SKIP", condition),
                Err(e) => format!("condition: {} -> ERROR({})", condition, e),
            }
        } else {
            "no condition".to_string()
        };

        let will_skip = if let Some(condition) = &step.condition {
            matches!(evaluator.evaluate_string(condition), Ok(false))
        } else {
            false
        };

        let status = if will_skip { "SKIP" } else { "RUN" };

        tracing::info!("");
        tracing::info!(
            "  [Step {}/{}] {} [{}]",
            step_num,
            total,
            step.name,
            status
        );
        tracing::info!("    target:    {}", target);
        tracing::info!("    commands:  {}", step.commands.len());
        for cmd in &step.commands {
            tracing::info!("      - {}", cmd);
        }
        if let Some(timeout) = step.timeout {
            tracing::info!("    timeout:   {}s", timeout);
        }
        tracing::info!("    {}", condition_result);
    }

    Ok(())
}

// 解析环境变量
async fn parse_environments(environments: Option<Vec<String>>) -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();

    if let Some(env_strings) = environments {
        for env_str in env_strings {
            if let Some((key, value)) = env_str.split_once('=') {
                env_map.insert(key.to_string(), value.to_string());
            } else {
                // 格式不正确时，记录警告并跳过
                tracing::warn!("跳过格式不正确的环境变量: {}", env_str);
            }
        }
    }

    Ok(env_map)
}
