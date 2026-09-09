# 任务 Hooks 机制实现设计

## 背景与目标

confkit 的步骤失败默认中断 (`continue_on_error` 缺省 false), 任务失败后没有机会再执行任何命令, 而"失败通知"恰恰是最核心的通知场景. 同时 hooks 需要感知任务结果 (`TASK_STATUS`、`FAILED_STEP` 等), 这些值只有任务跑完才确定, 而现有变量体系全部在任务开始前解析.

目标:

- 提供任务边界 (成功 / 失败 / 收尾) 的用户自定义钩子机制
- 注入任务结果变量, 使 hook 命令可消费任务状态与发布产物
- hooks 作为通用机制, 同时服务通知、清理、下游触发、GitOps 交接等场景

### 核心洞察: steps + conditions 已经是 90% 的 hooks

`ConfKitStepConfig` 已有 `condition` 字段, 条件引擎支持 `==` / `!=` / `&&` / `||` 等运算, 环境变量体系现成. hooks 真正缺的只有两样:

1. **保证执行的收尾阶段** — 步骤失败后仍有机会执行命令
2. **结果变量注入** — 任务结束后确定的值进入变量管线

## 方案概览

```yaml
# 项目 YAML
hooks:
  on_success:            # 宿主机执行, 不走 builder 容器
    - name: notify
      commands:
        - 'curl -s -X POST ${SLACK_WEBHOOK} -d "{\"text\": \"${PROJECT_NAME} ${GIT_HASH_SHORT} ok\"}"'
  on_failure:
    - name: alert
      commands: [...]
  on_finally:            # 无论成败都执行
    - name: cleanup
      commands: [...]
```

落地位置: `core/executor/task.rs` 执行循环外包一层 — 捕获状态 → 无论成败执行 hooks 阶段 → 再写 metadata.

## 模块职责

| 模块 | 职责 | 不负责 |
| ---- | ---- | ------ |
| `types/config.rs` | `ConfKitHookConfig` (复用 step 结构) | — |
| `core/executor/task.rs` | hooks 阶段调度, 状态捕获 | — |
| 变量解析管线 | `TASK_STATUS` 等结果变量注入 | — |
| `docs/variables.md` | 结果变量文档 | — |

## 关键结构

Hook 复用 `ConfKitStepConfig` 形态 (name / commands / timeout / condition), 差异点:

- **在宿主机执行**, 不指定 `container`
- **默认 `continue_on_error: true`** — hook 失败不改变任务判定, 日志记一笔即可
- 每个 hook 仍可带 `condition`, 复用现有条件引擎

新增结果变量:

| 变量 | 说明 | 示例 |
| ---- | ---- | ---- |
| `TASK_STATUS` | `success` \| `failure` | `success` |
| `FAILED_STEP` | 首个失败步骤名, 无失败时为空 | `build-frontend` |
| `TASK_DURATION` | 任务耗时 (秒) | `342` |
| `PUBLISHED_IMAGES` | 成功发布的镜像列表 (逗号分隔), 见发布架构 | `my-app:1.0.0-a1b2c3` |

## 关键流程

1. steps 全部执行完毕 (或失败中断), 产生任务状态
2. 注入结果变量到现有环境变量管线 (优先级高于任务前变量)
3. 按 `on_success` / `on_failure` 选择性执行, `on_finally` 无条件执行
4. hook 执行失败只记录日志, 不影响任务判定与退出码
5. 写入 task metadata, 任务结束

## 设计原则与约束

- **hook 点数量克制**: 只有 `on_success` / `on_failure` / `on_finally` 三个, 不做 per-step hooks (与 `condition` + `continue_on_error` 组合语义灾难, 且现有机制可拼出等价效果)
- **webhook URL 等敏感值走环境变量**, 不进 YAML (配置文件会进 git)
- **hooks 在 task 边界, 不在 publish 边界** — publish 只负责注入 `PUBLISHED_IMAGES`, 通知逻辑属于 hooks, 不做"publish 专属 hooks"的概念增殖
- 内置通知 (EventHub `WebhookSubscriber`, 覆盖 Slack / 飞书 / 钉钉卡片) 后置: hooks 上线后用户实际写的 curl 就是内置通知的需求来源

## 取舍与风险

| 主题 | 结论 | 影响 |
| ---- | ---- | ---- |
| 复用 step 结构而非新类型 | 实现最小, 概念不增殖 | 需在解析层约束 `container` 字段不可用 |
| hooks 宿主机执行 | 通知 / git 操作天然属于宿主机域 | 不适合在容器内做的钩子需用户自行 docker run |
| 结果变量在 hook 内解析而非全局 | 变量解析时机集中, 避免两套管线 | steps 内无法引用任务结果 (本来也不存在) |
| 不做 hook 超时默认值 | 复用 step 的 timeout 语义 | 长通知命令需用户显式设置 timeout |
