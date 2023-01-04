use volatile::Volatile;
use super::CharacterColor;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenCharacter {
    pub character: u8,
    pub color_code: CharacterColor,
}

pub const SCREEN_HEIGHT: usize = 25;
pub const SCREEN_WIDTH: usize = 80;

#[repr(transparent)]
pub struct ScreenBuffer {
    characters: [[Volatile<ScreenCharacter>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl ScreenBuffer {
    pub fn set_character(&mut self, row: usize, column: usize, character: ScreenCharacter) {
        self.characters[row][column].write(character);
    }

    pub fn copy_row(&mut self, target_row: usize, destination_row: usize) {
        let row = &self.characters[destination_row];
        self.characters[target_row] = row.clone();

    }

    pub fn fill_row(&mut self, row: usize, character: ScreenCharacter) {
        self.characters[row].fill(Volatile::new(character));
    }
}
