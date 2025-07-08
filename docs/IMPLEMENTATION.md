 # confkit engine CLI 技术实现文档

本文档介绍了confkit CLI的技术架构、项目结构和开发指南。

## 🏗️ 技术架构

### 核心技术栈

- **语言**: Rust 2021 Edition
- **异步运行时**: Tokio
- **命令行**: Clap 4.x
- **配置解析**: Serde + YAML/JSON
- **容器管理**: Bollard (Docker API)
- **Git操作**: Git2
- **日志系统**: Tracing + Tracing-subscriber

### 架构设计

```
┌─────────────────┐
│   CLI Layer     │  命令行接口层
├─────────────────┤
│  Core Engine    │  核心引擎层
├─────────────────┤
│ Infrastructure  │  基础设施层
└─────────────────┘
```

## 📁 项目结构

```
engine/
├── Cargo.toml                      # 项目配置
├── src/
│   ├── main.rs                     # 程序入口
│   ├── cli/                        # 命令行接口层
│   │   ├── mod.rs
│   │   ├── run.rs                  # run 子命令
│   │   ├── builder.rs              # builder 子命令
│   │   ├── task.rs                 # task 子命令
│   │   ├── logs.rs                 # logs 子命令
│   │   └── interactive.rs          # 交互式命令
│   ├── core/                       # 核心业务逻辑层
│   │   ├── mod.rs
│   │   ├── config/                 # 配置管理
│   │   ├── task/                   # 任务管理
│   │   ├── step/                   # 步骤执行
│   │   ├── builder/                # 构建器管理
│   │   └── git/                    # Git集成
│   ├── infrastructure/             # 基础设施层
│   │   ├── mod.rs
│   │   ├── docker.rs               # Docker客户端
│   │   ├── logging.rs              # 日志系统
│   │   ├── storage.rs              # 存储管理
│   │   └── network.rs              # 网络工具
│   └── utils/                      # 工具函数
│       ├── mod.rs
│       ├── error.rs                # 错误处理
│       └── validation.rs           # 验证工具
├── tests/                          # 测试
├── docs/                           # 文档
└── examples/                       # 示例配置
```

## 🔧 核心模块

### 1. 任务管理器 (TaskManager)

```rust
pub struct TaskManager {
    running_tasks: Arc<Mutex<HashMap<TaskId, TaskHandle>>>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    max_concurrent: usize,
}

impl TaskManager {
    pub async fn execute_task(&self, config: ProjectConfig) -> Result<TaskResult>;
    pub async fn kill_task(&self, task_id: &TaskId) -> Result<()>;
    pub fn list_tasks(&self) -> Vec<TaskInfo>;
}
```

### 2. 步骤执行引擎 (StepEngine)

```rust
pub struct StepEngine {
    docker_client: DockerClient,
    environment: EnvironmentManager,
}

impl StepEngine {
    pub async fn execute_step(&self, step: &Step, context: &TaskContext) -> Result<StepResult>;
    pub fn build_dependency_graph(&self, steps: &[Step]) -> DependencyGraph;
    pub async fn execute_parallel_group(&self, steps: &[Step]) -> Result<Vec<StepResult>>;
}
```

### 3. 构建器管理器 (BuilderManager)

```rust
pub struct BuilderManager {
    docker_client: DockerClient,
    builders: HashMap<String, Builder>,
}

impl BuilderManager {
    pub async fn create_builder(&self, name: &str, config: &BuilderConfig) -> Result<()>;
    pub async fn start_builder(&self, name: &str) -> Result<()>;
    pub async fn health_check(&self, name: &str) -> Result<HealthStatus>;
}
```

## 📦 依赖配置

```toml
[package]
name = "confkit-cli"
version = "1.0.0"
edition = "2021"

[dependencies]
# 核心依赖
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
anyhow = "1.0"
tracing = "0.1"

# Docker和Git
bollard = "0.15"
git2 = "0.18"

# 工具库
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
walkdir = "2.4"
```

## 🚀 开发指南

### 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone https://github.com/company/confkit-cli.git
cd confkit-cli

# 开发构建
cargo build

# 运行测试
cargo test

# 发布构建
cargo build --release
```

### 代码规范

- 使用 `rustfmt` 进行代码格式化
- 使用 `clippy` 进行代码检查
- 遵循 Rust 官方命名规范
- 使用 `tracing` 进行日志记录

### 测试策略

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_task_execution() {
        // 单元测试
    }
}

// 集成测试 - tests/integration/
#[tokio::test]
async fn test_full_confkit() {
    // 端到端测试
}
```

## 📊 性能优化

- 使用异步I/O处理容器操作
- 实现步骤并行执行
- 缓存构建依赖
- 优化日志写入性能
- 实现容器复用机制

## 🔒 安全设计

- 输入验证和清理
- 命令注入防护
- 容器安全配置
- 敏感信息脱敏
- 访问权限控制

这个技术实现文档为开发团队提供了清晰的架构指导和开发规范。