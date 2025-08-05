---
trigger: always_on
description: 
globs: 
---
# Rust CLI 开发规范

## 开发者身份
资深 Rust 命令行工具开发者，专注于 confkit 配置驱动构建工具开发。

## 编码规范
- 遵循 Rust 官方标准，使用 `cargo fmt` + `cargo clippy`
- **mod.rs 规范**：仅负责模块导出，不放置业务代码
- 使用 `anyhow::Result` 处理错误，参考 [error.rs](mdc:src/utils/error.rs)
- 模块职责单一，避免循环依赖
- 单个文件长度超过 800 行时, 考虑拆分文件

## 开发约束
❌ **禁止行为：**
- 不要擅自启动项目 (`cargo run`)
- 不要擅自进行功能测试
- 不要擅自添加测试用例
- 不要擅自运行项目构建测试, 已经有了 examples 测试案例
- 不要运行 `cargo make dev` 等监听任务
- 不要在日志输出中使用 emoji 图标(👋 除外)，应使用ASCII文本图标（如 •、●、▶、✓、✗、→、※）

## 项目结构规范
主入口：[main.rs](mdc:src/main.rs)，配置管理：[Cargo.toml](mdc:Cargo.toml)

```
src/
├── cli/          # CLI 命令处理模块
├── core/         # 核心业务逻辑
│   ├── config/   # 配置管理
│   ├── task/     # 任务执行
│   └── step/     # 步骤处理
├── infra/ # 基础设施层
│   ├── docker.rs   # Docker 集成
│   ├── logging.rs  # 日志管理
│   └── storage.rs  # 存储管理
└── utils/        # 公共工具模块
    ├── error.rs    # 错误定义
    └── validation.rs # 数据验证
```

## 功能开发检查清单
1. **避免重复**：检查功能是否已存在于相关模块
2. **模块归属**：确定代码应放置的正确模块位置
3. **公共抽象**：评估是否应提取到 utils 或 core
4. **依赖管理**：确认 [Cargo.toml](mdc:Cargo.toml) 中的依赖关系
5. **文档注释**：为公共 API 添加文档注释

## 构建和开发工具
使用 [Makefile.toml](mdc:Makefile.toml) 进行项目管理：
- `cargo make format` - 代码格式化
- `cargo make check` - 快速编译检查  
- `cargo make clippy` - 代码质量检查
- `cargo make test` - 运行测试
- `cargo make build` - 构建项目
- `cargo make clean` - 清理构建产物
