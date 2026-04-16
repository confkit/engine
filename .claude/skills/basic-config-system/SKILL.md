---
name: basic-config-system
description: Use when designing project-level configuration, environment variables, or runtime overrides. Enforce layered config, secret boundaries, and a stable merge priority across defaults, config files, env vars, and CLI args.
user-invocable: false
---

# Basic Config System

设计全局配置、环境变量和运行时覆盖机制时, 使用稳定分层模型, 不随意混放.

## 配置分层

- `default` - 代码内默认值, 负责兜底
- `config file` - 持久化、可读、可编辑的非敏感配置
- `env` - 敏感信息、环境差异、运行时注入
- `cli args` - 单次执行覆盖

## 强制规则

- 配置优先级固定为 `CLI > ENV > CONFIG > DEFAULT`, 且必须可预期
- 敏感信息只走 `env` 或 secret manager, 不写入 config file
- 稳定的非敏感偏好写入 config file, 不强迫用户每次通过 env 注入
- 需要长期配置且允许临时切换的字段, 同时支持 `config + env`
- 单次执行才有意义的覆盖项, 放 `cli args`, 不反向污染全局配置
- 同一语义只保留一套主字段模型, 不要在多层引入互相冲突的别名
- 项目已有配置系统时, 优先沿用现有结构补齐分层, 不新开第二套

## 设计要求

- merge 逻辑集中实现, 不在业务代码各处手写覆盖链
- 字段归类先做安全边界判断, 再决定落在哪一层
- config key、env key、cli flag 的映射关系必须稳定且可文档化
- 新增全局配置前, 先按需读取 `references/layering.md`
