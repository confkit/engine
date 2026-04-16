# Todo Template

## 最小结构

```md
# <todo-doc-title>

type: todo
id: TODO-DOC-<topic>-<hash6>
title: <short-title>
status: todo | doing | blocked | done | dropped
tags: <tag-a>, <tag-b>
source: <issue-id> | none
scope: feature | fix | refactor

## <phase-or-module>

### TODO-<topic>-<hash6> [OWNER] <short-item-title>
- status: todo | doing | blocked | done | dropped
- tags: <tag-a>, <tag-b>
- paths: <path-a>, <path-b>
- source: <issue-id> | none
- depends-on: <todo-id> | none
- acceptance: <如何验证完成>

## 阶段性验证
- 覆盖主路径验证
- 覆盖关键异常路径
```

## 约束

- todo 文档固定使用 2 级拆分: `##` 分组, `###` 任务项
- 不把零碎任务直接写在文档第一层
- 每个任务项必须有稳定 `ID`, 便于脚本检索和引用
- 任务项标题固定使用 `### ID [责任域] 标题`
- 责任域使用稳定大写标识, 例如 `[FE]`、`[BE]`、`[APP]`、`[API]`、`[INFRA]`、`[CORE]`
- 详情字段统一使用 `status`、`tags`、`paths`、`source`、`depends-on`、`acceptance`
- `status` 优先写 `todo | doing | blocked | done | dropped`
- 文档末尾必须有阶段性验证或验收用例
- 字段内容较长时, 使用 `- <field>: |` 加缩进块
- 验证区保持轻量 bullet list, 不再单独做条目级 CRUD

## 搜索友好性

- 一级分组只承担阶段 / 模块语义, 不承载大段说明
- 适合列表筛选的字段放在固定字段中, 不埋进长段落
- 同一任务只保留一个主 `ID`, 不写多个别名
- `tags` 和 `paths` 使用逗号分隔, 不混用不同分隔符
