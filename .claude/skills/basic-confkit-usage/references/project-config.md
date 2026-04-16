# Project Configuration

Each space contains project YAML files defining build steps.

## Example

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

## Top-Level Fields

### `name`

- Type: String
- Required: Yes
- Project name. Used as `PROJECT_NAME` environment variable.

### `description`

- Type: String
- Required: No

### `source`

- Type: Object
- Required: No

| Field | Type | Description |
|-------|------|-------------|
| `git_repo` | String | Repository URL |
| `git_branch` | String | Branch name |

### `environment_files`

- Type: Array
- Required: No
- External env files to load.

| Field | Type | Description |
|-------|------|-------------|
| `format` | String | File format (`yaml`) |
| `path` | String | File path |

### `environment`

- Type: Map (String -> String)
- Required: No
- Static environment variables. Supports `${VAR_NAME}` substitution.

```yaml
environment:
  APP_NAME: "my-app"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

### `environment_from_args`

- Type: Array
- Required: No
- Interactive variable definitions. See `references/variables.md` for full details.

### `steps`

- Type: Array
- Required: Yes
- Build step list. See Step Fields below.

## Step Fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | String | Yes | -- | Display name |
| `container` | String | Yes | -- | Builder container name |
| `working_dir` | String | No | `/workspace` | Container working directory |
| `commands` | Array\<String\> | Yes | -- | Commands to execute |
| `condition` | String | No | -- | Condition expression. See `references/conditions.md` |
| `timeout` | Number | No | -- | Timeout in seconds |
| `continue_on_error` | Boolean | No | `false` | Continue on failure |

All commands support `${VAR_NAME}` substitution.
