.PHONY: build run clean qemu test

QEMU := qemu-system-x86_64
QEMU_FLAGS := -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin \
              -display gtk -m 512M -cpu max

all: build

build:
	@echo "🔨 Building RustyOS..."
	@cargo install bootimage --quiet 2>/dev/null || true
	@cargo build --release
	@bootimage build --release
	@echo "✅ Build complete!"

run: build
	@echo "🚀 Launching QEMU..."
	@$(QEMU) $(QEMU_FLAGS)

qemu: build
	@$(QEMU) $(QEMU_FLAGS)

kvm: build
	@echo "⚡ Running with KVM acceleration..."
	@$(QEMU) $(QEMU_FLAGS) -enable-kvm -cpu host

test:
	@cargo test --lib

check:
	@cargo check

clean:
	@cargo clean
	@echo "🧹 Cleaned build artifacts"

help:
	@echo "RustyOS Build System"
	@echo "===================="
	@echo ""
	@echo "Available targets:"
	@echo "  make build   - Build the kernel image"
	@echo "  make run     - Build and run in QEMU"
	@echo "  make qemu    - Run in QEMU"
	@echo "  make kvm     - Run with KVM acceleration"
	@echo "  make test    - Run tests"
	@echo "  make check   - Check code without building"
	@echo "  make clean   - Clean build artifacts"
	@echo "  make help    - Show this help message"
