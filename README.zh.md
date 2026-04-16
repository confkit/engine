# ConfKit CLI

ConfKit 是一个配置驱动的构建和部署工具，专为现代化 CI/CD 流水线设计。

## 核心功能

- **构建器管理**: Docker 镜像与容器的完整生命周期管理
- **配置驱动**: 通过 YAML 配置文件定义构建流程
- **条件执行**: 基于环境变量和运行时条件的智能步骤执行
- **任务执行**: 支持本地和容器化命令执行
- **日志管理**: 完整的构建日志记录、查看和管理
- **Git 集成**: 原生支持 Git 仓库操作和环境变量注入
- **交互式界面**: 友好的命令行交互体验

## 快速开始

```bash
# 安装
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh

# 交互式模式
confkit

# 运行构建
confkit run --space hello --project hello-app
```

## 文档

| 文档 | 说明 |
|------|------|
| [快速开始](docs/getting-started.zh.md) | 安装、配置结构、基本使用 |
| [项目结构](docs/project-structure.zh.md) | 目录布局、空间/项目/镜像组织方式 |
| [配置文件参考](docs/configuration.zh.md) | 主配置（`.confkit.yml`）和项目配置字段详解 |
| [Compose 配置](docs/compose.zh.md) | 构建器容器定义、服务字段、Volume 挂载 |
| [环境变量](docs/variables.zh.md) | 系统变量、Git 变量、自定义变量、命令行注入、交互式输入 |
| [条件执行](docs/conditions.zh.md) | 条件表达式语法、运算符、示例 |
| [构建器管理](docs/builder.zh.md) | 镜像与容器生命周期管理 |
| [日志管理](docs/logs.zh.md) | 结构化日志存储、查询与清理 |
| [Volumes 目录](docs/volumes.zh.md) | 运行时数据目录：workspace、artifacts、cache、temp、logs、context |
| [CLI 命令参考](docs/cli-reference.zh.md) | 所有子命令的完整参考 |

## 许可证

MIT License
