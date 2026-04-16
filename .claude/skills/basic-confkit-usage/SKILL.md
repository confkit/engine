---
name: basic-confkit-usage
description: Use when writing or reviewing ConfKit usage documentation, or when answering questions about CLI commands, configuration fields, variables, or conditions.
user-invocable: false
---

# ConfKit Usage Reference

ConfKit CLI usage guide entry point. Read this first for routing, then load only the reference you need.

## Quick CLI Reference

```bash
confkit                                          # Interactive mode
confkit -v                                       # Version
confkit run -s <space> -p <project>              # Run build
confkit run -s <space> -p <project> --dry-run    # Preview
confkit run -s <space> -p <project> -e K=V       # Inject env
confkit builder list|create|start|stop|remove|health
confkit image list|create|remove
confkit log list|show|info|clean
confkit clean workspace|artifacts|cache|temp|log|all
confkit config show|validate
```

## Reference Routing

| Topic | Reference | When to Load |
|-------|-----------|-------------|
| `.confkit.yml` fields | `references/main-config.md` | Questions about main config: version, engine, shell, compose, spaces, images |
| `confkit-compose.yml` | `references/compose.md` | Questions about builder container definitions, service fields, volume mounts |
| Project YAML fields | `references/project-config.md` | Questions about project config: source, environment, steps, environment_from_args |
| Environment variables | `references/variables.md` | Questions about system/git/custom/CLI/interactive variables |
| Conditional execution | `references/conditions.md` | Questions about condition syntax, operators, expressions |
| Builder management | `references/builder.md` | Questions about image/container lifecycle commands |
| Log management | `references/logs.md` | Questions about log storage, querying, cleanup |
| Project structure | `references/project-structure.md` | Questions about directory layout, space/project/image organization |
| Volumes directory | `references/volumes.md` | Questions about workspace/artifacts/cache/temp/logs/context directories |

## Loading Rules

- Do not load all references at once; only load the one matching the current topic
- If multiple topics are involved, load only the references needed for those topics
- SKILL.md itself contains no detailed field descriptions - always defer to references
