# 构建器管理

ConfKit 提供分层的 Docker 镜像与容器管理。

## 分层架构

- **镜像层**：管理 Docker 镜像的拉取、构建和删除
- **容器层**：基于 docker-compose.yml 创建命名构建器容器
- **生命周期**：完整的启动、停止、健康检查流程

---

## 镜像管理

```bash
# 列出镜像
confkit image list

# 拉取/构建镜像
confkit image create golang:1.24

# 删除镜像
confkit image remove golang:1.24
```

## 容器管理

```bash
# 列出所有构建器状态
confkit builder list

# 创建构建器（基于 docker-compose.yml）
confkit builder create -n golang-builder

# 启动/停止构建器
confkit builder start -n golang-builder
confkit builder stop -n golang-builder

# 删除构建器
confkit builder remove -n golang-builder

# 健康检查（所有容器）
confkit builder health

# 健康检查（指定容器）
confkit builder health -n golang-builder
```

## 执行构建

```bash
# 构建项目
confkit run --space <space_name> --project <project_name>

# 预览执行步骤（不实际执行）
confkit run --space <space_name> --project <project_name> --dry-run
```
