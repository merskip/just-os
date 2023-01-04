use super::{CharacterColor, ScreenBuffer, ScreenCharacter, SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct ScreenWriter {
    row: usize,
    column: usize,
    default_color: CharacterColor,
    screen_buffer: &'static mut ScreenBuffer,
}

impl ScreenWriter {

    pub fn new(default_color: CharacterColor, buffer_address: usize) -> ScreenWriter {
        ScreenWriter {
            row: 0,
            column: 0,
            default_color,
            screen_buffer: unsafe { &mut *(buffer_address as *mut ScreenBuffer) },
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    
    fn write_byte(&mut self, character: u8) {
        match character {
            b'\n' => self.move_to_new_line(),
            _ => {
                if self.column >= SCREEN_WIDTH {
                    self.move_to_new_line();
                }
                let character = ScreenCharacter {
                    color_code: self.default_color,
                    character,
                };
                self.screen_buffer.set_character(self.row, self.column, character);
                self.column += 1;
            }
        }
    }

    fn move_to_new_line(&mut self) {
        self.row += 1;
        self.column = 0;

        if self.row >= SCREEN_HEIGHT {
            for target_row in 0..SCREEN_HEIGHT - 1 {
                self.screen_buffer.copy_row(target_row, target_row + 1);
            }
            let clear_character = ScreenCharacter {
                color_code: self.default_color,
                character: b' ',
            };
            self.screen_buffer.fill_row(SCREEN_HEIGHT - 1, clear_character);
            self.row = SCREEN_HEIGHT - 1;
        }
    }
}
