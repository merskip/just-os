use alloc::boxed::Box;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{RefCell};
use core::fmt::Write;

use pc_keyboard::DecodedKey;
use spin::Mutex;

use crate::command::command::Command;
use crate::geometry::position::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;
use crate::rtc::RTC;
use crate::serial_println;
use crate::vga_video::CharacterColor;
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
    header_writer: RefCell<ScreenFragmentWriter<'a>>,
    header: Header,
    rtc: Rc<Mutex<RTC>>,
    body_writer: Rc<RefCell<ScreenFragmentWriter<'a>>>,
    prompt: String,
    cursor: Rc<Mutex<dyn Cursor>>,
    command_buffer: RefCell<Vec<char>>,
    command_handler: Box<dyn Fn(Command)>,
}

impl<'a> TerminalScreen<'a> {
    pub fn new(
        screen_buffer: &'a RefCell<dyn FrameBuffer>,
        header: Header,
        rtc: Rc<Mutex<RTC>>,
        prompt: String,
        cursor: Rc<Mutex<dyn Cursor>>,
        command_handler: Box<dyn Fn(Command)>,
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
            header_writer: RefCell::new(header_writer),
            header,
            rtc,
            body_writer: Rc::new(RefCell::new(body_writer)),
            prompt,
            cursor,
            command_buffer: RefCell::new(Vec::with_capacity(255)),
            command_handler,
        }
    }

    pub fn begin(&mut self) {
        self.refresh_header();

        (*self.body_writer).borrow_mut().clear();
        self.cursor.lock().enable(CursorStyle::Underline);
        self.display_prompt();
    }

    pub fn get_standard_output(&self) -> Rc<RefCell<dyn Write + 'a>> {
        return self.body_writer.clone();
    }
}

impl TerminalScreen<'_> {
    pub fn refresh_header(&self) {
        let now = self.rtc.lock().read_datetime();
        self.display_header(
            "",
            &*format!("{} (ver. {})", self.header.name, self.header.version),
            &*now.to_string(),
        )
    }

    fn display_header(&self, left: &str, center: &str, right: &str) {
        let mut header_writer = self.header_writer.borrow_mut();

        let total_width = header_writer.get_size().width;
        let center_width = total_width - left.len() - right.len();
        let header = format!("{:<}{:^width$}{:>}",
                             left, center, right, width = center_width);

        header_writer.reset_position();
        header_writer.write_str(&*header).unwrap();
    }
}

impl TerminalScreen<'_> {
    pub fn handle_keypress(&self, key: DecodedKey) {
        match key {
            DecodedKey::Unicode(character) => match character {
                '\x08' => { // Backspace
                    if let Some(_) = self.command_buffer.borrow_mut().pop() {
                        self.write_body_char(character);
                    }
                }
                '\n' => { // Carriage Return
                    self.write_body_char(character);

                    self.process_command_text(self.command_buffer.borrow().iter().collect());
                    self.command_buffer.borrow_mut().clear();
                    self.display_prompt();
                }
                _ => {
                    self.write_body_char(character);
                    self.command_buffer.borrow_mut().push(character);
                }
            },
            DecodedKey::RawKey(key) => {
                serial_println!("KEYBOARD KEY_CODE={:?}", key);
            }
        }
    }

    pub fn display_prompt(&self) {
        self.write_body_str(&*self.prompt);
    }

    fn write_body_str(&self, str: &str) {
        let mut body_writer = (*self.body_writer).borrow_mut();

        body_writer.write_str(str).unwrap();

        let next_position = body_writer.get_next_position();
        self.cursor.lock().move_to(next_position);
    }

    fn write_body_char(&self, char: char) {
        let mut body_writer = (*self.body_writer).borrow_mut();

        body_writer.write_char(char).unwrap();

        let next_position = body_writer.get_next_position();
        self.cursor.lock().move_to(next_position);
    }

    fn process_command_text(&self, command_text: String) {
        let command = Command::parse(command_text);
        if let Some(command) = command {
            (self.command_handler)(command)
        }
    }
}
