# space_mship

## Overview

"space_mship is a comprehensive development framework for building and managing scalable microservice architectures. It provides a unified CLI interface, automated service generation, and Docker orchestration for Rust-based microservices."

This framework is designed for Unix-based systems and requires Docker for containerization.

## Features

- **Service Generation**: Automatically create new microservices from templates
- **Unified CLI**: Single command interface for all microservice operations
- **Docker Orchestration**: Automated container management with networking
- **Rust Workspace**: Integrated Cargo workspace with shared dependencies
- **Service Discovery**: Built-in service registry and port management
- **Development Tools**: Git, Docker, and debugging utilities included
- **Container-aware Execution**: Automatic detection of host vs container context

## Quick Start

space_mship comes with a default `web` microservice pre-configured as an example and starting point.

### Create a new microservice
```bash
./cli.sh commands/make/service/new.sh
```

### Manage services
```bash
# Build a service (e.g., the default web service)
./cli.sh commands/services/web/container-management/build.sh

# Start a service
./cli.sh commands/services/web/container-management/start.sh

# View logs
./cli.sh commands/services/web/container-management/logs-tail.sh

# Execute commands inside container
./cli.sh commands/services/web/in-container/start-service.sh
```

### Development tools
```bash
# List Docker containers
./cli.sh commands/tools/docker/ls.sh

# Clean Docker environment
./cli.sh commands/tools/docker/clean.sh

# Git commit helpers
./cli.sh commands/tools/git/commit/amend-keepmessage.sh
```

## Running commands

The `cli.sh` script provides intelligent command execution with automatic context detection:

### Host vs Container Execution
- **Host commands**: Most commands run on the host machine by default
- **Container commands**: Commands in `./commands/services/<service-name>/in-container/...` automatically execute inside the corresponding Docker container
- **Automatic routing**: The CLI detects the command path and determines the execution context
- **Interactive mode**: Container commands maintain TTY and interactive mode for proper signal handling (Ctrl+C)

### Container Shell Access
Access the shell of any running service container:
```bash
./cli.sh commands/services/web/container-management/shell.sh
```
This opens an interactive shell inside the service's Docker container, allowing for debugging, manual testing, or direct service interaction.

### Environment Management
- Automatically loads environment variables from `.env` file
- Passes environment context to both host and container commands
- Service-specific environment files are supported

### Custom Commands
You can extend the framework by adding your own commands:
- Create new `.sh` files in the `./commands/` directory structure
- Follow the existing naming conventions
- Commands are automatically discovered and available via `./cli.sh`
- System commands (paths containing `/_`) are hidden from the help listing

## Project Structure

```
your-project/
├── cli.sh                          # Main command interface
├── commands/                       # All available commands
│   ├── make/service/               # Service creation tools
│   ├── services/                   # Per-service management
│   │   └── web/                    # Default web service (included)
│   └── tools/                      # Development utilities
├── crates/                         # Rust workspace
│   ├── common/                     # Shared libraries
│   │   ├── httpserver/             # HTTP server utilities
│   │   └── services_manager/       # Service discovery
│   └── services/                   # Your microservices
│       └── web/                    # Default web service (included)
├── docker/                         # Docker configurations
│   └── web/                        # Default web service config (included)
└── .env                            # Environment configuration
```

## How to integrate space_mship into your project

To integrate space_mship without overwriting existing files:

1. Add as a remote:
```bash
git remote add space_mship https://github.com/spacecodeur/space_mship.git
```

2. Fetch changes:
```bash
git fetch space_mship
```

3. Merge with preservation of existing files:
```bash
git merge --allow-unrelated-histories space_mship/main -X ours
```

4. Commit the merge:
```bash
git commit -am "Integrated space_mship microservices framework"
```

5. Configure your project:
   - Set the `APP_NAME` in the `.env` file
   - Install Rust and Docker if not already available
   - Run `./cli.sh` to see available commands

## Microservice Development Workflow

### Creating a new service
1. Run `./cli.sh commands/make/service/new.sh`
2. Enter service name in snake_case (e.g., `user_management`)
3. Framework automatically:
   - Creates Rust crate with Axum web server
   - Generates Docker configuration
   - Updates Cargo workspace
   - Creates management scripts
   - Assigns unique port

### Service Architecture
Each microservice includes:
- **Rust crate** with Axum HTTP server
- **Docker container** for isolation
- **Management scripts** for build/start/stop/logs
- **Automatic service discovery** via shared registry
- **Inter-service networking** through Docker network

### Development Best Practices
- Use `./cli.sh` commands for all operations
- Implement business logic in `crates/services/{name}/src/`
- Share common code via `crates/common/` libraries
- Configure services through environment variables
- Test services individually using container management scripts

## Requirements

- **Docker**: For containerization and orchestration
- **Rust**: For microservice development (automatically managed in containers)
- **Unix-based OS**: Linux or macOS (Windows support limited)

## Getting Help

Run `./cli.sh` without arguments to see all available commands.