# 发布能力与 GitOps 交接架构

## 背景与目标

ConfKit 当前是配置驱动的任务执行器: YAML 定义 steps → 条件求值 → 在 builder 容器内执行 → 产出 artifacts. 构建完成后工具职责即结束, 没有镜像发布与部署交接的概念.

用户的部署习惯是以 git 仓库管理 compose file / k8s manifest, 这本身就是 GitOps 模式. 缺的不是 CD 系统, 而是 CI 与 GitOps 之间的交接环节 (handoff): 镜像进入 registry、部署仓库引用新 tag 这两步目前是手工操作.

目标:

- 补齐镜像发布能力 (push / tag), 使 confkit 产出的镜像可被任意 registry 消费
- 通过任务 hooks 与结果变量, 让"更新部署仓库中的 image tag"可被声明为构建流程的一部分
- k8s 支持作为自然结果获得, 而不是新增的负担

### 核心判断: CI 是执行器问题, CD 是状态机问题

CI 是"给定输入 → 确定性执行 → 产出工件", 无状态、幂等, 与现有 Task / Step / Condition 架构完全匹配. CD 的核心难题是状态管理 (什么版本部署在哪、期望态 vs 实际态、漂移检测、回滚), 这是 ArgoCD / Flux / Helm 解决的问题. confkit 若直接管理集群状态, 是在重造一个更差的 ArgoCD, 且单机 task 执行模型不适合演进为多目标状态协调器.

结论: **做"发布" (publish), 不做"部署" (deploy). git 仓库继续作为部署的唯一事实来源, 回滚即 git revert.**

## 方案概览

分三个阶段, 每阶段独立可用:

1. **镜像发布能力**: engine 层补齐 `tag_image` / `push_image`; `.confkit.yml` 增加 registries 配置
2. **项目级发布声明**: `ConfKitProjectConfig` 增加 `publish` 段, tag 复用现有变量体系 (`PROJECT_VERSION`、`GIT_HASH_SHORT`)
3. **GitOps 交接自动化**: `infra/git.rs` 补 commit / push 能力, 配合 hooks 更新部署仓库中的 image tag

## 模块职责

| 模块 | 职责 | 不负责 |
| ---- | ---- | ------ |
| `engine/` | 镜像 tag / push 的引擎抽象 (docker / podman 同 trait) | 凭证管理 (交给引擎 credential helper) |
| `types/config.rs` | registries / publish 配置模型 | 凭证字段 (绝不落盘到配置) |
| `infra/git.rs` | 部署仓库的 clone / pull / commit / push | 部署状态记录 |
| `core/executor/` | publish 阶段执行, 完成后注入 `PUBLISHED_IMAGES` 变量 | 通知 (由 hooks 承载) |
| 部署仓库 (外部) | 部署唯一事实来源, 期望态声明 | — |

## 关键结构

配置模型变更 (示意):

```yaml
# .confkit.yml
registries:
  - name: ghcr
    url: ghcr.io/myorg
    # 凭证走环境变量或 docker login, 不进配置文件

# 项目 YAML
publish:
  images:
    - name: my-app
      tag: "${PROJECT_VERSION}-${GIT_HASH_SHORT}"
      registry: ghcr
```

## 关键流程

1. 任务执行 steps 完成后进入 publish 阶段 (若声明了 `publish`)
2. 对每个 image: 解析 tag 变量 → `tag_image` → `push_image` → 校验远端存在
3. 汇总成功发布的镜像列表, 注入 `PUBLISHED_IMAGES="app:1.0.0-a1b2c3,..."` 环境变量
4. hooks 阶段 (见 [impl-task-hooks.md](impl-task-hooks.md)) 消费该变量, 可执行"更新部署仓库 image tag → commit → push"
5. ArgoCD / Flux / compose pull 感知部署仓库变更, 完成实际部署

## 设计原则与约束

- git 仓库是部署的唯一事实来源, confkit 不持久化任何部署状态 (不建部署状态数据库)
- 凭证 (registry、git push token) 全部走环境变量或引擎 credential helper, 不进 YAML
- confkit 保持单机 CLI 定位, 不做集群内运行, 不做需要维护的 server
- k8s 支持只通过"registry + git manifest"通用接口间接获得

## 取舍与风险

| 主题 | 结论 | 影响 |
| ---- | ---- | ---- |
| 不做部署状态管理 | 避免滑向自制 ArgoCD | 部署观测依赖 ArgoCD / Flux 自身能力 |
| 不做内置通知 | hooks 作为通用机制先行, 内置通知 (EventHub subscriber) 后置 | 初期需手写 curl, 但内置通知的需求来源更真实 |
| publish 无独立命令 | 复用 `confkit run` 流程, 以声明驱动 | 后续如需 `confkit publish` 单命令再演进 |
| 部分发布失败 | publish 阶段失败即任务失败, `PUBLISHED_IMAGES` 只含成功项 | 需在 hooks 中通过 `TASK_STATUS` 条件区分处理 |
| 多架构构建 (buildx) | 远期可选, 仅当 k8s 节点为 arm 时才需要 | 首期仅单架构 |

## 分阶段落地

| 阶段 | 内容 | 依赖 |
| ---- | ---- | ---- |
| Phase 1 | engine 层 `tag_image` / `push_image`, registries 配置 | 无 |
| Phase 2 | 项目 `publish` 段, `PUBLISHED_IMAGES` 变量注入 | Phase 1 |
| Phase 3 | git commit / push 能力 + hooks 交接部署仓库 | Phase 2, hooks 机制 |
| Phase 4 (可选) | 薄 `deploy` (无状态 shell out `kubectl apply` / `compose up`), CI 触发 (webhook / 轮询), buildx 多架构 | 按需决策 |
