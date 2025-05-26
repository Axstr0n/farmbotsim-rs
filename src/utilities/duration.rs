use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}
impl TimeUnit {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "s" | "sec" | "second" | "seconds" => Some(TimeUnit::Seconds),
            "m" | "min" | "minute" | "minutes" => Some(TimeUnit::Minutes),
            "h" | "hr" | "hour" | "hours" => Some(TimeUnit::Hours),
            "d" | "day" | "days" => Some(TimeUnit::Days),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "s",
            TimeUnit::Minutes => "min",
            TimeUnit::Hours => "h",
            TimeUnit::Days => "d",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Duration {
    pub value: u32,
    pub unit: TimeUnit,
}

impl Duration {
    /// Convert the duration into total seconds.
    fn to_seconds(&self) -> u32 {
        match self.unit {
            TimeUnit::Seconds => self.value,
            TimeUnit::Minutes => self.value * 60,
            TimeUnit::Hours => self.value * 60*60,
            TimeUnit::Days => self.value * 60*60*24,
        }
    }

    /// Convenience constructor
    fn seconds(val: u32) -> Self {
        Duration { value: val, unit: TimeUnit::Seconds }
    }

    fn minutes(val: u32) -> Self {
        Duration { value: val, unit: TimeUnit::Minutes }
    }

    fn hours(val: u32) -> Self {
        Duration { value: val, unit: TimeUnit::Hours }
    }

    fn days(val: u32) -> Self {
        Duration { value: val, unit: TimeUnit::Days }
    }
}

// Serialize as "2 min"
impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{} {}", self.value, self.unit.as_str()))
    }
}

// Deserialize from "2 min"
impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid duration format"));
        }

        let value = parts[0].parse::<u32>().map_err(serde::de::Error::custom)?;
        let unit = TimeUnit::from_str(parts[1]).ok_or_else(|| {
            serde::de::Error::custom(format!("Unknown time unit: {}", parts[1]))
        })?;

        Ok(Duration { value, unit })
    }
}
