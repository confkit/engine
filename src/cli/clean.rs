use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// clean all
    #[arg(short, long)]
    pub all: bool,

    /// clean workspace
    #[arg(short, long)]
    pub workspace: bool,

    /// clean artifacts
    #[arg(short, long)]
    pub artifacts: bool,

    /// clean space
    #[arg(short, long)]
    pub space: Option<String>,

    /// clean project
    #[arg(short, long)]
    pub project: Option<String>,
}

/// 处理 run 命令
pub async fn handle_clean(args: &CleanArgs) -> Result<()> {
    Ok(())
}
