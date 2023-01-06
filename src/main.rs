#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use crate::memory::BootInfoFrameAllocator;

mod memory;
mod video;
mod interrupts;
mod gdt;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}!", "world");

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    interrupts::init();
    gdt::init();
    interrupts::enable();

    interrupts::halt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    
    interrupts::disable();
    interrupts::halt_loop();
}
