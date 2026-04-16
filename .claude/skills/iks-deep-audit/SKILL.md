---
name: iks-deep-audit
description: Deep-audit a project by language with optional scope, diff, and risk-focus filters. Use only when manually invoking /iks-deep-audit.
disable-model-invocation: true
argument-hint: "[--rust | --typescript | --android | --go] [--scope <path>] [--diff] [--focus <risk>]"
---

# Deep Audit

用于从资深工程视角, 对项目做整体性深度审查.

## 调用方式

```bash
/iks-deep-audit --rust
/iks-deep-audit --typescript --scope src/features/order
/iks-deep-audit --android --diff
/iks-deep-audit --go --scope internal/payment --diff
/iks-deep-audit --rust --focus flow
/iks-deep-audit --typescript --scope src/editor --diff --focus state
```

## 参数规则

- 语言参数必选且只能选一个: `--rust` / `--typescript` / `--android` / `--go`
- `--scope <path>` 可选, 用于限制到目录、模块或单文件
- `--diff` 可选, 用于只检查当前未提交变更
- `--focus <risk>` 可选, 用于优先聚焦某一类风险
- 无 `--scope`、无 `--diff` 时, 对该语言的全项目代码做整体深度审查
- 同时提供 `--scope` 和 `--diff` 时, 只检查该范围内的未提交变更

## focus 语义

- 可选值: `crash` / `failure` / `state` / `flow` / `resource` / `structure`
- `--focus` 会提高该类风险的检查密度和输出优先级
- 即使指定 `--focus`, 仍必须报告范围内发现的跨类别 `Critical` 问题
- 未指定 `--focus` 时, 按完整风险模型做平衡式深审

## diff 语义

- `--diff` 默认覆盖 staged + unstaged + untracked
- 审查目标是当前工作区风险, 不只看已暂存内容
- 如果工作区没有相关变更, 明确告知开发者并停止深审

## 工作流程

1. 解析语言参数, 拒绝多语言混用
2. 解析 `--scope`、`--diff`、`--focus`, 计算最终审查范围和重点
3. 先读取 `references/core.md`
4. 再读取 `references/output-template.md`
5. 再读取对应语言 reference:
   - `references/rust.md`
   - `references/typescript.md`
   - `references/android.md`
   - `references/go.md`
6. 必要时补读对应 `rules/lang/*.md`, 用于对齐编码期预防规则
7. 如果存在 `--focus`, 先沿对应风险类别加深检查, 再补全跨类别高风险问题
8. 输出按严重度排序的 findings, 不默认进入修改

## 输出要求

- 先给 `Critical`, 再给 `High`, 最后给 `Medium`
- 每项问题说明位置、原因、触发条件、影响范围
- 优先指出崩溃、不可接受异常、主链路阻塞和结构失衡问题
- `--focus` 模式下, 先给该类别的高价值问题, 但不能漏报跨类别 `Critical`
- findings 结构尽量复用统一模板, 保持不同语言输出风格一致
- 结尾补充整体结论和优先处理顺序
- 默认只做审查和建议, 不直接修改代码
