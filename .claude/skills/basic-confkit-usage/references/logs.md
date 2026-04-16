# Log Management

Flat date-based storage with SQLite metadata indexing.

## Storage

```
volumes/logs/
  ├── tasks.db                     # SQLite metadata
  └── 2026-03-04/
      ├── <task_id>.log            # Timestamped log
      ├── <task_id>.meta.json      # Metadata snapshot
      └── ...
```

## Dual Storage

- **SQLite** (`tasks.db`): Paginated queries, cross-project filtering, fast task ID lookup
- **meta.json**: Offline-viewable snapshot (status, timing, per-step results)
- **log**: Full timestamped output
- Updated after each step in both SQLite and JSON; crashed task progress still inspectable

## Commands

```bash
# List (optional filters + pagination)
confkit log list
confkit log list -s hello
confkit log list -s hello -p hello-app
confkit log list -s hello -p hello-app --page 1 --size 10

# View
confkit log show --task <task_id>       # Log content
confkit log info --task <task_id>       # Metadata

# Clean
confkit log clean --all
confkit log clean -s hello
confkit log clean -s hello -p hello-app
confkit log clean --task <task_id>
```
