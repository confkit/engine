# DevOps confkit(Developer Experience ) CLI - 轻量级 CI/CD 工具设计文档

## 🎯 设计初衷

### 传统 CI/CD 工具的痛点

当前主流的 CI/CD 工具（如 Jenkins、GitLab CI、GitHub Actions）存在以下问题：

1. **臃肿复杂**: Jenkins 等工具功能过于复杂，安装配置繁琐，资源消耗大
2. **配置耦合**: 流水线配置往往与平台绑定，难以版本化管理和迁移
3. **缺乏灵活性**: 预定义的工作流模式限制了复杂场景的实现
4. **维护成本高**: 需要专门的运维团队维护 CI/CD 平台本身
5. **环境依赖**: 构建环境难以标准化，存在"在我机器上能跑"的问题

### 设计目标

1. **轻量级工具**: 单一可执行文件，无需复杂安装和维护
2. **配置即代码**: 流水线配置文件可以和项目代码一起进行版本管理
3. **灵活的步骤系统**: 支持任意自定义步骤，满足各种复杂的构建、测试、发布需求
4. **容器化标准**: 基于 Docker 容器确保构建环境的一致性和可重现性
5. **完善的日志体系**: 提供结构化日志记录，便于问题追踪和分析
6. **Git 集成**: 深度集成 Git 工作流，支持分支策略和代码变更驱动
7. **DevOps 基础**: 为后续 DevOps 功能扩展奠定架构基础

## 🚀 目标实现的用法

### 核心命令概览

```bash
# 构建和管理构建环境
confkit builder create <builder-name>    # 创建构建容器
confkit builder list                     # 列出构建容器
confkit builder start <builder-name>     # 启动构建容器
confkit builder stop <builder-name>      # 停止构建容器

# 项目构建流水线
confkit run <project-config>             # 执行项目构建
confkit run --interactive               # 交互式构建

# 任务和日志管理
confkit task list [project]             # 列出任务
confkit task show <task-id>             # 查看任务详情
confkit task kill <task-id>             # 终止任务
confkit logs <project> [date] [task-id] # 查看日志
```

### 构建环境管理

```bash
# 创建 Golang 构建环境
confkit builder create golang-1.24 \
  --dockerfile builders/golang/Dockerfile.1.24 \
  --name "Golang 1.24 Builder"

# 交互式创建构建环境
confkit builder create --interactive

# 启动所有构建环境
confkit builder start --all

# 查看构建环境状态
confkit builder list --status
```

### 项目构建流程

```bash
# 基本构建（使用配置文件中的设置）
confkit run projects/my-api.yml

# 命令行参数覆盖配置文件
confkit run projects/my-api.yml \
  --git-branch release/v2.0.0 \
  --task-id "release-v2.0.0-$(date +%s)" \
  --builder golang-builder-1.24 \
  --env API_VERSION=v2.0.0

# 交互式构建（选择项目、分支、参数等）
confkit run --interactive

# 只执行特定步骤（调试用）
confkit run projects/my-api.yml \
  --step "代码构建" \
  --step "单元测试" \
  --verbose

# 跳过某些步骤
confkit run projects/my-api.yml \
  --skip-step "单元测试" \
  --skip-step "代码检查"

# 并行执行独立步骤
confkit run projects/frontend.yml --parallel

# 预览执行计划（不实际执行）
confkit run projects/my-api.yml --dry-run
```

### 日志和任务管理

```bash
# 查看今天的构建日志
confkit logs my-api 2024-01-15

# 查看具体任务的详细日志
confkit logs my-api 2024-01-15 my-api-20240115-143022-a1b2c3

# 实时跟踪正在运行的任务
confkit logs my-api --follow

# 查看正在运行的任务
confkit task list --status running

# 查看任务详细信息和执行状态
confkit task show my-api-20240115-143022-a1b2c3

# 终止运行中的任务
confkit task kill my-api-20240115-143022-a1b2c3

# 清理旧日志（超过30天）
confkit logs --cleanup --days 30
```

