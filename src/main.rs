#![no_std]
#![no_main]
#![feature(const_mut_refs)]

use core::panic::PanicInfo;

mod vga_buffer;
use vga_buffer::{Writer, ColorCode, Color, Buffer};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
