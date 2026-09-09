# 架构文档索引

## 前缀清单

| 前缀 | 层级 | 说明 |
| ---- | ---- | ---- |
| `arch-` | 架构层 | 分层、模块边界、依赖方向、总体拆分 |
| `impl-` | 实现细节 | 单模块内的机制设计、状态、数据结构 |
| `infra-` | 基础设施 | 部署、构建、CI / CD、环境拓扑 |

## 文档索引

| 文档 | 主题 | 状态 |
| ---- | ---- | ---- |
| [arch-publish-and-gitops.md](arch-publish-and-gitops.md) | 发布能力与 GitOps 交接的总体架构 | 设计中 |
| [impl-task-hooks.md](impl-task-hooks.md) | 任务 hooks 机制实现设计 | 设计中 |

## 阅读顺序

1. `arch-publish-and-gitops.md` — 先理解发布层的定位与边界（做"发布"，不做"部署"）
2. `impl-task-hooks.md` — 再看 hooks 如何作为任务边界的通用机制落地
