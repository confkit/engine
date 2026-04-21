# 配置文件参考

ConfKit 使用 YAML 配置文件来定义构建流程。配置分为两个层级：**主配置**（`.confkit.yml`）和**项目配置**（每个 space 下的 YAML 文件）。

## 主配置文件（`.confkit.yml`）

主配置文件位于项目根目录。

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

### 字段详解

#### `version`

- **类型**: String
- **必填**: 是
- **说明**: 配置文件版本号，当前为 `1.0.0`。

#### `engine`

- **类型**: String（`docker` | `podman`）
- **必填**: 是
- **说明**: 使用的容器引擎。

#### `shell`

- **类型**: Object
- **必填**: 否
- **说明**: 命令执行时使用的 Shell 配置。

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `host` | String | `bash` | 宿主机 Shell 类型 |
| `container` | String | `bash` | 容器内 Shell 类型 |

#### `engine_compose`

- **类型**: Object
- **必填**: 是
- **说明**: 构建器容器的 Docker Compose 配置。

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `project` | String | `confkit` | 容器分组名称 |
| `file` | String | — | docker-compose.yml 文件路径 |

#### `spaces`

- **类型**: Object 数组
- **必填**: 是
- **说明**: 工作空间定义列表。

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | String | 空间标识名 |
| `description` | String | 空间描述 |
| `path` | String | 包含项目 YAML 文件的目录路径 |

#### `images`

- **类型**: Object 数组
- **必填**: 否
- **说明**: 需要管理的 Docker 镜像列表。

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | String | 目标镜像名称 |
| `base_image` | String | 基础镜像（自动拉取） |
| `tag` | String | 镜像标签（基础镜像与目标镜像共用） |
| `context` | String | 构建上下文目录 |
| `engine_file` | String | Dockerfile 路径 |

### `print_environment`

- **类型**: Boolean
- **必填**: 否
- **默认值**: `false`
- **说明**: 是否在任务日志中打印环境变量。可被项目配置覆盖。

---

## 项目配置

每个 space 包含一个或多个项目 YAML 文件，用于定义构建步骤。

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

environment_from_args:
  - name: "ENVIRONMENT"
    type: "radio"
    prompt: "选择部署环境"
    default: "staging"
    required: true
    options:
      - "development"
      - "staging"
      - "production"

steps:
  - name: "构建应用"
    container: "golang-builder"
    working_dir: "/workspace"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "echo 'Building ${APP_NAME} v${BUILD_VERSION}'"
      - "echo 'Git Hash: ${GIT_HASH}'"
      - "go build -o app ./main.go"
    timeout: 300
    continue_on_error: true

  - name: "运行测试"
    container: "golang-builder"
    working_dir: "/workspace"
    condition: "${ENVIRONMENT} != 'production'"
    commands:
      - "go test ./..."
    timeout: 180
```

### 字段详解

#### `name`

- **类型**: String
- **必填**: 是
- **说明**: 项目名称，同时作为 `PROJECT_NAME` 环境变量使用。

#### `description`

- **类型**: String
- **必填**: 否
- **说明**: 项目描述。

#### `source`

- **类型**: Object
- **必填**: 否
- **说明**: Git 仓库源配置。

| 字段 | 类型 | 说明 |
|------|------|------|
| `git_repo` | String | Git 仓库地址 |
| `git_branch` | String | 分支名（来自配置或当前分支） |

#### `environment_files`

- **类型**: Object 数组
- **必填**: 否
- **说明**: 需要加载的外部环境变量文件。

| 字段 | 类型 | 说明 |
|------|------|------|
| `format` | String | 文件格式：`yaml` 或 `env` |
| `path` | String | 环境变量文件路径 |

#### `format` 说明

- `yaml`: 标准 YAML 键值对（`KEY: "value"`）
- `env`: 行格式 `.env` 文件（`KEY=VALUE`），支持 `#` 注释和空行

#### `environment`

- **类型**: Map（String → String）
- **必填**: 否
- **说明**: 静态环境变量定义，支持 `${变量名}` 替换。

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

#### `environment_from_args`

- **类型**: Object 数组
- **必填**: 否
- **说明**: 交互式环境变量定义，详见 [变量指南](variables.zh.md#交互式环境变量)。

#### `print_environment`

- **类型**: Boolean
- **必填**: 否
- **说明**: 是否在任务日志中打印环境变量。覆盖 `.confkit.yml` 全局设置。默认：`false`。

#### `steps`

- **类型**: Object 数组
- **必填**: 是
- **说明**: 构建步骤列表，字段说明见下表。

### Step 字段详解

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `name` | String | 是 | — | 步骤显示名称 |
| `container` | String | 是 | — | 执行命令的构建器容器 |
| `working_dir` | String | 否 | `/workspace` | 容器内工作目录 |
| `commands` | Array\<String\> | 是 | — | 要执行的命令列表 |
| `condition` | String | 否 | — | 条件表达式，详见 [条件执行](conditions.zh.md) |
| `timeout` | Number | 否 | — | 步骤超时时间（秒） |
| `continue_on_error` | Boolean | 否 | `false` | 失败后是否继续执行下一步 |

所有命令均支持 `${变量名}` 变量替换。
