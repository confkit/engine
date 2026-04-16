# Main Configuration (`.confkit.yml`)

Root-level configuration file.

## Example

```yml
version: 1.0.0
engine: docker

shell:
  host: bash
  container: bash

engine_compose:
  file: ./confkit-compose.yml

spaces:
  - name: confkit
    description: "ConfKit toolchain release space"
    path: .confkit/spaces/confkit
  - name: hello
    description: "Hello ConfKit"
    path: .confkit/spaces/hello

images:
  - name: hello-builder
    base_image: alpine
    tag: 3.18
    context: volumes/context
    engine_file: ./.confkit/images/Dockerfile.alpine:3.18
```

## Field Reference

### `version`

- Type: String
- Required: Yes
- Config file version. Currently `1.0.0`.

### `engine`

- Type: `docker` | `podman`
- Required: Yes
- Container engine for all operations.

### `shell`

- Type: Object
- Required: No

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | `bash` \| `zsh` | `bash` | Host shell |
| `container` | `bash` \| `zsh` | `bash` | Container shell |

### `engine_compose`

- Type: Object
- Required: Yes

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `project` | String | `confkit` | Container group name |
| `file` | String | -- | Path to docker-compose.yml |

### `spaces`

- Type: Array
- Required: Yes

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Space identifier |
| `description` | String | Description |
| `path` | String | Directory containing project YAML files |

### `images`

- Type: Array
- Required: No

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Target image name |
| `base_image` | String | Base image (auto-pull) |
| `tag` | String | Tag (shared by base and target) |
| `context` | String | Build context directory |
| `engine_file` | String | Dockerfile path |
