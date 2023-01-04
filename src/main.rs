#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

// mod vga_buffer;
mod video;
mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello {}!", "world");
    
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
