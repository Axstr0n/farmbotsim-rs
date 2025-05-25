use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedUnit {
    MetersPerSecond,
    KilometersPerHour,
}
impl FromStr for SpeedUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "m/s" | "mps" => Ok(SpeedUnit::MetersPerSecond),
            "km/h" | "kmph" => Ok(SpeedUnit::KilometersPerHour),
            _ => Err(format!("Undefined SpeedUnit: {}", s)),
        }
    }
}
impl SpeedUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpeedUnit::MetersPerSecond => "m/s",
            SpeedUnit::KilometersPerHour => "km/h",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub value: f32,
    pub unit: SpeedUnit,
}

impl Velocity {
    pub fn to_meters_per_second(&self) -> f32 {
        match self.unit {
            SpeedUnit::MetersPerSecond => self.value,
            SpeedUnit::KilometersPerHour => self.value * 1000.0 / 3600.0,
        }
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.as_str())
    }
}

impl FromStr for Velocity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut parts = s.split_whitespace();
        let value_part = parts.next().ok_or("Missing value")?;
        let unit_part = parts.next().ok_or("Missing unit")?;

        let value = value_part.parse::<f32>().map_err(|_| "Invalid number")?;
        let unit = SpeedUnit::from_str(unit_part)?;

        Ok(Velocity { value, unit })
    }
}

impl Serialize for Velocity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Velocity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
