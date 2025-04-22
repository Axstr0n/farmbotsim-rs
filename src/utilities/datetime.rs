use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeManager {
    current_time: DateTime<Local>,
}

impl DateTimeManager {
    pub fn new(start_date: &str) -> Self {
        // Try to parse the provided date string
        let dt = NaiveDateTime::parse_from_str(start_date, "%d.%m.%Y %H:%M:%S")
            .map(|naive_dt| DateTime::from_naive_utc_and_offset(naive_dt, *Local::now().offset()))
            .unwrap_or_else(|_| {
                // If parsing fails, use current UTC time converted to local
                Utc::now().with_timezone(&Local)
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
            .map(|naive_dt| DateTime::from_naive_utc_and_offset(naive_dt, *Local::now().offset()))
            .unwrap_or_else(|_| {
                // If parsing fails, use current UTC time converted to local
                Utc::now().with_timezone(&Local)
            });
    }
}