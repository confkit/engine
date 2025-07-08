# confkit 开发 Todolist

## 开发原则
- 按 CLI 命令使用顺序开发 (builder → run → task → logs → interactive)
- 每个命令都能独立验证功能
- interactive 可调用所有其他命令功能
- 基于现有配置示例进行功能设计

## 阶段 1: 基础配置解析 (P0)
> 目标：能够解析和验证配置文件（所有命令的基础）

### 1.1 配置文件解析
- [ ] 完善 `ProjectConfig::from_file()` 实现
- [ ] 实现配置文件验证 `ProjectConfig::validate()`
- [ ] 支持环境变量替换 (`${GIT_REPO}`, `${PROJECT_NAME}`)
- [ ] 添加配置解析错误处理

**验证方式**: 能够成功解析 examples 中的配置文件

## 阶段 2: Builder 命令实现 (P0)
> 目标：实现构建器管理，为容器化构建做准备

### 2.1 Docker 客户端基础
- [ ] 实现 `DockerClient::new()` 连接检查
- [ ] 实现 `DockerClient::check_connection()`
- [ ] 实现镜像拉取 `pull_image()`

### 2.2 构建器管理
- [ ] 实现 `BuilderManager::create_builder()`
- [ ] 实现 `BuilderManager::list_builders()`
- [ ] 实现 `BuilderManager::health_check()`
- [ ] 实现 `BuilderManager::start_builder()` / `stop_builder()`

### 2.3 Builder 命令
- [ ] 实现 `confkit builder list`
- [ ] 实现 `confkit builder create <name> <image>`
- [ ] 实现 `confkit builder start/stop <name>`
- [ ] 实现 `confkit builder health <name>`

**验证方式**: 能够管理 Docker 容器作为构建器

## 阶段 3: Run 命令实现 (P0)
> 目标：实现任务执行的核心功能

### 3.1 任务管理核心
- [ ] 实现 `TaskManager::generate_task_id()`
- [ ] 实现任务状态跟踪 (Pending, Running, Success, Failed)
- [ ] 实现任务上下文 `TaskContext`
- [ ] 实现 `TaskManager::execute_task()` 框架

### 3.2 步骤执行引擎
- [ ] 实现 `StepEngine::execute_step()` 
- [ ] 实现本地命令执行（非容器模式）
- [ ] 实现容器化命令执行
- [ ] 实现工作目录和环境变量处理

### 3.3 Git 集成
- [ ] 实现 `GitClient::clone_repository()`
- [ ] 实现分支切换 `checkout_branch()`
- [ ] 集成 Git 操作到步骤执行

### 3.4 Run 命令
- [ ] 实现 `confkit run <config.yml>`
- [ ] 支持 `--git-branch`, `--parallel`, `--force` 等选项
- [ ] 实现 `--dry-run` 模式
- [ ] 实现步骤依赖解析和执行顺序

**验证方式**: 能够完整执行 examples 中的构建配置

## 阶段 4: Task 命令实现 (P1)
> 目标：实现任务状态查询和管理

### 4.1 任务查询
- [ ] 实现 `confkit task list`
- [ ] 实现 `confkit task show <task-id>`
- [ ] 实现任务状态格式化输出
- [ ] 实现任务历史记录

### 4.2 任务控制
- [ ] 实现 `confkit task kill <task-id>`
- [ ] 实现 `confkit task restart <task-id>`
- [ ] 实现 `confkit task clean` (清理已完成任务)

### 4.3 存储管理
- [ ] 实现工作空间管理 `create_task_workspace()`
- [ ] 实现产物管理 `save_artifact()`, `list_task_artifacts()`

**验证方式**: 运行任务后能查看和管理任务状态

## 阶段 5: Logs 命令实现 (P1)
> 目标：实现完整的日志记录和查看

### 5.1 日志系统
- [ ] 实现 `LoggingManager` 核心功能
- [ ] 实现任务日志文件创建和写入
- [ ] 实现步骤日志分离记录
- [ ] 实现日志级别和时间戳

### 5.2 Logs 命令
- [ ] 实现 `confkit logs <project-name>`
- [ ] 实现 `--follow` 实时跟踪模式
- [ ] 实现 `--step <step-name>` 步骤过滤
- [ ] 实现 `--lines <n>` 行数限制
- [ ] 实现 `--level <level>` 日志级别过滤

**验证方式**: 执行任务后能查看和跟踪各种格式的日志

## 阶段 6: Interactive 命令实现 (P2)
> 目标：实现交互式界面，集成所有命令功能

### 6.1 交互式核心
- [ ] 实现主菜单界面
- [ ] 实现项目配置选择
- [ ] 实现实时状态显示

### 6.2 命令集成
- [ ] 集成 builder 命令功能
- [ ] 集成 run 命令功能  
- [ ] 集成 task 命令功能
- [ ] 集成 logs 命令功能

### 6.3 交互式增强
- [ ] 实现快捷键操作
- [ ] 实现任务进度可视化
- [ ] 实现实时日志显示

**验证方式**: `confkit interactive` 能调用所有其他命令功能

## 阶段 7: 高级功能增强 (P2)
> 目标：完善并行执行、通知等高级特性

### 7.1 并行执行优化
- [ ] 实现依赖图构建和循环检测
- [ ] 实现 `parallel_group` 支持
- [ ] 实现步骤并发控制

### 7.2 通知系统
- [ ] 实现 Webhook 通知
- [ ] 实现构建状态通知

### 7.3 网络工具
- [ ] 实现网络连接测试
- [ ] 实现代理支持

**验证方式**: 验证复杂依赖关系和通知功能

## 验证里程碑

### 里程碑 1: 基础构建器 (阶段 1-2)
能够解析配置文件并管理 Docker 构建器

### 里程碑 2: 核心执行 (阶段 3) 
完整的任务执行流程，支持本地和容器化构建

### 里程碑 3: 状态管理 (阶段 4-5)
任务状态查询和完整的日志系统

### 里程碑 4: 交互完整 (阶段 6)
交互式界面集成所有命令功能

### 里程碑 5: 生产就绪 (阶段 7)
并行执行、通知等高级特性的生产级工具 