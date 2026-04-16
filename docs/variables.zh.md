# 环境变量

ConfKit 提供多种方式定义和注入环境变量。所有变量均支持 `${变量名}` 替换语法。

## 变量优先级

从高到低：

1. **命令行注入**（`-e KEY=VALUE`）
2. **交互式环境变量**（`environment_from_args`）
3. **自定义环境变量**（`environment`）
4. **环境变量文件**（`environment_files`）
5. **Git 变量**（自动注入）
6. **系统变量**（自动注入）

---

## 系统变量

ConfKit 在执行任务时自动注入：

| 变量 | 说明 | 示例 |
|------|------|------|
| `TASK_ID` | 任务唯一标识符 | `20250113-143022-a1b2c3` |
| `PROJECT_NAME` | 配置文件中的项目名称 | `hello-app` |
| `PROJECT_VERSION` | 项目版本号（取自 package.json / Cargo.toml） | `1.0.0` |
| `SPACE_NAME` | 空间名称 | `hello` |
| `HOST_VOLUMES_DIR` | 主机 volumes 根目录 | `volumes` |
| `HOST_WORKSPACE_DIR` | 主机任务工作空间目录 | `volumes/workspace/...` |
| `CONTAINER_WORKSPACE_DIR` | 容器任务工作空间目录 | `/workspace/...` |
| `HOST_ARTIFACTS_ROOT_DIR` | 主机任务产物根目录 | `volumes/artifacts` |
| `CONTAINER_ARTIFACTS_ROOT_DIR` | 容器任务产物根目录 | `/artifacts` |

## Git 变量

配置了 `source` 块时自动注入：

| 变量 | 说明 | 示例 |
|------|------|------|
| `GIT_REPO` | 配置文件中的 Git 仓库地址 | `https://github.com/example/repo.git` |
| `GIT_BRANCH` | Git 分支名（来自配置或当前分支） | `main` |
| `GIT_HASH` | 完整 commit hash | `a1b2c3d4e5f6...` |
| `GIT_HASH_SHORT` | 短 commit hash（前 8 个字符） | `a1b2c3d4` |

---

## 自定义变量

在项目配置的 `environment` 部分定义静态环境变量：

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

变量可以使用 `${变量名}` 语法引用其他变量。

## 环境变量文件

通过 `environment_files` 从外部文件加载变量：

```yaml
environment_files:
  - format: "yaml"
    path: "./volumes/environment.yml"
```

---

## 命令行参数注入

通过 `-e` / `--environments` 参数在运行时注入环境变量：

```bash
confkit run --space hello --project hello-app \
  -e ENVIRONMENT=production \
  -e BUILD_VERSION=2.0.0 \
  -e API_URL=https://api.example.com
```

- 格式：`KEY=VALUE`
- 格式不正确的条目会被跳过并输出警告
- 优先级最高，会覆盖所有其他来源的同名变量

---

## 交互式环境变量

在 `environment_from_args` 部分定义交互式提示，任务执行时会向用户收集输入。

### 支持的类型

#### `input` - 自由文本输入

```yaml
- name: "API_URL"
  type: "input"
  prompt: "请输入API地址"
  default: "https://api.example.com"
  required: true
```

#### `radio` - 单选

```yaml
- name: "ENVIRONMENT"
  type: "radio"
  prompt: "选择部署环境"
  default: "staging"
  required: true
  options:
    - "development"
    - "staging"
    - "production"
```

#### `checkbox` - 多选

```yaml
- name: "FEATURES"
  type: "checkbox"
  prompt: "选择要启用的功能"
  default: "auth"
  required: false
  options:
    - "auth"
    - "logging"
    - "metrics"
```

#### `confirm` - 是/否确认

```yaml
- name: "ENABLE_DEBUG"
  type: "confirm"
  prompt: "是否启用调试模式？"
  default: "false"
  required: false
```

返回值为 `"true"` 或 `"false"`。

### 配置选项

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `name` | String | 是 | — | 环境变量名称 |
| `type` | String | 是 | — | 交互类型：`input` / `radio` / `checkbox` / `confirm` |
| `prompt` | String | 是 | — | 用户提示文本 |
| `default` | String | 否 | — | 默认值 |
| `required` | Boolean | 否 | `true` | 是否必填 |
| `options` | Array\<String\> | 否 | — | 可选项（用于 `radio` / `checkbox`） |
