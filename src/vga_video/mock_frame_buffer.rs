use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

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
            characters: vec![(char::default(), CharacterColor::zero()); cols * rows],
            size: Size::new(cols, rows),
        }
    }

    pub fn get_chars(&self, x: usize, y: usize, length: usize) -> Vec<char> {
        let index = self.get_index(Point::new(x, y));
        let range = index..(index + length);
        self.characters[range]
            .iter()
            .map(|c| c.0).collect()
    }
}

impl FrameBuffer for MockFrameBuffer {
    fn get_size(&self) -> Size {
        self.size.clone()
    }

    fn set_char(
        &mut self,
        position: Point,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>> {
        let index = self.get_index(position);
        self.characters[index] = (character, color);
        Ok(())
    }

    fn copy_char(
        &mut self,
        source: Point,
        destination: Point,
    ) -> Result<(), Box<dyn Error>> {
        let destination_index = self.get_index(destination);
        let source_index = self.get_index(source);
        self.characters[destination_index] = self.characters[source_index];
        Ok(())
    }
}
