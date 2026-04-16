# Builder Management

Layered management: Image -> Container -> Lifecycle.

## Image Commands

```bash
confkit image list                   # List images
confkit image create <image:tag>     # Pull/build
confkit image remove <image:tag>     # Remove
```

## Container Commands

```bash
confkit builder list                 # List all builders
confkit builder create -n <name>     # Create (from docker-compose.yml)
confkit builder start -n <name>      # Start
confkit builder stop -n <name>       # Stop
confkit builder remove -n <name>     # Remove
confkit builder health               # Health check (all)
confkit builder health -n <name>     # Health check (specific)
```

## Execute Build

```bash
confkit run -s <space> -p <project>              # Run
confkit run -s <space> -p <project> --dry-run    # Preview
confkit run -s <space> -p <project> -e K=V       # With env injection
```
