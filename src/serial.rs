use alloc::fmt;
use lazy_static::lazy_static;
use spin::mutex::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL_1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => (
        $crate::serial::_print(format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! serial_println {
    () => (
        $crate::print!("\n")
    );
    ($($arg:tt)*) => (
        $crate::serial_print!("{}\n", format_args!($($arg)*))
    );
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}
