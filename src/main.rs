#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};

mod memory;
mod allocator;
mod video;
mod interrupts;
mod gdt;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}!", "world");
    
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::new(&boot_info.memory_map)
    };


    let x = Box::new(41);

    interrupts::init();
    gdt::init();
    interrupts::enable();

    interrupts::halt_loop();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    
    interrupts::disable();
    interrupts::halt_loop();
}
