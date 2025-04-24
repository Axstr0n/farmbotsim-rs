use chrono::{Datelike, Duration, Local, NaiveDateTime};

#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeManager {
    current_time: NaiveDateTime,
}

impl DateTimeManager {
    pub fn new(start_date: &str) -> Self {
        // Try to parse the provided date string
        let dt = NaiveDateTime::parse_from_str(start_date, "%d.%m.%Y %H:%M:%S")
            .unwrap_or_else(|_| {
                // If parsing fails, use current time
                Local::now().naive_local()
            });
        DateTimeManager {
            current_time: dt,
        }
    }
    
    pub fn advance_time(&mut self, seconds: i64) {
        self.current_time += Duration::seconds(seconds);
    }
    
    pub fn get_time(&self) -> String {
        self.current_time.format("%d.%m.%Y %H:%M:%S").to_string()
    }
    
    pub fn get_month(&self) -> u32 {
        self.current_time.month()
    }
    
    pub fn reset(&mut self, start_date: &str) {
        // Try to parse the provided date string
        self.current_time = NaiveDateTime::parse_from_str(start_date, "%d.%m.%Y %H:%M:%S")
            .unwrap_or_else(|_| {
                // If parsing fails, use current time
                Local::now().naive_local()
            });
    }
}