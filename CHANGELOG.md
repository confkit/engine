# 变更日志 / Changelog

本文档记录了 ConfKit CLI 的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本控制](https://semver.org/lang/zh-CN/)。

## [未发布] - Unreleased

### 新增 Added
- 待发布的新功能

### 变更 Changed
- 已有功能的变更

### 修复 Fixed
- 问题修复

### 移除 Removed
- 移除的功能

## [1.0.0] - 2025-01-13

### 新增 Added
- 🎉 ConfKit CLI 初始版本发布
- 📦 **Builder 管理系统**
  - Docker 镜像管理（拉取、构建、删除）
  - 容器生命周期管理（创建、启动、停止、删除）
  - 基于 docker-compose.yml 的构建器定义
  - 健康检查和状态监控
- 🔧 **配置驱动构建**
  - YAML 配置文件支持
  - 空间和项目管理
  - 环境变量配置
  - 构建步骤定义
- 🚀 **任务执行引擎**
  - 容器化命令执行
  - 工作目录管理
  - 超时控制
  - 并行执行支持
- 📝 **完整日志系统**
  - 自动日志记录
  - 智能日志匹配
  - 日志文件管理
  - 时间戳和任务ID追踪
- 🔗 **Git 集成**
  - 自动 Git 仓库克隆
  - 环境变量自动注入（GIT_HASH, GIT_TAG等）
  - 分支切换支持
- 💻 **交互式界面**
  - 友好的命令行菜单
  - Builder 管理界面
  - 日志查看界面
  - 运行任务界面
- 📋 **核心 CLI 命令**
  - `confkit builder` - 构建器管理
  - `confkit run` - 运行构建任务
  - `confkit log` - 日志管理
  - `confkit interactive` - 交互式模式

### 技术特性 Technical Features
- 🦀 **Rust 编写** - 高性能、内存安全
- 🐳 **Docker 集成** - 容器化构建环境
- 📊 **异步执行** - 基于 tokio 的异步运行时
- 🔐 **类型安全** - 强类型配置和错误处理
- 🌐 **跨平台** - 支持 Linux、macOS、Windows

### 支持平台 Supported Platforms
- Linux x86_64
- Linux ARM64
- macOS Intel
- macOS Apple Silicon
- Windows x86_64

### 安装方式 Installation Methods
- 📥 **一键安装**: `curl -sSL https://install.confkit.io | sh`
- 📦 **Cargo**: `cargo install confkit-engine`
- 🐳 **Docker**: `docker run -it confkit/cli:latest`
- 🍺 **包管理器**: 支持 Homebrew、Scoop、APT 等

## 发布说明 Release Notes

### 版本命名规则
- **主版本** (Major): 破坏性变更
- **次版本** (Minor): 新功能，向后兼容
- **修订版本** (Patch): 错误修复，向后兼容

### 贡献指南
如果您想为 ConfKit CLI 贡献代码：
1. Fork 本仓库
2. 创建功能分支
3. 提交变更
4. 发起 Pull Request

### 反馈和问题
- 🐛 **问题报告**: [GitHub Issues](https://github.com/confkit/engine/issues)
- 💡 **功能请求**: [GitHub Discussions](https://github.com/confkit/engine/discussions)
- 📧 **联系方式**: confkit@example.com

---

**注意**: 此变更日志仅包含面向用户的重要变更。完整的提交历史请查看 [Git 历史记录](https://github.com/confkit/engine/commits/main)。 