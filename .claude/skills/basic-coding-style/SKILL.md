---
name: basic-coding-style
description: Use when implementation or review work needs a compact overview of the repository's coding standards.
user-invocable: false
---

# Basic Coding Style

这是基础编码规范的总览入口. 需要快速了解通用编码风格时, 先看这里, 再按需进入对应技能或规则.

## 核心风格

- 默认交付生产级代码, 不写过渡性脚手架
- 优先清晰、可读、可维护的实现
- 优先遵循项目既有模式, 不为局部新意破坏整体一致性
- 模块、抽象、目录结构以便于开发、维护、排查问题为第一原则

## 必守约束

- 安全红线无例外, 必须严格遵守
- 默认不主动添加兼容垫片、迁移桥接或过渡注释
- 配置值、魔法数字、字符串字面量优先统一管理
- 状态、提示、语义使用明确文本、命名或结构表达, 不用 emoji 承载代码语义

## 实施检查

- 检查相关文件是否需要同步调整
- 检查测试、类型定义、设计文档是否需要同步更新
- 检查命名是否准确表达职责
- 检查结构是否已经出现职责混杂或边界失真

## 细则入口

- 安全和输出约束: `rules/safety-and-output.md`
- 实施规范: `skills/basic-standards/SKILL.md`
- UI 基础准则: `skills/basic-ui/SKILL.md`
