# Compose 配置 (`confkit-compose.yml`)

Compose 文件定义构建器容器，是一个标准 Docker Compose 文件，ConfKit 通过它管理容器生命周期。

## 示例

```yml
services:
  golang-builder:
    image: golang:1.24
    working_dir: /workspace
    volumes:
      - ./volumes/workspace:/workspace
      - ./volumes/artifacts:/artifacts
      - ./volumes/cache:/cache
    environment:
      - TZ=UTC

  rust-builder:
    image: rust:1.88-alpine
    working_dir: /workspace
    volumes:
      - ./volumes/workspace:/workspace
      - ./volumes/artifacts:/artifacts
    environment:
      - TZ=UTC
```

## 工作原理

- 每个服务成为命名的构建器容器
- 服务名即项目 step 配置中 `container` 的值
- ConfKit 管理生命周期：创建、启动、停止、删除、健康检查
- 容器以 `engine_compose.project` 名称分组（默认：`confkit`）

## 服务字段

ConfKit 从每个服务中读取以下字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `image` | String | 容器基础镜像 |
| `container_name` | String | 可选的显式容器名 |
| `working_dir` | String | 默认工作目录 |
| `volumes` | Array | 主机到容器的挂载映射 |
| `ports` | Array | 端口映射 |
| `environment` | Map | 容器环境变量 |
| `depends_on` | Array | 服务依赖 |

其他 Docker Compose 字段（networks、restart 策略等）会被保留但不受 ConfKit 直接管理。

## Volume 挂载

标准挂载约定：

| 主机路径 | 容器路径 | 用途 |
|---------|---------|------|
| `./volumes/workspace` | `/workspace` | 源码和构建输入 |
| `./volumes/artifacts` | `/artifacts` | 构建输出和交付物 |
| `./volumes/cache` | `/cache` | 持久化构建缓存 |
| `./volumes/temp` | `/tmp` | 临时文件（可选） |

这些路径与系统变量对应：
- `HOST_WORKSPACE_DIR` -> `CONTAINER_WORKSPACE_DIR` (`/workspace`)
- `HOST_ARTIFACTS_ROOT_DIR` -> `CONTAINER_ARTIFACTS_ROOT_DIR` (`/artifacts`)

## 与主配置的关系

Compose 文件在 `.confkit.yml` 中引用：

```yml
engine_compose:
  project: confkit           # Compose 项目名（默认）
  file: ./confkit-compose.yml
```
