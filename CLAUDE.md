# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作提供指引。

## 项目概述

ConfKit Engine 是一个基于 Rust 的 CLI 工具，用于配置驱动的容器构建与部署。它通过 YAML 配置文件（`.confkit.yml`）抽象 Docker/Podman 操作，支持条件步骤执行、交互式输入和环境变量注入。

二进制名称：`confkit`，crate 名称：`confkit-engine`，Rust edition 2021。

## 构建与开发命令

```bash
# 构建
cargo build                  # Debug 构建
cargo build --release        # Release 构建（启用 LTO）

# 格式化与 Lint（每次代码变更后必须执行）
cargo fmt                    # 格式化代码
cargo fmt --check            # 检查格式
cargo clippy --all-targets --all-features -- -W clippy::complexity -W clippy::correctness -A dead_code -A unused_imports -A unused_variables

# 测试
cargo test                   # 全部测试
cargo test --lib             # 仅单元测试
cargo test --test "*"        # 仅集成测试
cargo test --test condition_unit_tests -- --test-threads=1 --nocapture        # 条件模块单元测试
cargo test --test condition_expression_tests -- --test-threads=1 --nocapture  # 表达式解析测试

# cargo-make 任务（需要：cargo install cargo-make）
cargo make dev               # 完整流程：清理、格式检查、lint、测试、构建
cargo make watch             # 监听模式，自动重新构建
cargo make test-condition-all # 所有条件模块测试
```

## 架构

代码库采用分层架构，职责划分清晰：

- **`cli/`** - 命令路由层。子命令：`builder/`、`image/`、`run/`、`log/`、`clean/`、`interactive/`。通过 `clap` derive 宏解析参数。
- **`core/`** - 业务逻辑层。`executor/` 在容器中执行构建步骤，`condition/` 求值条件表达式（使用 `nom` 解析），`interactive/` 通过 `inquire` 处理用户交互。
- **`engine/`** - 容器引擎抽象层。`docker.rs` 和 `podman.rs` 实现相同 trait。活跃引擎通过全局单例设置（`shared/global.rs`）。
- **`types/`** - 数据结构定义。`config.rs` 为 YAML 配置模型，`condition.rs` 为表达式 AST，`project.rs` 为项目配置，`interactive.rs` 为交互输入类型。
- **`infra/`** - 基础设施层。`config.rs` 加载 `.confkit.yml`，`git.rs` 处理仓库操作，`event_hub/` 提供发布/订阅事件系统及 `LogSubscriber`。
- **`formatter/`** - 输出格式化，涵盖镜像、容器、日志和路径。
- **`shared/`** - `constants.rs` 定义目录路径（宿主机/容器）和配置文件名。`global.rs` 持有 ENGINE 单例。
- **`utils/`** - 工具函数。

### 核心流程

1. **启动流程**（`main.rs`）：解析 CLI -> 初始化 tracing -> 检查 `.confkit.yml` 是否存在 -> 初始化 EventHub -> 创建目录 -> 加载配置 -> 设置引擎 -> 执行命令 -> 优雅关闭（信号或完成时清理 EventHub）。
2. **任务执行**：从 space 路径加载项目 YAML -> 解析环境变量（静态 + git + 交互式）-> 遍历步骤 -> 求值条件表达式 -> 通过引擎在容器中执行命令。
3. **条件求值**：表达式字符串 -> `nom` 解析器 -> AST（`types/condition.rs`）-> 带环境变量替换的求值。支持 `==`、`!=`、`>`、`<`、`>=`、`<=`、`&&`、`||`、`!`。

### 宿主机目录布局（运行时）

`volumes/workspace/`、`volumes/artifacts/`、`volumes/logs/`、`volumes/cache/`、`volumes/temp/` - 启动时创建，挂载到容器的 `/workspace` 和 `/artifacts`。

## 代码风格

- 最大行宽：100 字符（`rustfmt.toml`）
- Clippy：对 `complexity` 和 `correctness` 告警，允许 `dead_code`、`unused_imports`、`unused_variables`
- 异步优先，使用 `tokio` full features
- 错误处理：应用错误使用 `anyhow::Result`，库错误类型使用 `thiserror`
