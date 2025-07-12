use crate::core::builder::image::inspector::ImageInspector;
use crate::core::executor::{DockerExecutor, Executor};
use anyhow::Result;

/// 镜像拉取器
pub struct ImagePuller;

impl ImagePuller {
    /// 确保基础镜像可用（检查本地，如果不存在则拉取）
    pub async fn ensure_base_image_available(base_image: &str) -> Result<()> {
        println!("• 检查基础镜像: {}", base_image);

        // 检查基础镜像是否在本地存在
        match ImageInspector::get_image_info(base_image).await? {
            Some(info) => {
                println!("✓ 基础镜像已存在!");
                println!("   → 镜像ID: {}", info.id);
                println!("   → 仓库: {}", info.repository);
                println!("   → 标签: {}", info.tag);
                println!("   → 创建时间: {}", info.created_at);
                println!("   → 大小: {}", info.size);
                Ok(())
            }
            None => {
                println!("! 基础镜像不存在本地: {}", base_image);
                println!("▶ 开始拉取基础镜像...");
                Self::pull_base_image(base_image).await
            }
        }
    }

    /// 拉取基础镜像
    async fn pull_base_image(base_image: &str) -> Result<()> {
        println!("▶ 正在拉取基础镜像: {}", base_image);

        // 创建 Docker 执行器
        let executor = DockerExecutor::new();

        tracing::debug!("Docker pull 镜像: {}", base_image);

        // 执行镜像拉取
        executor.pull_image(base_image).await?;

        println!("✓ 基础镜像拉取成功: {}", base_image);

        // 验证镜像是否成功拉取
        match ImageInspector::get_image_info(base_image).await? {
            Some(_) => {
                println!("✓ 基础镜像验证成功");
                Ok(())
            }
            None => Err(anyhow::anyhow!("基础镜像拉取后仍然无法找到: {}", base_image)),
        }
    }
}
