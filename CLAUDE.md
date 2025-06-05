# Microservices Development with space_mship Framework

## About space_mship Integration

This project uses [**space_mship**](https://github.com/spacecodeur/space_mship), a development framework for building and managing scalable microservice architectures. The framework provides a standardized command-line interface (`./cli.sh`) for managing the entire lifecycle of microservices in our distributed system.

## How space_mship Works in This Project

### Command Interface (`./cli.sh`)
The main entry point for all development operations. This script:
- Manages microservice lifecycle (build, start, stop, logs)
- Handles container orchestration with Docker
- Provides development tools for git, docker, and debugging
- Automatically detects whether commands should run on host or inside containers

### Available Command Categories

#### Microservice Management
```bash
# Create a new microservice
./cli.sh commands/make/service/new.sh

# Build a specific service
./cli.sh commands/services/{service_name}/container-management/build.sh

# Start/stop services
./cli.sh commands/services/{service_name}/container-management/start.sh
./cli.sh commands/services/{service_name}/container-management/stop.sh

# View logs
./cli.sh commands/services/{service_name}/container-management/logs-tail.sh

# Execute commands inside service containers
./cli.sh commands/services/{service_name}/in-container/{command}.sh
```

#### Development Tools
```bash
# Docker management
./cli.sh commands/tools/docker/ls.sh
./cli.sh commands/tools/docker/clean.sh

# Git helpers
./cli.sh commands/tools/git/commit/amend-keepmessage.sh

# Debug prompt generation
./cli.sh commands/tools/prompt/generate-for-debug.sh
```

### Project Structure Created by space_mship

```
project/
├── cli.sh                          # Main command interface
├── commands/                       # All available commands
│   ├── make/service/               # Service creation tools
│   ├── services/                   # Per-service management commands
│   └── tools/                      # Development utilities
├── crates/                         # Rust workspace
│   ├── common/                     # Shared libraries
│   └── services/                   # Your microservices
├── docker/                         # Docker configurations per service
└── .env                            # Environment configuration
```

### Microservice Architecture

space_mship comes with a default `web` microservice already configured, providing:
- A working example of the framework structure
- Basic HTTP server using Axum
- Ready-to-use Docker configuration
- All management scripts pre-configured

Each microservice in this project:
- Runs in its own Docker container for isolation
- Is implemented in Rust using the Axum web framework
- Has standardized build, deployment, and management scripts
- Communicates with other services via a shared Docker network
- Is automatically registered in the service discovery system

### Service Creation Workflow

When adding a new microservice:
1. Run `./cli.sh commands/make/service/new.sh`
2. Provide a service name in snake_case
3. space_mship automatically:
   - Creates service skeleton code
   - Updates Cargo workspace configuration
   - Generates Docker configurations
   - Creates management scripts
   - Assigns a unique port
   - Updates service registry

### Development Best Practices with space_mship

- **Service Isolation**: Each service is developed and deployed independently
- **Standardized Commands**: Use the provided cli.sh commands for consistency
- **Container-First**: All services run in Docker for development parity with production
- **Shared Libraries**: Use `crates/common/` for code shared between services
- **Environment Configuration**: All config goes through `.env` file

### Common Development Tasks

- **Adding a feature**: Create new service or modify existing service in `crates/services/`
- **Inter-service communication**: Use the `services_manager` crate for service discovery
- **Debugging**: Use container shell access via `./cli.sh commands/services/{name}/container-management/shell.sh`
- **Logs**: Monitor with `./cli.sh commands/services/{name}/container-management/logs-tail.sh`

## Integration Notes

This project leverages space_mship to focus on business logic rather than infrastructure setup. The framework handles:
- Docker orchestration and networking
- Service lifecycle management
- Development tooling and scripts
- Rust workspace configuration
- Port management and service discovery

When working on this project, use the provided `./cli.sh` commands for all microservice operations to maintain consistency with the space_mship conventions.