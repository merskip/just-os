use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug)]
pub enum Level {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

#[derive(Debug)]
pub struct Log {
    pub level: Level,
    pub message: String,
}

pub struct Logger {
    logs: Vec<Log>,
    listeners: Vec<Box<dyn LoggerListener>>,
}

pub trait LoggerListener {
    fn did_log(&mut self, log: &Log);
}

unsafe impl Send for Logger {}

impl Logger {
    pub fn new(capability: usize) -> Self {
        Logger {
            logs: Vec::with_capacity(capability),
            listeners: Vec::new(),
        }
    }

    pub fn register_listener(&mut self, listener: Box<dyn LoggerListener>) {
        self.listeners.push(listener);
    }
}

impl Logger {
    pub fn debug(&mut self, message: &str) {
        self.log(Level::DEBUG, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(Level::INFO, message);
    }

    pub fn warning(&mut self, message: &str) {
        self.log(Level::WARNING, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(Level::ERROR, message);
    }

    fn log(&mut self, level: Level, message: &str) {
        let log = Log {
            level,
            message: message.to_string(),
        };

        for listener in &mut self.listeners {
            listener.did_log(&log);
        }
        
        self.logs.push(log);
    }
}

lazy_static! {
    pub static ref KERNEL_LOGGER: Mutex<Logger> =
        Mutex::new(Logger::new(512));
}

#[macro_export]
macro_rules! log {
    ($( $args:tt )*) => {{
        use x86_64::instructions::interrupts;
        use alloc::format;

        interrupts::without_interrupts(|| {
            let mut logger = $crate::log::KERNEL_LOGGER.lock();
            logger.info(format!($($args)*).as_str());
        });
    }};
}
