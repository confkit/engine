# ConfKit CLI

ConfKit 是一个配置驱动的构建和部署工具，专为现代化 CI/CD 流水线设计。

## 📋 核心功能

- **构建器管理**: Docker 镜像与容器的完整生命周期管理
- **配置驱动**: 通过 YAML 配置文件定义构建流程
- **条件执行**: 基于环境变量和运行时条件的智能步骤执行
- **任务执行**: 支持本地和容器化命令执行
- **日志管理**: 完整的构建日志记录、查看和管理
- **Git 集成**: 原生支持 Git 仓库操作和环境变量注入
- **交互式界面**: 友好的命令行交互体验

## 🚀 快速开始

### 安装

#### 快速安装（推荐）

**安装最新版本：**

```bash
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh
```

**安装指定版本：**

```bash
# 方法1：使用命令行参数（推荐）
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh -s -- 1.2.3

# 方法2：使用 bash 进程替换
bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh) 1.2.3

# 方法3：使用环境变量和 bash
CONFKIT_VERSION=1.2.3 bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh)
```

这将自动：
- 检测您的平台和架构
- 从 GitHub 发布页面下载对应的二进制文件
- 安装到系统二进制目录（macOS 为 `/usr/local/bin`，Linux 为 `/usr/local/bin` 或 `~/.local/bin`）
- 自动将二进制文件添加到 PATH

**版本格式支持：**
- `latest` - 安装最新发布版本（默认）
- `1.2.3` - 自动转换为 `v1.2.3`
- `v1.2.3` - 使用精确版本标签

#### 支持的平台

- **Linux**: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin`

#### 手动安装

如果您倾向于从源码构建：

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

#### 验证安装

```bash
confkit --help
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

# 通过命令行参数注入环境变量
confkit run --space hello --project hello-app -e KEY1=value1 -e KEY2=value2

# 查看日志
confkit log list --space hello --project hello-app
confkit log show --space hello --project hello-app --task <task_id>
confkit log info --space hello --project hello-app --task <task_id>
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
    # 条件执行 - 仅在生产环境下运行
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "echo 'Building ${APP_NAME} v${BUILD_VERSION}'"
      - "echo 'Git Hash: ${GIT_HASH}'"
      - "go build -o app ./main.go"
    # 5m = 300s
    timeout: 300
    continue_on_error: true
    
  - name: "运行测试"
    container: "golang-builder"
    working_dir: "/workspace"
    # 条件执行 - 在生产环境下跳过测试
    condition: "${ENVIRONMENT} != 'production'"
    commands:
      - "go test ./..."
    timeout: 180
```

## 📋 日志管理

日志按层级目录结构存储：

```
volumes/logs/<space>/<project>/<date>/<time>-<task_id>/
  ├── task.meta.json   # 任务元数据（状态、耗时、步骤结果）
  └── task.log         # 完整任务日志输出
```

```bash
# 列出项目的任务日志
confkit log list --space hello --project hello-app

# 查看任务日志内容
confkit log show --space hello --project hello-app --task <task_id>

# 查看任务元数据（状态、耗时、各步骤详情）
confkit log info --space hello --project hello-app --task <task_id>
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
- `HOST_ARTIFACTS_ROOT_DIR` - 主机任务产物根目录
- `CONTAINER_ARTIFACTS_ROOT_DIR` - 容器任务产物根目录


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

#### 命令行参数注入

通过 `-e` / `--environments` 参数在运行时注入环境变量，无需修改配置文件：

```bash
confkit run --space hello --project hello-app \
  -e ENVIRONMENT=production \
  -e BUILD_VERSION=2.0.0 \
  -e API_URL=https://api.example.com
```

通过命令行注入的环境变量会与配置文件中的环境变量合并，命令行参数的优先级更高（会覆盖同名变量）。变量格式为 `KEY=VALUE`，格式不正确的条目会被跳过并输出警告。

#### 交互式环境变量

ConfKit 支持在任务执行过程中交互式输入环境变量。在 `environment_from_args` 部分定义：

