# Volumes 目录

运行时数据目录，启动时自动创建。所有路径相对于项目根目录。

## 目录概览

| 目录 | 主机路径 | 容器路径 | 用途 |
|------|---------|---------|------|
| Workspace | `volumes/workspace/` | `/workspace/` | 源码和构建输入 |
| Artifacts | `volumes/artifacts/` | `/artifacts/` | 构建输出和交付物 |
| Cache | `volumes/cache/` | `/cache/` | 持久化构建缓存 |
| Temp | `volumes/temp/` | -- | 临时文件（仓库检出信息等） |
| Logs | `volumes/logs/` | -- | 任务执行日志和元数据 |
| Context | `volumes/context/` | -- | 镜像构建上下文 |

## Workspace（`volumes/workspace/`）

源码工作目录。任务执行期间：

- Git 仓库在此克隆/检出
- 每个任务在 workspace 下获得专用子目录
- 挂载到构建器容器的 `/workspace`
- 由 `HOST_WORKSPACE_DIR` 和 `CONTAINER_WORKSPACE_DIR` 变量引用

## Artifacts（`volumes/artifacts/`）

构建输出目录：

- 步骤将构建结果写入此处（二进制、包等）
- 挂载到构建器容器的 `/artifacts`
- 由 `HOST_ARTIFACTS_ROOT_DIR` 和 `CONTAINER_ARTIFACTS_ROOT_DIR` 变量引用
- 跨任务持久保存，除非显式清理

## Cache（`volumes/cache/`）

持久化构建缓存：

- 包管理器缓存、依赖缓存
- Compose 文件中的可选挂载点（`/cache`）
- 跨构建持久保存以提升性能

## Temp（`volumes/temp/`）

临时存储：

- Git 仓库元数据和稀疏检出
- 任务执行期间的中间文件
- 可在构建间安全清理

## Logs（`volumes/logs/`）

任务执行日志：

```
volumes/logs/
  ├── tasks.db                     # SQLite 元数据索引
  └── 2026-03-04/                  # 按日期分区
      ├── <task_id>.log            # 带时间戳的完整日志
      └── <task_id>.meta.json      # 元数据快照
```

查询和清理命令见 [日志管理](logs.zh.md)。

## Context（`volumes/context/`）

Docker 镜像构建上下文：

- 用作镜像定义中的 `context` 字段
- 包含 `confkit image create` 时引用的文件

## 清理

```bash
confkit clean workspace     # 清理工作空间
confkit clean artifacts     # 清理产物
confkit clean cache         # 清理缓存
confkit clean temp          # 清理临时文件
confkit clean all           # 清理以上全部 + 日志
```
