use crate::core::builder::types::{ImageCheckResult, ImageInfo};
use crate::core::executor::{DockerExecutor, Executor};
use anyhow::Result;
use std::process::Command;

/// 镜像检查器
pub struct ImageInspector;

impl ImageInspector {
    /// 获取镜像 ID
    pub async fn get_image_id(image_name: &str) -> Result<String> {
        let output = Command::new("docker").args(&["images", "-q", image_name]).output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("无法获取镜像 ID"));
        }

        let image_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if image_id.is_empty() {
            return Err(anyhow::anyhow!("镜像不存在或构建失败"));
        }

        Ok(image_id)
    }

    /// 检查镜像是否存在
    pub async fn image_exists(image_name: &str) -> Result<bool> {
        let executor = DockerExecutor::new();
        executor.image_exists(image_name).await
    }

    /// 获取镜像详细信息
    pub async fn get_image_info(image_name: &str) -> Result<Option<ImageInfo>> {
        // 使用 docker images 命令获取详细信息
        let output = Command::new("docker")
            .args(&[
                "images",
                "--format",
                "{{.ID}}\t{{.Repository}}\t{{.Tag}}\t{{.CreatedAt}}\t{{.Size}}",
                image_name,
            ])
            .output()?;

        if !output.status.success() {
            return Ok(None);
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let line = output_str.trim();

        if line.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 5 {
            let info = ImageInfo {
                id: parts[0].to_string(),
                repository: parts[1].to_string(),
                tag: parts[2].to_string(),
                created_at: parts[3].to_string(),
                size: parts[4].to_string(),
            };
            Ok(Some(info))
        } else {
            Ok(None)
        }
    }

    /// 详细检查目标镜像并显示信息
    pub async fn check_target_image(image_name: &str) -> Result<ImageCheckResult> {
        println!("• 检查目标镜像: {}", image_name);

        // 首先检查镜像是否存在
        match Self::get_image_info(image_name).await? {
            Some(info) => {
                println!("✓ 目标镜像已存在!");
                println!("   → 镜像ID: {}", info.id);
                println!("   → 仓库: {}", info.repository);
                println!("   → 标签: {}", info.tag);
                println!("   → 创建时间: {}", info.created_at);
                println!("   → 大小: {}", info.size);

                Ok(ImageCheckResult::Exists(info))
            }
            None => {
                println!("! 目标镜像不存在: {}", image_name);

                // 检查是否有相似的镜像
                let similar_images = Self::find_similar_images(image_name).await?;
                if !similar_images.is_empty() {
                    println!("• 发现相似的镜像:");
                    for similar in &similar_images {
                        println!(
                            "   → {}:{} ({})",
                            similar.repository,
                            similar.tag,
                            similar.id[..12].to_string()
                        );
                    }
                }

                Ok(ImageCheckResult::NotExists)
            }
        }
    }

    /// 查找相似的镜像（基于镜像名称前缀）
    pub async fn find_similar_images(image_name: &str) -> Result<Vec<ImageInfo>> {
        // 提取镜像名称（去掉标签）
        let base_name =
            if let Some(pos) = image_name.find(':') { &image_name[..pos] } else { image_name };

        let output = Command::new("docker")
            .args(&[
                "images",
                "--format",
                "{{.ID}}\t{{.Repository}}\t{{.Tag}}\t{{.CreatedAt}}\t{{.Size}}",
                "--filter",
                &format!("reference={}*", base_name),
            ])
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut similar_images = Vec::new();

        for line in output_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                let info = ImageInfo {
                    id: parts[0].to_string(),
                    repository: parts[1].to_string(),
                    tag: parts[2].to_string(),
                    created_at: parts[3].to_string(),
                    size: parts[4].to_string(),
                };

                // 不包含完全匹配的镜像
                if format!("{}:{}", info.repository, info.tag) != image_name
                    && info.repository != "<none>"
                {
                    similar_images.push(info);
                }
            }
        }

        // 限制返回的相似镜像数量
        similar_images.truncate(5);
        Ok(similar_images)
    }

    /// 删除镜像
    pub async fn remove_image(image_name: &str, force: bool) -> Result<()> {
        let executor = DockerExecutor::new();
        let params = crate::core::executor::ImageOperationParams {
            image: image_name.to_string(),
            force,
            extra_args: vec![],
        };

        executor.remove_image(&params).await?;
        tracing::info!("镜像删除成功: {}", image_name);
        Ok(())
    }
}
