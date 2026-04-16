# Getting Started

## Installation

### Quick Install (Recommended)

**Install Latest Version:**

```bash
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh
```

**Install Specific Version:**

```bash
# Method 1: Use command line parameter (Recommended)
curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh | sh -s -- 1.2.3

# Method 2: Use bash process substitution
bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh) 1.2.3

# Method 3: Use environment variable with bash
CONFKIT_VERSION=1.2.3 bash <(curl -fsSL https://raw.githubusercontent.com/confkit/engine/main/install.sh)
```

This will automatically:
- Detect your platform and architecture
- Download the appropriate binary from GitHub releases
- Install to the system binary directory (`/usr/local/bin` on macOS, `/usr/local/bin` or `~/.local/bin` on Linux)
- Add the binary to your PATH automatically

**Version Format Support:**
- `latest` - Install the latest release (default)
- `1.2.3` - Automatically converts to `v1.2.3`
- `v1.2.3` - Use exact version tag

### Supported Platforms

- **Linux**: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin`

### Manual Installation

If you prefer to build from source:

```bash
git clone <repository-url>
cd confkit/engine
cargo build --release
```

### Verify Installation

```bash
confkit --help
```

## Quick Start

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

### Basic Usage

```bash
# View help
confkit --help

# View version
confkit -v

# Interactive mode (recommended for beginners)
confkit

# Manage builders
confkit builder list
confkit builder create -n golang-builder
confkit builder start -n golang-builder

# Run build tasks
confkit run --space hello --project hello-app

# Preview steps without executing (dry run)
confkit run --space hello --project hello-app --dry-run

# Inject environment variables via command line
confkit run --space hello --project hello-app -e KEY1=value1 -e KEY2=value2

# View logs (space/project are optional filters, supports pagination)
confkit log list
confkit log list --space hello --project hello-app --page 1 --size 10
confkit log show --task <task_id>
confkit log info --task <task_id>

# Clean log files
confkit log clean --all
confkit log clean --space hello
confkit log clean --space hello --project hello-app
confkit log clean --task <task_id>

# View/validate configuration
confkit config show
confkit config validate
```
