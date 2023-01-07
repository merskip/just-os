#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightPurple = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CharacterColor(u8);

impl CharacterColor {
    pub const fn new(foreground: Color, background: Color) -> CharacterColor {
        CharacterColor((background as u8) << 4 | (foreground as u8))
    }
}
