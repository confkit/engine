# Executor 架构设计文档

## 概述

`executor` 模块是 confkit 的核心抽象层，负责统一管理不同的执行引擎。该模块将执行逻辑从具体的实现（如 Docker）中抽象出来，为未来扩展其他执行引擎（如 Kubernetes、Podman）提供了良好的基础。

## 设计目标

1. **抽象化执行**: 提供统一的执行接口，隐藏底层实现细节
2. **命令生成**: 根据参数生成对应的执行命令，便于调试和验证
3. **扩展性**: 支持多种执行引擎，便于未来扩展
4. **类型安全**: 使用 Rust 的类型系统确保参数正确性

## 模块结构

```
src/core/executor/
├── mod.rs          # 模块导出
├── types.rs        # 核心类型定义和抽象接口
├── docker.rs       # Docker 执行引擎实现
└── examples.rs     # 使用示例和测试
```

## 核心组件

### 1. 执行引擎类型 (ExecutorType)

```rust
pub enum ExecutorType {
    Docker,        // Docker 容器执行
    Local,         // 本地主机执行
    Kubernetes,    // Kubernetes 执行（未来扩展）
    Podman,        // Podman 执行（未来扩展）
}
```

### 2. 执行上下文 (ExecutionContext)

定义执行环境的配置参数：

- **工作目录**: 命令执行的工作路径
- **环境变量**: 传递给执行环境的变量
- **卷挂载**: 主机与容器之间的文件共享
- **端口映射**: 网络端口配置
- **用户权限**: 执行用户设置

### 3. 执行引擎接口 (Executor trait)

统一的执行引擎抽象接口：

```rust
#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    fn executor_type(&self) -> ExecutorType;
    async fn is_available(&self) -> Result<bool>;
    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult>;
    async fn build_image(&self, params: &ImageBuildParams) -> Result<String>;
    async fn pull_image(&self, image: &str) -> Result<()>;
    async fn remove_image(&self, params: &ImageOperationParams) -> Result<()>;
    async fn list_images(&self) -> Result<Vec<String>>;
    async fn image_exists(&self, image: &str) -> Result<bool>;
}
```

## Docker 执行引擎实现

### DockerExecutor

实现了 `Executor` trait，提供完整的 Docker 操作支持：

- **命令执行**: 通过 `docker run` 执行容器化命令
- **镜像管理**: 构建、拉取、删除、列出镜像
- **状态检查**: 检查 Docker 可用性和镜像存在性

### DockerCommandBuilder

专门用于生成 Docker 命令字符串的工具类：

```rust
let builder = DockerCommandBuilder::new();

// 生成 docker run 命令
let cmd = builder.run_command(&execution)?;
// 输出: "docker run --rm -i -w /workspace golang:1.21 sh -c 'go version'"

// 生成 docker build 命令  
let cmd = builder.build_command(&build_params)?;
// 输出: "docker build -t my-app:latest -f Dockerfile --build-arg VERSION=1.0.0 ."
```

## 使用示例

### 1. 基本命令执行

```rust
use confkit::core::executor::*;

let executor = DockerExecutor::new();

let mut execution = CommandExecution::default();
execution.image = Some("golang:1.21".to_string());
execution.commands = vec!["go version".to_string()];
execution.context.working_dir = Some("/workspace".to_string());

let result = executor.execute_command(&execution).await?;
println!("输出: {}", result.stdout);
```

### 2. 复杂构建流水线

```rust
// 代码检查
let mut lint_execution = CommandExecution::default();
lint_execution.image = Some("golangci/golangci-lint:latest".to_string());
lint_execution.commands = vec!["golangci-lint run --timeout=10m".to_string()];
lint_execution.context.volumes.push(VolumeMount::read_only("./", "/workspace"));

// 单元测试
let mut test_execution = CommandExecution::default();
test_execution.image = Some("golang:1.21".to_string());
test_execution.commands = vec![
    "go mod download".to_string(),
    "go test -v -race ./...".to_string(),
];
test_execution.context.environment.insert("CGO_ENABLED".to_string(), "1".to_string());

// 构建镜像
let build_params = ImageBuildParams {
    tag: "my-app:v1.0.0".to_string(),
    dockerfile: "Dockerfile".to_string(),
    context: ".".to_string(),
    build_args: {
        let mut args = HashMap::new();
        args.insert("VERSION".to_string(), "v1.0.0".to_string());
        args
    },
    platform: Some("linux/amd64".to_string()),
    no_cache: false,
};
```

