use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, enum_iterator::Sequence)]
pub enum DurationUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl DurationUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            DurationUnit::Seconds => "s",
            DurationUnit::Minutes => "min",
            DurationUnit::Hours => "h",
            DurationUnit::Days => "days",
        }
    }
}
impl FromStr for DurationUnit {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "s" | "sec" | "second" | "seconds" => Ok(DurationUnit::Seconds),
            "min" | "minute" | "minutes" => Ok(DurationUnit::Minutes),
            "h" | "hr" | "hour" | "hours" => Ok(DurationUnit::Hours),
            "d" | "day" | "days" => Ok(DurationUnit::Days),
            _ => Err(format!("Unknown DurationUnit: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Duration {
    pub value: f32,
    pub unit: DurationUnit,
}

impl Duration {
    pub const fn new(value: f32, unit: DurationUnit) -> Self {
        Self { value, unit }
    }
    pub const fn seconds(value: f32) -> Self {
        Self::new(value, DurationUnit::Seconds)
    }
    pub const fn minutes(value: f32) -> Self {
        Self::new(value, DurationUnit::Minutes)
    }
    pub const fn hours(value: f32) -> Self {
        Self::new(value, DurationUnit::Hours)
    }
    pub const fn days(value: f32) -> Self {
        Self::new(value, DurationUnit::Days)
    }

    pub fn to_hour(self) -> f32 {
        match self.unit {
            DurationUnit::Seconds => self.value / 3600.0,
            DurationUnit::Minutes => self.value / 60.0,
            DurationUnit::Hours => self.value,
            DurationUnit::Days => self.value * 24.0,
        }
    }

    pub fn to_base_unit(self) -> f32 {
        match self.unit {
            DurationUnit::Seconds => self.value,
            DurationUnit::Minutes => self.value * 60.0,
            DurationUnit::Hours => self.value * 3600.0,
            DurationUnit::Days => self.value * 86400.0,
        }
    }
    pub fn from_base_unit(value: f32, unit: DurationUnit) -> Self {
        match unit {
            DurationUnit::Seconds => Self::new(value, unit),
            DurationUnit::Minutes => Self::new(value / 60.0, unit),
            DurationUnit::Hours => Self::new(value / 3600.0, unit),
            DurationUnit::Days => Self::new(value / 86400.0, unit),
        }
    }
}

pub fn format_duration(dur: &Duration) -> String {
    let secs = dur.to_base_unit();
    if secs >= 3600.0 {
        format!("{} h", secs / 3600.0)
    } else if secs >= 60.0 {
        format!("{} min", secs / 60.0)
    } else {
        format!("{} s", secs)
    }
}

pub fn average_duration(durations: &[Duration]) -> Duration {
    if durations.is_empty() {
        return Duration::seconds(0.0);
    }
    let total_secs: f32 = durations.iter().map(|d| d.to_base_unit()).sum();
    Duration::seconds(total_secs / durations.len() as f32)
}

impl Eq for Duration {}
impl Ord for Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}