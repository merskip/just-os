use alloc::vec;
use core::fmt::Write;
use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::serial_println;
use crate::vga_video::{CharacterColor};
use crate::vga_video::frame_buffer::FrameBuffer;

struct ScreenFragmentWriter<'a> {
    rect: Rect,
    default_color: CharacterColor,
    frame_buffer: &'a mut dyn FrameBuffer,
    next_position: Point,
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
                .unwrap();

            position.x += 1;
            if !self.rect.contains(position) {
                position.x = self.rect.min_x();
                position.y += 1;
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

    let mut frame_buffer = MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Abc").unwrap();

    assert_eq!(frame_buffer.get_chars(0, 0, 5), vec!['\0'; 5]);
    assert_eq!(frame_buffer.get_chars(0, 1, 5),
               ['\0', 'A', 'b', 'c', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 5), vec!['\0'; 5]);
}

#[test_case]
fn test_write_multiline_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;

    let mut frame_buffer = MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Lorem ipsu\
                             m dolor si\
                             t amet").unwrap();

    assert_eq!(frame_buffer.get_chars(0, 1, 12),
               ['\0', 'L', 'o', 'r', 'e', 'm', ' ', 'i', 'p', 's', 'u', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 12),
               ['\0', 'm', ' ', 'd', 'o', 'l', 'o', 'r', ' ', 's', 'i', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 3, 12),
               ['\0', 't', ' ', 'a', 'm', 'e', 't', '\0', '\0', '\0', '\0', '\0']);
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
