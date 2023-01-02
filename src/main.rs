#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga_buffer::print_something;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
