# Builder Management

ConfKit provides layered management for Docker images and containers.

## Layered Architecture

- **Image Layer**: Pull, build, and remove Docker images
- **Container Layer**: Create named builder containers based on docker-compose.yml
- **Lifecycle**: Complete start, stop, and health check workflows

---

## Image Management

```bash
# List images
confkit image list

# Pull/build image
confkit image create golang:1.24

# Remove image
confkit image remove golang:1.24
```

## Container Management

```bash
# List all builder statuses
confkit builder list

# Create builder (based on docker-compose.yml)
confkit builder create -n golang-builder

# Start/stop builder
confkit builder start -n golang-builder
confkit builder stop -n golang-builder

# Remove builder
confkit builder remove -n golang-builder

# Health check (all containers)
confkit builder health

# Health check (specific container)
confkit builder health -n golang-builder
```

## Execute Build

```bash
# Build project
confkit run --space <space_name> --project <project_name>

# Preview steps without executing
confkit run --space <space_name> --project <project_name> --dry-run
```
