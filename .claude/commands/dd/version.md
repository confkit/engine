---
allowed-tools: Read
---

# Version

显示 CCDD Helper 系统的版本信息和组件状态。

## Usage

```bash
/dd:version
```

## Instructions

### 1. 功能概述

显示：

- 系统版本号
- 组件完整性检查
- 配置文件状态
- 更新日志摘要

### 2. 执行方式

读取配置文件并显示：

```
📚 CCDD Helper - Claude Code 深度开发工作流系统
版本: 2.0.0
发布日期: 2024-01-15
构建: stable

🔧 组件状态:
  ✅ 智能体配置完整 (5个)
  ✅ 规则系统完整 (5个)
  ✅ 命令系统完整 (19个)
  ✅ 脚本系统完整 (19个)

🎯 当前项目状态:
  项目名称: {从project.md读取}
  初始化状态: {已初始化/未初始化}
  功能数量: {X个}
  活跃议题: {Y个}
```

### 3. 系统健康检查

验证关键文件存在：

- `.claude/CLAUDE.md`
- `.claude/rules/absolute.md`
- `.claude/context/` 目录结构
- 必需的命令和脚本文件

## Important Notes

提供 DD 工作流系统版本信息和完整性检查。
