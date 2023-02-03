use core::fmt::Write;
use crate::geometry::rect::Rect;
use crate::vga_video::frame_buffer::FrameBuffer;

struct ScreenFragmentWriter<'a> {
    rect: Rect,
    frame_buffer: &'a mut dyn FrameBuffer,
}

impl<'a> ScreenFragmentWriter<'a> {
    pub fn new(rect: Rect, frame_buffer: &'a mut dyn FrameBuffer) -> Self {
        Self { rect, frame_buffer }
    }
}

impl Write for ScreenFragmentWriter<'_> {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        todo!()
    }
}

#[test_case]
fn test_write_short_text() {
    use crate::vga_video::mock_frame_buffer::MockFrameBuffer;
    use crate::geometry::size::Size;
    use crate::geometry::position::Position;

    let mut frame_buffer= MockFrameBuffer::new(80, 25);
    let mut writer = ScreenFragmentWriter::new(
        Rect::new(Position::new(1, 1), Size::new(10, 100)),
        &mut frame_buffer,
    );

    writer.write_str("Hello world!").unwrap();

    assert_eq!(frame_buffer.get_character(1, 1), 'H');
    assert_eq!(frame_buffer.get_character(1, 2), 'e');
    assert_eq!(frame_buffer.get_character(1, 3), 'l');
    assert_eq!(frame_buffer.get_character(1, 4), 'l');
    assert_eq!(frame_buffer.get_character(1, 5), 'o');
    assert_eq!(frame_buffer.get_character(1, 6), ' ');
    assert_eq!(frame_buffer.get_character(1, 7), 'w');
    assert_eq!(frame_buffer.get_character(1, 8), 'o');
    assert_eq!(frame_buffer.get_character(1, 9), 'r');
    assert_eq!(frame_buffer.get_character(1, 10), 'l');
    assert_eq!(frame_buffer.get_character(1, 11), 'd');
    assert_eq!(frame_buffer.get_character(1, 12), '!');
}
