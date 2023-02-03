use core::fmt::Write;
use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::vga_video::{CharacterColor};
use crate::vga_video::frame_buffer::FrameBuffer;

struct ScreenFragmentWriter<'a> {
    rect: Rect,
    default_color: CharacterColor,
    frame_buffer: &'a mut dyn FrameBuffer,
    next_position: Point
}

impl<'a> ScreenFragmentWriter<'a> {
    pub fn new(rect: Rect, default_color: CharacterColor, frame_buffer: &'a mut dyn FrameBuffer) -> Self {
        Self { rect, default_color, frame_buffer, next_position: rect.corner_upper_left() }
    }
}

impl Write for ScreenFragmentWriter<'_> {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        let mut position = self.next_position;
        for char in string.chars() {

            self.frame_buffer.set_char(position, char, self.default_color)
                .expect("TODO: panic message");

            position.x += 1;
            if !self.rect.contains(position) {
                position.x = self.rect.min_x();
                position.y += 1
            }

            if !self.rect.contains(position) {
                panic!("Out of bounds!");
            }
        }
        self.next_position = position;
        Ok(())
    }
}

#[test_case]
fn test_write_short_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;

    let mut frame_buffer= MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Abc").unwrap();

    assert_eq!(frame_buffer.get_character(1, 1), 'A');
    assert_eq!(frame_buffer.get_character(2, 1), 'b');
    assert_eq!(frame_buffer.get_character(3, 1), 'c');
}

#[cfg(test)]
fn new_screen_fragment_writer(frame_buffer: &mut dyn FrameBuffer) -> ScreenFragmentWriter {
    use crate::geometry::size::Size;

    ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 10)),
        CharacterColor::default(),
        frame_buffer,
    )
}
