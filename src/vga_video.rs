use core::cell::RefCell;

pub use color::*;

use crate::vga_video::vga_frame_buffer::VgaFrameBuffer;

pub mod color;
pub mod vga_frame_buffer;
pub mod screen_fragment_writer;
pub mod frame_buffer;

#[cfg(test)]
pub mod mock_frame_buffer;
pub mod cursor;

pub static mut VGA_FRAME_BUFFER: RefCell<VgaFrameBuffer> = unsafe {
    RefCell::new(VgaFrameBuffer::new(0xb8000))
};
