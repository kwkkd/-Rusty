# RustyOS Configuration Files

This directory contains the RustyOS operating system project.

## Key Files

- `Cargo.toml` - Project manifest with dependencies
- `.cargo/config.toml` - Build configuration
- `src/main.rs` - Kernel entry point
- `Makefile` - Build automation
- `README.md` - Project documentation
- `QUICKSTART.md` - Quick start guide

## Build Commands

```bash
# Build OS
make build

# Run in QEMU
make run

# Check code
cargo check

# Format code
cargo fmt
```

## For More Information

See README.md or QUICKSTART.md
