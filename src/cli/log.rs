use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct LogCommand {
    #[command(subcommand)]
    command: LogSubcommand,
}

#[derive(Subcommand)]
pub enum LogSubcommand {
    /// 列出日志文件
    List {
        /// 空间名称（必需）
        #[arg(long, required = true)]
        space: String,
        /// 项目名称（必需）
        #[arg(long, required = true)]
        project: String,
        /// 显示详细信息
        #[arg(long)]
        verbose: bool,
        /// 显示最近的 N 个日志文件
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    /// 查看日志内容
    Show {
        /// 空间名称（必需）
        #[arg(long, required = true)]
        space: String,
        /// 项目名称（必需）
        #[arg(long, required = true)]
        project: String,
        /// 日志文件名（可以不带.txt后缀）
        filename: String,
        /// 跟踪日志输出
        #[arg(long, short = 'f')]
        follow: bool,
        /// 显示的行数
        #[arg(long, short = 'n', default_value = "100")]
        lines: usize,
        /// 显示时间戳
        #[arg(long)]
        timestamps: bool,
        /// 过滤步骤
        #[arg(long)]
        step: Option<String>,
        /// 日志级别过滤
        #[arg(long)]
        level: Option<String>,
    },
    /// 清理日志文件
    Clean {
        /// 保留天数
        #[arg(long, default_value = "7")]
        keep_days: u32,
        /// 干预模式，不实际删除
        #[arg(long)]
        dry_run: bool,
        /// 项目名称过滤
        #[arg(long)]
        project: Option<String>,
        /// 空间名称过滤
        #[arg(long)]
        space: Option<String>,
    },
}

impl LogCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            LogSubcommand::List { space, project, verbose, limit } => {
                handle_log_list(space, project, verbose, limit).await
            }
            LogSubcommand::Show {
                space,
                project,
                filename,
                follow,
                lines,
                timestamps,
                step,
                level,
            } => {
                handle_log_show(space, project, filename, follow, lines, timestamps, step, level)
                    .await
            }
            LogSubcommand::Clean { keep_days, dry_run, project, space } => {
                handle_log_clean(keep_days, dry_run, project, space).await
            }
        }
    }
}

/// 处理日志列表命令
pub async fn handle_log_list(
    space: String,
    project: String,
    verbose: bool,
    limit: usize,
) -> Result<()> {
    tracing::info!(
        "列出日志文件: space={}, project={}, verbose={}, limit={}",
        space,
        project,
        verbose,
        limit
    );

    let logs_dir = PathBuf::from("./volumes/logs");

    if !logs_dir.exists() {
        println!("※ 日志目录不存在: {}", logs_dir.display());
        return Ok(());
    }

    // 检查指定的空间和项目目录是否存在
    let project_dir = logs_dir.join(&space).join(&project);
    if !project_dir.exists() {
        println!("※ 未找到项目目录: {}/{}", space, project);
        return Ok(());
    }

    let mut log_files = Vec::new();

    // 扫描指定空间和项目的日志目录
    scan_log_directory(&logs_dir, &mut log_files, &Some(project.clone()), &Some(space.clone()))?;

    // 按修改时间排序（最新的在前）
    log_files.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));

    // 限制显示数量
    log_files.truncate(limit);

    if log_files.is_empty() {
        println!("※ 未找到日志文件");
        return Ok(());
    }

    println!("▶ 找到 {} 个日志文件:", log_files.len());
    println!();

    for log_file in &log_files {
        if verbose {
            println!("▶ 任务ID: {}", log_file.task_id);
            println!("  文件路径: {}", log_file.path.display());
            println!("  空间: {}", log_file.space);
            println!("  项目: {}", log_file.project);
            println!("  修改时间: {}", log_file.modified_time.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("  文件大小: {}", log_file.size_mb);
        } else {
            println!(
                "● {} | {} | {}",
                log_file.task_id,
                log_file.modified_time.format("%Y-%m-%d %H:%M:%S"),
                log_file.size_mb
            );
        }
        if verbose {
            println!();
        }
    }

    Ok(())
}

