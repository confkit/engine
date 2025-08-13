# ConfKit CLI

ConfKit 是一个配置驱动的构建和部署工具，专为现代化 CI/CD 流水线设计。

## 📋 核心功能

- **构建器管理**: Docker 镜像与容器的完整生命周期管理
- **配置驱动**: 通过 YAML 配置文件定义构建流程
- **任务执行**: 支持本地和容器化命令执行
- **日志管理**: 完整的构建日志记录、查看和管理
- **Git 集成**: 原生支持 Git 仓库操作和环境变量注入
- **交互式界面**: 友好的命令行交互体验

## 🚀 快速开始

### 安装

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### 配置示例结构

```
examples/
├── confkit-compose.yml
├── .confkit.yml
└── .confkit/
    ├── spaces/
    │   ├── hello/
    │   │   └── hello-confkit.yml
    │   └── confkit/
    │       └── engine.yml
    └── volumes/
        ├── cache/
        ├── logs/
        └── workspace/
```

### 基础配置文件
```yml
# .confkit.yml
version: 1.0.0

# 容器引擎: docker/podman
engine: docker

# 终端类型: bash/zsh
shell:
  host: bash
  container: bash

engine_compose:
  # 容器分组(default: confkit)
  # project: confkit
  # docker compose file
  file: ./confkit-compose.yml

# 空间列表
spaces:
  - name: confkit
    description: "ConfKit 工具链发布空间"
    # 项目执行配置文件
    path: .confkit/spaces/confkit
  - name: hello
    description: "Hello ConfKit"
    path: .confkit/spaces/hello

# 镜像管理列表
images:
    # 构建目标镜像名称
  - name: hello-builder
    # 基础镜像(自动拉取)
    base_image: alpine
    # 基础镜像标签(目标镜像共用)
    tag: 3.18
    context: volumes/context
    # Dockerfile 路径
    engine_file: ./.confkit/images/Dockerfile.alpine:3.18
  - name: rust-builder
    base_image: rust
    tag: 1.88-alpine
    context: volumes/context
    engine_file: ./.confkit/images/Dockerfile.rust.1.88-alpine

```

### 基本使用

```bash
# 查看帮助
confkit --help

# 交互式模式（推荐新手使用）
confkit

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
confkit image list

# 拉取/构建镜像
confkit image create golang:1.24

# 删除镜像
confkit image remove golang:1.24
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

### 执行构建

```bash
# 构建项目
confkit exec --space <space_name> --project-name <project_name>
```

### 项目配置示例

```yaml
name: "hello-confkit"
description: "Hello Confkit"

source:
  git_repo: "https://github.com/example/hello-go.git"
  git_branch: "main"

environment_files:
  - format: "yaml"
    path: "./volumes/environment.yml"

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
    continue_on_error: true
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

### 自动环境变量注入

ConfKit 在执行任务时会自动注入以下环境变量：

#### 系统变量

- `TASK_ID` - 任务唯一标识符（如 `20250113-143022-a1b2c3`）
- `PROJECT_NAME` - 配置文件中的项目名称
- `PROJECT_VERSION` - 项目版本号, 取自远程仓库(javascript: package.json, rust: Cargo.toml)
- `SPACE_NAME` - 空间名称
- `HOST_WORKSPACE_DIR` - 主机任务工作空间目录
- `CONTAINER_WORKSPACE_DIR` - 容器任务工作空间目录

#### Git 变量

- `GIT_REPO` - 配置文件中的 Git 仓库地址
- `GIT_BRANCH` - Git 分支名（来自配置或当前分支）
- `GIT_HASH` - 完整 commit hash
- `GIT_HASH_SHORT` - 短 commit hash（前 8 个字符）

#### 自定义变量

您还可以在项目配置中定义自定义环境变量：

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_COMMIT_SHORT}"
```

所有环境变量都支持使用 `${变量名}` 语法进行变量替换。

### 智能日志匹配

支持多种日志文件匹配方式：

- 完整文件名
- 文件名片段
- 任务 ID 片段
- 时间戳匹配

### 分层构建器管理

- **镜像层**: 管理 Docker 镜像的拉取、构建和删除
- **容器层**: 基于 docker-compose.yml 创建命名构建器容器
- **生命周期**: 完整的启动、停止、健康检查流程

## 📄 许可证

MIT License
