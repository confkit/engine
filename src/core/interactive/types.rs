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
        Self { workspace: ".".to_string(), config: None, max_history: 100, enable_colors: true }
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
        Self { config, history: Vec::new(), session_data: HashMap::new() }
    }

    pub fn add_to_history(&mut self, command: String) {
        self.history.push(command);
        if self.history.len() > self.config.max_history {
            self.history.remove(0);
        }
    }
}

/// 交互模式状态
#[derive(Debug, Clone)]
pub enum InteractiveMode {
    /// 主菜单
    MainMenu,
    /// Builder 菜单
    BuilderMenu,
    /// 镜像管理菜单
    ImageMenu,
    /// 容器管理菜单
    ContainerMenu,
    /// 运行菜单
    RunMenu,
    /// 镜像列表参数选择
    ImageListParams { verbose: bool, status_filter: Option<String> },
    /// 镜像创建参数选择
    ImageCreateParams,
    /// 镜像删除参数选择
    ImageRemoveParams,
    /// 运行项目参数选择
    RunProjectParams,
    /// 运行项目执行
    RunProjectExecution {
        space: String,
        project: String,
        verbose: bool,
        dry_run: bool,
        git_branch: Option<String>,
        force: bool,
    },
    /// Builder List 参数选择 (保留向后兼容)
    BuilderListParams { verbose: bool, status_filter: Option<String> },
    /// Builder Create 参数选择 (保留向后兼容)
    BuilderCreateParams,
}

/// 菜单项
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub key: String,
    pub title: String,
    pub description: String,
}

/// 基础命令类型
#[derive(Debug, Clone)]
pub enum BaseCommand {
    /// Builder 相关命令
    Builder,
    /// 任务相关命令
    Task,
    /// 配置相关命令
    Config,
    /// Git 相关命令
    Git,
}

/// Builder 子命令
#[derive(Debug, Clone)]
pub enum BuilderSubCommand {
    /// 列出构建器
    List,
    /// 创建构建器
    Create,
    /// 启动构建器
    Start,
    /// 停止构建器
    Stop,
    /// 删除构建器
    Remove,
    /// 健康检查
    Health,
}

/// 构建器状态选项
#[derive(Debug, Clone)]
pub enum BuilderStatusOption {
    /// 所有状态
    All,
    /// 未创建
    NotCreated,
    /// 已创建
    Created,
    /// 运行中
    Running,
    /// 已停止
    Stopped,
    /// 错误
    Error,
}

impl BuilderStatusOption {
    pub fn to_filter_string(&self) -> Option<String> {
        match self {
            BuilderStatusOption::All => None,
            BuilderStatusOption::NotCreated => Some("notcreated".to_string()),
            BuilderStatusOption::Created => Some("created".to_string()),
            BuilderStatusOption::Running => Some("running".to_string()),
            BuilderStatusOption::Stopped => Some("stopped".to_string()),
            BuilderStatusOption::Error => Some("error".to_string()),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            BuilderStatusOption::All => "所有状态",
            BuilderStatusOption::NotCreated => "未创建",
            BuilderStatusOption::Created => "已创建",
            BuilderStatusOption::Running => "运行中",
            BuilderStatusOption::Stopped => "已停止",
            BuilderStatusOption::Error => "错误",
        }
    }
}

/// 命令类型枚举 (保留向后兼容性)
#[derive(Debug, Clone)]
pub enum Command {
    /// 帮助命令
    Help,
    /// 退出命令
    Exit,
    /// 清屏命令
    Clear,
    /// Builder 列表命令
    BuilderList { verbose: bool, status_filter: Option<String> },
}

impl Command {
    /// 从输入字符串解析命令 (保留向后兼容性)
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

                Ok(Command::BuilderList { verbose, status_filter })
            }
            _ => Err(anyhow::anyhow!("未知 builder 子命令: {}", args[0])),
        }
    }
}

/// 运行子命令
#[derive(Debug, Clone)]
pub enum RunSubCommand {
    /// 列出可用项目
    List,
    /// 运行项目
    Run,
    /// 查看项目详情
    Show,
}
