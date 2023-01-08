use alloc::string::String;
use spin::Mutex;

use crate::{vga_video::{ScreenBuffer, CharacterColor, Color}};

const HEADER_SIZE: usize = 3;

pub struct TextScreen<'a> {
    screen_buffer: &'a mut ScreenBuffer,
    default_color: CharacterColor,
}

impl <'a> TextScreen<'a> {
    pub fn new(screen_buffer: &'a mut ScreenBuffer, default_color: CharacterColor) -> Self {
        TextScreen {
            screen_buffer,
            default_color,
        }
    }

    pub fn display(&mut self) {
        self.screen_buffer.clear_screen();
        self.screen_buffer.put_string(0, 0, "Just-OS", self.default_color.with_foreground(Color::Cyan));
    }
}
