use volatile::Volatile;
use crate::geometry::{size::Size, position::Position};

use super::{CharacterColor, Color};
use core::ops::{Deref, DerefMut};

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;
const SCREEN_SIZE: Size = Size::new(SCREEN_WIDTH, SCREEN_HEIGHT);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenCharacter {
    pub character: u8,
    pub color: CharacterColor,
}

#[repr(transparent)]
pub struct ScreenBuffer {
    characters: [[Volatile<ScreenCharacter>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl ScreenBuffer {
    pub unsafe fn new(address: u64) -> &'static mut Self {
        &mut *(address as *mut ScreenBuffer)
    }

    pub const fn size() -> Size {
        SCREEN_SIZE
    }
}

impl ScreenBuffer {
    pub fn set_character(&mut self, position: Position, character: u8, color: CharacterColor) {
        let screen_character = ScreenCharacter { character, color };
        self.characters[position.row][position.column].write(screen_character);
    }

    pub fn copy_row(&mut self, target_row: usize, destination_row: usize) {
        let row = &self.characters[destination_row];
        self.characters[target_row] = row.clone();
    }

    pub fn clear_screen(&mut self) {
        for row in 0..SCREEN_HEIGHT {
            self.clear_row(row);
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        const CLEAR_COLOR: CharacterColor = CharacterColor::new(Color::Black, Color::Black);
        const CLEAR_CHARACTER: ScreenCharacter = ScreenCharacter { character: 0b0, color: CLEAR_COLOR };

        self.characters[row].fill(Volatile::new(CLEAR_CHARACTER));
    }
}

impl Deref for ScreenCharacter {
    type Target = ScreenCharacter;

    fn deref(&self) -> &ScreenCharacter {
        &self
    }
}

impl DerefMut for ScreenCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
