# ConfKit CLI

ConfKit is a configuration-driven build and deployment tool designed for modern CI/CD pipelines.

## 📋 Core Features

- **Builder Management**: Full lifecycle management of Docker images and containers
- **Configuration Driven**: Define build processes via YAML configuration files
- **Task Execution**: Support for local and containerized command execution
- **Log Management**: Complete build log recording, viewing, and management
- **Git Integration**: Native support for Git repository operations and environment variable injection
- **Interactive Interface**: User-friendly command-line interactive experience

## 🚀 Quick Start

### Installation

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### Example Configuration Structure

```
examples/
├── docker-compose.yml
├── .confkit.yml
└── .confkit/
    ├── spaces/
    │   ├── hello/
    │   │   └── hello-confkit.yml
    │   └── confkit/
    │       └── engine.yml
    └── volumes/
        ├── logs/
        ├── context/
        ├── workspace/
        └── artifacts/
```

### Basic Usage

```bash
# Show help
confkit --help

# Interactive mode (recommended for beginners)
confkit interactive

# Manage builders
confkit builder list
confkit builder create golang-builder
confkit builder start golang-builder

# Run build tasks
confkit run --space hello --project hello-app

# View logs
confkit log list --space hello --project hello-app
confkit log show --space hello --project hello-app <filename>
```

## 🏗 Builder Management

### Image Management

```bash
# List images
confkit image list

# Pull/build image
confkit image create golang:1.24

# Remove image
confkit image remove golang:1.24
```

### Container Management

```bash
# List all builder statuses
confkit builder list

# Create builder (based on docker-compose.yml)
confkit builder create golang-builder

# Start/stop builder
confkit builder start golang-builder
confkit builder stop golang-builder

# Remove builder
confkit builder remove golang-builder

# Health check
confkit builder health golang-builder
```

### Execute Build

```bash
# Build project
confkit exec --space <space_name> --project-name <project_name>
```

### Project Configuration Example

```yaml
name: "hello-confkit"
description: "Hello Confkit"

source:
  git_repo: "https://github.com/example/hello-go.git"
  git_branch: "main"

environment_files:
  - format: "yaml"
    path: "./volumes/environment.yml"

environment:
  APP_NAME: "hello-app"
  BUILD_VERSION: "1.0.0"

steps:
  - name: "Build Application"
    container: "golang-builder"
    working_dir: "/workspace"
    commands:
      - "echo 'Building ${APP_NAME} v${BUILD_VERSION}'"
      - "echo 'Git Hash: ${GIT_HASH}'"
      - "go build -o app ./main.go"
    timeout: "5m"
```

## 📋 Log Management

```bash
# List log files
confkit log list --space hello --project hello-app

# View specific log
confkit log show --space hello --project hello-app abc123

# Support multiple matching methods
confkit log show --space hello --project hello-app "2025-01-13_12-00-00"
confkit log show --space hello --project hello-app complete-filename.txt
```

## 🖥 Interactive Mode

Start interactive mode for the best user experience:

```bash
confkit interactive
```

**Navigation Paths**:

- `[BUILDER] Builder Management` → Image and container management
- `[RUN] Run Management` → Execute project build tasks
- `[LOG] Log Management` → View project logs

## 🎯 Featured Functions

### Automatic Environment Variable Injection

ConfKit automatically injects the following environment variables when executing tasks:

#### System Variables

- `TASK_ID` - Unique task identifier (e.g., `20250113-143022-a1b2c3`)
- `PROJECT_NAME` - Project name from the configuration file
- `SPACE_NAME` - Space name
- `HOST_WORKSPACE_DIR` - Host task workspace directory
- `HOST_ARTIFACTS_DIR` - Host task artifacts directory
- `CONTAINER_WORKSPACE_DIR` - Container task workspace directory
- `CONTAINER_ARTIFACTS_DIR` - Container task artifacts directory

#### Git Variables

- `GIT_REPO` - Git repository address from the configuration file
- `GIT_BRANCH` - Git branch name (from config or current branch)
- `GIT_HASH` - Full commit hash
- `GIT_HASH_SHORT` - Short commit hash (first 8 characters)

#### Custom Variables

You can also define custom environment variables in the project configuration:

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_COMMIT_SHORT}"
```

All environment variables support variable substitution using the `${variable_name}` syntax.

### Smart Log Matching

Supports multiple log file matching methods:

- Full filename
- Filename fragment
- Task ID fragment
- Timestamp matching

### Layered Builder Management

- **Image Layer**: Manage Docker image pulling, building, and removal
- **Container Layer**: Create named builder containers based on docker-compose.yml
- **Lifecycle**: Complete start, stop, and health check processes

## 📄 License

MIT License