### 3. 命令生成模式

```rust
let builder = DockerCommandBuilder::new();

// 只生成命令字符串，不执行
let run_cmd = builder.run_command(&execution)?;
let build_cmd = builder.build_command(&build_params)?;
let pull_cmd = builder.pull_command("nginx:alpine");

println!("Run 命令: {}", run_cmd);
println!("Build 命令: {}", build_cmd);
println!("Pull 命令: {}", pull_cmd);
```

## 架构优势

### 1. 关注点分离

- **抽象层**: 统一的执行接口，隐藏实现细节
- **实现层**: 具体的执行引擎实现（Docker、Local 等）
- **应用层**: 业务逻辑只需要关心执行参数，不关心具体实现

### 2. 易于扩展

添加新的执行引擎只需要：

1. 在 `ExecutorType` 中添加新类型
2. 实现 `Executor` trait
3. 注册到执行引擎管理器

### 3. 命令透明化

通过 `CommandBuilder` 系列工具，可以：

- **调试友好**: 查看实际执行的命令
- **脚本生成**: 将命令导出为脚本文件
- **验证正确性**: 在执行前检查命令参数

### 4. 类型安全

使用 Rust 的类型系统确保：

- **参数正确性**: 编译时检查参数类型
- **生命周期安全**: 避免悬垂指针等内存安全问题
- **并发安全**: 通过 `Send + Sync` 确保线程安全

## 未来扩展

### 1. 本地执行引擎 (LocalExecutor)

```rust
pub struct LocalExecutor {
    shell: String, // bash, zsh, powershell
}

impl Executor for LocalExecutor {
    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult> {
        // 直接在本地执行命令，不使用容器
    }
}
```

### 2. Kubernetes 执行引擎 (K8sExecutor)

```rust
pub struct K8sExecutor {
    client: kube::Client,
    namespace: String,
}

impl Executor for K8sExecutor {
    async fn execute_command(&self, execution: &CommandExecution) -> Result<ExecutionResult> {
        // 在 Kubernetes Pod 中执行命令
    }
}
```

### 3. 执行引擎管理器

```rust
pub struct ExecutorManager {
    executors: HashMap<ExecutorType, Box<dyn Executor>>,
}

impl ExecutorManager {
    pub fn register_executor(&mut self, executor: Box<dyn Executor>) {
        self.executors.insert(executor.executor_type(), executor);
    }
    
    pub async fn execute(&self, executor_type: ExecutorType, execution: &CommandExecution) -> Result<ExecutionResult> {
        let executor = self.executors.get(&executor_type)?;
        executor.execute_command(execution).await
    }
}
```

## 与现有代码的集成

### 1. 替换直接的 Docker 调用

**之前**:
```rust
let mut cmd = Command::new("docker");
cmd.arg("run").arg("--rm").arg("-i");
cmd.arg("-w").arg("/workspace");
cmd.arg("golang:1.21");
cmd.arg("sh").arg("-c").arg("go version");
```

**现在**:
```rust
let executor = DockerExecutor::new();
let mut execution = CommandExecution::default();
execution.image = Some("golang:1.21".to_string());
execution.commands = vec!["go version".to_string()];
execution.context.working_dir = Some("/workspace".to_string());

let result = executor.execute_command(&execution).await?;
```

### 2. 统一错误处理

所有执行引擎返回统一的 `ExecutionResult`，包含：

- **退出码**: 命令执行的退出状态
- **标准输出**: 命令的正常输出
- **标准错误**: 命令的错误输出
- **执行时间**: 命令执行耗时
- **成功状态**: 是否执行成功

## 测试策略

### 1. 单元测试

- 测试命令生成的正确性
- 测试参数解析和验证
- 测试类型转换和默认值

### 2. 集成测试

- 测试与真实 Docker 环境的交互
- 测试复杂的构建流水线
- 测试错误处理和恢复

### 3. 性能测试

- 测试大量并发执行的性能
- 测试长时间运行任务的稳定性
- 测试资源使用情况

## 总结

新的 `executor` 模块提供了一个清晰、可扩展的执行引擎抽象层。它不仅解决了当前直接调用 Docker 命令的问题，还为未来支持多种执行环境奠定了基础。通过统一的接口和类型安全的设计，大大提高了代码的可维护性和可测试性。 