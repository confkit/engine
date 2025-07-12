use crate::core::builder::{image::inspector::ImageInspector, output_handler::BuildOutputHandler};
use anyhow::Result;
use std::io::BufReader;
use std::process::{Command, Stdio};

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

        let mut cmd = Command::new("docker");
        cmd.arg("pull").arg(base_image);

        // 设置标准输出和错误输出管道
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        tracing::debug!("Docker pull 命令: {:?}", cmd);

        // 启动进程
        let mut child = cmd.spawn()?;

        // 获取标准输出和错误输出
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // 实时读取和显示输出
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // 启动两个任务来并行读取输出
        let (stdout_logs, stderr_logs) = tokio::join!(
            BuildOutputHandler::read_and_display_pull_output(stdout_reader, "PULL"),
            BuildOutputHandler::read_and_display_pull_output(stderr_reader, "ERR")
        );

        // 等待进程完成
        let status = child.wait()?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "拉取基础镜像失败 (退出代码: {})\n{}",
                status.code().unwrap_or(-1),
                stderr_logs
            ));
        }

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
