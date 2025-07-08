use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TaskCommand {
    #[command(subcommand)]
    command: TaskSubcommand,
}

#[derive(Subcommand)]
pub enum TaskSubcommand {
    /// 列出所有任务
    List {
        /// 只显示运行中的任务
        #[arg(long)]
        running: bool,
        /// 限制显示数量
        #[arg(long, short = 'n', default_value = "10")]
        limit: usize,
    },
    /// 查看任务详情
    Show {
        /// 任务ID
        task_id: String,
    },
    /// 终止任务
    Kill {
        /// 任务ID
        task_id: String,
        /// 强制终止
        #[arg(long)]
        force: bool,
    },
    /// 重启任务
    Restart {
        /// 任务ID
        task_id: String,
    },
    /// 清理已完成的任务
    Clean {
        /// 清理天数
        #[arg(long, default_value = "7")]
        days: u32,
        /// 强制清理
        #[arg(long)]
        force: bool,
    },
}

impl TaskCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            TaskSubcommand::List { running, limit } => {
                tracing::info!("列出任务 (running: {}, limit: {})", running, limit);
                println!("暂未实现 - task list 命令");
            }
            TaskSubcommand::Show { task_id } => {
                tracing::info!("查看任务详情: {}", task_id);
                println!("暂未实现 - task show 命令");
            }
            TaskSubcommand::Kill { task_id, force } => {
                tracing::info!("终止任务: {} (force: {})", task_id, force);
                println!("暂未实现 - task kill 命令");
            }
            TaskSubcommand::Restart { task_id } => {
                tracing::info!("重启任务: {}", task_id);
                println!("暂未实现 - task restart 命令");
            }
            TaskSubcommand::Clean { days, force } => {
                tracing::info!("清理任务 (days: {}, force: {})", days, force);
                println!("暂未实现 - task clean 命令");
            }
        }
        Ok(())
    }
}
