# Issues Template

## 目录结构

```text
.docs/issues/
├── bugs/
│   ├── bugs-a.md
│   └── bugs-b.md
└── optimizations/
    ├── optimizations-a.md
    └── optimizations-b.md
```

## 单条最小结构

```md
# <issue-doc-title>

type: issue
id: ISSUE-DOC-<topic>-<hash6>
title: <short-title>
kind: bug | optimization
status: open | planned | doing | closed | dropped
tags: <tag-a>, <tag-b>

## BUG-<topic>-<hash6> <short-item-title>
- kind: bug
- status: open | planned | doing | closed | dropped
- priority: low | medium | high | critical
- tags: <tag-a>, <tag-b>
- todo: <todo-id> | none
- close-reason: <reason> | none
- 现象 / 问题: <summary>
- 原因: <cause>
- 处理方向: <direction>
- 影响范围: <scope>
```

## 约束

- bugs 与 optimizations 分目录沉淀
- 不使用 `overview.md`, 统一依赖稳定 `ID`、固定字段和 `docsctl`
- 单文件最多 50 条, 超过时新建下一卷
- 分卷文件只做存储分桶, 唯一识别统一依赖 `ID`
- 每条 issue 必须有稳定 `ID`, bug 使用 `BUG-<topic>-<hash6>`, optimization 使用 `OPT-<topic>-<hash6>`
- issue 不直接承载实施, 进入处理前必须先生成关联 `todo`
- 详情字段统一使用 `kind`、`status`、`priority`、`tags`、`todo`、`close-reason`、`现象 / 问题`、`原因`、`处理方向`、`影响范围`
- `status` 优先写 `open | planned | doing | closed | dropped`
- `tags` 使用逗号分隔, 不混用不同分隔符
- 字段内容较长时, 使用 `- <field>: |` 加缩进块

## 搜索友好性

- 标题只承载主题, 不把状态和优先级混进标题
- 适合筛选的属性放在固定字段中, 不埋进正文段落
- 同一 issue 只保留一个主 `ID`, 不写多个别名
