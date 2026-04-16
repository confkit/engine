# CLI Reference

Complete reference for all ConfKit CLI commands.

## Global Options

```bash
confkit --help     # Show help
confkit -v         # Show version (shorthand for --version)
```

---

## Interactive Mode

```bash
confkit
```

Starts interactive mode with a guided menu:

- `[RUN] Run Management` - Execute project build tasks
- `[BUILDER] Builder Management` - Image and container management
- `[IMAGE] Image Management` - Manage build images
- `[LOG] Log Management` - List, view, and inspect task logs (supports all projects / by space / by space+project)
- `[CLEAN] Clean Management` - Clean logs, workspace, artifacts, cache, temp

---

## Builder Commands

```bash
confkit builder list                    # List all builders
confkit builder create -n <name>        # Create builder
confkit builder start -n <name>         # Start builder
confkit builder stop -n <name>          # Stop builder
confkit builder remove -n <name>        # Remove builder
confkit builder health                  # Health check (all)
confkit builder health -n <name>        # Health check (specific)
```

## Image Commands

```bash
confkit image list                      # List images
confkit image create <image:tag>        # Pull/build image
confkit image remove <image:tag>        # Remove image
```

## Run Commands

```bash
confkit run --space <space> --project <project>                  # Run build
confkit run --space <space> --project <project> --dry-run        # Dry run (preview)
confkit run --space <space> --project <project> -e KEY=VALUE     # Inject env vars
```

## Log Commands

```bash
confkit log list                                              # List all logs
confkit log list --space <space>                              # Filter by space
confkit log list --space <space> --project <project>          # Filter by space+project
confkit log list --space <space> --project <project> \        # Pagination
  --page 1 --size 10
confkit log show --task <task_id>                             # View log content
confkit log info --task <task_id>                             # View metadata
confkit log clean --all                                       # Clean all logs
confkit log clean --space <space>                             # Clean by space
confkit log clean --space <space> --project <project>         # Clean by space+project
confkit log clean --task <task_id>                            # Clean specific task
```

## Clean Commands

```bash
confkit clean workspace          # Clean workspace directory
confkit clean artifacts          # Clean artifacts directory
confkit clean cache              # Clean cache directory
confkit clean temp               # Clean temp directory
confkit clean log --all          # Clean all logs (same as log clean --all)
confkit clean log --space <s>    # Clean logs by space
confkit clean log --space <s> --project <p>   # Clean logs by space+project
confkit clean log --task <id>    # Clean specific task log
confkit clean all                # Clean everything (workspace, artifacts, cache, temp, logs)
```

## Config Commands

```bash
confkit config show              # Show configuration overview (engine, spaces, projects, images)
confkit config validate          # Validate configuration file (paths, required fields, etc.)
```
