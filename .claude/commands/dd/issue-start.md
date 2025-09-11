---
allowed-tools: Task, Read, Write, Edit, MultiEdit, Bash, Grep, Glob
---

# Issue Start

智能判断议题状态，采取对应操作策略，集成 hooks 机制，执行具体议题。

## Usage

```bash
/dd:issue-start <功能名称>:<议题ID>
/dd:issue-start <功能名称>:<议题ID> --temp  # 临时需求
```

Options:
- `--temp` - 临时需求，使用特殊处理流程

## Instructions

### 1. 智能状态判断

自动检测议题当前状态，采取相应策略：

- **未开始** → 开始执行议题
- **进行中** → 继续执行未完成部分
- **已完成** → 显示议题完成情况，提示下一步

### 2. 上下文恢复功能

对于进行中的议题：

- 自动加载议题上下文和历史记录
- 恢复之前的工作状态和进度
- 继续未完成的 todo 项目
- 保持工作连续性

### 3. 深度执行分析

- 深度分析议题实现方案
- 自动更新 todo 列表和进度
- 同步议题和功能状态
- 实时反馈执行情况

### 4. 状态查询脚本使用

命令通过 hooks 机制配合查询脚本获取状态信息，常用脚本：

```bash
# 获取议题状态
bash .claude/scripts/dd/query/get-issue.sh "<功能名称>:<议题ID>"

# 获取功能状态（用于检查依赖）- 所有用法选项：
bash .claude/scripts/dd/query/get-feature.sh "<功能名称>"                    # 默认读取 overview.md
bash .claude/scripts/dd/query/get-feature.sh --status-only "<功能名称>"     # 仅显示状态信息，不显示文档内容
bash .claude/scripts/dd/query/get-feature.sh --all "<功能名称>"             # 读取所有文档 (overview + technical + acceptance)
bash .claude/scripts/dd/query/get-feature.sh --overview "<功能名称>"        # 仅读取功能概述文档 (overview.md)
bash .claude/scripts/dd/query/get-feature.sh --technical "<功能名称>"       # 仅读取技术方案文档 (technical.md)
bash .claude/scripts/dd/query/get-feature.sh --acceptance "<功能名称>"      # 仅读取验收标准文档 (acceptance.md)

# 获取 Git 信息
bash .claude/scripts/dd/utils/git-info.sh <command>

# Git 信息查询示例：
bash .claude/scripts/dd/utils/git-info.sh branch              # 当前分支
bash .claude/scripts/dd/utils/git-info.sh feature <name>      # 检查功能分支
bash .claude/scripts/dd/utils/git-info.sh clean              # 工作区是否干净
bash .claude/scripts/dd/utils/git-info.sh updates            # 检查远程更新
```

### 5. Hooks 机制配置

此命令支持用户自定义的 hooks 配置，在执行过程中的关键阶段会触发相应的钩子。

**Before 阶段 - 议题执行前**
- 触发时机：在议题执行启动之前
- 用途：状态检查、环境准备、权限验证

**Running 阶段 - 议题执行过程中**
- 触发时机：在议题执行分析过程中
- 用途：执行过程监控、状态验证、中间结果处理

**After 阶段 - 议题执行启动后**
- 触发时机：在议题开始执行或恢复执行完成后
- 用途：状态同步、通知发送、后续处理
- 示例：更新议题状态、发送开始通知、同步项目看板

### 6. Hooks 错误处理

如果 hooks 返回错误或阻塞信号，命令执行将：

- **Before hooks 失败**: 停止议题执行，显示错误信息和解决建议
- **Running hooks 失败**: 根据配置决定是否继续执行议题
- **After hooks 失败**: 记录警告但不影响议题启动状态

用户可在设置中配置 hooks，支持条件触发、错误处理和自定义参数。

### 7. 当前议题 Hooks 执行

**Before 阶段执行**
执行用户配置的前置钩子：

- **Git 安全检查** - 通过查询脚本获取议题状态和 Git 分支信息，确保 Git 工作区分支与议题所属功能分支一致；检查 Git 工作区是否干净，是否有远程更新
- **议题依赖检查** - 通过议题状态脚本检查前置议题完成情况，确保：议题文档存在，前置议题已完成

**Running 阶段执行**
- **执行分析** - 是否符合议题需求/技术方案，对当前执行进行智能分析

**After 阶段执行**
- **议题状态** - 智能理解执行进度，更新议题和功能状态

### 8. 智能体状态判断

- **议题内容理解** - 分析议题目标、实现要点、技术细节
- **当前状态评估** - 理解议题进展和待完成工作
- **策略制定** - 制定最适合当前状态的执行策略

### 9. 状态对应策略执行

根据状态采取对应操作：

**未开始议题 (status: 未开始)**
- 深度分析议题技术实现路径
- 识别潜在风险和技术依赖
- 制定详细执行计划和时间安排
- 创建会话上下文文件 `context/session/<功能名称>/<议题ID>.md`
- 调用 `/dd:issue-update <功能>:<议题ID>` 更新议题状态为 `进行中`，进度设为 1%

**进行中议题 (status: 进行中)**
- 加载会话上下文恢复工作状态
- 分析已完成和未完成的 Todo 项目
- 继续未完成的开发工作
- 调用 `/dd:issue-update <功能>:<议题ID>` 更新 Todo 列表和议题进度
- 保存当前工作状态到会话文件

**已完成议题 (status: 已完成)**
- 显示议题完成概要和成果
- 检查功能中是否有后续议题
- 计算和更新功能整体进度
- 提供下一步操作建议

### 10. 议题状态检测机制

- **文档状态** - 从 `issues/<议题ID>.md` 的 frontmatter 中读取 `status` 字段
- **进度状态** - 检查 `progress` 字段和 todo 列表完成情况
- **上下文状态** - 检查 `context/session/<议题ID>.md` 是否存在，若不存在，则创建文档，作为议题关键信息记录

### 11. 上下文管理

- **会话文件** - `context/session/<议题ID>.md` 保存议题执行上下文
- **进度跟踪** - 记录已完成的 todo 项目和当前工作状态
- **决策历史** - 保存重要的技术决策和原因

### 12. 实时同步和记录

- 实时更新议题文档的状态和进度
- 同步功能整体进度和状态
- 保存重要决策和技术选择到会话文件
- 记录 Todo 项目的完成情况

### 13. 智能完成检测

**自动完成判断**
议题在以下情况下自动标记为已完成：
- 所有 todo 项目都已勾选完成
- 议题进度达到 100%
- 满足议题的验收标准

**完成后自动操作**
1. 调用 `/dd:issue-update <功能>:<议题ID>` 更新议题状态为 `已完成`
2. 自动触发 `/dd:feature-update <功能>` 计算和更新功能整体进度
3. 检查并解锁依赖此议题的其他议题
4. 归档议题上下文到 `session/archive/`
5. 提供下一步操作建议

### 14. 深度思考要点

- **实现方案合理性** - 选择最佳技术路径
- **代码质量保证** - 遵循项目标准
- **功能正确性验证** - 确保功能按预期工作
- **文档同步更新** - 保持文档一致性

## Important Notes

支持 hooks 机制，允许用户自定义开发流程。
通过完整的状态查询脚本获取准确的议题和 Git 状态信息。
智能状态判断确保开发流程的连续性。
支持上下文恢复，确保工作连续性。