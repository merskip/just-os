use alloc::boxed::Box;

use crate::error::Error;
use crate::geometry::position::Point;
use crate::vga_video::CharacterColor;

pub trait FrameBuffer {
    fn set_char(
        &mut self,
        position: Point,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>>;
}
