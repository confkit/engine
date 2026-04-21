# Configuration Reference

ConfKit uses YAML configuration files to define build processes. There are two levels of configuration: the **main config** (`.confkit.yml`) and **project configs** (per-space YAML files).

## Main Configuration (`.confkit.yml`)

The main configuration file sits at the root of your project.

```yml
# .confkit.yml
version: 1.0.0

# Container engine: docker/podman
engine: docker

# Shell types: bash/zsh
shell:
  host: bash
  container: bash

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

### Field Reference

#### `version`

- **Type**: String
- **Required**: Yes
- **Description**: Configuration file version. Currently `1.0.0`.

#### `engine`

- **Type**: String (`docker` | `podman`)
- **Required**: Yes
- **Description**: Container engine to use for all operations.

#### `shell`

- **Type**: Object
- **Required**: No
- **Description**: Shell configuration for command execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | String | `bash` | Shell type on the host machine |
| `container` | String | `bash` | Shell type inside containers |

#### `engine_compose`

- **Type**: Object
- **Required**: Yes
- **Description**: Docker Compose configuration for builder containers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `project` | String | `confkit` | Container group name |
| `file` | String | — | Path to docker-compose.yml file |

#### `spaces`

- **Type**: Array of Objects
- **Required**: Yes
- **Description**: List of workspace definitions.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Space identifier |
| `description` | String | Space description |
| `path` | String | Path to directory containing project YAML files |

#### `images`

- **Type**: Array of Objects
- **Required**: No
- **Description**: List of Docker images to manage.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Target image name |
| `base_image` | String | Base image to use (auto-pulled) |
| `tag` | String | Image tag (shared by base and target) |
| `context` | String | Build context directory |
| `engine_file` | String | Path to Dockerfile |

### `print_environment`

- **Type**: Boolean
- **Required**: No
- **Default**: `false`
- **Description**: Whether to print environment variables in task logs. Can be overridden per project in the project YAML.

---

## Project Configuration

Each space contains one or more project YAML files that define build steps.

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

environment_from_args:
  - name: "ENVIRONMENT"
    type: "radio"
    prompt: "Select deployment environment"
    default: "staging"
    required: true
    options:
      - "development"
      - "staging"
      - "production"

steps:
  - name: "Build Application"
    container: "golang-builder"
    working_dir: "/workspace"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "echo 'Building ${APP_NAME} v${BUILD_VERSION}'"
      - "echo 'Git Hash: ${GIT_HASH}'"
      - "go build -o app ./main.go"
    timeout: 300
    continue_on_error: true

  - name: "Run Tests"
    container: "golang-builder"
    working_dir: "/workspace"
    condition: "${ENVIRONMENT} != 'production'"
    commands:
      - "go test ./..."
    timeout: 180
```

### Field Reference

#### `name`

- **Type**: String
- **Required**: Yes
- **Description**: Project name. Used as the `PROJECT_NAME` environment variable.

#### `description`

- **Type**: String
- **Required**: No
- **Description**: Project description.

#### `source`

- **Type**: Object
- **Required**: No
- **Description**: Git source configuration.

| Field | Type | Description |
|-------|------|-------------|
| `git_repo` | String | Git repository URL |
| `git_branch` | String | Branch name (from config or current branch) |

#### `environment_files`

- **Type**: Array of Objects
- **Required**: No
- **Description**: External environment variable files to load.

| Field | Type | Description |
|-------|------|-------------|
| `format` | String | File format: `yaml` or `env` |
| `path` | String | Path to environment file |

#### `format` Details

- `yaml`: Standard YAML key-value map (`KEY: "value"`)
- `env`: Line-based `.env` format (`KEY=VALUE`), supports `#` comments and blank lines

#### `environment`

- **Type**: Map (String → String)
- **Required**: No
- **Description**: Static environment variables. Supports `${VAR_NAME}` substitution.

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

#### `environment_from_args`

- **Type**: Array of Objects
- **Required**: No
- **Description**: Interactive environment variable definitions. See [Variables Guide](variables.md#interactive-environment-variables) for details.

#### `print_environment`

- **Type**: Boolean
- **Required**: No
- **Description**: Whether to print environment variables in task logs. Overrides `.confkit.yml` global setting. Default: `false`.

#### `steps`

- **Type**: Array of Objects
- **Required**: Yes
- **Description**: Build steps to execute. See below for step fields.

### Step Fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | String | Yes | — | Step display name |
| `container` | String | Yes | — | Builder container to run in |
| `working_dir` | String | No | `/workspace` | Working directory inside container |
| `commands` | Array\<String\> | Yes | — | List of commands to execute |
| `condition` | String | No | — | Conditional expression. See [Conditions Guide](conditions.md) |
| `timeout` | Number | No | — | Step timeout in seconds |
| `continue_on_error` | Boolean | No | `false` | Continue to next step on failure |

All commands support `${VAR_NAME}` variable substitution.
