#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![feature(associated_type_defaults)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
#[macro_use]
extern crate bitflags;

use alloc::boxed::Box;
use alloc::format;
use core::fmt::Write;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;
use crate::log::KERNEL_LOGGER;
use crate::rtc::RTC;
use crate::task::executor::Executor;
use crate::task::keyboard;
use crate::tui::panic_screen::PanicScreen;
use crate::vga_video::{CharacterColor, VGA_FRAME_BUFFER};
use crate::vga_video::screen_fragment_writer::ScreenFragmentWriter;

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
#[cfg(test)]
use qemu_exit::{ExitCode, qemu_exit};

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

    let mut logs_fragment_writer = ScreenFragmentWriter::new(
        Rect::new(Point::new(0, 2), Size::new(80, 23)),
        CharacterColor::default(),
        unsafe { &VGA_FRAME_BUFFER },
    );

    KERNEL_LOGGER.lock().register_listener(Box::new(move |log| {
        serial_println!("{:?}", &log);
        writeln!(logs_fragment_writer, "{}", log).unwrap();
    }));

    log_info!("{} (ver. {})", PKG_NAME, PKG_VERSION);

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
    interrupts::disable();

    serial_println!("[PANIC!]");
    serial_println!("{:#?}", info);

    let mut panic_screen = PanicScreen::new(unsafe { &VGA_FRAME_BUFFER });
    panic_screen.display(info);

    #[cfg(test)]
    qemu_exit(ExitCode::Failed);

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

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
