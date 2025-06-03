use chrono::{Datelike, Duration, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DateTimeManager {
    pub config: DateTimeConfig,
    pub current_time: NaiveDateTime,
}

pub const DATE_FORMAT: &str = "%d.%m.%Y";
pub const TIME_FORMAT: &str = "%H:%M:%S";
pub const DATETIME_FORMAT: &str = "%d.%m.%Y %H:%M:%S";

impl DateTimeManager {
    pub fn from_config(config: DateTimeConfig) -> Self {
        let combined = format!("{} {}", config.date, config.time);

        let dt = NaiveDateTime::parse_from_str(&combined, DATETIME_FORMAT)
            .unwrap_or_else(|_| {
                eprintln!("Warning: Failed to parse datetime '{}', falling back to Local::now()", combined);
                Local::now().naive_local()
            });

        DateTimeManager {
            config,
            current_time: dt,
        }
    }
    
    pub fn advance_time(&mut self, seconds: i64) {
        self.current_time += Duration::seconds(seconds);
    }
    
    pub fn get_time(&self) -> String {
        self.current_time.format(DATETIME_FORMAT).to_string()
    }
    
    pub fn get_month(&self) -> u32 {
        self.current_time.month()
    }
    
    pub fn reset(&mut self) {
        let combined = format!("{} {}", self.config.date, self.config.time);
        // Try to parse the provided date string
        self.current_time = NaiveDateTime::parse_from_str(&combined, DATETIME_FORMAT)
            .unwrap_or_else(|_| {
                // If parsing fails, use current time
                Local::now().naive_local()
            });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeConfig {
    pub date: String,
    pub time: String,
}
impl DateTimeConfig {
    pub fn from_string(datetime_str: String) -> Self {
        let dt = NaiveDateTime::parse_from_str(&datetime_str, DATETIME_FORMAT).expect("Couldn't parse datetime into chrono.");
        let date = dt.format(DATE_FORMAT).to_string();
        let time = dt.format(TIME_FORMAT).to_string();
        Self {
            date,
            time,
        }
    }
}
