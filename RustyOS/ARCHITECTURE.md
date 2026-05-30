# Architecture Documentation

## 🏗️ System Architecture

### Boot Sequence
```
1. UEFI Firmware
   ↓
2. Bootloader (bootloader 0.9 crate)
   ├─ Setup page tables
   ├─ Identity mapping
   ├─ Load kernel into memory
   └─ Jump to kernel entry point
   ↓
3. Kernel Entry (kernel_main)
   ├─ Initialize GDT
   ├─ Initialize IDT
   ├─ Initialize PIC
   ├─ Enable interrupts
   ├─ Initialize memory
   └─ Enter kernel main loop
```

## 📊 Memory Layout

```
Virtual Address Space (x86-64)
┌─────────────────────────────────────────┐
│ High addresses (FFFF...)                │
├─────────────────────────────────────────┤
│ Kernel memory (paged)                   │
│ ├─ Code (.text)                         │
│ ├─ Data (.data)                         │
│ ├─ BSS (.bss)                           │
│ ├─ Heap (dynamic)                       │
│ └─ Stack                                │
├─────────────────────────────────────────┤
│ (unused)                                │
├─────────────────────────────────────────┤
│ Bootloader region                       │
│ └─ Multiboot header                     │
├─────────────────────────────────────────┤
│ Low addresses (0000...)                 │
└─────────────────────────────────────────┘
```

## 🔌 Interrupt Architecture

### Interrupt Vector Table
```
Entry 0-31:   CPU Exceptions
Entry 32-47:  Hardware IRQs (PIC)
Entry 48-255: Software interrupts
```

### Interrupt Handling Flow
```
Hardware Interrupt
    ↓
PIC (Programmable Interrupt Controller)
    ↓
IDT (Interrupt Descriptor Table)
    ↓
ISR Handler (Interrupt Service Routine)
    ↓
Application/Kernel Processing
    ↓
EOI (End of Interrupt) sent to PIC
```

## 🧠 CPU Execution Model

### Privilege Levels
- **Ring 0** (Kernel): Kernel code, device drivers
- **Ring 1-2** (Reserved)
- **Ring 3** (User): User applications

### GDT Segments
```
Segment 0: NULL descriptor
Segment 1: Kernel Code (Ring 0)
Segment 2: Kernel Data (Ring 0)
Segment 3+: TSS, LDT, user segments
```

## 💾 Memory Management

### Paging System
```
Virtual Address
    ↓
Page Table Lookup
├─ PML4 (Page Map Level 4) - Top level
├─ PDPT (Page Directory Pointer Table)
├─ PD   (Page Directory)
├─ PT   (Page Table)
    ↓
Physical Address

Page Size: 4 KB (standard)
Levels: 4 (on x86-64)
```

### Allocator Strategy
- **Bump Allocator**: Used at boot for static allocation
- **Linked List Allocator**: Used for heap after bootstrap
- **Frame Allocator**: Manages physical page frames

## 🎯 Module Architecture

### Core Modules
```
main.rs          Entry point, initialization
├─ gdt.rs        Global Descriptor Table
├─ interrupts.rs IDT, PIC, interrupt handlers
├─ memory.rs     Paging, heap allocation
├─ vga.rs        VGA text mode display
├─ serial.rs     UART serial communication
├─ keyboard.rs   PS/2 keyboard input
├─ allocator.rs  Memory allocator
└─ task.rs       Task scheduling
```

## 🔄 Control Flow

### Initialization Flow
```
kernel_main()
    ├─ init()
    │  ├─ gdt::init()
    │  ├─ interrupts::init_idt()
    │  ├─ interrupts::PICS.initialize()
    │  └─ enable interrupts
    ├─ Initialize memory
    ├─ Print system info
    └─ Main kernel loop (hlt)
```

### Interrupt Handling Flow
```
Hardware generates interrupt
    ↓
CPU executes IDT handler
    ↓
ISR dispatches to appropriate handler
    ├─ Timer interrupt
    ├─ Keyboard interrupt
    └─ Other exceptions
    ↓
Send EOI to PIC
    ↓
Return to interrupted context
```

## 🔐 Security Considerations

### Memory Safety
- Rust prevents buffer overflows at compile time
- Unsafe code is isolated and documented
- Memory access is bounds-checked

### CPU Security Features
- Privilege separation (Ring 0/3)
- Memory protection via paging
- Interrupt isolation
- Stack protection (IST in TSS)

## 🚀 Extension Points

### Adding System Calls
1. Define syscall numbers
2. Create syscall handlers
3. Register in syscall dispatcher
4. Export to user space

### Adding Interrupt Handlers
1. Define handler function
2. Set handler in IDT entry
3. Handle interrupt context
4. Send EOI to PIC

### Adding Device Drivers
1. Detect device (PCI, USB, etc.)
2. Allocate I/O resources
3. Initialize device
4. Register interrupt handlers
5. Provide device interface

## 📈 Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Context switch | <1μs | Depends on cache state |
| Page fault | 10-100μs | Includes TLB reload |
| Interrupt latency | <1μs | From hardware to ISR |
| Memory allocation | O(1) | Bump allocator |

## 🔗 Hardware Interfaces

### x86-64 CPU Instructions
- `cli` / `sti` - Disable/enable interrupts
- `hlt` - Halt CPU
- `in` / `out` - Port I/O
- `lgdt` - Load GDT
- `lidt` - Load IDT
- `cr0/cr3` - Control registers

### I/O Ports
- `0x3f8-0x3ff` - Serial port (UART 16550)
- `0x60` - PS/2 keyboard
- `0x20-0x21` - PIC (master)
- `0xa0-0xa1` - PIC (slave)
- `0x3d4-0x3d5` - VGA
- `0x3b0-0x3bf` - VGA

---

**Last Updated**: May 30, 2026
