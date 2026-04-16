---
name: basic-refinement
description: Use when a task is structurally complex or implementation gets blocked. Pause coding, decompose the design, and update design docs and todos before continuing.
user-invocable: false
---

# Basic Refinement

项目编码过程中, 如果变更本身不简单, 或实现时遇到阻塞点, 说明这是一个需要先做设计拆解的任务.

## 工作流程

1. **判断复杂度** - 先区分是简单修改还是非简单变更
2. **分析范围** - 明确影响模块、接口、状态和边界条件
3. **暂停直写** - 非简单变更先进入设计阶段, 不直接堆实现
4. **拆解设计** - 创建或更新设计文档, 明确模块职责、状态流转、边界条件
5. **拆解任务** - 创建或更新 todo 文档, 将复杂功能拆分为可验证的小步骤
6. **确认后实施** - 设计和任务边界清楚后再继续实现

## 适用边界

- 错别字修复、小调整等简单变更可直接进行
- 涉及多模块协作、状态重组、接口调整的任务先走设计拆解

## 触发信号

当出现以下情况时, 触发此技能:

- 实现逻辑需要大量 if/else 分支
- 状态管理变得混乱
- 边界条件难以穷举
- 单个函数/方法超过 50 行
- 需要频繁修改已有代码来适配新功能
