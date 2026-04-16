---
description: Discover legacy project docs, back them up into .docs.backup, and migrate categorized docs into .docs through the docs workflow.
argument-hint: "[--source <path[,path...]>] [--target <path>] [--backup <path>]"
disable-model-invocation: true
---

# IK Doc Migrate

将项目旧文档迁移到 `.docs` 体系. 这是显式触发的一次性工作流, 不属于日常 docs skill 的常驻职责.

## 调用方式

```bash
/ik-doc-migrate
/ik-doc-migrate --source <path>
/ik-doc-migrate --source <path1,path2,...> --target .docs --backup .docs.backup
```

## 默认路径

- `--target` 默认 `.docs`
- `--backup` 默认 `.docs.backup`
- 未传 `--source` 时, 自动搜索项目中的旧文档

## 工作原则

- 必须先读取并使用 `docs-repo` skill
- 迁移前必须先备份, 不允许直接拿原文档做就地改写
- `.docs.backup` 只做保留, 不自动删除
- 迁移完成后由开发者自行确认是否手动删除 `.docs.backup`
- 不在本命令中重复定义 `.docs` 的通用 schema、字段和 CRUD 规则, 统一复用 `docs-repo`

## 搜索范围

候选:

- `docs/`, `doc/`, `documentation/`
- `design/`, `spec/`, `specs/`, `notes/`
- 项目根目录下明显属于项目设计的 Markdown 文档

排除:

- `.docs/`
- `.docs.backup/`
- `.claude/`, `.claude.template/`
- `node_modules/`, `dist/`, `build/`, `coverage/`, `target/`
- README、LICENSE、CHANGELOG、仓库说明类文档

## 迁移流程

1. **收集来源** - 根据 `--source` 或自动扫描结果, 列出候选旧文档
2. **分类归档** - 先判主职责, 不清楚的直接列入待人工确认
3. **统一备份** - 将本次选中的旧文档按原相对路径复制到 `.docs.backup/` 下的本次快照目录
4. **执行迁移** - 以 `.docs.backup` 中的快照为唯一输入源进行迁移, 不直接读取原文件
5. **结构写入** - 将迁移结果统一写入 `.docs/`
6. **校验结果** - 迁移后执行 `docsctl validate --root .docs`
7. **汇总报告** - 输出迁移范围、分类结果、未迁移文件、风险点和后续建议

## 分类规则

只看主职责, 不看文件名表象, 不为凑类型强行归类.

### 优先级

按以下顺序判断, 一旦命中就停止继续下探:

1. `todo` - 实施拆解、任务跟踪、责任分配、阶段推进
2. `issue` - 问题 / 优化记录、现象、原因、方向
3. `requirement` - 业务目标、流程、边界、验收
4. `architecture` - 模块设计、技术决策、结构约束
5. `testing` - 测试环境、步骤、预期结果

### 混合文档处理

- 主体职责明确, 次要内容很少: 迁主类型
- 多种职责接近: 不自动迁, 直接列入待人工确认
- issue 混有实施步骤时, 也不要直接改判为 todo

## 自动迁移边界

### 可自动迁

- 结构清晰, 主职责单一
- 关键字段可以稳定提取
- 少量缺失可按既有规则安全回填
- 迁移后可通过 `docsctl validate`

### 需要人工确认

- 同一文档混合多种职责, 无法判定主类型
- 字段大量缺失, 只能靠猜测补全
- 明显依赖图片、附件、外链或未收集材料
- 存在复杂交叉引用, 当前信息不足以安全展开

### 禁止自动迁

- README、周报、会议纪要、临时 brainstorming、学习笔记
- changelog、发布记录、纯操作手册
- 仅有零散想法, 没有稳定结构和明确用途的草稿

## 字段回填规则

- 只允许回填已在 `docs-repo` 通用规则中有明确默认策略, 且不会改变真实语义的字段
- 所有回填都必须进入迁移报告
- 不允许猜业务目标、技术决策理由、关联关系或依赖关系

## 迁移验收

- **结构通过** - 按 `docs-repo` 的规则校验通过
- **分类正确** - 每个迁入文档的类型与主职责一致
- **字段可用** - 必填字段齐全, 待补充项可控
- **关联可信** - 没有伪造 issue / todo / dependency 关系
- **备份完整** - 原始输入已保存在 `.docs.backup`
- **人工清单明确** - 待人工确认、未迁移、字段待补充项都有汇总

### 未能明确归类的文档

- 不强行迁移
- 直接列入“待人工确认”

## 输出要求

- 只汇总 source、backup、target, 已迁移 / 未迁移, 回填 / 缺失 / 待确认项
- 必须提醒开发者: `.docs.backup` 需确认无误后再手动删除

## 禁止事项

- 不要直接覆盖旧文档
- 不要跳过 `.docs.backup`
- 不要把 README、仓库说明、临时笔记强行塞进 `.docs`
- 不要在分类不清时擅自决定文档类型
