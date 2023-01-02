#![no_std]
#![no_main]
#![feature(const_mut_refs)]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