### 配置文件与Git集成

```bash
# 从Git仓库直接运行流水线
confkit run --git-repo https://github.com/company/api.git \
  --git-branch main \
  --config-path .confkit/build.yml

# 本地项目构建（自动检测.confkit目录）
cd /path/to/project
confkit run .confkit/build.yml

# 使用项目内的流水线配置
confkit run --auto-detect
```

### 批量和高级操作

```bash
# 批量构建多个项目
confkit run --batch projects/*.yml --parallel

# 监控模式（文件变化时自动构建）
confkit run projects/frontend.yml --watch

# 分布式构建（多机器协作）
confkit run projects/big-project.yml \
  --distributed \
  --workers node1:8080,node2:8080
```

## 🏗️ 整体设计

### 系统架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Interface │    │   Core Engine   │    │  Docker Daemon  │
│                 │    │                 │    │                 │
│  • run          │───▶│  • Task Manager │───▶│  • Containers   │
│  • builder      │    │  • Step Engine  │    │  • Images       │
│  • task         │    │  • Config Parser│    │  • Volumes      │
│  • logs         │    │  • Env Manager  │    └─────────────────┘
│  • interactive  │    │  • Git Client   │
└─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │  Storage Layer  │
                       │                 │
                       │  • Logs Store   │
                       │  • Config Store │
                       │  • Task Store   │
                       └─────────────────┘
```

### 核心模块设计

#### 1. 命令行接口层 (CLI Interface)
- **参数配置系统**: 支持命令行参数、配置文件、环境变量的多层次配置
- **交互式界面**: 提供友好的交互式项目构建和构建器管理
- **参数优先级**: 命令行参数 > 环境变量 > 配置文件 > 默认值
- **自动补全**: 支持 shell 自动补全功能

#### 2. 任务管理器 (Task Manager)
- **任务ID生成**: 基于项目名、时间戳和随机字符串生成唯一标识
- **状态跟踪**: 跟踪任务的整个生命周期（pending → running → completed/failed）
- **并发控制**: 管理同时运行的任务数量，避免资源竞争
- **中断处理**: 支持优雅中断和强制终止
- **任务队列**: 支持任务排队和优先级管理

#### 3. 步骤执行引擎 (Step Engine)
- **依赖解析**: 分析步骤间的依赖关系，构建执行图
- **并行执行**: 自动识别可并行执行的独立步骤
- **容器路由**: 根据配置将命令路由到指定容器或宿主机
- **错误处理**: 支持重试、超时和失败策略
- **步骤缓存**: 支持步骤结果缓存，提高构建效率

#### 4. 配置解析器 (Config Parser)
- **多格式支持**: 支持YAML、JSON配置文件格式
- **变量替换**: 支持环境变量和内置变量的动态替换
- **配置验证**: 验证配置文件的完整性和正确性
- **模板支持**: 支持配置模板和继承机制
- **参数覆盖**: 命令行参数可覆盖配置文件设置

#### 5. 构建器管理器 (Builder Manager)
- **镜像构建**: 从Dockerfile构建构建器镜像
- **容器生命周期**: 管理构建容器的启动、停止、重启
- **健康检查**: 定期检查构建器容器的健康状态
- **资源监控**: 监控容器的CPU、内存使用情况

#### 6. Git集成模块 (Git Client)
- **仓库操作**: 支持克隆、拉取、分支切换等Git操作
- **信息提取**: 自动提取commit hash、分支名、标签等信息
- **Webhook支持**: 支持Git webhook触发构建
- **多仓库支持**: 支持单次构建涉及多个Git仓库

### 参数配置系统设计

#### 配置优先级（从高到低）
1. **命令行参数**: 直接通过命令行传入的参数
2. **环境变量**: 系统环境变量（以confkit_前缀）
3. **项目配置文件**: 项目特定的配置文件
4. **全局配置文件**: 用户或系统级全局配置
5. **默认值**: 程序内置的默认值

#### 参数传递方式
```bash
# 1. 命令行参数（最高优先级）
confkit run projects/api.yml \
  --git-branch feature/new-api \
  --builder golang-1.24 \
  --env API_VERSION=v2.0.0 \
  --parallel \
  --timeout 30m

