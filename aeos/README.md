# AEOS - AI Agent Operating System

Advanced Operating System designed specifically for managing and orchestrating AI agents. Built with Rust for high performance, reliability, and concurrency.

## 🚀 Features

- **Agent Kernel**: Core kernel for managing AI agent lifecycle
- **Resource Management**: CPU, memory, and system resource monitoring
- **Task Scheduling**: Advanced task scheduling and execution
- **Inter-Agent Communication**: Message bus for agent-to-agent communication
- **Web Dashboard**: Real-time monitoring and management UI
- **CLI Interface**: Command-line tools for system management
- **RESTful API**: Complete REST API for integration
- **System Monitoring**: Real-time system metrics and statistics

## 🏗️ Architecture

```
AEOS Core System
├── Kernel (Agent Lifecycle Management)
├── Resource Manager (CPU, Memory, I/O)
├── Task Scheduler (Job Scheduling)
├── Message Bus (Inter-Agent Communication)
├── Storage Layer (Database)
└── User Interfaces
    ├── Web Dashboard (http://localhost:8080)
    ├── REST API (/api/*)
    └── CLI (aeos-cli)
```

## 📋 Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

## ⚙️ Installation

### 1. Clone and Build

```bash
cd aeos
cargo build --release
```

### 2. Environment Configuration

Create a `.env` file:

```env
AEOS_PORT=8080
AEOS_HOST=0.0.0.0
DATABASE_URL=sqlite:./aeos.db
RUST_LOG=info
AEOS_MAX_AGENTS=100
```

### 3. Run the System

```bash
# Start the main server
cargo run --release

# Or use the binary directly
./target/release/aeos
```

The web dashboard will be available at `http://localhost:8080`

## 🎯 Quick Start

### Using the Web Dashboard

1. Open http://localhost:8080 in your browser
2. View system status and metrics in real-time
3. Create and manage agents
4. Monitor task execution

### Using the CLI

```bash
# Build CLI tool
cargo build --release --bin aeos-cli

# Check system status
./target/release/aeos-cli status

# List agents
./target/release/aeos-cli agent list

# Create a new agent
./target/release/aeos-cli agent create "MyAgent"

# Create a task
./target/release/aeos-cli task create "MyTask" -a agent-001 -c "echo hello"

# View resources
./target/release/aeos-cli resources

# Show logs
./target/release/aeos-cli logs --lines 100
```

### Using the REST API

```bash
# Get system status
curl http://localhost:8080/api/system/status

# List agents
curl http://localhost:8080/api/agents

# Create agent
curl -X POST http://localhost:8080/api/agents \
  -H "Content-Type: application/json" \
  -d '{"name": "Agent1"}'

# Create task
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name": "Task1", "agent_id": "agent-001", "command": "echo test"}'

# Get resources
curl http://localhost:8080/api/system/resources
```

## 📁 Project Structure

```
aeos/
├── src/
│   ├── main.rs              # Main application entry
│   ├── lib.rs               # Library exports
│   ├── bin/cli.rs           # CLI tool
│   ├── core/                # Kernel and core systems
│   ├── agents/              # Agent management
│   ├── scheduler/           # Task scheduling
│   ├── resources/           # Resource management
│   ├── communication/       # Message bus
│   ├── storage/             # Database layer
│   ├── ui/                  # Web UI handlers
│   └── config/              # Configuration
├── Cargo.toml              # Project manifest
└── README.md               # This file
```

## 🔧 Configuration Options

| Variable | Default | Description |
|----------|---------|-------------|
| AEOS_PORT | 8080 | Web server port |
| AEOS_HOST | 0.0.0.0 | Web server host |
| DATABASE_URL | sqlite:./aeos.db | Database connection string |
| RUST_LOG | info | Log level (debug, info, warn, error) |
| AEOS_MAX_AGENTS | 100 | Maximum concurrent agents |

## 📊 API Endpoints

### Agents
- `GET /api/agents` - List all agents
- `POST /api/agents` - Create new agent
- `GET /api/agents/:id` - Get agent details
- `DELETE /api/agents/:id` - Delete agent
- `POST /api/agents/:id/run` - Run agent
- `POST /api/agents/:id/stop` - Stop agent

### Tasks
- `GET /api/tasks` - List all tasks
- `POST /api/tasks` - Create new task
- `GET /api/tasks/:id` - Get task details
- `DELETE /api/tasks/:id` - Delete task

### System
- `GET /api/system/status` - System status
- `GET /api/system/resources` - Resource usage
- `GET /api/system/logs` - System logs

### Dashboard
- `GET /` - Web dashboard
- `GET /dashboard` - Dashboard page

## 🚀 Development

### Build for Debug
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Create Release Build
```bash
cargo build --release
```

## 📝 CLI Commands Reference

```
aeos-cli status              # Show system status
aeos-cli resources           # Show system resources
aeos-cli logs [--lines N]    # Show system logs

aeos-cli agent list          # List all agents
aeos-cli agent create NAME   # Create new agent
aeos-cli agent show ID       # Show agent details
aeos-cli agent delete ID     # Delete agent
aeos-cli agent run ID        # Run agent
aeos-cli agent stop ID       # Stop agent

aeos-cli task list           # List all tasks
aeos-cli task create NAME \
  -a AGENT_ID \
  -c "command"              # Create task
aeos-cli task show ID        # Show task details
aeos-cli task delete ID      # Delete task
```

## 🔒 Security Notes

- The system is designed for local/internal networks
- CORS is enabled for development (should be restricted in production)
- Implement authentication/authorization for production use
- Secure the database with proper permissions

## 🐛 Troubleshooting

### Port Already in Use
```bash
# Change port via environment variable
AEOS_PORT=8081 cargo run --release
```

### High Memory Usage
- Reduce `AEOS_MAX_AGENTS`
- Check agent task configurations
- Review system logs for memory leaks

### Database Issues
```bash
# Reset database
rm aeos.db
cargo run --release
```

## 📚 Dependencies

Key dependencies:
- **tokio** - Async runtime
- **axum** - Web framework
- **sysinfo** - System information
- **sqlx** - Database access
- **serde** - Serialization
- **uuid** - Unique identifiers
- **chrono** - Date/time handling

See `Cargo.toml` for complete list.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit PRs or issues.

## 📄 License

MIT License - See LICENSE file for details

## 🎓 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Web Framework](https://docs.rs/axum/latest/axum/)
- [System Programming with Rust](https://doc.rust-lang.org/embedded-book/)

## 📞 Support

For issues, questions, or suggestions:
- Open an issue on GitHub
- Check existing documentation
- Review API examples

---

**AEOS v0.1.0** - Powered by Rust & Tokio | Built for AI Agent Management
