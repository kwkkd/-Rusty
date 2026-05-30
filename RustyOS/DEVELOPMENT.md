# RustyOS Development Guide

## 🔧 Development Setup

### IDE Setup
We recommend using VS Code with these extensions:
- rust-analyzer
- Rust Test Explorer
- CodeLLDB (for debugging)
- TOML

### Rust Components
```bash
# Install all necessary components
rustup toolchain install nightly
rustup component add rustfmt
rustup component add clippy
rustup component add llvm-tools-embedded
rustup target add x86_64-unknown-none
```

### Required Tools
```bash
# Bootimage builder
cargo install bootimage

# Optional: improved Cargo wrapper
cargo install cargo-watch

# Optional: documentation server
cargo install cargo-serve
```

## 📝 Code Style

### Formatting
We use `rustfmt` for consistent code formatting:

```bash
# Format all code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting
We use `clippy` for code quality:

```bash
# Check for warnings
cargo clippy

# Fix common issues automatically
cargo clippy --fix
```

### Documentation
All public items should have documentation:

```rust
/// Initializes the GDT and loads it into the GDTR.
/// 
/// # Safety
/// This function uses inline assembly to load the GDT.
/// It must only be called once during kernel initialization.
pub fn init() {
    // ...
}
```

## 🧪 Testing

### Unit Tests
```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test gdt::tests::

# Run with output
cargo test -- --nocapture
```

### Integration Tests
Place tests in `tests/` directory:

```bash
# Run integration tests
cargo test --test test_name
```

## 🐛 Debugging

### Using GDB
```bash
# Terminal 1: Start QEMU with debug server
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin -s -S

# Terminal 2: Connect GDB
gdb -ex "file target/x86_64-unknown-none/release/kernel" \
    -ex "target remote :1234" \
    -ex "break main.rs:100" \
    -ex "continue"
```

### Debugging Commands
```gdb
# Show registers
info registers

# Show memory
x/16wx $esp

# Show backtrace
backtrace

# Continue execution
continue

# Step instruction
stepi

# View source
list
```

### Printf-style Debugging
Use macros for debugging output:

```rust
// To console (VGA)
println!("Debug: value = {}", x);

// To serial port
serial_println!("Serial debug: {}", x);
```

## 🏗️ Adding New Modules

### Creating a New Module
1. Create file in `src/module_name.rs`
2. Add `mod module_name;` to `src/main.rs`
3. Implement functionality
4. Add tests in module
5. Update documentation

### Module Template
```rust
//! Module documentation
//! 
//! Detailed description of what this module does.

/// Function description
pub fn public_function() {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        assert_eq!(public_function(), expected);
    }
}
```

## 🔄 Git Workflow

### Commit Messages
Follow conventional commits:

```
feat: add keyboard interrupt handler
fix: correct GDT descriptor offset
docs: update memory management section
refactor: simplify interrupt dispatch
test: add unit tests for allocator
chore: update dependencies
```

### Branch Naming
```
feature/feature-name
fix/bug-description
docs/documentation-topic
refactor/component-name
```

## 📦 Dependency Management

### Adding Dependencies
```bash
# Add to Cargo.toml with exact version
cargo add --git https://github.com/user/repo crate-name

# Update all dependencies
cargo update

# Check for vulnerabilities
cargo audit
```

### Unsafe Code
Minimize unsafe code and always document it:

```rust
/// SAFETY: This pointer is guaranteed to be valid because...
unsafe {
    // Code here
}
```

## 🔍 Code Review Checklist

Before submitting code:
- [ ] Code compiles without warnings
- [ ] `cargo test` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt` is applied
- [ ] Comments explain complex logic
- [ ] Unsafe code is documented
- [ ] Memory safety is verified
- [ ] Performance is considered

## 🚀 Performance Optimization

### Profiling
```bash
# Build with profiling info
RUSTFLAGS="-g" cargo build --release

# Run under perf
perf record -g qemu-system-x86_64 ...
perf report
```

### Common Optimizations
1. Use `#[inline]` for hot functions
2. Avoid allocations in hot loops
3. Batch I/O operations
4. Cache frequently accessed data
5. Use `const` for compile-time computation

## 📚 Important Files

| File | Purpose |
|------|---------|
| `src/main.rs` | Kernel entry and initialization |
| `Cargo.toml` | Project configuration |
| `.cargo/config.toml` | Build system configuration |
| `clippy.toml` | Lint configuration |
| `.rustfmt.toml` | Format configuration |

## 🆘 Common Issues

### Issue: "bootimage not found"
**Solution:** `cargo install bootimage`

### Issue: "llvm-tools not found"
**Solution:** `rustup component add llvm-tools-embedded`

### Issue: "x86_64-unknown-none target not found"
**Solution:** `rustup target add x86_64-unknown-none --toolchain nightly`

### Issue: Build takes too long
**Solution:** 
- Use `cargo check` to skip codegen
- Use sccache: `cargo install sccache`
- Use faster linker: `lld`

## 📖 References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [OSDev Resources](https://wiki.osdev.org/)
- [x86-64 Manuals](https://www.intel.com/)

---

**Last Updated**: May 30, 2026
