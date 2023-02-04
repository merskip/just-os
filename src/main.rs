#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(associated_type_defaults)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
#[macro_use]
extern crate bitflags;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use core::arch::asm;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use crate::log::KERNEL_LOGGER;
use crate::rtc::RTC;
use crate::task::executor::Executor;
use crate::task::keyboard;
use crate::tui::panic_screen::PanicScreen;
use crate::tui::text_screen::{Header, TextScreen};
use crate::vga_video::{CharacterColor, Color, ScreenBuffer, vga_screen_buffer};
use crate::vga_video::screen_writer::ScreenWriter;

mod allocator;
mod gdt;
mod interrupts;
mod log;
mod memory;
mod rtc;
mod stream;
mod task;
mod tui;
mod vga_video;
mod geometry;
mod serial;
mod error;

#[cfg(test)]
mod qemu_exit;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    unsafe {
        let mut mapper = memory::init(VirtAddr::new(boot_info.physical_memory_offset));
        let mut frame_allocator = memory::BootInfoFrameAllocator::new(&boot_info.memory_map);

        allocator::init(&mut mapper, &mut frame_allocator)
            .expect("heap allocator initialization failed");
    }

    let screen_writer = ScreenWriter::new(vga_screen_buffer());
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
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    let mut panic_screen = PanicScreen::new(vga_screen_buffer());
    panic_screen.display(info);

    interrupts::disable();
    interrupts::halt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    use crate::qemu_exit::*;

    serial_println!("\x1b[37mRunning {} tests...\x1b[0m", tests.len());
    for test in tests {
        test.run();
    }

    qemu_exit(ExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) -> () {
        let start_timestamp = unsafe { x86::time::rdtsc() };
        serial_print!("{}...", core::any::type_name::<T>());
        self();
        let end_timestamp = unsafe { x86::time::rdtsc() };
        serial_println!("\x1b[32m[OK]\x1b[0m in {} cycles", end_timestamp - start_timestamp);
    }
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu_exit::*;

    serial_println!("\x1b[31m");
    serial_println!("[PANIC]");
    serial_println!("{:#?}", info);
    serial_print!("\x1b[0m");
    qemu_exit(ExitCode::Failed)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
