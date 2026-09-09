# 镜像发布能力与 GitOps 交接

## 背景

confkit 当前是配置驱动的任务执行器, 构建完成后工具职责即结束. 用户的部署习惯是以 git 仓库管理 compose file / k8s manifest (本身即 GitOps 模式), 缺的是 CI 与 GitOps 之间的交接环节: 镜像进入 registry、部署仓库引用新 tag, 这两步目前是手工操作.

方向决策: **做"发布" (publish), 不做"部署" (deploy)**. CD 的核心难题是状态管理 (期望态 vs 实际态、漂移、回滚), 那是 ArgoCD / Flux 的领域; confkit 若管理集群状态等于重造更差的 ArgoCD. git 仓库继续作为部署唯一事实来源, 回滚即 git revert.

## 目标

- 补齐镜像发布能力 (tag / push), 使 confkit 产出的镜像可被任意 registry 消费
- 让"更新部署仓库中的 image tag → commit → push"可声明为构建流程的一部分 (配合 hooks, 见 [[FEAT-task-hooks-and-result-vars-1f4d13]])
- k8s 支持作为自然结果获得 (registry + git manifest 通用接口), 而非新增负担

## 范围

- engine 层新增 `tag_image` / `push_image` 抽象 (docker / podman 同 trait)
- `.confkit.yml` 新增 `registries` 配置 (name + url, 凭证不进配置)
- 项目 YAML 新增 `publish` 段: images 列表, tag 复用现有变量体系 (`${PROJECT_VERSION}`、`${GIT_HASH_SHORT}`)
- 任务流程新增 publish 阶段: steps 完成后执行, 成功后注入 `PUBLISHED_IMAGES` 变量
- `infra/git.rs` 补充 commit / push 能力用于部署仓库交接
- 详细设计见 `.docs/architectures/arch-publish-and-gitops.md`

## 非目标

- 不做部署状态管理 / 部署数据库 — git 仓库就是状态
- 不做密钥存储 — registry 凭证、git push token 走环境变量或引擎 credential helper
- 不做集群内运行 / server 化 — 保持单机 CLI 定位
- 不做多架构构建 (buildx) — 远期按需, 仅当 k8s 节点为 arm 时才有意义

## 方案讨论

- publish 复用 `confkit run` 流程而非独立命令: 声明驱动, 交互一致; 后续如需 `confkit publish` 单命令再演进
- publish 阶段失败即任务失败, `PUBLISHED_IMAGES` 只含成功项: hooks 通过 `TASK_STATUS` 条件区分处理
- hooks 在 task 边界而非 publish 边界: 通知逻辑属于 hooks, 不做"publish 专属 hooks"的概念增殖

## 风险与边界

- registry 凭证依赖引擎登录态 (docker login / podman login), 需在文档明确前置条件
- 部署仓库 push 需要写权限 token, 泄漏影响面大, 必须走环境变量
- tag 变量解析失败时的行为需明确 (跳过该镜像并告警, 而非发布空 tag)

## 验收口径

- 声明 `publish` 的任务在 steps 成功后完成 tag + push, 远端 registry 可拉取到该镜像
- `PUBLISHED_IMAGES` 在 hooks 中可被 `${PUBLISHED_IMAGES}` 正确替换
- 未声明 `publish` 的项目行为与现状完全一致 (零回归)
- docker 与 podman 两种引擎下发布行为一致
- 部署仓库交接示例: hooks 中更新 manifest 的 image tag 并 push 成功
