use std::io::BufRead;

/// 构建输出处理器
pub struct BuildOutputHandler;

impl BuildOutputHandler {
    /// 实时读取和显示输出
    pub async fn read_and_display_output<R: BufRead + Send + 'static>(
        mut reader: R,
        prefix: &str,
    ) -> String {
        let prefix = prefix.to_string();

        tokio::task::spawn_blocking(move || {
            let mut output = String::new();
            let mut line = String::new();

            while let Ok(bytes_read) = reader.read_line(&mut line) {
                if bytes_read == 0 {
                    break;
                }

                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    println!("▶ [{}] {}", prefix, trimmed);
                    output.push_str(&line);
                }
                line.clear();
            }

            output
        })
        .await
        .unwrap_or_default()
    }

    /// 智能读取和显示构建输出（区分正常信息和错误）
    pub async fn read_and_display_build_output<R: BufRead + Send + 'static>(
        mut reader: R,
    ) -> String {
        tokio::task::spawn_blocking(move || {
            let mut output = String::new();
            let mut line = String::new();

            while let Ok(bytes_read) = reader.read_line(&mut line) {
                if bytes_read == 0 {
                    break;
                }

                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    // 根据内容类型选择合适的前缀和图标
                    if Self::is_build_step(trimmed) {
                        println!("● [STEP] {}", trimmed);
                    } else if Self::is_build_progress(trimmed) {
                        println!("▶ [BUILD] {}", trimmed);
                    } else if Self::is_build_success(trimmed) {
                        println!("✓ [DONE] {}", trimmed);
                    } else if Self::is_build_error(trimmed) {
                        println!("✗ [ERR] {}", trimmed);
                    } else {
                        println!("→ [INFO] {}", trimmed);
                    }
                    output.push_str(&line);
                }
                line.clear();
            }

            output
        })
        .await
        .unwrap_or_default()
    }

    /// 实时读取和显示拉取输出
    pub async fn read_and_display_pull_output<R: BufRead + Send + 'static>(
        mut reader: R,
        prefix: &str,
    ) -> String {
        let prefix = prefix.to_string();

        tokio::task::spawn_blocking(move || {
            let mut output = String::new();
            let mut line = String::new();

            while let Ok(bytes_read) = reader.read_line(&mut line) {
                if bytes_read == 0 {
                    break;
                }

                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    // 为拉取过程使用不同的图标
                    if prefix == "PULL" {
                        if trimmed.contains("Pulling") {
                            println!("● [{}] {}", prefix, trimmed);
                        } else if trimmed.contains("Downloading") {
                            println!("▼ [{}] {}", prefix, trimmed);
                        } else if trimmed.contains("Extracting") {
                            println!("▶ [{}] {}", prefix, trimmed);
                        } else if trimmed.contains("Pull complete") {
                            println!("✓ [{}] {}", prefix, trimmed);
                        } else {
                            println!("→ [{}] {}", prefix, trimmed);
                        }
                    } else {
                        println!("✗ [{}] {}", prefix, trimmed);
                    }
                    output.push_str(&line);
                }
                line.clear();
            }

            output
        })
        .await
        .unwrap_or_default()
    }

    /// 判断是否为构建步骤信息
    fn is_build_step(line: &str) -> bool {
        line.starts_with('#')
            && (line.contains("FROM")
                || line.contains("RUN")
                || line.contains("COPY")
                || line.contains("ADD")
                || line.contains("WORKDIR")
                || line.contains("ENV")
                || line.contains("EXPOSE")
                || line.contains("CMD")
                || line.contains("ENTRYPOINT"))
    }

    /// 判断是否为构建进度信息
    fn is_build_progress(line: &str) -> bool {
        line.contains("naming to")
            || line.contains("unpacking to")
            || line.contains("extracting")
            || line.contains("downloading")
            || line.contains("pulling")
            || line.contains("exporting to")
            || line.contains("writing image")
            || line.contains("CACHED")
            || line.contains("sha256:")
    }

    /// 判断是否为成功信息
    fn is_build_success(line: &str) -> bool {
        line.contains("DONE")
            || line.contains("done")
            || line.contains("complete")
            || line.contains("success")
            || line.contains("finished")
    }

    /// 判断是否为错误信息
    fn is_build_error(line: &str) -> bool {
        line.to_lowercase().contains("error")
            || line.to_lowercase().contains("failed")
            || line.to_lowercase().contains("fatal")
    }
}
