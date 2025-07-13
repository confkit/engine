# ConfKit 自发布流程

这个目录包含了使用 ConfKit 来发布 ConfKit 自己的完整配置和脚本。这是一个很好的"dogfooding"示例，展示了 ConfKit 的强大功能。

## 📋 文件说明

### 配置文件
- `.confkit/spaces/release/config.yml` - 发布空间配置
- `.confkit/spaces/release/projects/confkit-release.yml` - 发布项目配置
- `release-docker-compose.yml` - 发布环境容器定义

### 脚本文件
- `release.sh` - 发布脚本，简化发布流程
- `install.sh` - 一键安装脚本（Linux/macOS）
- `install.ps1` - 一键安装脚本（Windows）

### 文档文件
- `RELEASE_README.md` - 发布流程使用说明（本文件）
- `ENVIRONMENT_VARIABLES.md` - 环境变量使用指南
- `SELF_RELEASE_OVERVIEW.md` - 自发布解决方案概览

## 🚀 使用方法

### 1. 快速发布

```bash
# 进入 examples 目录
cd examples

# 设置环境变量
export CARGO_REGISTRY_TOKEN="your-crates-token"
export DOCKER_USERNAME="your-docker-username"
export DOCKER_PASSWORD="your-docker-password"
export GITHUB_TOKEN="your-github-token"

# 发布版本 1.0.0
./release.sh 1.0.0
```

### 2. 测试发布流程

```bash
# 运行 dry-run 测试
./release.sh 1.0.0 --dry-run
```

### 3. 使用 ConfKit 命令发布

```bash
# 手动运行发布任务
export RELEASE_TAG="v1.0.0"
export BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# 启动发布环境
docker-compose -f release-docker-compose.yml --profile release up -d

# 运行发布
confkit run --space release --project confkit-release
```

## 🔧 环境准备

### 1. 必要工具
- ConfKit CLI (已安装)
- Docker 和 Docker Compose
- Git

### 2. 环境变量

ConfKit 使用三类环境变量，详细说明请参考 [环境变量使用指南](ENVIRONMENT_VARIABLES.md)。

#### 必需变量（外部设置）
```bash
export CARGO_REGISTRY_TOKEN="your-crates-token"
export DOCKER_USERNAME="your-docker-username"
export DOCKER_PASSWORD="your-docker-password"
export GITHUB_TOKEN="your-github-token"
```

#### 可选变量
```bash
export SLACK_WEBHOOK_URL="your-slack-webhook"  # Slack 通知
```

#### 自动注入变量（ConfKit 提供）
ConfKit 会自动注入以下变量：
- `TASK_ID` - 任务唯一标识符
- `PROJECT_NAME` - 项目名称
- `SPACE_NAME` - 空间名称
- `GIT_HASH` - Git commit hash
- `GIT_COMMIT_SHORT` - 短 commit hash
- `GIT_BRANCH` - Git 分支名
- `GIT_TAG` - Git 标签（如果有）

💡 **重要**: 外部变量在配置文件中使用 `$VARIABLE` 语法，内置变量使用 `${VARIABLE}` 语法。

### 3. 获取令牌

#### crates.io 令牌
1. 访问 https://crates.io/me
2. 点击 "New Token"
3. 设置权限为 "Publish"

#### Docker Hub 令牌
1. 访问 https://hub.docker.com/settings/security
2. 创建新的 Access Token

#### GitHub 令牌
1. 访问 https://github.com/settings/tokens
2. 创建 Personal Access Token
3. 需要 `repo` 和 `write:packages` 权限

## 📦 发布流程详解

### 1. 代码检查阶段
- 代码格式检查 (`cargo fmt`)
- 代码质量检查 (`cargo clippy`)
- 安全审计 (`cargo audit`)
- 运行测试套件

### 2. 多平台构建
- Linux x86_64
- Linux ARM64
- macOS Intel
- macOS Apple Silicon
- Windows x86_64

### 3. Docker 镜像构建
- 构建多架构镜像
- 标记版本标签
- 测试镜像功能

### 4. 发布到各渠道
- **GitHub Releases**: 上传所有平台的二进制文件
- **crates.io**: 发布 Rust 包
- **Docker Hub**: 推送容器镜像

### 5. 验证和通知
- 验证各渠道发布成功
- 发送通知（Slack 等）

