use chrono::{Datelike, Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::logger::log_error_and_panic;

/// Manages date and time based on a configuration.
#[derive(Debug, Clone)]
pub struct DateTimeManager {
    pub config: DateTimeConfig,
    pub current_time: NaiveDateTime,
}

pub const DATE_FORMAT: &str = "%d.%m.%Y";
pub const TIME_FORMAT: &str = "%H:%M:%S";
pub const DATETIME_FORMAT: &str = "%d.%m.%Y %H:%M:%S";

impl DateTimeManager {
    /// Creates a new `DateTimeManager` from `DateTimeConfig`.
    /// Panics if parsing fails.
    pub fn from_config(config: DateTimeConfig) -> Self {
        let combined = format!("{} {}", config.date, config.time);

        let dt = NaiveDateTime::parse_from_str(&combined, DATETIME_FORMAT).unwrap_or_else(|e| {
            let msg = format!(
                "Failed to parse datetime '{combined}' with format '{DATETIME_FORMAT}': {e}"
            );
            log_error_and_panic(&msg)
        });

        DateTimeManager {
            config,
            current_time: dt,
        }
    }
    /// Advances the current time by the specified number of seconds.
    pub fn advance_time(&mut self, seconds: i64) {
        self.current_time += Duration::seconds(seconds);
    }
    /// Returns the current date and time formatted as a string.
    pub fn get_time(&self) -> String {
        self.current_time.format(DATETIME_FORMAT).to_string()
    }
    /// Returns the current month (1-12).
    pub fn get_month(&self) -> u32 {
        self.current_time.month()
    }
    /// Resets the current time to the initial configured date and time.
    /// Panics if parsing fails.
    pub fn reset(&mut self) {
        let combined = format!("{} {}", self.config.date, self.config.time);
        self.current_time = NaiveDateTime::parse_from_str(&combined, DATETIME_FORMAT)
            .unwrap_or_else(|e| {
                let msg = format!(
                    "Failed to parse datetime '{combined}' with format '{DATETIME_FORMAT}': {e}"
                );
                log_error_and_panic(&msg)
            });
    }
}

/// Configuration for date and time used by `DateTimeManager`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeConfig {
    /// Date string (format "dd.mm.yyyy").
    pub date: String,
    /// Time string (format "HH:MM:SS").
    pub time: String,
}
impl DateTimeConfig {
    /// Creates a `DateTimeConfig` from a single combined datetime string.
    /// Panics if parsing fails.
    pub fn from_string(datetime_str: String) -> Self {
        let dt = NaiveDateTime::parse_from_str(&datetime_str, DATETIME_FORMAT).unwrap_or_else(|e| {
            let msg = format!("Failed to parse datetime '{datetime_str}' with format '{DATETIME_FORMAT}': {e}");
            log_error_and_panic(&msg)
        });
        let date = dt.format(DATE_FORMAT).to_string();
        let time = dt.format(TIME_FORMAT).to_string();
        Self { date, time }
    }
}
