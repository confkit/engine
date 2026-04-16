# Environment Variables

ConfKit provides multiple ways to define and inject environment variables. All variables support `${VAR_NAME}` substitution syntax.

## Variable Priority

From highest to lowest:

1. **Command line injection** (`-e KEY=VALUE`)
2. **Interactive environment variables** (`environment_from_args`)
3. **Custom environment variables** (`environment`)
4. **Environment files** (`environment_files`)
5. **Git variables** (auto-injected)
6. **System variables** (auto-injected)

---

## System Variables

Auto-injected by ConfKit during task execution:

| Variable | Description | Example |
|----------|-------------|---------|
| `TASK_ID` | Unique task identifier | `20250113-143022-a1b2c3` |
| `PROJECT_NAME` | Project name from config file | `hello-app` |
| `PROJECT_VERSION` | Project version from remote repo (package.json / Cargo.toml) | `1.0.0` |
| `SPACE_NAME` | Space name | `hello` |
| `TASK_WORKSPACE_DIR` | Task workspace directory name | `hello--app-a0eb394d` |
| `HOST_VOLUMES_DIR` | Host volumes root directory | `volumes` |
| `HOST_WORKSPACE_DIR` | Host task workspace directory | `volumes/workspace/...` |
| `CONTAINER_WORKSPACE_DIR` | Container task workspace directory | `/workspace/...` |
| `HOST_ARTIFACTS_ROOT_DIR` | Host artifact root directory | `volumes/artifacts` |
| `CONTAINER_ARTIFACTS_ROOT_DIR` | Container artifact root directory | `/artifacts` |

## Git Variables

Auto-injected when a `source` block is configured:

| Variable | Description | Example |
|----------|-------------|---------|
| `GIT_REPO` | Git repository URL from config | `https://github.com/example/repo.git` |
| `GIT_BRANCH` | Git branch name (from config or current) | `main` |
| `GIT_HASH` | Full commit hash | `a1b2c3d4e5f6...` |
| `GIT_HASH_SHORT` | Short commit hash (first 8 chars) | `a1b2c3d4` |

---

## Custom Variables

Define static environment variables in the `environment` section of your project config:

```yaml
environment:
  APP_NAME: "my-app"
  BUILD_VERSION: "1.0.0"
  CUSTOM_VAR: "${PROJECT_NAME}-${GIT_HASH_SHORT}"
```

Variables can reference other variables using `${VAR_NAME}` syntax.

## Environment Files

Load variables from external files via the `environment_files` section:

```yaml
environment_files:
  - format: "yaml"
    path: "./volumes/environment.yml"
```

---

## Command Line Injection

Inject environment variables at runtime via `-e` / `--environments` flag:

```bash
confkit run --space hello --project hello-app \
  -e ENVIRONMENT=production \
  -e BUILD_VERSION=2.0.0 \
  -e API_URL=https://api.example.com
```

- Format: `KEY=VALUE`
- Malformed entries are skipped with a warning
- Highest priority — overrides all other sources

---

## Interactive Environment Variables

Define interactive prompts in the `environment_from_args` section. Users are prompted during task execution.

### Supported Types

#### `input` - Free Text Input

```yaml
- name: "API_URL"
  type: "input"
  prompt: "Please enter API URL"
  default: "https://api.example.com"
  required: true
```

#### `radio` - Single Choice

```yaml
- name: "ENVIRONMENT"
  type: "radio"
  prompt: "Select deployment environment"
  default: "staging"
  required: true
  options:
    - "development"
    - "staging"
    - "production"
```

#### `checkbox` - Multiple Choice

```yaml
- name: "FEATURES"
  type: "checkbox"
  prompt: "Select features to enable"
  default: "auth"
  required: false
  options:
    - "auth"
    - "logging"
    - "metrics"
```

#### `confirm` - Yes/No Confirmation

```yaml
- name: "ENABLE_DEBUG"
  type: "confirm"
  prompt: "Enable debug mode?"
  default: "false"
  required: false
```

Returns `"true"` or `"false"` as the value.

### Configuration Options

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | String | Yes | — | Environment variable name |
| `type` | String | Yes | — | Interactive type: `input` / `radio` / `checkbox` / `confirm` |
| `prompt` | String | Yes | — | User prompt text |
| `default` | String | No | — | Default value |
| `required` | Boolean | No | `true` | Whether input is required |
| `options` | Array\<String\> | No | — | Available choices (for `radio` / `checkbox`) |
