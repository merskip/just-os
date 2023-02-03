use alloc::boxed::Box;

use crate::error::Error;
use crate::geometry::position::Position;
use crate::vga_video::CharacterColor;

pub trait FrameBuffer {
    fn set_char(
        &mut self,
        position: Position,
        character: char,
        color: CharacterColor,
    ) -> Result<(), Box<dyn Error>>;
}