```yaml
environment_from_args:
  # input 类型 - 自由文本输入
  - name: "API_URL"
    type: "input"
    prompt: "请输入API地址"
    default: "https://api.example.com"
    required: true
    
  # radio 类型 - 单选
  - name: "ENVIRONMENT"
    type: "radio"
    prompt: "选择部署环境"
    default: "staging"
    required: true
    options:
      - "development"
      - "staging"
      - "production"
      
  # checkbox 类型 - 多选
  - name: "FEATURES"
    type: "checkbox"
    prompt: "选择要启用的功能"
    default: "auth"
    required: false
    options:
      - "auth"
      - "logging"
      - "metrics"
      
  # confirm 类型 - 是/否确认
  - name: "ENABLE_DEBUG"
    type: "confirm"
    prompt: "是否启用调试模式？"
    default: "false"
    required: false
```

**支持的交互类型：**
- `input`：自由文本输入
- `radio`：从选项中单选
- `checkbox`：从选项中多选
- `confirm`：是/否确认（返回 "true" 或 "false"）

**配置选项：**
- `name`：环境变量名称
- `type`：交互类型（input/radio/checkbox/confirm）
- `prompt`：用户提示文本
- `default`：默认值（可选）
- `required`：是否必填（默认：true）
- `options`：radio/checkbox 类型的可选项

所有环境变量都支持使用 `${变量名}` 语法进行变量替换。

### 条件步骤执行

ConfKit 支持基于环境变量和运行时条件的条件步骤执行。在任何步骤中添加 `condition` 字段来控制其执行条件。

#### 基础条件语法

```yaml
steps:
  - name: "生产环境构建"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "npm run build:prod"
      
  - name: "开发环境构建"  
    condition: "${ENVIRONMENT} == 'development'"
    commands:
      - "npm run build:dev"
```

#### 支持的运算符

**比较运算符：**
- `==` - 等于
- `!=` - 不等于
- `>` - 大于
- `<` - 小于
- `>=` - 大于等于
- `<=` - 小于等于

**逻辑运算符：**
- `&&` - 逻辑与
- `||` - 逻辑或
- `!` - 逻辑非

#### 高级条件示例

```yaml
steps:
  # 多条件逻辑组合
  - name: "部署到测试环境"
    condition: "${ENVIRONMENT} == 'staging' && ${GIT_BRANCH} == 'main'"
    commands:
      - "deploy.sh staging"
      
  # 数值比较
  - name: "性能测试"
    condition: "${BUILD_NUMBER} > 100"
    commands:
      - "npm run test:performance"
      
  # 复杂嵌套条件
  - name: "质量门禁"
    condition: "(${ENVIRONMENT} == 'production' || ${ENVIRONMENT} == 'staging') && !${SKIP_TESTS}"
    commands:
      - "npm run test:quality"
      
  # 布尔变量
  - name: "调试模式"
    condition: "${ENABLE_DEBUG} == true"
    commands:
      - "echo '调试模式已启用'"
```

#### 降级策略

当条件表达式无法解析或求值时：
- **默认行为**: 跳过步骤执行（安全降级）
- **可配置**: 可配置为无条件执行或使用自定义降级逻辑

#### 性能优化

- 表达式解析一次后缓存供重复使用
- 任务执行期间缓存环境变量值
- 简单表达式求值 < 10ms，复杂表达式 < 50ms

### 结构化任务日志

每个任务产生结构化输出：

- **`task.meta.json`**：实时元数据，包含任务状态、开始/结束时间、总耗时、以及各步骤的执行结果（状态、退出码、错误信息）
- **`task.log`**：带时间戳的完整日志输出
- 元数据在每个步骤执行完毕后实时更新，即使任务中途崩溃也可查看执行进度

### 分层构建器管理

- **镜像层**: 管理 Docker 镜像的拉取、构建和删除
- **容器层**: 基于 docker-compose.yml 创建命名构建器容器
- **生命周期**: 完整的启动、停止、健康检查流程

## 📄 许可证

MIT License
