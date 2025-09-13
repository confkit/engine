---
allowed-tools: Task, Read, Bash
---

# Feature Start

智能判断功能状态，采取对应操作策略，集成 hooks 机制，开始或继续功能开发。

## Usage

```bash
/dd:feature-start <功能名称>
/dd:feature-start <功能名称> --temp  # 临时需求
```

Options:

- `--temp` - 临时需求，使用特殊处理流程

## Instructions

### 1. 智能状态判断

自动检测功能当前状态，采取相应策略：

- **未开始** → 开始功能开发
- **开发中** → 继续开发进程
- **已完成** → 询问是否重新开发

### 2. 状态查询脚本使用

命令通过 hooks 机制配合查询脚本获取状态信息，常用脚本：

```bash
# 获取功能状态 - 所有用法选项：
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
bash .claude/scripts/dd/utils/git-info.sh all                # 所有信息
```

### 3. Hooks 机制配置

此命令支持用户自定义的 hooks 配置，在执行过程中的关键阶段会触发相应的钩子。

**Before 阶段 - 功能开发执行前**

- 触发时机: 在功能开发启动之前
- 用途: 状态读取、环境检查、权限验证、前置条件确认
- 示例: 读取功能状态、检查开发分支、验证权限、确认依赖功能状态

**Running 阶段 - 命令执行过程中**

- 触发时机: 在状态判断和策略制定过程中
- 用途: 执行过程监控、状态验证、中间结果处理
- 示例: 记录执行日志、发送状态通知、执行自定义检查

**After 阶段 - 命令执行完成后**

- 触发时机: 在功能开发启动完成后
- 用途: 后续处理、通知发送、状态同步
- 示例: 发送开始通知、更新项目看板、触发 CI/CD 流程

### 4. Hooks 错误处理

如果 hooks 返回错误或阻塞信号，命令执行将：

- **Before hooks 失败**: 停止命令执行，显示错误信息
- **Running hooks 失败**: 根据配置决定是否继续执行
- **After hooks 失败**: 记录警告但不影响命令完成状态

用户可在设置中配置 hooks，包括脚本路径、执行条件、错误处理策略等。

### 5. 当前功能开发过程 Hooks

**Before 阶段执行**
执行用户配置的前置钩子：

- **Git 安全检查** - 通过查询脚本获取功能状态和 Git 分支信息，确保 Git 工作区分支与功能分支一致；检查 Git 工作区是否干净，是否有远程更新
- **检查功能文档** - 确保 overview、technical、acceptance 文档存在，否则拒绝继续执行
- **议题拆解** - 确保功能议题已完成拆解，否则拒绝继续执行

**Running 阶段执行**

- **改动分析** - 是否符合设计/需求，对后续改动进行智能分析

**After 阶段执行**
暂无

### 6. 智能体状态判断

- **功能内容理解** - 分析功能目标、技术方案、当前进度
- **当前状态评估** - 理解功能开发进展和议题分布
- **策略制定** - 制定最适合当前状态的执行策略

### 7. 状态对应策略

| 功能状态 | 操作策略 | 描述                           |
| -------- | -------- | ------------------------------ |
| `未开始` | 开始开发 | 初始化开发环境，开始第一个议题 |
| `开发中` | 继续开发 | 继续当前进行中的议题           |
| `已完成` | 询问重开 | 确认是否重新开发               |

### 8. 智能功能状态更新

通过命令 `/dd:feature-update <功能名称>` 同步功能状态

### 9. 智能议题执行

- 自动查找第一个 进行中/未开始 的议题
- 调用 `/dd:issue-start <功能名称>:<议题ID>` 命令开始议题

### 10. 深度思考维度

- **开发准备度**: 功能是否已完成足够的前期设计
- **资源冲突**: 当前开发议题是否与其他工作冲突
- **技术准备**: 开发环境和技术栈是否就绪
- **优先级合理性**: 是否是当前最应该开发的功能

### 11. 输出规范

- 功能开发状态报告
- Git 安全检查结果
- 当前执行功能开发信息
- 下一步操作建议

## Important Notes

支持 hooks 机制，允许用户自定义开发流程。
通过完整的状态查询脚本获取准确的功能和 Git 状态信息。
智能状态判断确保开发流程的连续性。
临时需求使用 --temp 标志进行特殊处理。
