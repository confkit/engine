# ConfKit CLI

ConfKit is a configuration-driven build and deployment tool designed for modern CI/CD pipelines.

## 📋 Core Features

- **Builder Management**: Complete lifecycle management of Docker images and containers
- **Configuration-Driven**: Define build processes through YAML configuration files
- **Task Execution**: Support for both local and containerized command execution
- **Log Management**: Complete build log recording, viewing, and management
- **Git Integration**: Native Git repository operations and environment variable injection
- **Interactive Interface**: Friendly command-line interactive experience

## 🚀 Quick Start

### Installation

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
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
confkit builder image list

# Pull/build images
confkit builder image create golang:1.24

# Remove images
confkit builder image remove golang:1.24
```

### Container Management
```bash
# List all builder status
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

## 📝 Configuration File Examples

Complete configuration examples are available in the `examples/` directory:

```bash
examples/
├── builder.yml           # Builder configuration
├── docker-compose.yml    # Container service definition
└── .confkit/
    └── spaces/
        └── hello/
            ├── config.yml          # Space configuration
            └── projects/
                └── hello-app.yml   # Project configuration
```

### Project Configuration Example

```yaml
# examples/.confkit/spaces/hello/projects/hello-app.yml
name: "hello-app"
type: "golang"
description: "Hello World Go Application"

source:
  git_repo: "https://github.com/example/hello-go.git"
  git_branch: "main"

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

Launch interactive mode for the best user experience:

```bash
confkit interactive
```

**Navigation Paths**:
- `[BUILDER] Builder Management` → Image and container management
- `[RUN] Run Management` → Execute project build tasks  
- `[LOG] Log Management` → View project logs

## 🎯 Key Features

### Automatic Git Environment Variable Injection

Git information is automatically injected into environment variables when executing tasks:
- `GIT_HASH` - Complete commit hash
- `GIT_COMMIT_HASH` - Complete commit hash (alias)
- `GIT_COMMIT_SHORT` - Short commit hash
- `GIT_TAG` - Current tag (if available)

### Smart Log Matching

Support multiple log file matching methods:
- Complete filename
- Filename fragment
- Task ID fragment
- Timestamp matching

### Layered Builder Management

- **Image Layer**: Manage Docker image pulling, building, and deletion
- **Container Layer**: Create named builder containers based on docker-compose.yml
- **Lifecycle**: Complete start, stop, and health check processes

## 📂 Project Structure

```
examples/                # Example configurations
├── builder.yml         # Builder configuration
├── docker-compose.yml  # Container service definition
└── .confkit/           # ConfKit workspace
    └── spaces/         # Space management
        └── hello/      # Example space
volumes/                # Runtime data
├── logs/              # Task logs
├── workspace/         # Build workspace  
└── artifacts/         # Build artifacts
```

## 🛠 Development Status

### ✅ Completed
- Builder management (image + container)
- Configuration file parsing and validation
- Task execution engine (basic)
- Log system (complete)
- Git integration and environment variable injection
- Interactive interface (Builder + Log)

### 🚧 In Development
- Task management commands
- Advanced parallel execution
- Notification system

## 📄 License

MIT License 