use x86_64::instructions::port::Port;
use crate::geometry::position::Point;
use crate::geometry::size::Size;
use crate::vga_video::vga_frame_buffer::VGA_SCREEN_SIZE;

pub enum CursorStyle {
    Underline
}

pub trait Cursor {
    fn enable(&mut self, style: CursorStyle);

    fn disable(&mut self);

    fn move_to(&mut self, position: Point);
}

pub struct VgaCursor {
    control_port: Port<u8>,
    data_port: Port<u8>,
}

const VGA_LOW_BYTE: u8 = 0x0f;
const VGA_HIGH_BYTE: u8 = 0x0e;

impl VgaCursor {
    pub fn new() -> Self {
        Self {
            control_port: Port::new(0x3d4),
            data_port: Port::new(0x3d5),
        }
    }

    fn get_index(&self, position: Point) -> usize {
        let size = self.get_size();
        position.y * size.width + position.x
    }

    fn get_size(&self) -> Size {
        VGA_SCREEN_SIZE
    }
}

impl Cursor for VgaCursor {
    fn enable(&mut self, style: CursorStyle) {
        match style {
            CursorStyle::Underline => unsafe {
                self.control_port.write(0x0A);
                let low = self.data_port.read();
                self.data_port.write(low & 0xC0 | 12);

                self.control_port.write(0x0B);
                let high = self.data_port.read();
                self.data_port.write(high & 0xE0 | 14);
            }
        }
    }

    fn disable(&mut self) {
        unsafe {
            self.control_port.write(0x0A);
            self.data_port.write(0x20);
        }
    }

    fn move_to(&mut self, position: Point) {
        let index = self.get_index(position);
        let index_bytes = index.to_le_bytes();
        unsafe {
            self.control_port.write(VGA_LOW_BYTE);
            self.data_port.write(index_bytes[0]);
            self.control_port.write(VGA_HIGH_BYTE);
            self.data_port.write(index_bytes[1]);
        }
    }
}