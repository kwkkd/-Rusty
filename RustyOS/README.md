# RustyOS 🖥️

A modern, fully-featured operating system written entirely in **Rust** for x86-64 architecture.

## 🎯 Project Overview

RustyOS is an educational and functional operating system that demonstrates:
- Custom bootloader integration
- x86-64 kernel development
- Memory management (paging, heap allocation)
- Interrupt handling (IDT, GDT, PIC)
- Keyboard and serial input
- VGA text mode display
- Task scheduling foundation

## ✨ Features

### Core Components
- **Bootloader** - Using `bootloader 0.9` crate for UEFI booting
- **Memory Management**
  - Virtual memory with paging
  - Heap allocation with bump allocator
  - Frame allocator for physical memory
- **CPU Management**
  - Global Descriptor Table (GDT)
  - Interrupt Descriptor Table (IDT)
  - Programmable Interrupt Controller (PIC8259)
- **I/O Subsystem**
  - Serial port communication (UART 16550)
  - VGA text mode (80x25)
  - Keyboard input handling
- **Interrupts & Exceptions**
  - Double fault handling
  - Page fault handling
  - Timer interrupts
  - Keyboard interrupts

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with `nightly` toolchain
- QEMU (for testing)
- bootimage tool

### Installation

```bash
# Clone or open the project
cd RustyOS

# Install Rust nightly (if not already installed)
rustup toolchain install nightly
rustup component add llvm-tools-embedded

# Install bootimage
cargo install bootimage
```

### Building

```bash
# Build and create bootable image
make build

# Or using cargo directly
cargo build --release
bootimage build --release
```

### Running

```bash
# Run in QEMU
make run

# Run with KVM acceleration
make kvm

# Manual QEMU invocation
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin
```

## 📁 Project Structure

```
RustyOS/
├── src/
│   ├── main.rs           # Kernel entry point
│   ├── gdt.rs            # Global Descriptor Table setup
│   ├── interrupts.rs     # Interrupt handling (IDT, PIC)
│   ├── memory.rs         # Memory management & allocator
│   ├── vga.rs            # VGA text mode driver
│   ├── serial.rs         # Serial port communication
│   ├── keyboard.rs       # Keyboard input handling
│   ├── allocator.rs      # Heap allocator
│   └── task.rs           # Task scheduling
├── .cargo/config.toml    # Cargo build configuration
├── Cargo.toml            # Project dependencies
├── Makefile              # Build automation
├── build.sh              # Build script
└── README.md             # This file
```

## 🛠️ Development

### Building Variants
```bash
# Debug build
cargo build --target x86_64-unknown-none

# Release build with optimizations
cargo build --release

# Check code without building
cargo check
```

### Testing
```bash
# Run unit tests
cargo test --lib

# Integration tests (in tests/ directory)
make test
```

### Debugging
```bash
# Run with GDB support
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin \
                   -s -S &
# In another terminal:
gdb -ex "file target/x86_64-unknown-none/release/kernel" \
    -ex "target remote :1234"
```

## 📊 System Specifications

| Component | Specification |
|-----------|---|
| **Architecture** | x86-64 (Intel/AMD compatible) |
| **Boot Mode** | UEFI via Bootloader 0.9 |
| **Memory** | Paging enabled, heap allocation |
| **Display** | VGA text mode (80x25) |
| **Interrupts** | IDT with 256 entries |
| **Terminal** | Serial port (UART 16550) |
| **Keyboard** | PS/2 keyboard support |

## 🎮 Keyboard Controls

| Key | Function |
|-----|----------|
| `Ctrl+C` | Break (if supported) |
| `Enter` | Line break |
| `Backspace` | Delete character |

## 📦 Dependencies

### Build Dependencies
- `bootloader` v0.9 - Bootloader integration
- `x86_64` v0.14 - x86-64 architecture support
- `volatile` v0.4 - Volatile memory access
- `uart_16550` v0.2 - Serial port driver
- `pic8259` v0.10 - Programmable interrupt controller
- `pc-keyboard` v0.7 - Keyboard layout handling
- `lazy_static` v1.4 - Static initialization
- `spin` v0.9 - Spinlock implementation
- `linked-list-allocator` v0.10 - Heap allocator

## 🔧 Configuration

### Environment Variables
```bash
# Set log level (debug, info, warn, error)
RUST_LOG=debug

# Custom build target
RUST_TARGET=x86_64-unknown-none
```

### Build Configuration (.cargo/config.toml)
- Target: `x86_64-unknown-none`
- Linker: LLD (via bootimage)
- Linker flags: No standard startup files

## 📖 Learning Resources

- [OSDev.org](https://wiki.osdev.org/) - Operating System Development Wiki
- [Writing an OS in Rust](https://os.phil-opp.com/) - Excellent tutorial
- [x86-64 Documentation](https://en.wikipedia.org/wiki/X86-64) - Architecture reference
- [Intel 64 Manual](https://www.intel.com/content/dam/develop/external/us/en/documents/manual/64-ia-32-architectures-software-developer-manual-combined-volumes-1-2a-2b-2c-2d-3a-3b-3c-3d-and-4.pdf)

## 🐛 Troubleshooting

### Build Fails
```bash
# Update Rust toolchain
rustup update
rustup component add llvm-tools-embedded

# Clean and rebuild
cargo clean
cargo build --release
```

### QEMU Not Found
```bash
# Install QEMU
# Ubuntu/Debian
sudo apt-get install qemu-system-x86

# macOS
brew install qemu

# Windows
choco install qemu
```

### Bootimage Errors
```bash
# Reinstall bootimage
cargo install bootimage --force
```

## 🚧 Roadmap

### Phase 1: ✅ Core Kernel
- [x] Boot from UEFI bootloader
- [x] Basic interrupt handling
- [x] Memory management
- [x] VGA output
- [x] Serial communication

### Phase 2: ⏳ Advanced Features
- [ ] Filesystem (FAT32/ext2)
- [ ] Multi-tasking/scheduling
- [ ] Network stack
- [ ] Device drivers (ATA, NVMe)
- [ ] User mode execution
- [ ] System calls

### Phase 3: 🔮 System Services
- [ ] Shell/command interpreter
- [ ] File utilities
- [ ] System utilities
- [ ] Package manager
- [ ] GUI/window manager
- [ ] Compiler integration

## 📝 License

This project is provided for educational purposes.

## 🤝 Contributing

Contributions are welcome! Areas for contribution:
- Driver development
- Filesystem implementation
- Scheduler improvements
- Documentation
- Testing and bug fixes

## 📞 Support

For questions and discussions:
- Open an issue on the repository
- Check existing documentation
- Review OSDev.org for OS concepts

---

**RustyOS v0.1.0** - Built with ❤️ using Rust | Designed for x86-64 | Educational OS Project

```
╔════════════════════════════════════════════════════════╗
║                   🖥️ RustyOS v0.1.0                    ║
║         A Modern Operating System Written in Rust      ║
╚════════════════════════════════════════════════════════╝
```
