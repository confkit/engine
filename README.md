# confkit CLI

confkit是一个配置驱动的构建和部署工具，专为现代化CI/CD流水线设计。

## 📋 功能特性

- **配置驱动**: 通过YAML配置文件定义构建流程
- **容器化构建**: 支持Docker容器作为构建环境
- **并行执行**: 智能任务依赖解析和并行执行
- **Git集成**: 原生支持Git仓库操作
- **实时日志**: 完整的构建日志记录和查看
- **任务管理**: 强大的任务生命周期管理
- **交互式界面**: 友好的命令行交互体验
- **智能多选**: 支持级联选择和批量操作的多选界面

## 🎯 最新功能 - 交互式镜像删除多选

### ✨ 功能亮点

1. **状态指示清晰**：
   - `✓ 已构建` - 显示镜像大小和创建时间
   - `未构建` - 清晰标识未构建的镜像，自动过滤

2. **智能多选界面**：
   - 支持批量选择多个镜像标签
   - 使用 `● 选择全部 [仓库名] 镜像` 实现级联选择
   - 25行分页显示，支持大量镜像管理

3. **用户友好设计**：
   - 按仓库分组显示镜像
   - 显示每个仓库的可删除镜像数量
   - 清晰的操作指引和反馈

### 🚀 使用方法

```bash
# 启动交互式模式
confkit --interactive

# 导航路径：
# 主菜单 → [BUILDER] Builder 管理 → [IMAGE] 镜像管理 → [REMOVE] 删除镜像
```

### 📱 界面示例

```
请选择要删除的镜像:
● 选择全部 golang 镜像 (3 个)
  [1.24] - ✓ 已构建 - 245MB (2024-01-15 14:30:22)
  [1.23] - ✓ 已构建 - 238MB (2024-01-14 10:15:30)
  [latest] - ✓ 已构建 - 245MB (2024-01-15 14:30:22)

● 选择全部 node 镜像 (2 个)
  [22] - ✓ 已构建 - 156MB (2024-01-13 16:45:10)
  [20] - ✓ 已构建 - 148MB (2024-01-12 09:20:15)

[返回] 返回镜像管理菜单
```

### 🔧 操作指南

- **级联选择**: 选择 `● 选择全部 [仓库名] 镜像` 可一次性选择该仓库的所有已构建镜像
- **单项选择**: 直接选择具体的镜像标签进行精确删除
- **智能过滤**: 自动过滤未构建的镜像，只显示可删除的镜像
- **批量操作**: 支持同时选择多个仓库和单个镜像的混合操作

## 🚀 快速开始

### 安装

```bash
# 从源码编译
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### 基本使用

```bash
# 查看帮助
confkit --help

# 运行构建任务
confkit run examples/simple-go-project.yml

# 查看任务列表
confkit task list

# 查看构建日志
confkit logs <project-name>

# 管理构建器
confkit builder list
```

## 📂 项目结构

```
src/
├── main.rs              # 程序入口
├── cli/                 # CLI命令行界面
│   ├── mod.rs
│   ├── run.rs           # run子命令
│   ├── builder.rs       # builder子命令
│   ├── task.rs          # task子命令
│   ├── logs.rs          # logs子命令
│   └── interactive.rs   # interactive子命令
├── core/                # 核心业务逻辑
│   ├── mod.rs
│   ├── config/          # 配置解析
│   ├── task/            # 任务管理
│   ├── step/            # 步骤执行
│   ├── builder/         # 构建器管理
│   └── git/             # Git操作
├── infrastructure/      # 基础设施层
│   ├── mod.rs
│   ├── docker.rs        # Docker客户端
│   ├── logging.rs       # 日志系统
│   ├── storage.rs       # 存储管理
│   └── network.rs       # 网络工具
└── utils/               # 工具函数
    ├── mod.rs
    ├── error.rs         # 错误处理
    └── validation.rs    # 验证工具
```

## 📝 配置示例

### 简单的Go项目构建

```yaml
# simple-go-project.yml
project:
  name: "simple-go-app"
  type: "golang"
  description: "简单的Go应用构建示例"

source:
  git_repo: "https://github.com/example/simple-go-app.git"
  git_branch: "main"

environment:
  CGO_ENABLED: "0"
  GOOS: "linux"
  GOARCH: "amd64"

steps:
  - name: "代码拉取"
    working_dir: "./volumes/workspace"
    commands:
      - "git clone ${GIT_REPO} ${PROJECT_NAME}"
    timeout: "5m"

  - name: "构建应用"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "go build -o app ./cmd/main.go"
    depends_on: ["代码拉取"]
```

### Node.js Web应用构建

```yaml
# node-web-app.yml
project:
  name: "node-web-app"
  type: "node"

source:
  git_repo: "https://github.com/example/node-web-app.git"
  git_branch: "main"

steps:
  - name: "安装依赖"
    container: "node-builder-22"
    commands:
      - "npm ci"

  - name: "运行测试"
    container: "node-builder-22"
    commands:
      - "npm run test"
    depends_on: ["安装依赖"]

  - name: "构建应用"
    container: "node-builder-22"
    commands:
      - "npm run build"
    depends_on: ["运行测试"]
```

## 🛠 命令行界面

### 运行构建任务

```bash
confkit run [OPTIONS] <PROJECT_CONFIG>

选项:
  -b, --git-branch <BRANCH>    Git分支名称
      --parallel               并行执行
      --force                  强制重新构建
      --priority <PRIORITY>    任务优先级
      --timeout <TIMEOUT>      超时时间
```

### 管理构建器

```bash
confkit builder <COMMAND>

子命令:
  list        列出所有构建器
  create      创建新的构建器
  start       启动构建器
  stop        停止构建器
  remove      删除构建器
  health      健康检查
```

### 任务管理

```bash
confkit task <COMMAND>

子命令:
  list        列出所有任务
  show        查看任务详情
  kill        终止任务
  restart     重启任务
  clean       清理已完成的任务
```

### 日志查看

```bash
confkit logs [OPTIONS] <PROJECT_OR_TASK>

选项:
  --task-id <TASK_ID>     任务ID
  -f, --follow            跟踪日志输出
  -n, --lines <LINES>     显示的行数
  --timestamps            显示时间戳
  --step <STEP>           过滤步骤
  --level <LEVEL>         日志级别过滤
```

## 🏗 开发状态

这是项目的初始结构搭建版本，包含了完整的架构设计和接口定义。当前所有功能都是框架代码，具体实现将在后续版本中完成。

### 已完成

- ✅ 项目结构搭建
- ✅ CLI命令行界面设计
- ✅ 核心模块架构
- ✅ 配置文件结构定义
- ✅ 错误处理和验证工具
- ✅ 基础设施层抽象

### 待实现

- 🚧 配置文件解析实现
- 🚧 Docker容器管理
- 🚧 Git操作实现
- 🚧 任务调度引擎
- 🚧 步骤执行引擎
- 🚧 日志系统实现
- 🚧 存储管理实现
- 🚧 网络工具实现

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交Issue和Pull Request！ 