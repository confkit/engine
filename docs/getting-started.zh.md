# 快速开始

## 安装

### 快速安装（推荐）

**安装最新版本：**

```bash
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh
```

**安装指定版本：**

```bash
# 方法1：使用命令行参数（推荐）
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh -s -- 1.2.3

# 方法2：使用 bash 进程替换
bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh) 1.2.3

# 方法3：使用环境变量和 bash
CONFKIT_VERSION=1.2.3 bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh)
```

这将自动：
- 检测您的平台和架构
- 从 GitHub 发布页面下载对应的二进制文件
- 安装到系统二进制目录（macOS 为 `/usr/local/bin`，Linux 为 `/usr/local/bin` 或 `~/.local/bin`）
- 自动将二进制文件添加到 PATH

**版本格式支持：**
- `latest` - 安装最新发布版本（默认）
- `1.2.3` - 自动转换为 `v1.2.3`
- `v1.2.3` - 使用精确版本标签

### 支持的平台

- **Linux**: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin`

### 手动安装

如果您倾向于从源码构建：

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### 验证安装

```bash
confkit --help
```

## 快速上手

### 配置示例结构

```
examples/
├── confkit-compose.yml
├── .confkit.yml
└── .confkit/
    ├── spaces/
    │   ├── hello/
    │   │   └── hello-confkit.yml
    │   └── confkit/
    │       └── engine.yml
    └── volumes/
        ├── cache/
        ├── logs/
        └── workspace/
```

### 基本使用

```bash
# 查看帮助
confkit --help

# 查看版本
confkit -v

# 交互式模式（推荐新手使用）
confkit

# 管理构建器
confkit builder list
confkit builder create -n golang-builder
confkit builder start -n golang-builder

# 运行构建任务
confkit run --space hello --project hello-app

# 预览执行步骤（不实际执行）
confkit run --space hello --project hello-app --dry-run

# 通过命令行参数注入环境变量
confkit run --space hello --project hello-app -e KEY1=value1 -e KEY2=value2

# 查看日志（space/project 为可选过滤条件，支持分页）
confkit log list
confkit log list --space hello --project hello-app --page 1 --size 10
confkit log show --task <task_id>
confkit log info --task <task_id>

# 清理日志
confkit log clean --all
confkit log clean --space hello
confkit log clean --space hello --project hello-app
confkit log clean --task <task_id>

# 查看/校验配置
confkit config show
confkit config validate
```