## 🏗️ 构建器说明

### rust-builder
- 基于 `rust:1.75-slim`
- 用于基础 Rust 构建和 Linux x86_64

### rust-cross-builder
- 基于 `ghcr.io/cross-rs/cross:main`
- 用于交叉编译 Linux ARM64

### rust-mac-builder
- 基于 `joseluisq/rust-linux-darwin-builder`
- 用于 macOS 平台交叉编译

### rust-windows-builder
- 基于 `rust:1.75-slim` + MinGW
- 用于 Windows 平台交叉编译

### docker-builder
- 基于 `docker:24-dind`
- 用于 Docker 镜像构建和推送

### release-tools
- 基于 `alpine:latest` + GitHub CLI
- 用于 GitHub Releases 管理

## 🔄 发布脚本功能

### 主要功能
- 自动验证环境和依赖
- 版本号管理和验证
- 一键发布到多个平台
- 发布后自动验证

### 选项说明
- `--dry-run`: 测试模式，不实际发布
- `--skip-tests`: 跳过测试阶段
- `--help`: 显示帮助信息

### 错误处理
- 自动清理发布环境
- 详细的错误信息和建议
- 支持中断后恢复

## 🔍 故障排除

### 常见问题

#### 1. 构建失败
```bash
# 检查构建环境
docker-compose -f release-docker-compose.yml --profile release logs

# 手动进入容器调试
docker exec -it confkit-rust-builder bash
```

#### 2. 发布失败
```bash
# 检查环境变量
echo $CARGO_REGISTRY_TOKEN
echo $DOCKER_USERNAME

# 测试网络连接
curl -s https://crates.io/api/v1/crates/confkit-engine
```

#### 3. 容器启动失败
```bash
# 清理并重新启动
docker-compose -f release-docker-compose.yml --profile release down
docker-compose -f release-docker-compose.yml --profile release up -d
```

## 🎯 最佳实践

### 1. 发布前检查
- 确保所有测试通过
- 更新 CHANGELOG.md
- 检查依赖版本

### 2. 版本管理
- 遵循语义化版本控制
- 使用有意义的标签消息
- 保持版本号一致性

### 3. 安全考虑
- 使用环境变量存储敏感信息
- 定期更新令牌
- 限制令牌权限

### 4. 监控和维护
- 监控发布成功率
- 定期更新构建镜像
- 备份重要配置

## 🚀 自动化建议

### 1. CI/CD 集成
虽然这里展示的是手动发布，但可以很容易地集成到 CI/CD 流程中：

```yaml
# .github/workflows/release.yml
name: Release with ConfKit
on:
  push:
    tags: ['v*']
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Release with ConfKit
        run: |
          cd examples
          ./release.sh ${GITHUB_REF#refs/tags/v}
```

### 2. 定时发布
可以设置定时任务进行夜间发布：

```bash
# 添加到 crontab
0 2 * * * cd /path/to/confkit/examples && ./release.sh $(date +%Y.%m.%d)
```

## 📚 进阶用法

### 1. 自定义构建器
可以在 `release-docker-compose.yml` 中添加自定义构建器：

```yaml
custom-builder:
  image: your-custom-image
  working_dir: /workspace
  volumes:
    - ../:/workspace:cached
  command: tail -f /dev/null
```

### 2. 添加新的发布渠道
在 `confkit-release.yml` 中添加新的发布步骤：

```yaml
- name: "发布到新渠道"
  container: "custom-publisher"
  working_dir: "/workspace"
  commands:
    - "your-publish-command"
```

### 3. 通知集成
可以集成更多通知渠道：

```yaml
notifications:
  - type: "teams"
    webhook_url: "${TEAMS_WEBHOOK_URL}"
  - type: "discord"
    webhook_url: "${DISCORD_WEBHOOK_URL}"
```

## 💡 总结

这个自发布流程展示了 ConfKit 的核心理念：
- **配置驱动**: 所有发布逻辑都在配置文件中
- **容器化**: 使用 Docker 确保一致的构建环境
- **可重复**: 任何人都可以复现发布过程
- **可扩展**: 可以轻松添加新的平台和渠道

通过这个示例，您可以看到 ConfKit 不仅是一个构建工具，更是一个完整的 DevOps 解决方案。 