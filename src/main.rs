#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use crate::rtc::RTC;
use crate::task::executor::Executor;
use crate::task::keyboard;

mod allocator;
mod gdt;
mod interrupts;
mod memory;
mod vga_video;
mod task;
mod rtc;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello {}!", "world");

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };
    allocator::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    interrupts::init();
    gdt::init();
    interrupts::enable();

    let mut rtc = RTC::new();
    let now = rtc.read_datetime();
    println!("Now: {}", now);

    let mut executor = Executor::new();
    executor.spawn(keyboard::print_keypresses());
    executor.run();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC!]");
    println!("{}", info);

    interrupts::disable();
    interrupts::halt_loop();
}
