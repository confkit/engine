---
name: docs-repo
description: Use when working with .docs. Route todo/issue through docsctl, and write requirement/architecture/testing from the matching template reference.
user-invocable: false
---

# Repository Docs

创建或调整 `.docs` 文档时, 先判断文档类型, 再决定走脚本还是走模板.

## 标准目录

- `.docs/requirements/`
- `.docs/architectures/`
- `.docs/todo/`
- `.docs/testing/`
- `.docs/issues/bugs/`
- `.docs/issues/optimizations/`

## 分流规则

### 流程型文档

只包括:

- `todo`
- `issue`

要求:

- 查询、创建、更新、关闭、删除、校验统一走 `node scripts/docsctl.mjs`
- 不手工维护路径、命名、ID、分桶、关联关系
- 不直接编辑 `todo / issue` Markdown; 凡是流程型文档变更, 都先走 `docsctl`
- `todo / issue` 不使用 `overview.md`
- `issue` 进入处理前先展开关联 `todo`

最小用法:

```bash
node scripts/docsctl.mjs todo list --status doing
node scripts/docsctl.mjs todo show --id TODO-xxx
node scripts/docsctl.mjs todo batch-create --file todo.json
node scripts/docsctl.mjs todo batch-append --doc-id TODO-DOC-xxx --file append.json
node scripts/docsctl.mjs issue batch-plan --id BUG-xxx --file plan.json
node scripts/docsctl.mjs validate --entity all
```

常用命令:

| 对象 | 命令 | 用途 |
| --- | --- | --- |
| `todo` | `list` | 查询条目 |
| `todo` | `show --id` | 查看条目或文档详情 |
| `todo` | `batch-create --file` | 一次生成多分组、多条目文档 |
| `todo` | `batch-append --doc-id --file` | 向已有 `TODO-DOC-...` 追加条目 |
| `issue` | `batch-create --file` | 一次创建多个 issue |
| `issue` | `batch-plan --id --file` | 为 issue 生成完整 todo 拆解 |
| `issue` | `batch-update / batch-close / batch-delete --file` | 批量维护 issue |
| `validate` | `validate --entity all` | 校验 `.docs` 结构与引用 |

关键参数:

| 参数 | 适用命令 | 说明 |
| --- | --- | --- |
| `--id` | `show / issue batch-plan / update / close / delete` | 按稳定 ID 操作 |
| `--doc-id` | `todo batch-append` | 指定目标 `TODO-DOC-...` |
| `--file` / `--stdin` | 所有 `batch-*` | 读取 JSON manifest |
| `--dry-run` | 所有 `batch-*` | 预览结果, 不写文件 |
| `--json` | 查询、结果输出 | 供 agent / 脚本消费 |
| `--root` | 全部 | 指定 `.docs` 根目录 |
| `--owner` | `todo` 相关 create / append / update / plan | 项目责任域标识; 使用稳定大写 code, 例如 `FE`, `BE`, `APP`, `API`, `INFRA`, `CORE` |
| `--status` / `--tag` / `--text` / `--limit` | `list` | 用于筛选查询结果 |
| `--slug` | `create / batch-create` | 英文 slug; 标题含中文时应显式传入 |

顺序控制:

| 参数 | 适用命令 | 说明 |
| --- | --- | --- |
| `--position <start|end>` | `todo append / batch-append` | 粗粒度放到组首或组尾 |
| `--before <TODO-...>` / `--after <TODO-...>` | `todo append / batch-append` | 相对已有条目精确插入 |
| `--before-group <title>` / `--after-group <title>` | `todo batch-append` | 新分组编排位置 |

细节来源:

- `node scripts/docsctl.mjs help`

字段约束:

### todo

强约束:

- `meta.type` 必须是 `todo`
- `meta.id` 必须匹配 `TODO-DOC-<slug>-<hash6>`
- `meta.status` 是聚合字段, 必须与条目状态汇总一致; 不要手填自定义值
- 文档至少要有一个 `##` 分组
- 验证区标题只能是 `阶段性验证` 或 `验收用例`

