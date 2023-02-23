use core::cell::RefCell;
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::{geometry::position::Point, vga_video::{CharacterColor, Color}};
use crate::geometry::rect::Rect;
use crate::vga_video::frame_buffer::FrameBuffer;
use crate::vga_video::screen_fragment_writer::ScreenFragmentWriter;

pub struct PanicScreen<'a> {
    frame_buffer: &'a RefCell<dyn FrameBuffer>,
}

impl<'a> PanicScreen<'a> {
    pub fn new(frame_buffer: &'a RefCell<dyn FrameBuffer>) -> Self {
        PanicScreen { frame_buffer }
    }
}

impl PanicScreen<'_> {
    pub fn display(&mut self, info: &PanicInfo) {
        self.frame_buffer.borrow_mut()
            .clear().unwrap();

        let mut writer = ScreenFragmentWriter::new(
            Rect::new(Point::default(), self.frame_buffer.borrow().get_size()),
            CharacterColor::new(Color::Red, Color::Black),
            self.frame_buffer,
        );

        writeln!(writer, "[PANIC!]").unwrap();
        writeln!(writer, "{:#?}", info).unwrap();
    }
}