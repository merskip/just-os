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

#[doc(hidden)]
pub fn log(level: Level, message: &str) {
    use x86_64::instructions::interrupts;
    
    interrupts::without_interrupts(|| {
        let mut logger = KERNEL_LOGGER.lock();
        logger.log(level, message);
    });
}

#[macro_export]
macro_rules! log_debug {
    ($($args:tt)*) => {{
        use alloc::format;
        $crate::log::log($crate::log::Level::DEBUG, format!($($args)*).as_str());
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($args:tt)*) => {{
        use alloc::format;
        $crate::log::log($crate::log::Level::INFO, format!($($args)*).as_str());
    }};
}

#[macro_export]
macro_rules! log_warning {
    ($($args:tt)*) => {{
        use alloc::format;
        $crate::log::log($crate::log::Level::WARNING, format!($($args)*).as_str());
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($args:tt)*) => {{
        use alloc::format;
        $crate::log::log($crate::log::Level::ERROR, format!($($args)*).as_str());
    }};
}
