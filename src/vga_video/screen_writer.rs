use crate::geometry::position::Point;

use super::{CharacterColor, ScreenBuffer};

pub struct ScreenWriter<'a> {
    frozen_rows: usize,
    buffer: &'a mut ScreenBuffer,
}

impl <'a> ScreenWriter<'a> {
    pub fn new(buffer: &'a mut ScreenBuffer) -> Self {
        ScreenWriter {
            frozen_rows: 0,
            buffer
        }
    }

    pub fn set_frozen_rows(&mut self, frozen_rows: usize) {
        self.frozen_rows = frozen_rows
    }
}

impl ScreenWriter<'_> {
    pub fn clear(&mut self) {
        self.buffer.clear_screen();
    }

    pub fn write_string(&mut self, position: Point, string: &str, color: CharacterColor) -> Point {
        let mut position = position;
        for character in string.as_bytes() {
            position = self.write_char(position, *character, color);
        }
        position
    }

    pub fn write_char(&mut self, position: Point, character: u8, color: CharacterColor) -> Point {
        match character {
            b'\n' => position.next_row(),
            _ => {
                let position = self.scroll_if_needed(position);
                self.buffer.put_char(position, character, color);
                position.next()
            }
        }
    }

    fn scroll_if_needed(&mut self, position: Point) -> Point {
        if self.needs_scroll(position) {
            self.scroll_one_row_up();
            Point::new(position.x, position.y - 1)
        } else {
            position
        }
    }

    fn needs_scroll(&self, position: Point) -> bool {
        position.y >= ScreenBuffer::size().height
    }

    fn scroll_one_row_up(&mut self) {
        let first_row = self.frozen_rows + 1;
        let last_row = ScreenBuffer::size().height - 1;
        
        for row in first_row .. last_row {
            self.buffer.copy_row(row, row + 1);
        }
        self.buffer.clear_row(last_row);
    }
}
