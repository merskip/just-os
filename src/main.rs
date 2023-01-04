#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

// mod vga_buffer;
mod video;
mod interrupts;
mod gdt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello {}!", "world");
    
    interrupts::init();
    gdt::init();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
