# Project Structure

A ConfKit CI project follows a fixed directory convention.

## Directory Layout

```
project-root/
├── .confkit.yml                    # Main config (entry point)
├── confkit-compose.yml             # Builder container definitions
├── .confkit/
│   ├── spaces/                     # Space directories
│   │   ├── hello/
│   │   │   └── hello-confkit.yml   # Project config YAML
│   │   └── confkit/
│   │       └── engine.yml          # Project config YAML
│   └── images/                     # Custom Dockerfiles
│       ├── Dockerfile.alpine:3.18
│       └── Dockerfile.rust.1.88-alpine
├── volumes/                        # Runtime data (auto-created)
│   ├── workspace/                  # Source code workspace
│   ├── artifacts/                  # Build artifacts output
│   ├── cache/                      # Build cache
│   ├── temp/                       # Temporary files (repo checkout, etc.)
│   ├── logs/                       # Task logs
│   │   ├── tasks.db                # SQLite metadata index
│   │   └── 2026-03-04/             # Date-partitioned logs
│   │       ├── <task_id>.log
│   │       └── <task_id>.meta.json
│   └── context/                    # Image build context
└── environment.yml                 # Shared environment variables (optional)
```

## Key Concepts

### Space

A space groups related projects under a name. Each space is a directory under `.confkit/spaces/` containing one or more project YAML files. The `path` in `.confkit.yml` points to the space directory.

### Project

A project YAML defines one build pipeline: source, environment, and steps. Multiple projects can coexist in a single space directory.

### Images

Custom Dockerfiles live in `.confkit/images/`. The `engine_file` field in the main config references these files for image building.

### Volumes

Runtime data directories, auto-created on startup. See [Volumes Guide](volumes.md) for details.