# 2. 环境变量
export confkit_GIT_BRANCH=develop
export confkit_BUILDER=golang-1.24
export confkit_PARALLEL=true
confkit run projects/api.yml

# 3. 配置文件覆盖
confkit run projects/api.yml --config-override '{
  "git_branch": "hotfix/urgent-fix",
  "environment": {"DEBUG": "true"},
  "parallel": true
}'

# 4. 交互式配置
confkit run --interactive
```

### 交互式命令设计

#### 项目构建交互流程
```
┌─ 项目构建向导 ─────────────────────────┐
│                                     │
│ 1. 选择项目配置                        │
│    [ ] projects/api.yml             │
│    [x] projects/frontend.yml        │
│    [ ] projects/worker.yml          │
│                                     │
│ 2. Git 配置                         │
│    仓库: https://github.com/...      │
│    分支: [main] ________________     │
│                                     │
│ 3. 构建器选择                        │
│    [x] node-builder-22  (运行中)     │
│    [ ] golang-builder-1.24 (已停止)  │
│                                     │
│ 4. 环境变量                          │
│    NODE_ENV=production              │
│    API_URL=https://api.prod.com     │
│    [添加更多...]                     │
│                                     │
│ 5. 构建选项                          │
│    [x] 并行执行  [ ] 跳过测试         │
│    [x] 详细日志  [ ] 预览模式         │
│                                     │
│ [开始构建] [保存配置] [取消]            │
└─────────────────────────────────────┘
```

#### 构建器管理交互流程
```
┌─ 构建器管理 ─────────────────────────┐
│                                     │
│ 当前构建器状态:                       │
│ ┌─────────────────────────────────┐  │
│ │ golang-1.24    🟢 运行中  8GB   │  │
│ │ node-22        ⚪ 已停止   -    │  │
│ │ rust-latest    🟢 运行中  4GB   │  │
│ │ python-3.11    ❌ 构建失败 -    │  │
│ └─────────────────────────────────┘  │
│                                     │
│ 操作选择:                            │
│ 1. 创建新构建器                       │
│ 2. 启动构建器                        │
│ 3. 停止构建器                        │
│ 4. 重新构建镜像                       │
│ 5. 查看构建器日志                     │
│ 6. 删除构建器                        │
│                                     │
│ 请选择操作 [1-6]: _                  │
└─────────────────────────────────────┘
```

## 📚 相关文档

详细的配置格式和技术实现请参考以下文档：

- **[配置文档](confkit_CONFIG.md)** - 详细的配置文件格式、示例和最佳实践
- **[技术实现文档](confkit_IMPLEMENTATION.md)** - Rust项目结构、技术架构和开发指南

## 🚀 快速开始

1. **安装工具**
   ```bash
   # 从源码编译（推荐）
   git clone https://github.com/company/confkit-cli.git
   cd confkit-cli && cargo build --release
   sudo cp target/release/confkit /usr/local/bin/
   ```

2. **创建构建环境**
   ```bash
   # 交互式创建构建器
   confkit builder create --interactive
   
   # 或直接创建特定构建器
   confkit builder create golang-1.24 --dockerfile builders/golang/Dockerfile.1.24
   ```

3. **运行第一个构建**
   ```bash
   # 交互式构建
   confkit run --interactive
   
   # 或使用配置文件
   confkit run projects/my-project.yml
   ```

这个设计为替代传统臃肿的CI/CD工具提供了轻量级、灵活的解决方案，让团队能够通过Git仓库管理流水线配置，实现真正的"配置即代码"。 