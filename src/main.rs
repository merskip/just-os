#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

// mod vga_buffer;
mod video;
mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    // println!("Hello World{}", "!");
    println!("Hello {}!", "world");
    
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();

    println!("Message 1");
    println!("Message 2");
    println!("Message 3");
    println!("Message 4");
    println!("Message 5");
    println!("Message 6");
    println!("Message 7");
    println!("Message 8");
    println!("Message 9");
    println!("Message 10");
    println!("Message 11");
    println!("Message 12");
    println!("Message 13");
    println!("Message 14");
    println!("Message 15");
    println!("Message 16");
    println!("Message 17");
    println!("Message 18");
    println!("Message 19");
    println!("Message 20");
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);
    loop {}
}
