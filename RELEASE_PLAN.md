# ConfKit CLI 发布方案

## 🎯 发布策略概述

ConfKit CLI 作为一个现代化的 Rust CLI 工具，需要提供多样化的安装渠道，确保不同用户群体都能便捷地获取和使用。

## 📦 发布渠道规划

### 1. 主要发布渠道

#### 🚀 GitHub Releases (主发布渠道)
- **优势**: 完全控制，支持预发布版本
- **内容**: 多平台二进制文件 + 源码包
- **目标平台**:
  - `x86_64-unknown-linux-gnu` (Linux)
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)
  - `x86_64-pc-windows-msvc` (Windows)
  - `aarch64-unknown-linux-gnu` (Linux ARM64)

#### 📦 Crates.io (Rust 官方)
- **优势**: Rust 开发者的首选安装方式
- **安装**: `cargo install confkit-engine`
- **维护**: 与 GitHub 版本同步发布

#### 🐳 Docker Hub (容器化)
- **镜像**: `confkit/cli:latest`, `confkit/cli:v1.0.0`
- **用途**: CI/CD 集成，无需本地安装
- **基础镜像**: `debian:bookworm-slim` (小体积)

### 2. 包管理器集成

#### 🍺 Homebrew (macOS/Linux)
```bash
# 创建 Homebrew Formula
brew tap confkit/tap
brew install confkit
```

#### 📦 Scoop (Windows)
```bash
# Windows 包管理器
scoop bucket add confkit https://github.com/confkit/scoop-bucket
scoop install confkit
```

#### 🐧 APT/YUM (Linux)
- **deb 包**: 用于 Debian/Ubuntu
- **rpm 包**: 用于 RHEL/CentOS/Fedora
- **PPA**: 个人包存档便于维护

#### 🦀 Nix (通用包管理器)
- 支持 NixOS 和其他发行版
- 声明式配置，版本管理优秀

### 3. 一键安装脚本

#### 🔧 通用安装脚本
```bash
# 自动检测平台并安装
curl -sSL https://install.confkit.io | sh

# 或者使用 PowerShell (Windows)
irm https://install.confkit.io/install.ps1 | iex
```

## 🏗 构建和打包流程

### 1. 自动化 CI/CD 流程

#### GitHub Actions 配置
```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: ubuntu-latest  
            target: aarch64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    
    steps:
      - name: Build release binary
        run: |
          cargo build --release --target ${{ matrix.target }}
      
      - name: Package binary
        run: |
          # 创建压缩包，包含二进制文件、README、LICENSE
          tar -czf confkit-${{ matrix.target }}.tar.gz \
            target/${{ matrix.target }}/release/confkit \
            README.md README.zh.md LICENSE
      
      - name: Upload to release
        uses: actions/upload-release-asset@v1
```

### 2. 版本管理策略

#### 语义化版本控制
- **主版本** (Major): 1.x.x - 不兼容的 API 更改
- **次版本** (Minor): x.1.x - 向后兼容的功能增加
- **修订版本** (Patch): x.x.1 - 向后兼容的问题修复

#### 发布频率
- **稳定版**: 每月发布一次
- **补丁版**: 重要 bug 修复立即发布
- **预发布版**: 每周发布 beta 版本

### 3. 质量保证流程

#### 自动化测试
- **单元测试**: 每次 PR 自动运行
- **集成测试**: 包含真实 Docker 环境测试
- **多平台测试**: 确保跨平台兼容性

#### 发布前检查清单
- [ ] 所有测试通过
- [ ] 文档更新完整
- [ ] 版本号符合语义化规范
- [ ] 变更日志 (CHANGELOG.md) 更新
- [ ] 安全扫描通过

## 📋 安装渠道详细方案

### 1. 快速安装 (推荐)

#### 一键安装脚本
```bash
# Linux/macOS
curl -sSL https://install.confkit.io | sh

# 验证安装
confkit --version
```