| 字段 | 类型 / 允许值 | 说明 |
| --- | --- | --- |
| `owner` | `string` | 必填, 项目责任域标识; 使用稳定大写 code, 例如 `FE`, `BE`, `APP`, `API`, `INFRA`, `CORE` |
| `status` | `todo` \| `doing` \| `blocked` \| `done` \| `dropped` | 必填, 只允许这 5 个值 |
| `tags` | `string[]` | 必填, 不能为空 |
| `paths` | `string[]` | 可选, 路径列表 |
| `source` | `string` | 必填, 上游文档 / issue ID, 无则写 `none` |
| `depends-on` | `string` | 必填, 依赖条目 ID, 无则写 `none` |
| `acceptance` | `string` | 必填, 不能为空 |

- `owner` 保持项目内稳定, 推荐使用大写字母开头, 并只使用 `A-Z`、`0-9`、`-`

### issue

强约束:

- `meta.type` 必须是 `issue`
- `meta.id` 必须匹配 `ISSUE-DOC-<slug>-<hash6>`
- `meta.kind` 只能是 `bug` 或 `optimization`
- `meta.status` 是聚合字段, 必须与条目状态汇总一致; 不要手填自定义值
- bucket 文档中的条目 `kind` 必须与文档 `meta.kind` 一致

| 字段 | 类型 / 允许值 | 说明 |
| --- | --- | --- |
| `kind` | `bug` \| `optimization` | 必填 |
| `status` | `open` \| `planned` \| `doing` \| `closed` \| `dropped` | 必填, 只允许这 5 个值 |
| `priority` | `low` \| `medium` \| `high` \| `critical` | 必填 |
| `tags` | `string[]` | 必填, 不能为空 |
| `todo` | `TODO-...` \| `TODO-DOC-...` \| `none` | 必填, 关联 todo 条目或文档 |
| `close-reason` | `string` | 关闭 / 丢弃时使用; 无理由时默认 `none` |
| `summary` | `string` | 必填, 问题现象 / 优化摘要 |
| `direction` | `string` | 必填, 处理方向 |
| `scope` | `string` | 必填, 影响范围 |
| `cause` | `string` | 可选, 原因分析 |

状态联动:

- `issue.status` 为 `planned` / `doing` / `closed` 时, `todo` 不能是 `none`
- `issue.status` 为 `closed` / `dropped` 且关联 todo 未完成时, `close-reason` 不能是 `none`

### 模板型文档

只包括:

- `requirement`
- `architecture`
- `testing`

要求:

- 不走脚本 CRUD
- 按需读取对应 `references/*.md`, 然后直接编写
- 每类目录保留一个简洁的 `overview.md`
- 非 `overview.md` 文件一个文件只承载一个主题
- 文件名使用英文 slug

对应 reference:

- `requirements` -> `references/requirements.md`
- `architectures` -> `references/architectures.md`
- `testing` -> `references/testing.md`

最小用法:

- 先读取对应 `references/*.md`
- 再读取目标文档或 `overview.md`
- 按当前类型的模板结构直接编写或更新

## 通用约束

- 不一次性加载全部 reference, 只读取当前文档类型需要的模板
- 子文档只负责单一主题, 不混合需求、架构、任务、测试、issues
- `issues` 单文件最多记录 50 条, 超过继续分桶
- slug 只允许 `a-z`、`0-9`、`-`
- 长内容:
  - `todo / issue` 使用固定字段或块字段
  - `requirement / architecture / testing` 优先拆节、表格、流程步骤

## 这个 skill 负责什么

- 判断文档走脚本型还是模板型路径
- 为模板型文档选择正确 reference
- 为流程型文档调用 `docsctl`

## 这个 skill 不负责什么

- 不绕过 `docsctl` 手工维护 `todo / issue`
- 不把 `requirement / architecture / testing` 写成字段卡片
- 不一次性引入所有文档模板到上下文

## CLAUDE.md 同步

项目结构性变更时, 必须同步更新 `CLAUDE.md`:

- 新增、删除、移动重要目录或模块
- 变更项目架构或技术栈
- 修改构建、部署流程
- 新增或变更核心依赖

## 最小化原则

- 不创建不必要的文档
- 不自动生成未经请求的文档
- 新增文档前先核对现有结构

未经明确请求, 禁止创建:

- `temp.md`
- `draft.md`
- `notes.md`
- `refactor.md`
- `migration.md`
- `changelog.md`
