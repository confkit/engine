use crate::core::project::{ProjectLoader, ProjectRunner, RunOptions, TaskStatus};
use anyhow::Result;
use clap::Args;

/// Run 命令参数
#[derive(Debug, Args)]
pub struct RunArgs {
    /// 空间名称
    pub space: String,

    /// 项目名称
    pub project: String,

    /// 详细输出模式
    #[arg(long, short)]
    pub verbose: bool,

    /// 预检查模式，不实际执行命令
    #[arg(long)]
    pub dry_run: bool,

    /// 指定 Git 分支
    #[arg(long)]
    pub git_branch: Option<String>,

    /// 强制执行，忽略检查
    #[arg(long)]
    pub force: bool,
}

/// 处理 run 命令
pub async fn handle_run(args: &RunArgs) -> Result<()> {
    // 创建项目加载器
    let loader = ProjectLoader::from_current_dir()?;

    // 加载项目配置
    let (space_config, project_config) =
        loader.load_project_config(&args.space, &args.project).await?;

    // 创建运行选项
    let options = RunOptions {
        verbose: args.verbose,
        dry_run: args.dry_run,
        git_branch: args.git_branch.clone(),
        force: args.force,
    };

    // 创建项目执行引擎
    let runner = ProjectRunner::new().await?;

    // 执行项目
    let result = runner
        .run_project(
            args.space.clone(),
            args.project.clone(),
            space_config,
            project_config,
            options,
        )
        .await?;

    // 根据执行结果设置退出码
    match result.status {
        crate::core::project::types::TaskStatus::Success => {
            if args.verbose {
                println!("任务执行成功: {}", result.task_id);
            }
            Ok(())
        }
        crate::core::project::types::TaskStatus::Failed => {
            eprintln!("任务执行失败: {}", result.task_id);
            std::process::exit(1);
        }
        _ => {
            eprintln!("任务状态异常: {:?}", result.status);
            std::process::exit(1);
        }
    }
}
