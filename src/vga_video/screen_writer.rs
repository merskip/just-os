use crate::geometry::position::Position;
use super::{ScreenBuffer, CharacterColor};

pub struct ScreenWriter<'a> {
    buffer: &'a mut ScreenBuffer,
}

impl <'a> ScreenWriter<'a> {
    pub fn new(buffer: &'a mut ScreenBuffer) -> Self {
        ScreenWriter { buffer }
    }
}

impl ScreenWriter<'_> {
    pub fn clear(&mut self) {
        self.buffer.clear_screen();
    }

    pub fn write_string(&mut self, position: Position, string: &str, color: CharacterColor) -> Position {
        let mut position = position;
        for character in string.as_bytes() {
            position = self.write_char(position, *character, color);
        }
        position
    }

    pub fn write_char(&mut self, position: Position, character: u8, color: CharacterColor) -> Position {
        match character {
            b'\n' => position.next_row(),
            _ => {
                self.buffer.set_character(position, character, color);
                position.next()
            }
        }
    }
}