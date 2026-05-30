# OS Development Workspace

This workspace contains two operating system projects written in Rust, each serving different purposes and architectural goals.

## 📁 Projects

### 🖥️ RustyOS
A bare-metal operating system for x86-64 architecture.

**Purpose**: Educational kernel development project demonstrating low-level OS concepts.

**Key Features**:
- Custom bootloader integration (UEFI)
- x86-64 kernel with memory management
- Interrupt handling (IDT, GDT, PIC)
- VGA text mode display
- Serial port communication
- Keyboard input handling
- Task scheduling foundation

**Architecture**: Bare-metal, runs directly on hardware without an underlying OS.

**Target**: x86-64 (Intel/AMD compatible)

**Location**: `./RustyOS/`

**Documentation**: [RustyOS README](./RustyOS/README.md)

---

### 🤖 AEOS (AI Agent Operating System)
A high-level operating system for managing and orchestrating AI agents.

**Purpose**: Production-ready system for AI agent lifecycle management with web interface.

**Key Features**:
- Agent kernel for lifecycle management
- Resource management (CPU, memory, I/O)
- Task scheduling and execution
- Inter-agent communication via message bus
- Web dashboard for real-time monitoring
- RESTful API for integration
- CLI interface for system management
- SQLite database for persistence

**Architecture**: Hosted application running on top of a host OS (Linux/Windows/macOS).

**Tech Stack**: Rust + Tokio + Axum + SQLite

**Location**: `./aeos/`

**Documentation**: [AEOS README](./aeos/README.md)

---

## 🔄 Comparison

| Aspect | RustyOS | AEOS |
|--------|---------|------|
| **Type** | Bare-metal kernel | Hosted application |
| **Architecture** | x86-64 bare-metal | Cross-platform (host OS) |
| **Purpose** | Educational/Low-level | Production AI agent management |
| **Interface** | VGA text mode, Serial | Web dashboard, REST API, CLI |
| **Concurrency** | No_std, manual | Tokio async runtime |
| **Storage** | Memory only | SQLite database |
| **Networking** | Not implemented | HTTP server (Axum) |

---

## 🚀 Quick Start

### RustyOS
```bash
cd RustyOS
make build
make run
```

### AEOS
```bash
cd aeos
cargo build --release
cargo run --release
# Web dashboard at http://localhost:8080
```

---

## 📊 Workspace Structure

```
os/
├── RustyOS/              # Bare-metal x86-64 OS
│   ├── src/
│   │   ├── main.rs       # Kernel entry point
│   │   ├── gdt.rs        # Global Descriptor Table
│   │   ├── interrupts.rs # Interrupt handling
│   │   ├── memory.rs     # Memory management
│   │   ├── vga.rs        # VGA display
│   │   ├── serial.rs     # Serial communication
│   │   ├── keyboard.rs   # Keyboard input
│   │   ├── allocator.rs  # Heap allocator
│   │   └── task.rs       # Task scheduling
│   ├── Cargo.toml
│   ├── Makefile
│   └── README.md
│
└── aeos/                 # AI Agent Operating System
    ├── src/
    │   ├── main.rs       # Web server entry
    │   ├── core/         # Kernel and core systems
    │   ├── agents/       # Agent management
    │   ├── scheduler/    # Task scheduling
    │   ├── resources/    # Resource management
    │   ├── communication/# Message bus
    │   ├── storage/      # Database layer
    │   ├── ui/           # Web UI handlers
    │   └── config/       # Configuration
    ├── Cargo.toml
    └── README.md
```

---

## 🛠️ Development Requirements

### Common Requirements
- Rust 1.70+
- Cargo

### RustyOS Specific
- QEMU (for testing)
- `bootimage` tool: `cargo install bootimage`
- Rust nightly toolchain: `rustup toolchain install nightly`

### AEOS Specific
- SQLite3
- No additional system dependencies required

---

## 📖 Learning Path

### For Low-Level OS Development
Start with **RustyOS** to learn:
- x86-64 architecture
- Memory management and paging
- Interrupt handling
- Hardware drivers
- Boot process

### For Systems Programming with Rust
Explore **AEOS** to learn:
- Async programming with Tokio
- Web framework (Axum)
- Resource management
- Agent-based systems
- Database integration

---

## 🎯 Use Cases

### RustyOS
- OS development education
- Understanding computer architecture
- Learning Rust's `no_std` ecosystem
- Embedded systems concepts

### AEOS
- Managing AI agent workloads
- Task scheduling and orchestration
- Building agent-based applications
- System monitoring and resource management
- RESTful API development

---

## 📚 Additional Documentation

- [RustyOS Architecture](./RustyOS/ARCHITECTURE.md)
- [RustyOS Development Progress](./RustyOS/PROGRESS.md)
- [RustyOS Quick Start](./RustyOS/QUICKSTART.md)
- [RustyOS Development Guide](./RustyOS/DEVELOPMENT.md)

---

## 🤝 Contributing

Both projects are open for contributions. Please refer to individual project READMEs for contribution guidelines.

---

## 📄 License

Each project has its own license. Please refer to individual project directories for license information.

---

**Last Updated**: May 30, 2026