/// 处理日志显示命令
pub async fn handle_log_show(
    space: String,
    project: String,
    filename: String,
    follow: bool,
    lines: usize,
    timestamps: bool,
    step: Option<String>,
    level: Option<String>,
) -> Result<()> {
    tracing::info!(
        "查看日志: space={}, project={}, filename={}, follow={}, lines={}",
        space,
        project,
        filename,
        follow,
        lines
    );

    let logs_dir = PathBuf::from("./volumes/logs");
    if !logs_dir.exists() {
        println!("※ 日志目录不存在: {}", logs_dir.display());
        return Ok(());
    }

    // 构建日志文件路径
    let project_dir = logs_dir.join(&space).join(&project);
    if !project_dir.exists() {
        println!("※ 未找到项目目录: {}/{}", space, project);
        return Ok(());
    }

    // 扫描该项目目录下的所有日志文件，查找匹配的文件名
    let mut log_files = Vec::new();
    scan_log_directory(&logs_dir, &mut log_files, &Some(project.clone()), &Some(space.clone()))?;

    // 清理用户输入的文件名，去掉可能的 .txt 后缀
    let cleaned_filename =
        if filename.ends_with(".txt") { &filename[..filename.len() - 4] } else { &filename };

    // 查找匹配的日志文件 - 支持多种匹配方式
    let target_file = log_files.iter().find(|f| {
        let file_stem = f.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

        // 1. 文件名完全匹配（不带后缀）
        file_stem == cleaned_filename ||
        // 2. 文件名包含输入的字符串
        file_stem.contains(cleaned_filename) ||
        // 3. 任务ID匹配（兼容性）
        f.task_id == cleaned_filename ||
        f.task_id.contains(cleaned_filename)
    });

    if target_file.is_none() {
        println!("※ 未找到文件名为 {} 的日志文件", cleaned_filename);
        println!("  可用的日志文件:");
        for (i, log_file) in log_files.iter().take(5).enumerate() {
            let filename = log_file.path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");
            println!("    {}. {}", i + 1, filename);
        }
        if log_files.len() > 5 {
            println!("    ... 还有 {} 个日志文件", log_files.len() - 5);
        }
        return Ok(());
    }

    let log_file = target_file.unwrap();
    let actual_filename = log_file.path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");

    println!("▶ 查看日志: {}", log_file.path.display());
    println!("  空间: {}", log_file.space);
    println!("  项目: {}", log_file.project);
    println!("  文件名: {}", actual_filename);
    println!("  任务ID: {}", log_file.task_id);
    println!("  修改时间: {}", log_file.modified_time.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  文件大小: {}", log_file.size_mb);

    if cleaned_filename != actual_filename.trim_end_matches(".txt") {
        println!("  匹配方式: 部分匹配 (输入: {})", cleaned_filename);
    }
    println!();

    // 读取并显示日志内容
    display_log_content(&log_file.path, lines, timestamps, step.as_deref(), level.as_deref())?;

    // 如果启用了跟踪模式，实时显示新增内容
    if follow {
        println!();
        println!("▶ 跟踪日志输出... (按 Ctrl+C 停止)");
        follow_log_file(&log_file.path, timestamps, step.as_deref(), level.as_deref()).await?;
    }

    Ok(())
}

/// 处理日志清理命令
async fn handle_log_clean(
    keep_days: u32,
    dry_run: bool,
    project: Option<String>,
    space: Option<String>,
) -> Result<()> {
    tracing::info!(
        "清理日志: keep_days={}, dry_run={}, project={:?}, space={:?}",
        keep_days,
        dry_run,
        project,
        space
    );

    println!("暂未实现 - log clean 命令");
    println!("保留天数: {}", keep_days);
    println!("干预模式: {}", dry_run);
    if let Some(project) = project {
        println!("项目过滤: {}", project);
    }
    if let Some(space) = space {
        println!("空间过滤: {}", space);
    }

    Ok(())
}

/// 日志文件信息
#[derive(Debug, Clone)]
pub struct LogFileInfo {
    pub path: PathBuf,
    pub space: String,
    pub project: String,
    pub task_id: String,
    pub modified_time: chrono::DateTime<chrono::Utc>,
    pub size_mb: String,
}

/// 扫描日志目录
pub fn scan_log_directory(
    logs_dir: &PathBuf,
    log_files: &mut Vec<LogFileInfo>,
    project_filter: &Option<String>,
    space_filter: &Option<String>,
) -> Result<()> {
    use std::fs;

    // 遍历空间目录
    for space_entry in fs::read_dir(logs_dir)? {
        let space_entry = space_entry?;
        let space_path = space_entry.path();

        if !space_path.is_dir() {
            continue;
        }

        let space_name =
            space_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();

        // 应用空间过滤
        if let Some(filter) = space_filter {
            if !space_name.contains(filter) {
                continue;
            }
        }

        // 遍历项目目录
        for project_entry in fs::read_dir(&space_path)? {
            let project_entry = project_entry?;
            let project_path = project_entry.path();

            if !project_path.is_dir() {
                continue;
            }

            let project_name =
                project_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();

            // 应用项目过滤
            if let Some(filter) = project_filter {
                if !project_name.contains(filter) {
                    continue;
                }
            }

            // 遍历日志文件
            for log_entry in fs::read_dir(&project_path)? {
                let log_entry = log_entry?;
                let log_path = log_entry.path();

                if !log_path.is_file() || !log_path.extension().map_or(false, |ext| ext == "txt") {
                    continue;
                }

                let file_name = log_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");

                // 解析任务ID（从文件名中提取）
                let task_id = extract_task_id_from_filename(file_name);

                // 获取文件元数据
                let metadata = fs::metadata(&log_path)?;
                let modified_time = metadata.modified()?;
                let modified_time = chrono::DateTime::from(modified_time);

                let size_bytes = metadata.len();
                let size_mb = if size_bytes > 1024 * 1024 {
                    format!("{:.1}MB", size_bytes as f64 / (1024.0 * 1024.0))
                } else if size_bytes > 1024 {
                    format!("{:.1}KB", size_bytes as f64 / 1024.0)
                } else {
                    format!("{}B", size_bytes)
                };

                log_files.push(LogFileInfo {
                    path: log_path,
                    space: space_name.clone(),
                    project: project_name.clone(),
                    task_id,
                    modified_time,
                    size_mb,
                });
            }
        }
    }

    Ok(())
}

/// 从文件名中提取任务ID
fn extract_task_id_from_filename(filename: &str) -> String {
    // 文件名格式：2025-07-13_05-45-40_task_id.txt
    if let Some(last_underscore) = filename.rfind('_') {
        let task_part = &filename[last_underscore + 1..];
        if let Some(dot_pos) = task_part.find('.') {
            task_part[..dot_pos].to_string()
        } else {
            task_part.to_string()
        }
    } else {
        filename.to_string()
    }
}

/// 在日志文件列表中查找匹配的文件
fn find_log_file(
    log_files: &[LogFileInfo],
    target: &str,
    task_id: &Option<String>,
) -> Result<Option<LogFileInfo>> {
    // 如果提供了 task_id，优先使用
    if let Some(task_id) = task_id {
        for log_file in log_files {
            if log_file.task_id == *task_id {
                return Ok(Some(log_file.clone()));
            }
        }
        return Ok(None);
    }

    // 尝试将 target 作为完整的任务ID
    for log_file in log_files {
        if log_file.task_id == target {
            return Ok(Some(log_file.clone()));
        }
    }

    // 尝试将 target 作为项目名称，返回最新的日志文件
    let mut matching_files: Vec<&LogFileInfo> =
        log_files.iter().filter(|f| f.project == target).collect();

    if matching_files.is_empty() {
        return Ok(None);
    }

    // 按修改时间排序，返回最新的
    matching_files.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));
    Ok(Some(matching_files[0].clone()))
}

