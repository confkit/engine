# Engine 容器化运行与宿主机路径翻译

## 背景

当前 CI engine 直接跑在宿主机上. 希望支持容器化启动 engine: confkit 在容器内运行, 通过挂载宿主机容器引擎 socket (DooD 模式) 操作宿主机 daemon, 使 builder 容器创建在宿主机上、与宿主机共享缓存, 达到"在 engine 环境执行命令、操作其他容器进行构建"的效果.

技术路线决策:

- **socket 挂载 (DooD)** — engine 容器挂载 `/var/run/docker.sock`, 操作宿主机 daemon, builder 为兄弟容器. 选定此路线.
- **DinD** — 嵌套 daemon (需 privileged), builder 被隔离在内层, 宿主机不可见, 与目标相反, 排除.
- **远程 daemon (DOCKER_HOST)** — 路径翻译问题相同, 收益不大, 作为可选变体.

这是 Drone / Woodpecker / GitLab Runner 的成熟模式. 核心工程量不在容器化本身, 而在路径翻译层: daemon 收到的挂载路径按宿主机文件系统解析, engine 容器内路径必须换算后下发.

前置依赖 (执行顺序建议):

1. [[FEAT-task-hooks-and-result-vars-1f4d13]] — host steps / hooks 的语义变化只需文档化一次
2. [[FEAT-publish-and-gitops-handoff-b3e3b1]] — engine 镜像分发复用 publish 能力 (自举)

## 目标

- engine 可在容器内运行, 通过 socket 挂载操作宿主机 daemon, builder 对宿主机可见
- 宿主机路径映射可配置, 挂载路径 / HOST_* 环境变量统一经翻译层换算
- engine 自身部署 GitOps 化: compose 仓库声明 engine 服务, 新机器一条命令获得完整 CI

## 范围

- engine 镜像打包: confkit 二进制 + git + docker CLI
- `.confkit.yml` 新增 `engine.host_paths` 映射配置 (如 `volumes: /srv/confkit/volumes`)
- `PathFormatter` 增加宿主机路径换算层, 所有下发给 daemon 的挂载路径与 `HOST_*_DIR` 变量注入统一走该层
- docker socket 挂载运行验证, 提供 compose 部署示例 (one-shot: `docker compose run --rm confkit run ...`)
- 文档化 host steps 与 hooks 的语义变化 ("host" = engine 运行环境, 不再必然是物理宿主机)

## 非目标

- 不做 DinD (privileged 嵌套 daemon)
- 不做 server 化 / 常驻服务 — 首期保持 one-shot 运行形态; webhook 触发属远期方向, 到时仅是运行方式变化而非架构变化
- podman 容器化首期不承诺 — rootless socket 路径差异 (`/run/user/<uid>/podman/podman.sock`) 与 SELinux label 问题后补
- 不做多机器 / 分布式执行

## 方案讨论

- 路径翻译收敛在 `PathFormatter` 与 `HOST_*` 变量注入两处, 不散改代码库; 参考 GitLab Runner 的 volumes 映射设计
- engine 镜像分发走自身 publish 能力, 形成自举闭环
- 使用形态 one-shot (`docker compose run --rm`), 与"不做 server"的既有决策一致

## 风险与边界

- socket 挂载等价于授予 engine 容器宿主机 root 权限, 自托管单机场景可接受, 文档必须显著警示
- Docker Desktop on macOS 的"宿主机"实为 VM, 路径映射语义易混淆, 文档需说明支持口径以 Linux 服务器为主
- 未配置 `host_paths` 但运行于容器内时, 必须显式报错而非挂载不存在路径静默失败
- 缓存 / workspace 数据落在宿主机声明路径, 迁移与备份责任在用户

## 验收口径

- engine 容器内执行 `confkit run`, builder 容器出现在宿主机 `docker ps`, 构建产物 / 缓存落在宿主机映射路径
- `HOST_*_DIR` 变量在容器化与宿主机两种运行形态下均指向正确的宿主机真实路径
- 挂载路径经翻译层换算, builder 内工作区 / artifacts 读写正常
- 容器内运行但未配置 `host_paths` 时启动即报错, 提示明确
- 未容器化 (现状) 行为零回归
