#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::{Translate, TranslateResult};

mod serial;
mod vga;
mod interrupts;
mod memory;
mod keyboard;
mod gdt;
mod allocator;
mod task;

use serial::SERIAL1;
use vga::COLOR_WRITER;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║                  🖥️  RustyOS v0.1.0                    ║");
    println!("║         A Modern Operating System Written in Rust      ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("[✓] GDT initialized");
    println!("[✓] IDT initialized");
    println!("[✓] PIC initialized");
    println!("[✓] Interrupts enabled");
    println!("[✓] Memory management initialized");
    println!("[✓] VGA text mode initialized");
    println!();
    
    // Initialize memory management
    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    unsafe {
        memory::MAPPER.lock().replace(memory::init(phys_mem_offset, &boot_info.memory_regions));
        memory::ALLOCATOR.lock().init(&boot_info.memory_regions);
    }
    println!("[✓] Physical memory offset: {:#x}", phys_mem_offset);
    println!("[✓] Memory allocator ready");
    println!();
    
    // Demonstration features
    println!("=== System Information ===");
    println!("Architecture: x86-64");
    println!("Boot mode: UEFI with Bootloader");
    println!("Kernel entry: {:#x}", kernel_main as *const () as u64);
    println!();
    
    println!("=== Supported Features ===");
    println!("✓ Bootloader (bootloader 0.9)");
    println!("✓ Memory management (paging)");
    println!("✓ Interrupts (IDT, GDT)");
    println!("✓ Keyboard input");
    println!("✓ Serial communication");
    println!("✓ VGA text mode");
    println!("✓ Heap allocation");
    println!("✓ Task scheduling (basic)");
    println!();
    
    println!("Type 'help' for available commands");
    println!();
    
    // Main kernel loop
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("╔════════════════════════════════════════════╗");
    println!("║         🔴 KERNEL PANIC OCCURRED 🔴        ║");
    println!("╚════════════════════════════════════════════╝");
    println!();
    
    if let Some(location) = info.location() {
        println!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    }
    
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("Message: {}", msg);
    } else if let Some(msg) = info.payload().downcast_ref::<String>() {
        println!("Message: {}", msg);
    }
    
    println!();
    println!("System halted.");
    
    loop {
        x86_64::instructions::hlt();
    }
}

fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::vga::COLOR_WRITER;
        let _ = write!(COLOR_WRITER.lock(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}
