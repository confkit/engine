# 项目结构

ConfKit CI 项目遵循固定的目录约定。

## 目录布局

```
project-root/
├── .confkit.yml                    # 主配置（入口）
├── confkit-compose.yml             # 构建器容器定义
├── .confkit/
│   ├── spaces/                     # 空间目录
│   │   ├── hello/
│   │   │   └── hello-confkit.yml   # 项目配置 YAML
│   │   └── confkit/
│   │       └── engine.yml          # 项目配置 YAML
│   └── images/                     # 自定义 Dockerfile
│       ├── Dockerfile.alpine:3.18
│       └── Dockerfile.rust.1.88-alpine
├── volumes/                        # 运行时数据（自动创建）
│   ├── workspace/                  # 源码工作空间
│   ├── artifacts/                  # 构建产物输出
│   ├── cache/                      # 构建缓存
│   ├── temp/                       # 临时文件（仓库检出等）
│   ├── logs/                       # 任务日志
│   │   ├── tasks.db                # SQLite 元数据索引
│   │   └── 2026-03-04/             # 按日期分区
│   │       ├── <task_id>.log
│   │       └── <task_id>.meta.json
│   └── context/                    # 镜像构建上下文
└── environment.yml                 # 共享环境变量（可选）
```

## 核心概念

### Space（空间）

空间按名称分组相关项目。每个空间是 `.confkit/spaces/` 下的一个目录，包含一个或多个项目 YAML 文件。`.confkit.yml` 中的 `path` 指向空间目录。

### Project（项目）

项目 YAML 定义一条构建流水线：源码、环境和步骤。一个空间目录下可以共存多个项目。

### Images（镜像）

自定义 Dockerfile 存放在 `.confkit/images/`。主配置中的 `engine_file` 字段引用这些文件进行镜像构建。

### Volumes（卷）

运行时数据目录，启动时自动创建。详见 [Volumes 指南](volumes.zh.md)。
