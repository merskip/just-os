use alloc::boxed::Box;
use core::mem;

use volatile::Volatile;

use crate::error::Error;
use crate::geometry::position::Point;
use crate::geometry::size::Size;
use crate::vga_video::CharacterColor;
use crate::vga_video::frame_buffer::FrameBuffer;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenCharacter {
    pub character: u8,
    pub color: CharacterColor,
}

pub(crate) const VGA_SCREEN_SIZE: Size = Size::new(80, 25);

#[repr(transparent)]
pub struct VgaFrameBuffer {
    characters: *mut [Volatile<ScreenCharacter>; VGA_SCREEN_SIZE.area()],
}

impl VgaFrameBuffer {
    pub const unsafe fn new(address: u64) -> Self {
        Self {
            characters: mem::transmute(address)
        }
    }
}

impl FrameBuffer for VgaFrameBuffer {
    fn get_size(&self) -> Size {
        VGA_SCREEN_SIZE
    }

    fn set_char(&mut self, position: Point, character: char, color: CharacterColor) -> Result<(), Box<dyn Error>> {
        let index = self.get_index(position);
        let screen_character = ScreenCharacter {
            character: u8::try_from(character).unwrap(),
            color
        };

        unsafe {
            let characters = self.characters.as_mut().unwrap();
            characters[index] = Volatile::new(screen_character);
        }
        Ok(())
    }

    fn copy_char(&mut self, source: Point, destination: Point) -> Result<(), Box<dyn Error>> {
        let source_index = self.get_index(source);
        let destination_index = self.get_index(destination);
        unsafe {
            let characters = self.characters.as_mut().unwrap();
            let value = characters[source_index].clone();
            characters[destination_index] = value;
        }
        Ok(())
    }
}
