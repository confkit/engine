---
allowed-tools: Task
---

# Chat

注入项目持久上下文，提供项目感知的智能对话。

## Usage

```bash
/dd:chat
```

## Instructions

### 1. 智能体调用

调用 chat-assistant 智能体执行上下文加载：

Run `bash .claude/scripts/dd/chat.sh` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.  
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.

### 2. 上下文注入内容

自动注入项目完整上下文信息：

- **项目架构** - 系统架构和技术栈信息
- **功能进度** - 当前功能开发状态和进度
- **议题状态** - 各议题的执行情况和依赖关系
- **历史决策** - 重要技术决策和变更记录
- **开发规范** - 项目编码标准和最佳实践

### 3. 智能对话增强

基于完整项目上下文提供：

- **针对性建议** - 基于项目实际情况的解决方案
- **技术约束考虑** - 考虑现有技术栈和依赖关系
- **规范符合性** - 确保建议符合项目开发规范

### 4. 对话体验优化

- **自然对话** - 保持流畅的对话体验
- **项目感知** - 理解项目背景和上下文
- **精准回答** - 提供符合项目实际的具体建议

## Important Notes

提供项目感知的智能对话，相比无上下文的通用建议更加精准和实用。
