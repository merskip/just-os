use alloc::boxed::Box;
use core::fmt::Write;
use crate::error::Error;
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
            if self.needs_scroll(position) {
                position = self.scroll_up(position).unwrap();
            }

            match char {
                '\n' => {
                    position = self.next_line(position);
                }
                _ => {
                    self.frame_buffer.set_char(position, char, self.default_color)
                        .unwrap();
                    position = self.next_position(position);
                }
            }
        }
        self.next_position = position;
        Ok(())
    }
}

impl<'a> ScreenFragmentWriter<'a> {
    fn next_position(&self, mut position: Point) -> Point {
        position.x += 1;

        // Check if needed move to next line
        if !self.rect.contains(position) {
            position = self.next_line(position);
        }
        position
    }

    fn next_line(&self, mut position: Point) -> Point {
        position.x = self.rect.min_x();
        position.y += 1;
        position
    }

    fn needs_scroll(&self, position: Point) -> bool {
        position.y > self.rect.max_y()
    }

    fn scroll_up(&mut self, mut position: Point) -> Result<Point, Box<dyn Error>> {
        let rows = self.rect.min_y()..=self.rect.max_y();
        let columns = self.rect.min_x()..=self.rect.max_x();

        for row in rows.skip(1) {
            let source_row = row;
            let destination_row = row - 1;
            for column in columns.clone() {
                self.frame_buffer.copy_char(
                    Point::new(column, source_row),
                    Point::new(column, destination_row),
                )?;
            }
        }

        let last_row = self.rect.max_y();
        for column in columns {
            self.frame_buffer.set_char(
                Point::new(column, last_row),
                char::default(),
                CharacterColor::default(),
            )?;
        }
        position.y -= 1;
        Ok(position)
    }
}

#[test_case]
fn test_write_short_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;

    let mut frame_buffer = MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Abc").unwrap();

    assert_eq!(frame_buffer.get_chars(0, 0, 5), ['\0'; 5]);
    assert_eq!(frame_buffer.get_chars(0, 1, 5), ['\0', 'A', 'b', 'c', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 5), ['\0'; 5]);
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

#[test_case]
fn test_write_text_with_new_line() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;

    let mut frame_buffer = MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Lorem\nipsum").unwrap();

    assert_eq!(frame_buffer.get_chars(0, 1, 12),
               ['\0', 'L', 'o', 'r', 'e', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 12),
               ['\0', 'i', 'p', 's', 'u', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
}

#[test_case]
fn test_write_text_with_scroll() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;

    let mut frame_buffer = MockFrameBuffer::new(80, 25);
    let mut writer = new_screen_fragment_writer(&mut frame_buffer);

    writer.write_str("Lorem\nipsum\ndolor\nsit").unwrap();

    assert_eq!(frame_buffer.get_chars(0, 1, 12),
               ['\0', 'i', 'p', 's', 'u', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 12),
               ['\0', 'd', 'o', 'l', 'o', 'r', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 3, 12),
               ['\0', 's', 'i', 't', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0']);
}

#[cfg(test)]
fn new_screen_fragment_writer(frame_buffer: &mut dyn FrameBuffer) -> ScreenFragmentWriter {
    use crate::geometry::size::Size;

    ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 3)),
        CharacterColor::default(),
        frame_buffer,
    )
}
