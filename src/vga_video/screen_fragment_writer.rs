use alloc::boxed::Box;
use core::cell::RefCell;
use core::fmt::Write;

use crate::error::Error;
use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::vga_video::CharacterColor;
use crate::vga_video::frame_buffer::FrameBuffer;

pub struct ScreenFragmentWriter<'a> {
    rect: Rect,
    default_color: CharacterColor,
    frame_buffer: &'a RefCell<dyn FrameBuffer>,
    next_position: Point,
}

impl<'a> ScreenFragmentWriter<'a> {
    pub fn new(rect: Rect, default_color: CharacterColor, frame_buffer: &'a RefCell<dyn FrameBuffer>) -> Self {
        Self { rect, default_color, frame_buffer, next_position: rect.corner_upper_left() }
    }
}

impl Write for ScreenFragmentWriter<'_> {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        for char in string.chars() {
            if self.needs_scroll() {
                self.scroll_up().unwrap();
            }

            match char {
                '\n' => {
                    self.move_to_next_line();
                }
                _ => {
                    self.frame_buffer
                        .borrow_mut()
                        .set_char(self.next_position, char, self.default_color)
                        .unwrap();
                    self.move_to_next_position();
                }
            }
        }
        Ok(())
    }
}

impl ScreenFragmentWriter<'_> {
    fn move_to_next_position(&mut self) {
        self.next_position.x += 1;

        // Check if needed move to next line
        if !self.rect.contains(self.next_position) {
            self.move_to_next_line();
        }
    }

    fn move_to_next_line(&mut self) {
        self.next_position.x = self.rect.min_x();
        self.next_position.y += 1;
    }

    fn needs_scroll(&self) -> bool {
        self.next_position.y > self.rect.max_y()
    }

    fn scroll_up(&mut self) -> Result<(), Box<dyn Error>> {
        let rows = self.rect.min_y()..=self.rect.max_y();
        let columns = self.rect.min_x()..=self.rect.max_x();
        let mut frame_buffer = self.frame_buffer.borrow_mut();

        for row in rows.skip(1) {
            let source_row = row;
            let destination_row = row - 1;
            for column in columns.clone() {
                frame_buffer.copy_char(
                    Point::new(column, source_row),
                    Point::new(column, destination_row),
                )?;
            }
        }

        let last_row = self.rect.max_y();
        for column in columns {
            frame_buffer.set_char(
                Point::new(column, last_row),
                char::default(),
                CharacterColor::default(),
            )?;
        }
        self.next_position.y -= 1;
        Ok(())
    }
}

#[test_case]
fn test_write_short_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;
    use crate::geometry::size::Size;

    let frame_buffer = RefCell::new(
        MockFrameBuffer::new(80, 25)
    );
    let mut writer = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 3)),
        CharacterColor::default(),
        &frame_buffer,
    );

    writer.write_str("Abc").unwrap();

    let frame_buffer = frame_buffer.borrow();
    assert_eq!(frame_buffer.get_chars(0, 0, 5), ['\0'; 5]);
    assert_eq!(frame_buffer.get_chars(0, 1, 5), ['\0', 'A', 'b', 'c', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 5), ['\0'; 5]);
}

#[test_case]
fn test_write_multiline_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;
    use crate::geometry::size::Size;

    let frame_buffer = RefCell::new(
        MockFrameBuffer::new(80, 25)
    );
    let mut writer = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 3)),
        CharacterColor::default(),
        &frame_buffer,
    );

    writer.write_str("Lorem ipsu\
                             m dolor si\
                             t amet").unwrap();

    let frame_buffer = frame_buffer.borrow();
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
    use crate::geometry::size::Size;

    let frame_buffer = RefCell::new(
        MockFrameBuffer::new(80, 25)
    );
    let mut writer = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 3)),
        CharacterColor::default(),
        &frame_buffer,
    );

    writer.write_str("Lorem\nipsum").unwrap();

    let frame_buffer = frame_buffer.borrow();
    assert_eq!(frame_buffer.get_chars(0, 1, 12),
               ['\0', 'L', 'o', 'r', 'e', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 12),
               ['\0', 'i', 'p', 's', 'u', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
}

#[test_case]
fn test_write_text_with_scroll() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;
    use crate::geometry::size::Size;

    let frame_buffer = RefCell::new(
        MockFrameBuffer::new(80, 25)
    );
    let mut writer = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 3)),
        CharacterColor::default(),
        &frame_buffer,
    );

    writer.write_str("Lorem\nipsum\ndolor\nsit").unwrap();

    let frame_buffer = frame_buffer.borrow();
    assert_eq!(frame_buffer.get_chars(0, 1, 12),
               ['\0', 'i', 'p', 's', 'u', 'm', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 12),
               ['\0', 'd', 'o', 'l', 'o', 'r', '\0', '\0', '\0', '\0', '\0', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 3, 12),
               ['\0', 's', 'i', 't', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0']);
}

#[test_case]
fn test_write_two_fragments() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;
    use crate::geometry::size::Size;

    let frame_buffer = RefCell::new(
        MockFrameBuffer::new(80, 25)
    );

    let mut writer_1 = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 1), Size::new(10, 1)),
        CharacterColor::default(),
        &frame_buffer,
    );

    let mut writer_2 = ScreenFragmentWriter::new(
        Rect::new(Point::new(1, 3), Size::new(10, 1)),
        CharacterColor::default(),
        &frame_buffer,
    );

    writer_1.write_str("Lorem").unwrap();
    writer_2.write_str("ipsum").unwrap();

    let frame_buffer = frame_buffer.borrow();
    assert_eq!(frame_buffer.get_chars(0, 0, 7), ['\0'; 7]);
    assert_eq!(frame_buffer.get_chars(0, 1, 7), ['\0', 'L', 'o', 'r', 'e', 'm', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 2, 7), ['\0'; 7]);
    assert_eq!(frame_buffer.get_chars(0, 3, 7), ['\0', 'i', 'p', 's', 'u', 'm', '\0']);
    assert_eq!(frame_buffer.get_chars(0, 4, 7), ['\0'; 7]);
}
