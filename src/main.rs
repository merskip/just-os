#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

mod video;
mod interrupts;
mod gdt;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}!", "world");

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