#### 手动安装
```bash
# 1. 下载二进制文件
wget https://github.com/confkit/engine/releases/latest/download/confkit-linux-x64.tar.gz

# 2. 解压并安装
tar -xzf confkit-linux-x64.tar.gz
sudo mv confkit /usr/local/bin/

# 3. 验证
confkit --help
```

### 2. 包管理器安装

#### macOS (Homebrew)
```bash
brew tap confkit/tap
brew install confkit
```

#### Windows (Scoop)
```bash
scoop bucket add confkit https://github.com/confkit/scoop-bucket
scoop install confkit
```

#### Linux (APT)
```bash
# 添加 APT 源
curl -sSL https://packages.confkit.io/gpg.key | sudo apt-key add -
echo "deb https://packages.confkit.io/apt stable main" | sudo tee /etc/apt/sources.list.d/confkit.list

# 安装
sudo apt update
sudo apt install confkit
```

### 3. 开发者安装

#### Rust 开发者
```bash
# 从 crates.io 安装
cargo install confkit-engine

# 从源码编译
git clone https://github.com/confkit/engine.git
cd engine
cargo build --release
```

#### Docker 用户
```bash
# 运行容器
docker run -it --rm confkit/cli:latest --help

# 集成到 CI/CD
# 在 .github/workflows/build.yml 中使用
container: confkit/cli:latest
```

## 🔄 发布流程规范

### 1. 准备发布

#### 版本准备
```bash
# 1. 更新版本号
vim Cargo.toml  # 更新 version = "1.1.0"

# 2. 更新变更日志
vim CHANGELOG.md

# 3. 更新文档
vim README.md README.zh.md

# 4. 提交更改
git add .
git commit -m "chore: prepare release v1.1.0"
```

#### 创建发布标签
```bash
# 创建带注释的标签
git tag -a v1.1.0 -m "Release v1.1.0

Features:
- 新增任务并行执行
- 改进日志输出格式
- 优化容器启动速度

Bug Fixes:
- 修复配置文件解析错误
- 解决 Windows 路径问题
"

# 推送标签触发发布
git push origin v1.1.0
```

### 2. 自动化发布

#### GitHub Actions 触发
- 推送标签后自动触发构建
- 多平台并行构建
- 自动创建 GitHub Release
- 同步发布到 crates.io

#### 发布后验证
```bash
# 验证各平台安装
curl -sSL https://install.confkit.io | sh
confkit --version

# 验证 Docker 镜像
docker pull confkit/cli:v1.1.0
docker run --rm confkit/cli:v1.1.0 --version
```

### 3. 发布通知

#### 通知渠道
- **GitHub Release Notes**: 详细变更说明
- **项目文档**: 更新安装指南
- **社交媒体**: 重要版本发布公告
- **邮件通知**: 注册用户更新提醒

## 📊 监控和反馈

### 1. 下载统计
- GitHub Releases 下载量
- crates.io 下载统计
- Docker Hub 拉取次数

### 2. 用户反馈
- GitHub Issues 问题跟踪
- 社区讨论和建议
- 用户调研和满意度

### 3. 质量监控
- 崩溃报告收集
- 性能监控
- 兼容性问题跟踪

## 🚀 路线图

### 短期目标 (3个月)
- [ ] 建立完整的 CI/CD 流程
- [ ] 发布 v1.0.0 稳定版
- [ ] 完善主要平台的包管理器集成

### 中期目标 (6个月)
- [ ] 建立用户社区
- [ ] 插件系统开发
- [ ] 企业级功能增强

### 长期目标 (12个月)
- [ ] 多语言支持
- [ ] Web 界面开发
- [ ] 云服务集成

## 📝 总结

通过多渠道发布策略，ConfKit CLI 将能够：
- 覆盖不同技术栈的开发者
- 提供便捷的安装体验
- 确保高质量的发布流程
- 建立活跃的用户社区

这个发布方案将随着项目发展持续优化调整，确保 ConfKit CLI 能够服务更广泛的用户群体。 