/// 显示日志内容
fn display_log_content(
    log_path: &std::path::Path,
    lines: usize,
    timestamps: bool,
    step_filter: Option<&str>,
    level_filter: Option<&str>,
) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(log_path)?;
    let reader = BufReader::new(file);
    let mut log_lines = Vec::new();

    // 读取所有行
    for line in reader.lines() {
        let line = line?;

        // 应用过滤器
        if let Some(step) = step_filter {
            if !line.contains(step) {
                continue;
            }
        }

        if let Some(level) = level_filter {
            if !line.contains(level) {
                continue;
            }
        }

        log_lines.push(line);
    }

    // 显示最后 N 行
    let start_index = if log_lines.len() > lines { log_lines.len() - lines } else { 0 };

    println!("▶ 日志内容 (显示最后 {} 行):", std::cmp::min(lines, log_lines.len()));
    println!("{}", "─".repeat(60));

    for line in &log_lines[start_index..] {
        if timestamps {
            println!("{}", line);
        } else {
            // 尝试移除时间戳（如果行以时间戳开头）
            let clean_line = if line.len() > 23 && line.chars().nth(4) == Some('-') {
                // 格式: 2025-01-13 10:30:45.123 UTC [INFO] ...
                if let Some(bracket_pos) = line.find('[') {
                    &line[bracket_pos..]
                } else {
                    line
                }
            } else {
                line
            };
            println!("{}", clean_line);
        }
    }

    Ok(())
}

/// 跟踪日志文件的变化
async fn follow_log_file(
    log_path: &std::path::Path,
    timestamps: bool,
    step_filter: Option<&str>,
    level_filter: Option<&str>,
) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Seek, SeekFrom};
    use tokio::time::{sleep, Duration};

    let mut file = File::open(log_path)?;

    // 移动到文件末尾
    file.seek(SeekFrom::End(0))?;

    loop {
        let mut reader = BufReader::new(&file);
        let mut buffer = String::new();

        // 读取新增的内容
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let line = buffer.trim_end();

                    // 应用过滤器
                    if let Some(step) = step_filter {
                        if !line.contains(step) {
                            continue;
                        }
                    }

                    if let Some(level) = level_filter {
                        if !line.contains(level) {
                            continue;
                        }
                    }

                    // 显示新行
                    if timestamps {
                        println!("{}", line);
                    } else {
                        // 尝试移除时间戳
                        let clean_line = if line.len() > 23 && line.chars().nth(4) == Some('-') {
                            if let Some(bracket_pos) = line.find('[') {
                                &line[bracket_pos..]
                            } else {
                                line
                            }
                        } else {
                            line
                        };
                        println!("{}", clean_line);
                    }
                }
                Err(_) => break,
            }
        }

        // 等待一段时间再检查
        sleep(Duration::from_millis(500)).await;

        // 重新打开文件以检查是否有新内容
        file = File::open(log_path)?;
        file.seek(SeekFrom::End(0))?;
    }
}
