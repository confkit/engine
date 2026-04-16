# Log Management

Logs are stored in a flat date-based directory structure with SQLite metadata indexing.

## Storage Structure

```
volumes/logs/
  ├── tasks.db                    # SQLite metadata database
  └── 2026-03-04/                 # Flat storage by date
      ├── <task_id>.log           # Task log file
      ├── <task_id>.meta.json     # Metadata snapshot (for offline viewing)
      └── ...
```

## Structured Task Logs

Each task produces structured output with dual storage:

- **SQLite database** (`tasks.db`): Metadata index supporting paginated queries, cross-project filtering by space/project, and fast lookups by task ID
- **`<task_id>.meta.json`**: Metadata snapshot file for offline viewing, including task status, start/finish time, duration, and per-step results (status, exit code, errors)
- **`<task_id>.log`**: Full timestamped log output
- Metadata is updated after each step in both SQLite and JSON, so a crashed task's progress can still be inspected

## CLI Commands

```bash
# List task logs (supports optional filtering and pagination)
confkit log list
confkit log list --space hello
confkit log list --space hello --project hello-app
confkit log list --space hello --project hello-app --page 1 --size 10

# View task log content (only task ID required)
confkit log show --task <task_id>

# View task metadata (status, duration, step details)
confkit log info --task <task_id>

# Clean log files
confkit log clean --task <task_id>
confkit log clean --space hello
confkit log clean --space hello --project hello-app
confkit log clean --all
```
