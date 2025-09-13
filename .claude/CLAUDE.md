# DD 工作流系统配置

> 仔细思考并实现最简洁的解决方案, 尽可能少地更改代码.

## ⚠️ 绝对规则 - 最高优先级

**必须首先阅读并严格遵守: `.claude/rules/absolute.md`, `.claude/rules/root.md`**

这些规则绝不可违反, 包括:

- Git 操作绝对禁令
- 文件操作边界限制
- 敏感信息处理规范
- 代码质量铁律
- 用户控制原则

## 智能上下文系统

### 绝对遵循的规则

- **absolute.md**: 最高优先级的绝对规则, 任何时候都不可违反
- **root.md**: 根级规则, 覆盖系统核心行为和安全边界

### 自动上下文注入

系统会智能匹配并自动注入以下上下文:

- **项目上下文**: `.claude/context/*` - 项目状态、架构、进度等
- **功能上下文**: 当前操作相关的功能和议题信息
- **会话上下文**: 历史对话和决策记录

**注意**: 这些上下文会根据当前操作自动加载, 无需手动指定.

## 基于智能体的架构

系统使用专门的子智能体进行上下文优化:

- **chat-assistant**: 基于持久上下文的对话
- **code-analyzer**: 深度代码分析、逻辑跟踪、漏洞检测
- **feature-designer**: 功能需求分析和设计方案制定
- **issue-executor**: 议题执行和进度跟踪管理
- **framework-architect**: 系统架构设计和技术选型
- **deep-thinker**: 深度思考和决策分析
- **claude-md-merger**: 智能合并 CLAUDE.md 配置文件

## 智能体使用要求

- **项目上下文**: `/dd:chat` 自动调用 chat-assistant
- **代码分析**: 使用 Task 工具调用 code-analyzer
- **功能设计**: 使用 Task 工具调用 feature-designer
- **议题执行**: 使用 Task 工具调用 issue-executor
- **架构设计**: 使用 Task 工具调用 framework-architect
- **深度思考**: 使用 Task 工具调用 deep-thinker
- **配置合并**: init 完成后使用 Task 工具调用 claude-md-merger

## 工作行为规范

- 保持简洁直接, 避免冗长解释
- 按要求执行议题, 不多不少
- 优先编辑现有文件而不是创建新文件
- 永远不要主动创建文档文件
- 欢迎批评, 保持怀疑态度
- 不要奉承或过度解释, 除非用户要求

## DD 命令系统

所有工作流命令使用 `/dd:` 前缀, 共 22 个命令:

**智能对话类** (1个): `/dd:chat`
**帮助状态类** (3个): `/dd:help` `/dd:status` `/dd:version`
**项目初始化类** (4个): `/dd:init` `/dd:init-local` `/dd:prd` `/dd:ui`
**架构管理类** (4个): `/dd:framework-init` `/dd:framework-audit` `/dd:framework-adjust` `/dd:prd-decompose`
**功能管理类** (7个): `/dd:feature-add` `/dd:feature-decompose` `/dd:feature-start` `/dd:feature-update` `/dd:feature-status` `/dd:feature-refactory` `/dd:feature-remove`
**议题管理类** (2个): `/dd:issue-start` `/dd:issue-update`
**代码质量类** (1个): `/dd:code-reflect`

**总计 22 个命令**

### 典型工作流程

```bash
/dd:init                      # 项目初始化
/dd:init-local               # 本地项目初始化
/dd:prd                      # 需求设计
/dd:framework-init           # 架构设计
/dd:feature-add 用户认证      # 添加功能
/dd:feature-decompose 用户认证 # 功能分解
/dd:feature-start 用户认证    # 开始开发
/dd:code-reflect            # 代码反思
/dd:chat                    # 技术咨询
```

## 工作流文件组织

```
.claude/
├── commands/dd/     # DD 工作流命令
├── agents/          # AI 智能体配置
├── rules/           # 安全和操作规则
├── context/         # 项目上下文和状态
├── chats/           # 对话历史记录
├── features/        # 功能定义和议题
└── scripts/         # 实用工具脚本
```

## 实用工具脚本

系统提供了丰富的实用工具脚本来支持开发工作流:

### 通用信息获取工具

**位置**: `.claude/scripts/dd/utils/info-getter.sh`

**功能**: 通过入参返回系统信息和项目文件内容

**使用方法**:

```bash
# 时间信息
.claude/scripts/dd/utils/info-getter.sh time           # 本地时间 (兼容 mac/linux)
.claude/scripts/dd/utils/info-getter.sh datetime       # 详细日期时间

# 项目文件内容
.claude/scripts/dd/utils/info-getter.sh project        # 项目介绍
.claude/scripts/dd/utils/info-getter.sh architecture   # 架构文件
.claude/scripts/dd/utils/info-getter.sh tech-stack     # 技术栈文件
.claude/scripts/dd/utils/info-getter.sh requirements   # 需求文件

# 灵活文件获取
.claude/scripts/dd/utils/info-getter.sh context <filename>  # 获取指定上下文文件
.claude/scripts/dd/utils/info-getter.sh all-context         # 列出所有可用文件

# 系统信息
.claude/scripts/dd/utils/info-getter.sh system         # 系统基础信息
.claude/scripts/dd/utils/info-getter.sh all           # 完整信息报告
```

### Git 信息查询工具

**位置**: `.claude/scripts/dd/utils/git-info.sh`

**功能**: 提供精简的 Git 状态信息

```bash
.claude/scripts/dd/utils/git-info.sh branch    # 当前分支名
.claude/scripts/dd/utils/git-info.sh status    # 工作区状态
.claude/scripts/dd/utils/git-info.sh clean     # 工作区是否干净
```

### 会话信息获取工具

**位置**: `.claude/scripts/dd/utils/get-session.sh`

**功能**: 获取当前 Claude Code 会话的对话历史和统计信息

## 配置管理

系统使用 YAML 元数据配置工具限制, 规则层次从绝对安全要求到行为指导.
