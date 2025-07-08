use anyhow::Result;
use clap::Args;

use crate::core::interactive::{InteractiveConfig, InteractiveEngine};

#[derive(Args)]
pub struct InteractiveCommand {
    /// 工作空间目录
    #[arg(long, default_value = ".")]
    pub workspace: String,

    /// 默认配置文件
    #[arg(long)]
    pub config: Option<String>,
}

impl InteractiveCommand {
    pub async fn execute(self) -> Result<()> {
        tracing::info!("启动交互式模式 (workspace: {})", self.workspace);

        // 创建交互式配置
        let config = InteractiveConfig {
            workspace: self.workspace,
            config: self.config,
            ..Default::default()
        };

        // 创建并运行交互式引擎
        let mut engine = InteractiveEngine::new(config)?;
        engine.run().await?;

        Ok(())
    }
}
