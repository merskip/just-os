#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod video;
mod interrupts;
mod gdt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
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
