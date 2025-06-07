use std::fs::{OpenOptions};
use std::io::Write;
use chrono::Local;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Logger {
    file_path: String,
}

impl Logger {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn log(&self, level: LogLevel, message: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let timestamp = Local::now().format("%d.%m.%Y %H:%M:%S");
        writeln!(file, "[{:?}] {} - {}", level, timestamp, message)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

// Global logger instance
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger::new("log.txt".to_string()))
});

pub fn log_error_and_panic(msg: &str) -> ! {
    let logger = LOGGER.lock().expect("Failed to lock logger");
    let _ = logger.log(LogLevel::Error, msg);
    panic!("{}", msg);
}