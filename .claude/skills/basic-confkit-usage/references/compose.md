# Compose Configuration (`confkit-compose.yml`)

The compose file defines builder containers. It is a standard Docker Compose file that ConfKit uses to manage container lifecycle.

## Example

```yml
services:
  golang-builder:
    image: golang:1.24
    working_dir: /workspace
    volumes:
      - ./volumes/workspace:/workspace
      - ./volumes/artifacts:/artifacts
      - ./volumes/cache:/cache
    environment:
      - TZ=UTC

  rust-builder:
    image: rust:1.88-alpine
    working_dir: /workspace
    volumes:
      - ./volumes/workspace:/workspace
      - ./volumes/artifacts:/artifacts
    environment:
      - TZ=UTC
```

## How It Works

- Each service becomes a named builder container
- Service name is used as the `container` value in project step configs
- ConfKit manages the lifecycle: create, start, stop, remove, health check
- Containers are grouped under the `engine_compose.project` name (default: `confkit`)

## Service Fields

ConfKit reads the following fields from each service:

| Field | Type | Description |
|-------|------|-------------|
| `image` | String | Base image for the container |
| `container_name` | String | Optional explicit container name |
| `working_dir` | String | Default working directory |
| `volumes` | Array | Host-to-container mount mappings |
| `ports` | Array | Port mappings |
| `environment` | Map | Container environment variables |
| `depends_on` | Array | Service dependencies |

All other Docker Compose fields (networks, restart policies, etc.) are preserved but not directly managed by ConfKit.

## Volume Mounts

The standard mount convention:

| Host Path | Container Path | Purpose |
|-----------|---------------|---------|
| `./volumes/workspace` | `/workspace` | Source code and build input |
| `./volumes/artifacts` | `/artifacts` | Build output and deliverables |
| `./volumes/cache` | `/cache` | Persistent build cache |
| `./volumes/temp` | `/tmp` | Temporary files (optional) |

These paths align with the system variables:
- `HOST_WORKSPACE_DIR` -> `CONTAINER_WORKSPACE_DIR` (`/workspace`)
- `HOST_ARTIFACTS_ROOT_DIR` -> `CONTAINER_ARTIFACTS_ROOT_DIR` (`/artifacts`)

## Relationship to Main Config

The compose file is referenced in `.confkit.yml`:

```yml
engine_compose:
  project: confkit           # Compose project name (default)
  file: ./confkit-compose.yml
```
