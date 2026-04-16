# Environment Variables

All variables support `${VAR_NAME}` substitution syntax.

## Priority (high to low)

1. Command line (`-e KEY=VALUE`)
2. Interactive (`environment_from_args`)
3. Custom (`environment`)
4. Files (`environment_files`)
5. Git variables (auto-injected)
6. System variables (auto-injected)

## System Variables

| Variable | Description |
|----------|-------------|
| `TASK_ID` | Unique task ID (e.g. `20250113-143022-a1b2c3`) |
| `PROJECT_NAME` | Project name from config |
| `PROJECT_VERSION` | Version from remote repo (package.json / Cargo.toml) |
| `SPACE_NAME` | Space name |
| `HOST_WORKSPACE_DIR` | Host workspace directory |
| `CONTAINER_WORKSPACE_DIR` | Container workspace directory |
| `HOST_ARTIFACTS_ROOT_DIR` | Host artifacts root |
| `CONTAINER_ARTIFACTS_ROOT_DIR` | Container artifacts root |

## Git Variables

Injected when `source` block is configured:

| Variable | Description |
|----------|-------------|
| `GIT_REPO` | Repository URL from config |
| `GIT_BRANCH` | Branch name |
| `GIT_HASH` | Full commit hash |
| `GIT_HASH_SHORT` | Short hash (first 8 chars) |

## Custom Variables

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

## CLI Injection

```bash
confkit run -s hello -p hello-app -e ENVIRONMENT=production -e BUILD_VERSION=2.0.0
```

Format: `KEY=VALUE`. Malformed entries skipped with warning. Highest priority.

## Interactive Variables

Defined in `environment_from_args`. Users are prompted during execution.

### Types

| Type | Description |
|------|-------------|
| `input` | Free text input |
| `radio` | Single choice from options |
| `checkbox` | Multiple choice from options |
| `confirm` | Yes/No (returns `"true"` or `"false"`) |

### Field Options

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | String | Yes | -- | Variable name |
| `type` | String | Yes | -- | `input` / `radio` / `checkbox` / `confirm` |
| `prompt` | String | Yes | -- | User prompt text |
| `default` | String | No | -- | Default value |
| `required` | Boolean | No | `true` | Whether required |
| `options` | Array\<String\> | No | -- | Choices for `radio` / `checkbox` |

### Example

```yaml
environment_from_args:
  - name: "API_URL"
    type: "input"
    prompt: "Please enter API URL"
    default: "https://api.example.com"
    required: true

  - name: "ENVIRONMENT"
    type: "radio"
    prompt: "Select environment"
    default: "staging"
    required: true
    options:
      - "development"
      - "staging"
      - "production"

  - name: "FEATURES"
    type: "checkbox"
    prompt: "Select features"
    default: "auth"
    required: false
    options:
      - "auth"
      - "logging"
      - "metrics"

  - name: "ENABLE_DEBUG"
    type: "confirm"
    prompt: "Enable debug mode?"
    default: "false"
    required: false
```
