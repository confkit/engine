# 任务 Hooks 机制与结果变量注入

## 背景

confkit 的步骤失败默认中断 (`continue_on_error` 缺省 false), 任务失败后没有机会再执行任何命令, 而"失败通知"恰恰是通知最核心的场景. 同时 hook 需要感知任务结果 (`TASK_STATUS`、`FAILED_STEP` 等), 这些值只有任务跑完才确定, 而现有变量体系全部在任务开始前解析.

另一个背景是发布能力 (见 [[FEAT-publish-and-gitops-handoff-b3e3b1]]): publish 完成后的通知不应做成"publish 专属通知", 而应由任务边界的通用 hooks 机制承载.

## 目标

- 提供任务边界 (成功 / 失败 / 收尾) 的用户自定义钩子机制
- 注入任务结果变量, 使 hook 命令可消费任务状态与发布产物
- hooks 作为通用机制, 同时服务通知、清理、下游触发、GitOps 交接

## 范围

- 项目 YAML 新增 `hooks` 段: `on_success` / `on_failure` / `on_finally`, 每段为 hook 列表
- Hook 复用 step 结构 (name / commands / timeout / condition), 宿主机执行, 默认 `continue_on_error: true`
- 结果变量注入: `TASK_STATUS` / `FAILED_STEP` / `TASK_DURATION` (为 publish 预留 `PUBLISHED_IMAGES`)
- hooks 在 task 边界执行: 捕获状态 → 执行 hooks → 再写 metadata
- 详细设计见 `.docs/architectures/impl-task-hooks.md`

## 非目标

- 不做 per-step hooks — 与 `condition` + `continue_on_error` 组合语义灾难, 现有机制可拼出等价效果
- 不做内置通知平台 (Slack / 飞书 / 钉钉) — 后置, hooks 上线后用户的实际用法是内置通知的需求来源
- 不做 hook 内容器执行 — hooks 天然属于宿主机域 (通知 / git 操作)

## 方案讨论

- 复用 step 结构而非新类型: 实现最小, 概念不增殖; 需在解析层约束 `container` 字段不可用
- 结果变量仅在 hook 执行前注入: 变量解析时机集中, 避免两套管线
- hook 失败不改变任务判定: 通知挂了不能反过来把构建判失败, 日志记一笔即可

## 风险与边界

- webhook URL 等敏感值必须走环境变量, 不进 YAML (配置文件会进 git)
- hook 命令在宿主机执行, 权限等同于 confkit 进程本身, 文档需提示
- 结果变量与用户自定义变量同名时的优先级需明确 (结果变量优先)

## 验收口径

- 声明 `on_failure` 的项目在步骤失败后 hook 仍被执行, 任务判定为失败
- `on_finally` 无论成败均执行; hook 自身失败不影响任务判定与退出码
- hook 命令内 `${TASK_STATUS}` / `${FAILED_STEP}` / `${TASK_DURATION}` 替换正确
- 带 `condition` 的 hook 按条件跳过
- 未声明 hooks 的项目行为与现状完全一致 (零回归)
