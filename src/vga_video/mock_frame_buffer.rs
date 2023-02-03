use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

use crate::error::Error;
use crate::geometry::position::Point;
use crate::geometry::size::Size;
use crate::vga_video::CharacterColor;
use crate::vga_video::frame_buffer::FrameBuffer;

pub struct MockFrameBuffer {
    characters: Vec<(char, CharacterColor)>,
    size: Size,
}

impl MockFrameBuffer {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            characters: vec![('\0', CharacterColor::zero()); cols * rows],
            size: Size::new(cols, rows)
        }
    }

    pub fn get_character(&self, x: usize, y: usize) -> char {
        let index = self.get_index(Point::new(x, y));
        self.characters[index].0
    }

    fn get_index(&self, position: Point) -> usize {
        position.y * self.size.width + position.x
    }
}

impl FrameBuffer for MockFrameBuffer {
    fn get_size(&self) -> Size {
        self.size
    }

    fn set_char(
        &mut self,
        position: Point,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>> {
        if position.y >= self.size.width || position.x >= self.size.height {
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
