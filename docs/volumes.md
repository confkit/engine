# Volumes Directory

Runtime data directories, auto-created on startup. All paths are relative to the project root.

## Directory Overview

| Directory | Host Path | Container Path | Purpose |
|-----------|-----------|----------------|---------|
| Workspace | `volumes/workspace/` | `/workspace/` | Source code and build input |
| Artifacts | `volumes/artifacts/` | `/artifacts/` | Build output and deliverables |
| Cache | `volumes/cache/` | `/cache/` | Persistent build cache |
| Temp | `volumes/temp/` | -- | Temporary files (repo checkout info, etc.) |
| Logs | `volumes/logs/` | -- | Task execution logs and metadata |
| Context | `volumes/context/` | -- | Image build context |

## Workspace (`volumes/workspace/`)

Source code working directory. During task execution:

- Git repositories are cloned/checked out here
- Each task gets a dedicated subdirectory under workspace
- Mounted to `/workspace` inside builder containers
- Referenced by `HOST_WORKSPACE_DIR` and `CONTAINER_WORKSPACE_DIR` variables

## Artifacts (`volumes/artifacts/`)

Build output directory:

- Steps write build results here (binaries, packages, etc.)
- Mounted to `/artifacts` inside builder containers
- Referenced by `HOST_ARTIFACTS_ROOT_DIR` and `CONTAINER_ARTIFACTS_ROOT_DIR` variables
- Persisted across tasks unless explicitly cleaned

## Cache (`volumes/cache/`)

Persistent build cache:

- Package manager caches, dependency caches
- Optional mount point (`/cache`) in compose file
- Persists across builds for performance

## Temp (`volumes/temp/`)

Temporary storage:

- Git repository metadata and sparse checkout
- Intermediate files during task execution
- Can be safely cleaned between builds

## Logs (`volumes/logs/`)

Task execution logs:

```
volumes/logs/
  ├── tasks.db                     # SQLite metadata index
  └── 2026-03-04/                  # Date-partitioned
      ├── <task_id>.log            # Full timestamped log
      └── <task_id>.meta.json      # Metadata snapshot
```

See [Log Management](logs.md) for query and cleanup commands.

## Context (`volumes/context/`)

Docker image build context:

- Used as the `context` field in image definitions
- Contains files referenced during `confkit image create`

## Cleanup

```bash
confkit clean workspace     # Clean workspace
confkit clean artifacts     # Clean artifacts
confkit clean cache         # Clean cache
confkit clean temp          # Clean temp
confkit clean all           # Clean all of the above + logs
```
