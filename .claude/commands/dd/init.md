---
allowed-tools: Task, Read, Write, Edit, Bash
---

# Init

通过深度对话为新项目初始化 DD 工作流系统。

## Usage

```bash
/dd:init
```

**注意**: 现有项目请使用 `/dd:init-local` 命令。

## Instructions

### 1. 收集项目信息

使用 deep-thinker 智能体收集项目基础信息：
- 项目名称、类型、规模、目标用户
- 技术选型倾向、团队技术栈经验

### 2. 确认技术决策

讨论关键技术选择：
- **项目类型**: Web/移动/桌面应用 | 命令行工具 | API 服务 | 工具库
- **开发语言**: 主要语言选择 | 多语言开发需求
- **UI 技术**: 前端技术栈 | 移动端适配需求
- **数据存储**: 数据库类型 | 性能要求
- **部署运维**: 部署环境 | 容器化需求

### 3. 深度讨论和验证

AI 主动提出质疑和建议：
- 技术选型与项目需求匹配度分析
- 团队技术栈熟悉度评估
- 项目可行性和风险识别
- 需求范围边界确认
- 时间和资源合理性评估

### 4. 生成项目内容

基于对话结果，生成个性化的项目上下文内容：
- **项目描述内容** - 基于对话的具体项目信息
- **技术栈详情** - 深入的技术选型分析
- **架构设计** - 个性化的架构方案
- **需求文档** - 基于对话的项目需求分析
- **项目状态** - 当前阶段和下一步规划

### 5. 总结对话要点

使用内置脚本分析当前会话：

```bash
bash .claude/scripts/dd/utils/get-session.sh
```

根据脚本建议，总结关键决策和结论作为 conversation 数据（避免冗长记录）

### 6. 执行 after-init.sh 后处理脚本

```bash
bash .claude/scripts/dd/after-init.sh "$structured_data"
```

**structured_data 数据结构**:

```json
{
  "project_name": "项目名称",
  "project_type": "项目类型",
  "tech_stack": "技术栈",
  "architecture": "架构模式",
  "conversation": "通过 /messages 获取的完整对话历史",
  "project_content": "AI生成的项目描述内容",
  "tech_content": "AI生成的技术栈详情",
  "architecture_content": "AI生成的架构设计",
  "requirements_content": "AI生成的需求文档内容",
  "status_content": "AI生成的项目状态"
}
```

### 7. 合并配置文件

调用 claude-md-merger 智能体，将 `.claude/CLAUDE.md` 合并到根目录 `CLAUDE.md`

## 上下文文档生成格式要求

所有上下文文档必须遵循统一的格式结构：

### .claude/context/project.md 格式
```yaml
---
name: 项目名称
type: 项目类型
status: 规划中
---

# 项目标题
## 项目概述
## 目标用户
## 核心功能
## 技术特点
## 项目规划
```

### .claude/context/tech-stack.md 格式
```yaml
---
# 技术栈规划
## 核心技术栈
## 架构设计原则
## 开发工具链
## 技术选型理由
## 风险评估
```

### .claude/context/architecture.md 格式
```yaml
---
architecture: 架构模式
---

# 项目架构设计
## 总体架构
## 核心模块
## 数据流设计
## 扩展性考虑
## 技术决策记录
```

### .claude/context/requirements.md 格式
```yaml
---
type: 项目类型
---

# 项目需求规划
## 功能需求
## 用户场景
## 非功能需求
## 需求优先级
## 约束条件
## 后续需求规划
```

### .claude/context/current-status.md 格式
```yaml
---
phase: 项目初始化完成
progress: 10%
---

# 当前项目状态
## 项目阶段
## 技术准备
## 下一步计划
## DD工作流状态
```

### 8. 完成提示

显示初始化成功信息和下一步建议：

```
✅ DD 新项目初始化完成！

📁 生成文件:
   • .claude/chats/init/comm-{timestamp}.md - 项目规划对话记录
   • .claude/context/*.md - 项目上下文文件
   • CLAUDE.md - 合并配置文件

🎯 规划成果:
   • 明确的项目目标和技术方向
   • 完整的技术栈选型和架构设计
   • 详细的需求分析和开发计划
   • 系统的项目管理框架

🚀 建议下一步:
   /dd:prd-decompose - 拆解项目功能
   /dd:feature-add   - 添加第一个核心功能
   /dd:chat          - 深入讨论实现方案
   /dd:status        - 查看项目状态
```

## Important Notes

新项目初始化专注于从零开始的系统规划和架构设计。
必须使用 deep-thinker 智能体进行深度项目分析。
基于实际对话生成个性化内容，而不是使用模板。
中英文间距规范：英文单词与中文字符间必须有空格。