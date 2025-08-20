# ConfKit CLI

ConfKit is a configuration-driven build and deployment tool designed for modern CI/CD pipelines.

## 📋 Core Features

- **Builder Management**: Full lifecycle management of Docker images and containers
- **Configuration Driven**: Define build processes via YAML configuration files
- **Task Execution**: Supports both local and containerized command execution
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
├── confkit-compose.yml
├── .confkit.yml
└── .confkit/
    ├── spaces/
    │   ├── hello/
    │   │   └── hello-confkit.yml
    │   └── confkit/
    │       └── engine.yml
    └── volumes/
        ├── cache/
        ├── logs/
        └── workspace/
```

### Basic Configuration File
```yml
# .confkit.yml
version: 1.0.0

# Container engine: docker/podman
engine: docker

engine_compose:
  # Container group (default: confkit)
  # project: confkit
  # docker compose file
  file: ./confkit-compose.yml

# Space list
spaces:
  - name: confkit
    description: "ConfKit toolchain release space"
    # Project execution config file
    path: .confkit/spaces/confkit
  - name: hello
    description: "Hello ConfKit"
    path: .confkit/spaces/hello

# Image management list
images:
    # Target image name
  - name: hello-builder
    # Base image (auto pull)
    base_image: alpine
    # Base image tag (shared by target image)
    tag: 3.18
    context: volumes/context
    # Dockerfile path
    engine_file: ./.confkit/images/Dockerfile.alpine:3.18
  - name: rust-builder
    base_image: rust
    tag: 1.88-alpine
    context: volumes/context
    engine_file: ./.confkit/images/Dockerfile.rust.1.88-alpine
```

### Basic Usage

```bash
# View help
confkit --help

# Interactive mode (recommended for beginners)
confkit

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
    # 5m = 300s
    timeout: 300
```

## 📋 Log Management

```bash
# List log files
confkit log list --space hello --project hello-app

# View specific log
confkit log show --space hello --project hello-app abc123

# Supports multiple matching methods
confkit log show --space hello --project hello-app "2025-01-13_12-00-00"
confkit log show --space hello --project hello-app complete-filename.txt
```

## 🖥 Interactive Mode

Start interactive mode for the best user experience:

```bash
confkit interactive
```

**Navigation Path**:

- `[BUILDER] Builder Management` → Image and container management
- `[RUN] Run Management` → Execute project build tasks
- `[LOG] Log Management` → View project logs

## 🎯 Featured Functions

### Automatic Environment Variable Injection

ConfKit automatically injects the following environment variables when executing tasks:

#### System Variables

- `TASK_ID` - Unique task identifier (e.g. `20250113-143022-a1b2c3`)
- `PROJECT_NAME` - Project name from config file
- `PROJECT_VERSION` - Project version from remote repository(javascript: package.json, rust: Cargo.toml)
- `SPACE_NAME` - Space name
- `HOST_WORKSPACE_DIR` - Host task workspace directory
- `CONTAINER_WORKSPACE_DIR` - Container task workspace directory

#### Git Variables

- `GIT_REPO` - Git repository address from config file
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

#### Interactive Environment Variables

ConfKit supports interactive environment variable input during task execution. Define them in the `environment_from_args` section:

```yaml
environment_from_args:
  # Input type - free text input
  - name: "API_URL"
    type: "input"
    prompt: "Please enter API URL"
    default: "https://api.example.com"
    required: true
    
  # Radio type - single choice selection
  - name: "ENVIRONMENT"
    type: "radio"
    prompt: "Select deployment environment"
    default: "staging"
    required: true
    options:
      - "development"
      - "staging"
      - "production"
      
  # Checkbox type - multiple choice selection
  - name: "FEATURES"
    type: "checkbox"
    prompt: "Select features to enable"
    default: "auth"
    required: false
    options:
      - "auth"
      - "logging"
      - "metrics"
      
  # Confirm type - yes/no confirmation
  - name: "ENABLE_DEBUG"
    type: "confirm"
    prompt: "Enable debug mode?"
    default: "false"
    required: false
```

**Supported Interactive Types:**
- `input`: Free text input
- `radio`: Single choice selection from options
- `checkbox`: Multiple choice selection from options
- `confirm`: Yes/No confirmation (returns "true" or "false")

**Configuration Options:**
- `name`: Environment variable name
- `type`: Interactive type (input/radio/checkbox/confirm)
- `prompt`: User prompt text
- `default`: Default value (optional)
- `required`: Whether input is required (default: true)
- `options`: Available choices for radio/checkbox types

All environment variables support variable substitution using the `${VAR_NAME}` syntax.

### Smart Log Matching

Supports multiple log file matching methods:

- Full file name
- File name fragment
- Task ID fragment
- Timestamp matching

### Layered Builder Management

- **Image Layer**: Manage pulling, building, and removing Docker images
- **Container Layer**: Create named builder containers based on docker-compose.yml
- **Lifecycle**: Complete start, stop, and health check processes

## 📄 License

MIT License
