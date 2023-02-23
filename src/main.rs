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
use alloc::rc::Rc;
use alloc::string::String;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use spin::Mutex;
use x86_64::VirtAddr;

#[cfg(test)]
use qemu_exit::{ExitCode, qemu_exit};

use crate::command::command_register::CommandRegister;
use crate::command::ping_pong_command::ping_pong_command;
use crate::log::KERNEL_LOGGER;
use crate::rtc::RTC;
use crate::task::executor::Executor;
use crate::task::keyboard;
use crate::tui::panic_screen::PanicScreen;
use crate::tui::terminal_screen::{Header, TerminalScreen};
use crate::vga_video::{VGA_FRAME_BUFFER};
use crate::vga_video::cursor::VgaCursor;

mod allocator;
mod gdt;
mod interrupts;
mod log;
mod memory;
mod rtc;
mod task;
mod tui;
mod vga_video;
mod geometry;
mod serial;
mod error;
mod command;
mod io;

#[cfg(test)]
mod qemu_exit;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    unsafe {
        serial_println!("Physical memory offset: {:#x}", boot_info.physical_memory_offset);
        let mut mapper = memory::init(VirtAddr::new(boot_info.physical_memory_offset));
        let mut frame_allocator = memory::BootInfoFrameAllocator::new(&boot_info.memory_map);

        allocator::init(&mut mapper, &mut frame_allocator)
            .expect("heap allocator initialization failed");
    }

    KERNEL_LOGGER.lock().register_listener(Box::new(move |log| {
        serial_println!("LOG: {}", &log);
    }));

    log_info!("{} (ver. {})", PKG_NAME, PKG_VERSION);

    interrupts::init();
    log_info!("Interrupts initialized");

    gdt::init();
    log_info!("GDT initialized");

    interrupts::enable();
    log_info!("Interrupts enabled");

    let mut command_register = CommandRegister::new();
    command_register.register("ping", Box::new(ping_pong_command));

    let rtc = Rc::new(Mutex::new(RTC::new()));
    let cursor = Rc::new(Mutex::new(VgaCursor::new()));
    let mut terminal_screen = TerminalScreen::new(
        unsafe { &VGA_FRAME_BUFFER },
        Header::new(String::from(PKG_NAME), String::from(PKG_VERSION)),
        rtc.clone(),
        String::from("> "),
        cursor,
        Box::new(move |command| command_register.perform(command)),
    );
    terminal_screen.begin();

    io::set_standard_output_writer(terminal_screen.get_standard_output());

    let terminal_screen = Rc::new(terminal_screen);
    let terminal_screen_2 = terminal_screen.clone();
    interrupts::set_timer_handler(Box::new(move || {
        terminal_screen_2.refresh_header();
    }));

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    let terminal_screen_3 = terminal_screen.clone();
    executor.spawn(keyboard::keyboard_decoding_task(Box::new(move |key| {
        terminal_screen_3.handle_keypress(key);
    })));
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
