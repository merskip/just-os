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

    loop {
        print!("-");
        for _ in 0..1_000_000 {
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
