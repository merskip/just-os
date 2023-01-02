#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    just_os::init();
    x86_64::instructions::interrupts::int3();
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
