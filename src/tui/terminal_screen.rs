use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt::Write;
use pc_keyboard::{DecodedKey};
use spin::Mutex;

use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;
use crate::rtc::RTC;
use crate::serial_println;
use crate::vga_video::{CharacterColor};
use crate::vga_video::cursor::{Cursor, CursorStyle};
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
    prompt: String,
    cursor: Rc<Mutex<dyn Cursor>>,
    command_buffer: Vec<char>
}

impl<'a> TerminalScreen<'a> {
    pub fn new(
        screen_buffer: &'a RefCell<dyn FrameBuffer>,
        header: Header,
        rtc: Rc<Mutex<RTC>>,
        prompt: String,
        cursor: Rc<Mutex<dyn Cursor>>,
    ) -> Self {
        let screen_size = screen_buffer.borrow().get_size();
        let header_writer = ScreenFragmentWriter::new(
            Rect::new(Point::new(0, 0), Size::new(screen_size.width, 1)),
            CharacterColor::default(),
            screen_buffer,
        );

        let body_writer = ScreenFragmentWriter::new(
            Rect::new(Point::new(0, 1), Size::new(screen_size.width, screen_size.height - 1)),
            CharacterColor::default(),
            screen_buffer,
        );

        Self {
            header_writer,
            header,
            rtc,
            body_writer,
            prompt,
            cursor,
            command_buffer: Vec::with_capacity(255),
        }
    }

    pub fn begin(&mut self) {
        self.refresh_header();

        self.body_writer.clear();
        self.cursor.lock().enable(CursorStyle::Underline);
        self.display_prompt();
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

        self.header_writer.reset_position();
        self.header_writer.write_str(&*header).unwrap();
    }
}

impl TerminalScreen<'_> {
    pub fn handle_keypress(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::Unicode(character) => match character {
                '\x08' => { // Backspace
                    self.body_writer.write_char(character).unwrap();
                    self.command_buffer.pop();
                },
                '\n' => { // Carriage Return
                    self.body_writer.write_char(character).unwrap();

                    self.process_command(self.command_buffer.clone());
                    self.command_buffer.clear();
                    self.display_prompt();
                },
                _ => {
                    self.body_writer.write_char(character).unwrap();
                    self.command_buffer.push(character);
                },
            },
            DecodedKey::RawKey(key) => {
                serial_println!("KEYBOARD KEY_CODE={:?}", key);
            },
        }
        self.refresh_cursor();
    }

    fn display_prompt(&mut self) {
        self.body_writer.write_str(&*self.prompt).unwrap();
        self.refresh_cursor();
    }

    fn refresh_cursor(&mut self) {
        let next_position = self.body_writer.get_next_position();
        self.cursor.lock().move_to(next_position);
    }

    fn process_command(&mut self, command: Vec<char>) {
        let command: String = command.iter().collect();
        writeln!(self.body_writer, "TODO: Process command: {:?}", command).unwrap();
    }
}
