use volatile::{Volatile};
use super::{CharacterColor, Color};
use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenCharacter {
    pub character: u8,
    pub color: CharacterColor,
}

pub const SCREEN_HEIGHT: usize = 25;
pub const SCREEN_WIDTH: usize = 80;

#[repr(transparent)]
pub struct ScreenBuffer {
    characters: [[Volatile<ScreenCharacter>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl ScreenBuffer {
    pub unsafe fn new(address: u64) -> &'static mut Self {
        &mut *(address as *mut ScreenBuffer)
    }
}

impl ScreenBuffer {
    pub fn put_string(&mut self, row: usize, column: usize, string: &str, color: CharacterColor) {
        let mut column = column;
        for character in string.as_bytes() {
            self.put_char(row, column, *character, color);
            column += 1;
        }
    }

    pub fn put_char(&mut self, row: usize, column: usize, character: u8, color: CharacterColor) {
        self.characters[row][column].write(ScreenCharacter { character, color });
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
