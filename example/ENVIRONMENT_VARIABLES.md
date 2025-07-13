# ConfKit 环境变量使用指南

## 📋 环境变量分类

ConfKit 处理三种类型的环境变量：

### 1. 🔧 内置系统变量 (自动注入)

这些变量由 ConfKit 自动注入，无需手动设置：

| 变量名 | 说明 | 示例值 |
|--------|------|--------|
| `TASK_ID` | 任务唯一标识符 | `confkit-release-20250113-143022-a1b2c3` |
| `PROJECT_NAME` | 项目名称 | `confkit-release` |
| `SPACE_NAME` | 空间名称 | `release` |

### 2. 🔗 Git 变量 (自动注入)

基于 Git 仓库信息自动注入：

| 变量名 | 说明 | 示例值 |
|--------|------|--------|
| `GIT_REPO` | Git 仓库地址 | `https://github.com/confkit/engine.git` |
| `GIT_BRANCH` | Git 分支名 | `main` |
| `GIT_HASH` | 完整 commit hash | `2373442e2de493b9f97ad6aa5e0f2845811a5e3e` |
| `GIT_COMMIT_HASH` | 完整 commit hash (别名) | `2373442e2de493b9f97ad6aa5e0f2845811a5e3e` |
| `GIT_COMMIT_SHORT` | 短 commit hash | `2373442e` |
| `GIT_TAG` | 当前 tag (如果有) | `v1.0.0` |

### 3. 🌍 外部环境变量 (手动设置)

这些变量需要在运行 ConfKit 之前设置：

| 变量名 | 说明 | 设置方式 |
|--------|------|----------|
| `RELEASE_VERSION` | 发布版本号 | `export RELEASE_VERSION="1.0.0"` |
| `BUILD_DATE` | 构建日期 | `export BUILD_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"` |
| `CARGO_REGISTRY_TOKEN` | crates.io 令牌 | `export CARGO_REGISTRY_TOKEN="your-token"` |
| `DOCKER_USERNAME` | Docker Hub 用户名 | `export DOCKER_USERNAME="your-username"` |
| `DOCKER_PASSWORD` | Docker Hub 密码 | `export DOCKER_PASSWORD="your-password"` |
| `GITHUB_TOKEN` | GitHub 令牌 | `export GITHUB_TOKEN="your-token"` |

## 📝 配置文件中的使用

### 1. 引用内置变量

在配置文件的 `commands` 中使用 ConfKit 内置变量：

```yaml
commands:
  - "echo 'Task ID: ${TASK_ID}'"
  - "echo 'Project: ${PROJECT_NAME}'"
  - "echo 'Git Hash: ${GIT_HASH}'"
```

### 2. 引用外部变量

在配置文件的 `commands` 中使用外部环境变量：

```yaml
commands:
  - "echo 'Version: $RELEASE_VERSION'"
  - "echo 'Build Date: $BUILD_DATE'"
  - "cargo login $CARGO_REGISTRY_TOKEN"
```

### 3. 环境变量声明

在配置文件的 `environment` 部分声明自定义变量：

```yaml
environment:
  # 自定义变量 (基于内置变量)
  BUILD_TARGET_DIR: "target"
  ARTIFACTS_DIR: "/artifacts"
  VERSION_TAG: "${PROJECT_NAME}-${GIT_COMMIT_SHORT}"
  
  # 注释说明外部变量 (无需声明，ConfKit 会继承)
  # RELEASE_VERSION - 发布版本号 (外部设置)
  # BUILD_DATE - 构建日期 (外部设置)
```

## 🔄 变量替换语法

### ConfKit 配置文件语法

使用 `${VARIABLE_NAME}` 语法：

```yaml
commands:
  - "echo 'Building ${PROJECT_NAME} with Git hash ${GIT_HASH}'"
  - "cp README.md /artifacts/${PROJECT_NAME}/"
```

### Shell 命令语法

在 commands 数组的 shell 命令中使用标准 shell 语法：

```yaml
commands:
  # 标准 shell 语法
  - "echo 'Version: $RELEASE_VERSION'"
  - "docker build -t myapp:$RELEASE_VERSION ."
  
  # 或者使用大括号 (推荐)
  - "echo 'Version: ${RELEASE_VERSION}'"
  - "docker tag myapp:${RELEASE_VERSION} myapp:latest"
```

## 🎯 实际示例

### 发布脚本设置

```bash
#!/bin/bash

# 设置外部环境变量
export RELEASE_VERSION="1.0.0"
export BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
export CARGO_REGISTRY_TOKEN="your-token"
export DOCKER_USERNAME="your-username"
export DOCKER_PASSWORD="your-password"
export GITHUB_TOKEN="your-token"

# 运行 ConfKit (会继承这些环境变量)
confkit run --space release --project confkit-release
```

### 配置文件中的使用

```yaml
name: "confkit-release"
type: "rust"

environment:
  # 自定义变量
  BUILD_TARGET_DIR: "target"
  RELEASE_ARCHIVE: "confkit-${GIT_COMMIT_SHORT}.tar.gz"

steps:
  - name: "准备发布"
    commands:
      # 混合使用内置变量和外部变量
      - "echo 'Releasing ${PROJECT_NAME} v$RELEASE_VERSION'"
      - "echo 'Git: ${GIT_HASH} on ${GIT_BRANCH}'"
      - "echo 'Build: $BUILD_DATE'"
      
  - name: "构建镜像"
    commands:
      # 外部变量用于版本标记
      - "docker build -t confkit/cli:$RELEASE_VERSION ."
      - "docker tag confkit/cli:$RELEASE_VERSION confkit/cli:latest"
      
  - name: "发布到 crates.io"
    commands:
      # 外部变量用于认证
      - "cargo login $CARGO_REGISTRY_TOKEN"
      - "cargo publish"
```

## ⚠️ 注意事项

1. **变量优先级**: 外部环境变量 > 配置文件环境变量 > 内置变量
2. **变量继承**: ConfKit 会继承 shell 环境中的所有变量
3. **安全性**: 敏感信息（令牌、密码）应通过外部环境变量传入，避免写在配置文件中
4. **可见性**: 所有环境变量在执行的容器中都是可见的

## 🐛 常见错误

### 错误: 在 environment 中声明外部变量

```yaml
# ❌ 错误做法
environment:
  RELEASE_VERSION: "${RELEASE_TAG}"  # RELEASE_TAG 不是内置变量
  CARGO_TOKEN: "${CARGO_REGISTRY_TOKEN}"  # 不必要的声明
```

```yaml
# ✅ 正确做法
environment:
  # 只声明真正的自定义变量
  BUILD_TARGET: "${PROJECT_NAME}-build"
  
# 外部变量直接在 commands 中使用，无需声明
commands:
  - "echo 'Version: $RELEASE_VERSION'"
  - "cargo login $CARGO_REGISTRY_TOKEN"
```

### 错误: 混淆变量语法

```yaml
# ❌ 错误做法
commands:
  - "echo 'Project: $PROJECT_NAME'"  # 应该用 ${PROJECT_NAME}
  - "docker build -t app:${RELEASE_VERSION}"  # shell 中应该用 $RELEASE_VERSION
```

```yaml
# ✅ 正确做法
commands:
  - "echo 'Project: ${PROJECT_NAME}'"  # ConfKit 内置变量用 ${}
  - "docker build -t app:$RELEASE_VERSION"  # 外部变量在 shell 中用 $
``` 