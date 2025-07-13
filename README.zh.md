# ConfKit CLI

ConfKit 是一个配置驱动的构建和部署工具，专为现代化CI/CD流水线设计。

## 📋 核心功能

- **构建器管理**: Docker镜像与容器的完整生命周期管理
- **配置驱动**: 通过YAML配置文件定义构建流程
- **任务执行**: 支持本地和容器化命令执行
- **日志管理**: 完整的构建日志记录、查看和管理
- **Git集成**: 原生支持Git仓库操作和环境变量注入
- **交互式界面**: 友好的命令行交互体验

## 🚀 快速开始

### 安装

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### 基本使用

```bash
# 查看帮助
confkit --help

# 交互式模式（推荐新手使用）
confkit interactive

# 管理构建器
confkit builder list
confkit builder create golang-builder
confkit builder start golang-builder

# 运行构建任务
confkit run --space hello --project hello-app

# 查看日志
confkit log list --space hello --project hello-app
confkit log show --space hello --project hello-app <filename>
```

## 🏗 Builder 管理

### 镜像管理
```bash
# 列出镜像
confkit builder image list

# 拉取/构建镜像
confkit builder image create golang:1.24

# 删除镜像
confkit builder image remove golang:1.24
```

### 容器管理
```bash
# 列出所有构建器状态
confkit builder list

# 创建构建器（基于docker-compose.yml）
confkit builder create golang-builder

# 启动/停止构建器
confkit builder start golang-builder
confkit builder stop golang-builder

# 删除构建器
confkit builder remove golang-builder

# 健康检查
confkit builder health golang-builder
```

## 📝 配置文件示例

在 `examples/` 目录下有完整的配置示例：

```bash
examples/
├── builder.yml           # 构建器配置
├── docker-compose.yml    # 容器服务定义
└── .confkit/
    └── spaces/
        └── hello/
            ├── config.yml          # 空间配置
            └── projects/
                └── hello-app.yml   # 项目配置
```

### 项目配置示例

```yaml
# examples/.confkit/spaces/hello/projects/hello-app.yml
name: "hello-app"
type: "golang"
description: "Hello World Go应用"

source:
  git_repo: "https://github.com/example/hello-go.git"
  git_branch: "main"

environment:
  APP_NAME: "hello-app"
  BUILD_VERSION: "1.0.0"

steps:
  - name: "构建应用"
    container: "golang-builder"
    working_dir: "/workspace"
    commands:
      - "echo 'Building ${APP_NAME} v${BUILD_VERSION}'"
      - "echo 'Git Hash: ${GIT_HASH}'"
      - "go build -o app ./main.go"
    timeout: "5m"
```

## 📋 日志管理

```bash
# 列出日志文件
confkit log list --space hello --project hello-app

# 查看具体日志
confkit log show --space hello --project hello-app abc123

# 支持多种匹配方式
confkit log show --space hello --project hello-app "2025-01-13_12-00-00"
confkit log show --space hello --project hello-app complete-filename.txt
```

## 🖥 交互式模式

启动交互式模式获得最佳用户体验：

```bash
confkit interactive
```

**导航路径**：
- `[BUILDER] Builder 管理` → 镜像和容器管理
- `[RUN] Run 管理` → 执行项目构建任务  
- `[LOG] Log 管理` → 查看项目日志

## 🎯 特色功能

### Git环境变量自动注入

执行任务时自动注入Git信息到环境变量：
- `GIT_HASH` - 完整commit hash
- `GIT_COMMIT_HASH` - 完整commit hash（别名）
- `GIT_COMMIT_SHORT` - 短commit hash
- `GIT_TAG` - 当前tag（如果有）

### 智能日志匹配

支持多种日志文件匹配方式：
- 完整文件名
- 文件名片段
- 任务ID片段
- 时间戳匹配

### 分层构建器管理

- **镜像层**: 管理Docker镜像的拉取、构建和删除
- **容器层**: 基于docker-compose.yml创建命名构建器容器
- **生命周期**: 完整的启动、停止、健康检查流程

## 📂 项目结构

```
examples/                # 示例配置
├── builder.yml         # 构建器配置
├── docker-compose.yml  # 容器服务定义
└── .confkit/           # ConfKit工作空间
    └── spaces/         # 空间管理
        └── hello/      # 示例空间
volumes/                # 运行时数据
├── logs/              # 任务日志
├── workspace/         # 构建工作空间  
└── artifacts/         # 构建产物
```

## 🛠 开发状态

### ✅ 已完成
- Builder管理（镜像+容器）
- 配置文件解析和验证
- 任务执行引擎（基础）
- 日志系统（完整）
- Git集成和环境变量注入
- 交互式界面（Builder + Log）

### 🚧 开发中
- Task管理命令
- 高级并行执行
- 通知系统

## 📄 许可证

MIT License 