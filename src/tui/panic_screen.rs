use alloc::format;
use core::panic::PanicInfo;

use crate::{geometry::position::Point, vga_video::{CharacterColor, Color, ScreenBuffer}};

pub struct PanicScreen<'a> {
    buffer: &'a mut ScreenBuffer,
}

impl <'a> PanicScreen<'a> {
    pub fn new(buffer: &'a mut ScreenBuffer) -> Self {
        PanicScreen { buffer }
    }
}

impl PanicScreen<'_> {
    pub fn display(&mut self, info: &PanicInfo) {
        self.buffer.clear_screen();

        let color = CharacterColor::new(Color::Red, Color::Black);
        self.buffer.put_string(Point::default(), "[PANIC!]", color);
        self.buffer.put_string(Point::new(0, 2), format!("{:#?}", info).as_str(), color);
    }
}