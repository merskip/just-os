use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

use crate::error::Error;
use crate::geometry::position::Position;
use crate::vga_video::CharacterColor;
use crate::vga_video::frame_buffer::FrameBuffer;

pub struct MockFrameBuffer {
    characters: Vec<(char, CharacterColor)>,
    cols: usize,
    rows: usize,
}

impl MockFrameBuffer {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            characters: vec![('\0', CharacterColor::default()); cols * rows],
            cols,
            rows,
        }
    }

    pub fn get_character(&self, column: usize, row: usize) -> char {
        let index = self.get_index(Position::new(column, row));
        self.characters[index].0
    }

    fn get_index(&self, position: Position) -> usize {
        position.row * self.cols + position.column
    }
}

impl FrameBuffer for MockFrameBuffer {
    fn set_char(
        &mut self,
        position: Position,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>> {
        if position.row >= self.rows || position.column >= self.cols {
            return Err(Box::new(MockFrameBufferError::OutOfBounds));
        }

        let index = self.get_index(position);
        self.characters[index] = (character, color);
        Ok(())
    }
}

#[derive(Debug)]
enum MockFrameBufferError {
    OutOfBounds
}

impl Display for MockFrameBufferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MockFrameBufferError {}
