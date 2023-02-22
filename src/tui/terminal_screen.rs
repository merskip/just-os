use alloc::fmt::format;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use core::fmt::Write;
use spin::Mutex;

use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;
use crate::rtc::RTC;
use crate::serial_println;
use crate::vga_video::{CharacterColor, Color};
use crate::vga_video::frame_buffer::FrameBuffer;
use crate::vga_video::screen_fragment_writer::ScreenFragmentWriter;

pub struct Header {
    name: String,
    version: String,
}

impl Header {
    pub fn new(name: String, version: String) -> Self {
        Header { name, version }
    }
}

pub struct TerminalScreen<'a> {
    header_writer: ScreenFragmentWriter<'a>,
    header: Header,
    rtc: Rc<Mutex<RTC>>,
    body_writer: ScreenFragmentWriter<'a>,
}

impl<'a> TerminalScreen<'a> {
    pub fn new(
        screen_buffer: &'a RefCell<dyn FrameBuffer>,
        header: Header,
        rtc: Rc<Mutex<RTC>>,
    ) -> Self {
        let screen_size = screen_buffer.borrow().get_size();
        let header_writer = ScreenFragmentWriter::new(
            Rect::new(Point::new(0, 0), Size::new(screen_size.width, 1)),
            CharacterColor::default(),
            screen_buffer,
        );

        let body_writer = ScreenFragmentWriter::new(
            Rect::new(Point::new(0, 2), Size::new(screen_size.width, screen_size.width - 1)),
            CharacterColor::default(),
            screen_buffer,
        );

        Self {
            header_writer,
            header,
            rtc,
            body_writer,
        }
    }
}

impl TerminalScreen<'_> {
    pub fn refresh_header(&mut self) {
        let now = self.rtc.lock().read_datetime();
        self.display_header(
            "",
            &*format!("{} (ver. {})", self.header.name, self.header.version),
            &*now.to_string(),
        )
    }

    fn display_header(&mut self, left: &str, center: &str, right: &str) {
        let total_width = self.header_writer.get_size().width;
        let center_width = total_width - left.len() - right.len();
        let header = format!("{:<}{:^width$}{:>}",
                             left, center, right, width = center_width);

        self.header_writer.reset_cursor();
        self.header_writer.write_str(&*header).unwrap();
    }
}
