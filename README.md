# ConfKit CLI

ConfKit is a configuration-driven build and deployment tool designed for modern CI/CD pipelines.

## Core Features

- **Builder Management**: Full lifecycle management of Docker images and containers
- **Configuration Driven**: Define build processes via YAML configuration files
- **Conditional Execution**: Smart step execution based on environment variables and runtime conditions
- **Task Execution**: Supports both local and containerized command execution
- **Log Management**: Complete build log recording, viewing, and management
- **Git Integration**: Native support for Git repository operations and environment variable injection
- **Interactive Interface**: User-friendly command-line interactive experience

## Quick Start

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh

# Interactive mode
confkit

# Run a build
confkit run --space hello --project hello-app
```

## Documentation

| Document | Description |
|----------|-------------|
| [Getting Started](docs/getting-started.md) | Installation, configuration structure, basic usage |
| [Project Structure](docs/project-structure.md) | Directory layout, space/project/image organization |
| [Configuration Reference](docs/configuration.md) | Main config (`.confkit.yml`) and project config field details |
| [Compose Configuration](docs/compose.md) | Builder container definitions, service fields, volume mounts |
| [Environment Variables](docs/variables.md) | System variables, Git variables, custom variables, CLI injection, interactive input |
| [Conditional Execution](docs/conditions.md) | Condition expression syntax, operators, examples |
| [Builder Management](docs/builder.md) | Image and container lifecycle management |
| [Log Management](docs/logs.md) | Structured log storage, querying, and cleanup |
| [Volumes Directory](docs/volumes.md) | Runtime data directories: workspace, artifacts, cache, temp, logs, context |
| [CLI Reference](docs/cli-reference.md) | Complete command reference for all subcommands |

## License

MIT License
