use alloc::format;
use alloc::string::String;

use crate::geometry::position::Point;
use crate::log::{Log, LoggerListener};
use crate::vga_video::{CharacterColor, Color};

pub struct Header {
    name: String,
    version: String
}

impl Header {
    pub fn new(name: String, version: String) -> Self {
        Header { name, version }
    }
}

pub struct TextScreen<'a> {
    screen_writer: ScreenWriter<'a>,
    default_color: CharacterColor,
    header: Header,
    next_log_position: Point,
}

impl <'a> TextScreen<'a> {
    pub fn new(
        screen_writer: ScreenWriter<'a>,
        default_color: CharacterColor,
        header: Header,
    ) -> Self {
        TextScreen {
            screen_writer,
            default_color,
            header,
            next_log_position: Point::new(0, 2),
        }
    }
}

impl LoggerListener for TextScreen<'_> {
    fn did_log(&mut self, log: &Log) {
        self.display_log(log);
    }
}

impl TextScreen<'_> {

    pub fn display(&mut self) {
        self.screen_writer.clear();
        self.display_header();

    }

    fn display_header(&mut self) {
        self.screen_writer.write_string(
            Point::default(),
            format!("{} (v{})", self.header.name, self.header.version).as_str(),
            self.default_color.with_foreground(Color::Cyan),
        );
        self.screen_writer.write_string(
            Point::new(0, 1),
            "=== [Logs] === ",
            self.default_color
        );

        self.screen_writer.set_frozen_rows(2);
    }

    fn display_log(&mut self, log: &Log) {
        let position = self.screen_writer.write_string(
            self.next_log_position,
            format!("[{:?}] ", log.level).as_str(),
            self.default_color.with_foreground(Color::DarkGray)
        );

        let position = self.screen_writer.write_string(
            position,
            log.message.as_str(),
            self.default_color
        );

        self.next_log_position = position.next_row();
    }
}
