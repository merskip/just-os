use alloc::boxed::Box;

use crate::error::Error;
use crate::geometry::position::Point;
use crate::geometry::size::Size;
use crate::vga_video::CharacterColor;

pub trait FrameBuffer {
    fn get_size(&self) -> Size;

    fn set_char(
        &mut self,
        position: Point,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>>;

    fn copy_char(
        &mut self,
        source: Point,
        destination: Point,
    ) -> Result<(), Box<dyn Error>>;

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        for index in self.get_size().points() {
            self.set_char(index, char::default(), CharacterColor::zero())?;
        }
        Ok(())
    }

    fn get_index(&self, position: Point) -> usize {
        let size = self.get_size();
        position.y * size.width + position.x
    }
}