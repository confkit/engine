# 日志管理

日志采用扁平化日期目录 + SQLite 元数据索引存储。

## 存储结构

```
volumes/logs/
  ├── tasks.db                    # SQLite 元数据库
  └── 2026-03-04/                 # 按日期扁平存储
      ├── <task_id>.log           # 任务日志文件
      ├── <task_id>.meta.json     # 元数据快照（便于离线查看）
      └── ...
```

## 结构化任务日志

每个任务产生双重存储的结构化输出：

- **SQLite 数据库**（`tasks.db`）：元数据索引，支持分页查询、按 space/project 跨项目筛选、按 task ID 快速查找
- **`<task_id>.meta.json`**：元数据快照文件，便于离线查看，包含任务状态、开始/结束时间、总耗时、以及各步骤的执行结果（状态、退出码、错误信息）
- **`<task_id>.log`**：带时间戳的完整日志输出
- 元数据在每个步骤执行完毕后同时更新 SQLite 和 JSON，即使任务中途崩溃也可查看执行进度

## CLI 命令

```bash
# 列出任务日志（支持可选过滤和分页）
confkit log list
confkit log list --space hello
confkit log list --space hello --project hello-app
confkit log list --space hello --project hello-app --page 1 --size 10

# 查看任务日志内容（仅需 task ID）
confkit log show --task <task_id>

# 查看任务元数据（状态、耗时、各步骤详情）
confkit log info --task <task_id>

# 清理日志
confkit log clean --task <task_id>
confkit log clean --space hello
confkit log clean --space hello --project hello-app
confkit log clean --all
```
