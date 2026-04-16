# 文档治理

## 先判断是否进入文档治理

任何文档 CRUD 前, 先判断当前操作是否需要遵循 `docs-repo` skill.

优先判断以下情况:

- 目标位于 `.docs/`
- 准备创建、更新、删除需求 / 架构 / todo / issue / testing 文档
- 准备新增或修改 `overview.md`
- 准备调整工作项文档
- 准备处理 issue 与 todo 的联动

命中以上任一情况时:

- 先读取 `docs-repo` skill
- 按 `docs-repo` 的分流规则决定走脚本还是走模板
- 后续约束、字段、状态、命名、结构说明都以 `docs-repo` 为准

## 最小原则

- rules 只负责前置判断和进入 skill
- 不在本规则中重复 `docs-repo` 的正文约束
