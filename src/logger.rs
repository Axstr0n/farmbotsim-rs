use std::fs::{OpenOptions};
use std::io::Write;
use chrono::Local;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// A file-based logger that writes timestamped log messages to a file.
#[derive(Debug)]
pub struct Logger {
    /// Path to the log file.
    file_path: String,
}

impl Logger {
    /// Creates a new `Logger` with the specified file path.
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    /// Logs a message with a specified log level to the file.
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

/// Represents the severity level of a log message.
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// A globally accessible logger instance, protected by a `Mutex`.
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger::new("log.txt".to_string()))
});

/// Logs an error message and panics with the same message.
pub fn log_error_and_panic(msg: &str) -> ! {
    let logger = LOGGER.lock().unwrap_or_else(|e| {
        panic!("Failed to acquired LOGGER mutex: {}", e);
    });
    let _ = logger.log(LogLevel::Error, msg);
    panic!("{}", msg);
}