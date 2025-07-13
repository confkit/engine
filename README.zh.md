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

### 自动环境变量注入

ConfKit 在执行任务时会自动注入以下环境变量：

#### 系统变量
- `TASK_ID` - 任务唯一标识符（如 `api-20250113-143022-a1b2c3`）
- `PROJECT_NAME` - 配置文件中的项目名称
- `SPACE_NAME` - 空间名称

#### Git 变量
- `GIT_REPO` - 配置文件中的 Git 仓库地址
- `GIT_BRANCH` - Git 分支名（来自配置或当前分支）
- `GIT_HASH` - 完整commit hash
- `GIT_COMMIT_HASH` - 完整commit hash（别名）
- `GIT_COMMIT_SHORT` - 短commit hash（前8个字符）
- `GIT_TAG` - 当前tag（如果有）

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
├── release.sh          # 自发布脚本
├── release-docker-compose.yml  # 发布环境
├── RELEASE_README.md   # 发布流程文档
└── .confkit/           # ConfKit工作空间
    └── spaces/         # 空间管理
        ├── hello/      # 示例空间
        └── release/    # 发布空间（自发布）
volumes/                # 运行时数据
├── logs/              # 任务日志
├── workspace/         # 构建工作空间  
└── artifacts/         # 构建产物
```

## 🔄 ConfKit 自发布

ConfKit 可以使用自己的构建系统来发布自己！这展示了配置驱动构建的强大能力：

```bash
# 进入 examples 目录
cd examples

# 设置必要的环境变量
export CARGO_REGISTRY_TOKEN="your-crates-token"
export DOCKER_USERNAME="your-docker-username"
export DOCKER_PASSWORD="your-docker-password"
export GITHUB_TOKEN="your-github-token"

# 发布版本 1.0.0
./release.sh 1.0.0

# 或者测试发布流程
./release.sh 1.0.0 --dry-run
```

有关自发布流程的详细信息，请参见 [examples/RELEASE_README.md](examples/RELEASE_README.md)。

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