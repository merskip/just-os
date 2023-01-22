#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[macro_use]
extern crate bitflags;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use bootloader::{entry_point, BootInfo};
use tui::panic_screen::{PanicScreen};

use core::panic::PanicInfo;
use x86_64::VirtAddr;

use crate::log::KERNEL_LOGGER;
use crate::rtc::RTC;
use crate::task::executor::Executor;
use crate::task::keyboard;
use crate::tui::text_screen::{TextScreen, Header};
use crate::vga_video::screen_writer::ScreenWriter;
use crate::vga_video::{CharacterColor, Color};
use crate::vga_video::screen_buffer::ScreenBuffer;

mod log;
mod allocator;
mod gdt;
mod interrupts;
mod memory;
mod vga_video;
mod stream;
mod tui;
mod task;
mod rtc;
mod geometry {
    pub mod position;
    pub mod size;
}
#[cfg(test)]
mod qemu_exit;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };
    allocator::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let screen_buffer = unsafe { ScreenBuffer::new(0xb8000) };
    let screen_writer = ScreenWriter::new(screen_buffer);
    let default_color = CharacterColor::new(Color::Gray, Color::Black);

    let mut text_screen = Box::new(TextScreen::new(
        screen_writer,
        default_color,
        Header::new(PKG_NAME.to_string(), PKG_VERSION.to_string()),
    ));
    text_screen.display();

    KERNEL_LOGGER.lock().register_listener(text_screen);

    log_info!("Hello {}!", "world");

    interrupts::init();
    gdt::init();
    interrupts::enable();

    let mut rtc = RTC::new();
    let now = rtc.read_datetime();
    log_info!("Now: {}", now);

    #[cfg(test)]
    test_main();

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
    let screen_buffer = unsafe { ScreenBuffer::new(0xb8000) };
    let mut panic_screen = PanicScreen::new(screen_buffer);
    panic_screen.display(info);

    interrupts::disable();
    interrupts::halt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu_exit::*;

    log_info!("Running tests...");
    for test in tests {
        test();
    }

    qemu_exit(ExitCode::Success);
    panic!("QEMU not exited!");

}

#[test_case]
fn trivial_assertion() {
    log_info!("trivial assertion... ");
    assert_eq!(1, 1);
    log_info!("[ok]");
}