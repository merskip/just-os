use alloc::fmt;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Write;
use crate::{serial_print};

static mut STANDARD_OUTPUT_WRITER: Option<Rc<RefCell<dyn Write>>> = None;

pub fn set_standard_output_writer(writer: Rc<RefCell<dyn Write>>) {
    unsafe {
        STANDARD_OUTPUT_WRITER.replace(writer);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::io::_print(format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! println {
    () => (
        $crate::print!("\n")
    );
    ($($arg:tt)*) => (
        $crate::print!("{}\n", format_args!($($arg)*))
    );
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(writer) = &STANDARD_OUTPUT_WRITER {
            writer.borrow_mut().write_fmt(args).unwrap();
        }
        serial_print!("STDOUT: {}", args);
    }
}
