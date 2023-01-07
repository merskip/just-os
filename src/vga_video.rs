pub mod color;
pub use color::*;

pub mod screen_buffer;
pub use screen_buffer::*;

pub mod screen_writer;
pub use screen_writer::*;

use lazy_static::lazy_static;
use spin::Mutex;

static VGA_BUFFER_ADDRESS: usize = 0xb8000;

lazy_static! {
    static ref DEFAULT_COLOR: CharacterColor = CharacterColor::new(Color::Gray, Color::Black);
    pub static ref SCREEN_WRITER: Mutex<ScreenWriter> =
        Mutex::new(ScreenWriter::new(*DEFAULT_COLOR, VGA_BUFFER_ADDRESS));
}

use core::fmt;

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ( $( $arg:tt )* ) => {{
            use core::fmt::Write;
            use x86_64::instructions::interrupts;

            interrupts::without_interrupts(|| {
                $crate::vga_video::SCREEN_WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
            });
        }};
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ( $($arg:tt)* ) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
