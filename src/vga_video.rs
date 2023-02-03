pub use color::*;
pub use screen_buffer::*;

pub mod color;
pub mod screen_buffer;
pub mod screen_writer;
pub mod screen_fragment_writer;
mod frame_buffer;
#[cfg(test)]
pub mod mock_frame_buffer;

pub fn vga_screen_buffer() -> &'static mut ScreenBuffer {
    unsafe { ScreenBuffer::new(0xb8000) }
}
