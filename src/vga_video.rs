pub mod color;
pub use color::*;

pub mod screen_buffer;
pub use screen_buffer::*;
pub mod screen_writer;

#[macro_export]
#[deprecated]
macro_rules! println {
    () => ();
    ( $($arg:tt)* ) => ($crate::log!($($arg)*));
}
