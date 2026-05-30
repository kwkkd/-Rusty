QUICKSTART GUIDE - RustyOS
===========================

## 🔍 What is RustyOS?

RustyOS is a real, functional operating system written entirely in Rust for x86-64 architecture.
It includes bootloader, kernel, memory management, interrupt handling, and I/O support.

## 🚀 Quick Start (5 minutes)

### Step 1: Prerequisites
Ensure you have:
- Rust (latest stable + nightly)
- QEMU (for running the OS)

```bash
# Install Rust nightly
rustup toolchain install nightly
rustup component add llvm-tools-embedded

# Verify installation
cargo --version
rustc --version
```

### Step 2: Build the OS

```bash
cd RustyOS

# Install bootimage tool
cargo install bootimage

# Build the OS
make build

# Or manually:
cargo build --release
bootimage build --release
```

### Step 3: Run in QEMU

```bash
# Simple run
make run

# Or with acceleration
make kvm

# Manual QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin
```

**Expected Output:**
```
╔═══════════════════════════════════════════════════════╗
║                  🖥️  RustyOS v0.1.0                    ║
║         A Modern Operating System Written in Rust      ║
╚═══════════════════════════════════════════════════════╝

[✓] GDT initialized
[✓] IDT initialized
[✓] PIC initialized
[✓] Interrupts enabled
[✓] Memory management initialized
[✓] VGA text mode initialized

[✓] Physical memory offset: 0xffffffff80000000
[✓] Memory allocator ready

=== System Information ===
Architecture: x86-64
Boot mode: UEFI with Bootloader
...
```

## 📦 System Architecture

```
┌─────────────────────────────────────────────┐
│           QEMU/VirtualBox/Real HW           │
├─────────────────────────────────────────────┤
│  Bootloader (bootloader 0.9 crate)         │
├─────────────────────────────────────────────┤
│  RustyOS Kernel (Rust)                      │
│  ├─ GDT (CPU segmentation)                  │
│  ├─ IDT (Interrupt handling)                │
│  ├─ PIC (Interrupt controller)              │
│  ├─ Memory Manager (paging, heap)           │
│  ├─ VGA Driver (text mode display)          │
│  ├─ Serial Driver (UART 16550)              │
│  ├─ Keyboard Driver (PS/2)                  │
│  └─ Task Scheduler                          │
├─────────────────────────────────────────────┤
│  User Applications (future)                 │
└─────────────────────────────────────────────┘
```

## 🛠️ Common Tasks

### Check Compilation
```bash
cargo check
```

### Run Tests
```bash
cargo test --lib
```

### View Documentation
```bash
cargo doc --open
```

### Debug in GDB
```bash
# Terminal 1
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin -s -S

# Terminal 2
gdb -ex "file target/x86_64-unknown-none/release/kernel" -ex "target remote :1234"
```

### Clean Build
```bash
cargo clean
make build
```

## 📚 File Structure

| File | Purpose |
|------|---------|
| `src/main.rs` | Kernel entry point and initialization |
| `src/gdt.rs` | Global Descriptor Table setup |
| `src/interrupts.rs` | Interrupt handling (IDT, PIC) |
| `src/memory.rs` | Memory management & allocator |
| `src/vga.rs` | VGA text mode driver |
| `src/serial.rs` | Serial port communication |
| `src/keyboard.rs` | Keyboard input handling |
| `Cargo.toml` | Project dependencies |
| `Makefile` | Build automation |

## 🎯 Next Steps

1. **Explore the source code** - Start with `src/main.rs`
2. **Modify and recompile** - Change code and rebuild
3. **Add features** - Implement filesystem, network, etc.
4. **Create drivers** - Add support for hardware devices
5. **Build utilities** - Create shell commands and tools

## ⚡ Performance Tips

- Use `make kvm` for faster QEMU execution (requires KVM support)
- Build with `--release` for optimizations
- Use 512MB+ of RAM in QEMU: `-m 512M`

## 🐛 Troubleshooting

### QEMU Crashes
- Ensure 512MB RAM: `qemu-system-x86_64 ... -m 512M`
- Use compatible architecture: `-cpu max`

### Build Fails
```bash
# Update toolchain
rustup update nightly
rustup component add llvm-tools-embedded

# Reinstall bootimage
cargo install bootimage --force
```

### Slow Compilation
- Use incremental builds: `CARGO_INCREMENTAL=1 cargo build`
- Link-time optimization: disabled in debug builds
- Consider caching: `sccache`

## 📖 Further Reading

- **OSDev.org** - https://wiki.osdev.org/
- **Rust OS Book** - https://os.phil-opp.com/
- **UEFI Specification** - https://uefi.org/specifications/
- **x86-64 Architecture** - Intel/AMD manuals

## ✅ What's Included

✓ Working bootloader  
✓ Kernel in Rust  
✓ CPU setup (GDT, IDT)  
✓ Interrupt handling  
✓ Memory management  
✓ VGA output  
✓ Serial I/O  
✓ Keyboard input  
✓ Error handling  
✓ Build automation  

## ❓ Have Questions?

Check the README.md for detailed documentation, or explore the source code comments.

---

**RustyOS** - A real operating system in Rust! 🚀
