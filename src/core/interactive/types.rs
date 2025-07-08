use anyhow::Result;
use std::collections::HashMap;

/// 交互式引擎配置
#[derive(Debug, Clone)]
pub struct InteractiveConfig {
    /// 工作空间目录
    pub workspace: String,
    /// 默认配置文件
    pub config: Option<String>,
    /// 命令历史记录最大数量
    pub max_history: usize,
    /// 是否启用颜色输出
    pub enable_colors: bool,
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            workspace: ".".to_string(),
            config: None,
            max_history: 100,
            enable_colors: true,
        }
    }
}

/// 命令执行结果
#[derive(Debug)]
pub enum CommandResult {
    /// 继续执行
    Continue,
    /// 退出程序
    Exit,
    /// 显示帮助
    Help(String),
    /// 错误
    Error(String),
}

/// 命令上下文
#[derive(Debug)]
pub struct CommandContext {
    /// 配置
    pub config: InteractiveConfig,
    /// 命令历史
    pub history: Vec<String>,
    /// 会话状态
    pub session_data: HashMap<String, String>,
}

impl CommandContext {
    pub fn new(config: InteractiveConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
            session_data: HashMap::new(),
        }
    }

    pub fn add_to_history(&mut self, command: String) {
        self.history.push(command);
        if self.history.len() > self.config.max_history {
            self.history.remove(0);
        }
    }
}

/// 命令类型枚举
#[derive(Debug, Clone)]
pub enum Command {
    /// 帮助命令
    Help,
    /// 退出命令
    Exit,
    /// 清屏命令
    Clear,
    /// Builder 列表命令
    BuilderList {
        verbose: bool,
        status_filter: Option<String>,
    },
}

impl Command {
    /// 从输入字符串解析命令
    pub fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("空命令"));
        }

        match parts[0] {
            "help" | "h" => Ok(Command::Help),
            "exit" | "quit" | "q" => Ok(Command::Exit),
            "clear" | "cls" => Ok(Command::Clear),
            "builder" | "b" => Self::parse_builder_command(&parts[1..]),
            _ => Err(anyhow::anyhow!("未知命令: {}", parts[0])),
        }
    }

    fn parse_builder_command(args: &[&str]) -> Result<Self> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("builder 命令需要子命令"));
        }

        match args[0] {
            "list" | "ls" => {
                let mut verbose = false;
                let mut status_filter = None;

                // 解析参数
                let mut i = 1;
                while i < args.len() {
                    match args[i] {
                        "-v" | "--verbose" => verbose = true,
                        "--status" => {
                            if i + 1 < args.len() {
                                status_filter = Some(args[i + 1].to_string());
                                i += 1;
                            } else {
                                return Err(anyhow::anyhow!("--status 需要一个值"));
                            }
                        }
                        _ => {
                            return Err(anyhow::anyhow!("未知参数: {}", args[i]));
                        }
                    }
                    i += 1;
                }

                Ok(Command::BuilderList {
                    verbose,
                    status_filter,
                })
            }
            _ => Err(anyhow::anyhow!("未知 builder 子命令: {}", args[0])),
        }
    }
}
