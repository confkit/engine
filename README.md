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