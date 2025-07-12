use anyhow::Result;
use std::collections::HashMap;

/// 执行引擎类型
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorType {
    /// Docker 容器执行
    Docker,
    /// 本地主机执行
    Local,
    /// Kubernetes 执行（未来扩展）
    Kubernetes,
    /// Podman 执行（未来扩展）
    Podman,
}

/// 执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 工作目录
    pub working_dir: Option<String>,
    /// 环境变量
    pub environment: HashMap<String, String>,
    /// 用户（格式：uid:gid 或 username）
    pub user: Option<String>,
    /// 网络模式
    pub network: Option<String>,
    /// 卷挂载
    pub volumes: Vec<VolumeMount>,
    /// 端口映射
    pub ports: Vec<PortMapping>,
    /// 超时时间（秒）
    pub timeout: Option<u64>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            working_dir: None,
            environment: HashMap::new(),
            user: None,
            network: None,
            volumes: Vec::new(),
            ports: Vec::new(),
            timeout: None,
        }
    }
}

/// 卷挂载配置
#[derive(Debug, Clone)]
pub struct VolumeMount {
    /// 主机路径
    pub host_path: String,
    /// 容器路径
    pub container_path: String,
    /// 是否只读
    pub read_only: bool,
}

/// 端口映射配置
#[derive(Debug, Clone)]
pub struct PortMapping {
    /// 主机端口
    pub host_port: u16,
    /// 容器端口
    pub container_port: u16,
    /// 协议
    pub protocol: String, // tcp, udp
}

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// 退出码
    pub exit_code: i32,
    /// 标准输出
    pub stdout: String,
    /// 标准错误输出
    pub stderr: String,
    /// 执行耗时（毫秒）
    pub duration_ms: u64,
    /// 是否成功
    pub success: bool,
}

/// 命令执行配置
#[derive(Debug, Clone)]
pub struct CommandExecution {
    /// 要执行的命令
    pub commands: Vec<String>,
    /// 执行上下文
    pub context: ExecutionContext,
    /// 容器镜像（仅Docker执行时使用）
    pub image: Option<String>,
    /// 容器名称（仅Docker执行时使用）
    pub container_name: Option<String>,
    /// 是否自动删除容器（仅Docker执行时使用）
    pub auto_remove: bool,
}

impl Default for CommandExecution {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            context: ExecutionContext::default(),
            image: None,
            container_name: None,
            auto_remove: false,
        }
    }
}

/// 镜像构建参数
#[derive(Debug, Clone)]
pub struct ImageBuildParams {
    /// 镜像标签
    pub tag: String,
    /// Dockerfile 路径
    pub dockerfile: String,
    /// 构建上下文路径
    pub context: String,
    /// 构建参数
    pub build_args: HashMap<String, String>,
    /// 目标平台
    pub platform: Option<String>,
    /// 无缓存构建
    pub no_cache: bool,
}

/// 镜像操作参数
#[derive(Debug, Clone)]
pub struct ImageOperationParams {
    /// 镜像名称或ID
    pub image: String,
    /// 强制操作
    pub force: bool,
    /// 额外参数
    pub extra_args: Vec<String>,
}

/// 执行引擎抽象接口
#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    /// 获取执行引擎类型
    fn executor_type(&self) -> ExecutorType;

    /// 检查执行引擎是否可用
    async fn is_available(&self) -> Result<bool>;

    /// 执行命令
    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult>;

    /// 构建镜像（仅支持容器执行引擎）
    async fn build_image(&self, params: &ImageBuildParams) -> Result<String>;

    /// 拉取镜像（仅支持容器执行引擎）
    async fn pull_image(&self, image: &str) -> Result<()>;

    /// 删除镜像（仅支持容器执行引擎）
    async fn remove_image(&self, params: &ImageOperationParams) -> Result<()>;

    /// 列出镜像（仅支持容器执行引擎）
    async fn list_images(&self) -> Result<Vec<String>>;

    /// 检查镜像是否存在（仅支持容器执行引擎）
    async fn image_exists(&self, image: &str) -> Result<bool>;
}

impl ExecutionResult {
    /// 创建成功的执行结果
    pub fn success(stdout: String, duration_ms: u64) -> Self {
        Self { exit_code: 0, stdout, stderr: String::new(), duration_ms, success: true }
    }

    /// 创建失败的执行结果
    pub fn failure(exit_code: i32, stderr: String, duration_ms: u64) -> Self {
        Self { exit_code, stdout: String::new(), stderr, duration_ms, success: false }
    }
}

impl VolumeMount {
    /// 创建读写挂载
    pub fn read_write(host_path: impl Into<String>, container_path: impl Into<String>) -> Self {
        Self {
            host_path: host_path.into(),
            container_path: container_path.into(),
            read_only: false,
        }
    }

    /// 创建只读挂载
    pub fn read_only(host_path: impl Into<String>, container_path: impl Into<String>) -> Self {
        Self { host_path: host_path.into(), container_path: container_path.into(), read_only: true }
    }
}

impl PortMapping {
    /// 创建TCP端口映射
    pub fn tcp(host_port: u16, container_port: u16) -> Self {
        Self { host_port, container_port, protocol: "tcp".to_string() }
    }

    /// 创建UDP端口映射
    pub fn udp(host_port: u16, container_port: u16) -> Self {
        Self { host_port, container_port, protocol: "udp".to_string() }
    }
}
